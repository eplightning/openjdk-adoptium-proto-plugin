use crate::adoptium_api::{fetch_release_asset, fetch_release_versions};
use extism_pdk::*;
use proto_pdk::*;
use rustc_hash::FxHashMap;

static NAME: &str = "Eclipse Adoptium OpenJDK";

static JDK_BINS: [&str; 5] = ["java", "javac", "javadoc", "jar", "keytool"];

fn semver_to_release(semver: &Version) -> String {
    if semver.major >= 9 {
        match (semver.major, semver.minor, semver.patch, &semver.build) {
            (major, 0, 0, build) => format!("jdk-{}+{}", major, build),
            (major, minor, 0, build) => format!("jdk-{}.{}+{}", major, minor, build),
            (major, minor, patch, build) => format!("jdk-{}.{}.{}+{}", major, minor, patch, build),
        }
    } else {
        // jdk8u412-b08
        if semver.build.len() == 1 {
            format!("jdk{}u{}-b0{}", semver.major, semver.patch, semver.build)
        } else {
            format!("jdk{}u{}-b{}", semver.major, semver.patch, semver.build)
        }
    }
}

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        minimum_proto_version: Some(Version::new(0, 46, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = input
        .context
        .version
        .as_version()
        .ok_or(PluginError::Message("Unsupported version type.".into()))?;

    let env = get_host_environment()?;
    let release = semver_to_release(version);

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations![
            HostOS::Linux => [
                HostArch::X64,
                HostArch::Arm64,
                HostArch::Arm,
                HostArch::Riscv64,
                HostArch::S390x
            ],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X86, HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let asset = fetch_release_asset(&env, &release)?;
    let binary = asset
        .binaries
        .into_iter()
        .next()
        .ok_or(PluginError::Message("API returned no binaries.".into()))?;

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(release.clone()),
        checksum: Some(Checksum {
            algo: ChecksumAlgorithm::Sha256,
            hash: Some(binary.package.checksum),
            key: None,
        }),
        download_url: binary.package.link,
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    let exes: FxHashMap<String, _> = JDK_BINS
        .into_iter()
        .map(|bin| {
            let exe_name = env.os.get_exe_name(bin);
            let exe_path = match env.os {
                HostOS::MacOS => format!("Contents/Home/bin/{exe_name}"),
                _ => format!("bin/{exe_name}"),
            };

            let config = match bin {
                "java" => ExecutableConfig::new_primary(exe_path),
                _ => ExecutableConfig::new(exe_path),
            };

            (String::from(bin), config)
        })
        .collect();

    let exes_dirs = match env.os {
        HostOS::MacOS => vec!["Contents/Home/bin".into()],
        _ => vec!["bin".into()],
    };

    Ok(Json(LocateExecutablesOutput {
        exes_dirs,
        exes,
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let env = get_host_environment()?;
    let releases = fetch_release_versions(&env)?;

    let versions = releases
        .versions
        .into_iter()
        .map(|release| {
            format!(
                "{major}.{minor}.{patch}+{build}",
                major = release.major,
                minor = release.minor,
                patch = release.patch,
                build = release.build
            )
        })
        .map(|version| VersionSpec::parse(&version))
        .collect::<Result<Vec<_>, _>>()?;

    let latest = versions
        .first()
        .and_then(|v| v.as_version())
        .cloned()
        .unwrap_or(Version::new(0, 0, 0));

    let mut aliases = FxHashMap::default();
    aliases.insert("latest".into(), UnresolvedVersionSpec::Semantic(SemVer(latest.clone())));

    Ok(Json(LoadVersionsOutput {
        versions,
        aliases,
        latest: Some(UnresolvedVersionSpec::Semantic(SemVer(latest))),
        ..Default::default()
    }))
}
