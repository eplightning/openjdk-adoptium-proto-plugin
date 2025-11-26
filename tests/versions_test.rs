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
}
