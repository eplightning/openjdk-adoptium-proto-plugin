use proto_pdk_test_utils::*;

mod openjdk_adoptium_tool {
    use super::*;

    mod legacy_jdk {
        use super::*;

        generate_download_install_tests!("java-legacy-test", "8.0.472+8");
    }

    mod modern_jdk {
        use super::*;

        generate_download_install_tests!("java-modern-test", "25.0.1+8");
    }
}
