use extism_pdk::FnResult;
use proto_pdk::{fetch_json, HostArch, HostEnvironment, HostLibc, HostOS};
use serde::Deserialize;
use url::Url;

const API_BASE_URL: &str = "https://api.adoptium.net";

#[derive(Deserialize)]
pub struct Asset {
    pub binaries: Vec<AssetBinary>
}

#[derive(Deserialize)]
pub struct AssetBinary {
    pub package: AssetPackage,
}

#[derive(Deserialize)]
pub struct AssetPackage {
    pub name: String,
    pub checksum: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct ReleaseVersion {
    pub major: u64,
    pub minor: u64,
    #[serde(rename = "security")]
    pub patch: u64,
    pub build: u64,
    pub semver: String,
}

#[derive(Deserialize)]
pub struct ReleaseVersions {
    pub versions: Vec<ReleaseVersion>,
}

fn env_to_arch_and_os(env: &HostEnvironment) -> (String, String) {
    let architecture = match env.arch {
        HostArch::Arm64 => "aarch64".into(),
        HostArch::Powerpc64 => "ppc64le".into(),
        arch => arch.to_string(),
    };

    let os = match (env.os, env.libc) {
        (HostOS::Linux, HostLibc::Musl) => "alpine-linux".into(),
        (HostOS::MacOS, _) => "mac".into(),
        (os, _) => os.to_string(),
    };

    (architecture, os)
}

pub fn fetch_release_asset(env: &HostEnvironment, release: &str) -> FnResult<Asset> {
    let mut url = Url::parse(&format!("{API_BASE_URL}/v3/assets/release_name"))?;

    url.path_segments_mut().unwrap()
        .push("eclipse").push(release);

    let (architecture, os) = env_to_arch_and_os(env);

    url.query_pairs_mut()
        .clear()
        .append_pair("architecture", &architecture)
        .append_pair("os", &os)
        .append_pair("heap_size", "normal")
        .append_pair("image_type", "jdk")
        .append_pair("jvm_impl", "hotspot")
        .append_pair("project", "jdk");

    Ok(fetch_json(url.as_str())?)
}

pub fn fetch_release_versions(env: &HostEnvironment) -> FnResult<ReleaseVersions> {
    let mut url = Url::parse(&format!("{API_BASE_URL}/v3/info/release_versions"))?;

    let (architecture, os) = env_to_arch_and_os(env);

    url.query_pairs_mut()
        .clear()
        .append_pair("architecture", &architecture)
        .append_pair("os", &os)
        .append_pair("heap_size", "normal")
        .append_pair("image_type", "jdk")
        .append_pair("jvm_impl", "hotspot")
        .append_pair("page_size", "20")
        .append_pair("project", "jdk")
        .append_pair("release_type", "ga")
        .append_pair("vendor", "eclipse");

    let mut versions: ReleaseVersions = fetch_json(url.as_str())?;

    let mut page = 0;
    let mut page_items = versions.versions.len();

    while page_items >= 20 {
        page += 1;

        let mut page_url = url.clone();
        page_url.query_pairs_mut().append_pair("page", &page.to_string());

        match fetch_json::<&str, ReleaseVersions>(page_url.as_str()) {
            Ok(page_versions) => {
                page_items = page_versions.versions.len();
                versions.versions.extend(page_versions.versions);
            }
            Err(_) => {
                // TODO: Can we actually distinguish between errors and not having any more releases?
                break;
            }
        }
    }

    Ok(versions)
}

