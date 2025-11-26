# Java plugin

Java plugin for proto, using Eclipse Adoptium OpenJDK distribution.

This plugin uses [https://api.adoptium.net/](Adoptium API) to fetch available versions and to provide download URLs to proto.

## Installation

Register the plugin by adding the following to your `.prototools` file:
```toml
[plugins]
java = "github://eplightning/openjdk-adoptium-proto-plugin"
```

Alternatively, use `proto` cli:
```shell
proto plugin add java "github://eplightning/openjdk-adoptium-proto-plugin"
```

## Usage

```shell
# fully qualified version (MUST contain build number)
proto install java 25.0.1+8

# most recent OpenJDK 25
proto install java 25

# list all available versions
proto versions java
```

## Known issues

- Fully qualified semantic version must contain OpenJDK build number to successfully install (e.g. `25.0.1+8`).

## Gradle

For gradle users wanting to manage gradle versions with proto instead of the wrapper, a simple non-WASM plugin is also available:

```toml
[plugins]
gradle = "https://raw.githubusercontent.com/eplightning/openjdk-adoptium-proto-plugin/main/build-plugins/gradle.toml"
```