use proto_pdk_test_utils::*;

mod openjdk_adoptium_tool {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn sets_java_home() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("java-test").await;

        let output = plugin
            .activate_environment(ActivateEnvironmentInput::default())
            .await;

        assert!(output.env.contains_key("JAVA_HOME"));
    }
}
