use proto_pdk_test_utils::*;

fn create_download_input(version: &str) -> DownloadPrebuiltInput {
    DownloadPrebuiltInput {
        context: PluginContext {
            version: VersionSpec::parse(version).unwrap(),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn create_locate_input(version: &str) -> LocateExecutablesInput {
    LocateExecutablesInput {
        context: PluginContext {
            version: VersionSpec::parse(version).unwrap(),
            ..Default::default()
        },
        ..Default::default()
    }
}

mod openjdk_adoptium_tool {
    use super::*;

    mod legacy_jdk {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        async fn supports_linux_x64() {
            let sandbox = create_empty_proto_sandbox();
            let plugin = sandbox
                .create_plugin_with_config("java-legacy-test", |config| {
                    config.host(HostOS::Linux, HostArch::X64);
                })
                .await;

            let output = plugin
                .download_prebuilt(create_download_input("8.0.472+8"))
                .await;

            assert_eq!(output.archive_prefix, Some("jdk8u472-b08".into()));
            assert_eq!(
                output.download_url,
                "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u472-b08/OpenJDK8U-jdk_x64_linux_hotspot_8u472b08.tar.gz"
            );
            assert!(output.checksum.is_some());

            let output = plugin
                .locate_executables(create_locate_input("8.0.472+8"))
                .await;

            assert_eq!(
                output.exes.get("java").unwrap().exe_path,
                Some("bin/java".into())
            );
            assert_eq!(
                output.exes.get("javac").unwrap().exe_path,
                Some("bin/javac".into())
            );
            assert_eq!(
                output.exes.get("jar").unwrap().exe_path,
                Some("bin/jar".into())
            );
            assert_eq!(
                output.exes.get("javadoc").unwrap().exe_path,
                Some("bin/javadoc".into())
            );
        }
    }

    mod modern_jdk {
        use super::*;

        #[tokio::test(flavor = "multi_thread")]
        async fn supports_linux_x64() {
            let sandbox = create_empty_proto_sandbox();
            let plugin = sandbox
                .create_plugin_with_config("java-modern-test", |config| {
                    config.host(HostOS::Linux, HostArch::X64);
                })
                .await;

            let output = plugin
                .download_prebuilt(create_download_input("25.0.1+8"))
                .await;

            assert_eq!(output.archive_prefix, Some("jdk-25.0.1+8".into()));
            assert_eq!(
                output.download_url,
                "https://github.com/adoptium/temurin25-binaries/releases/download/jdk-25.0.1%2B8/OpenJDK25U-jdk_x64_linux_hotspot_25.0.1_8.tar.gz"
            );
            assert!(output.checksum.is_some());

            let output = plugin
                .locate_executables(create_locate_input("25.0.1+8"))
                .await;

            assert_eq!(
                output.exes.get("java").unwrap().exe_path,
                Some("bin/java".into())
            );
            assert_eq!(
                output.exes.get("javac").unwrap().exe_path,
                Some("bin/javac".into())
            );
            assert_eq!(
                output.exes.get("jar").unwrap().exe_path,
                Some("bin/jar".into())
            );
            assert_eq!(
                output.exes.get("javadoc").unwrap().exe_path,
                Some("bin/javadoc".into())
            );
        }
    }
}
