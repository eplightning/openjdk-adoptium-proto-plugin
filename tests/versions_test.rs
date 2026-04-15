use proto_pdk_test_utils::*;

mod openjdk_adoptium_tool {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn loads_versions_from_api() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("java-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(!output.versions.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parses_java_version_file() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("java-test").await;

        let output = plugin
            .parse_version_file(ParseVersionFileInput {
                content: "temurin-17.0.16+8".into(),
                file: ".java-version".into(),
                ..Default::default()
            })
            .await;
        assert_eq!(output.version.unwrap().to_string(), "17.0.16+8");

        let output = plugin
            .parse_version_file(ParseVersionFileInput {
                content: "zulu-11.0.12\n".into(),
                file: ".java-version".into(),
                ..Default::default()
            })
            .await;
        assert_eq!(output.version.unwrap().to_string(), "11.0.12");

        let output = plugin
            .parse_version_file(ParseVersionFileInput {
                content: "17.0.16+8".into(),
                file: ".java-version".into(),
                ..Default::default()
            })
            .await;
        assert_eq!(output.version.unwrap().to_string(), "17.0.16+8");
    }
}
