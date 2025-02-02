![Zebra logotype](https://zfnd.org/wp-content/uploads/2022/03/zebra-logotype.png)

---

[![CI Docker](https://github.com/ZcashFoundation/zebra/actions/workflows/ci-integration-tests-gcp.yml/badge.svg)](https://github.com/ZcashFoundation/zebra/actions/workflows/ci-integration-tests-gcp.yml) [![CI OSes](https://github.com/ZcashFoundation/zebra/actions/workflows/ci-unit-tests-os.yml/badge.svg)](https://github.com/ZcashFoundation/zebra/actions/workflows/ci-unit-tests-os.yml) [![Continuous Delivery](https://github.com/ZcashFoundation/zebra/actions/workflows/cd-deploy-nodes-gcp.yml/badge.svg)](https://github.com/ZcashFoundation/zebra/actions/workflows/cd-deploy-nodes-gcp.yml) [![codecov](https://codecov.io/gh/ZcashFoundation/zebra/branch/main/graph/badge.svg)](https://codecov.io/gh/ZcashFoundation/zebra) [![Build docs](https://github.com/ZcashFoundation/zebra/actions/workflows/docs-deploy-firebase.yml/badge.svg)](https://github.com/ZcashFoundation/zebra/actions/workflows/docs-deploy-firebase.yml)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

## Contents

- [About](#about)
- [Getting Started](#getting-started)
  - [Docker](#docker)
  - [Building Zebra](#building-zebra)
    - [Optional Configs & Features](#optional-configs--features)
- [Known Issues](#known-issues)
- [Future Work](#future-work)
- [Documentation](#documentation)
- [User support](#user-support)
- [Security](#security)
- [License](#license)

## About

[Zebra](https://zebra.zfnd.org/) is the Zcash Foundation's independent,
consensus-compatible implementation of a Zcash node.

Zebra's network stack is interoperable with `zcashd`, and Zebra implements all
the features required to reach Zcash network consensus, including the validation
of all the consensus rules for the NU5 network upgrade.
[Here](https://docs.rs/zebrad/latest/zebrad/index.html#zebra-advantages) are some
benefits of Zebra.

Zebra validates blocks and transactions, but needs extra software to generate
them:

- To generate transactions, [run Zebra with `lightwalletd`](https://zebra.zfnd.org/user/lightwalletd.html).
- To generate blocks, use a mining pool or miner with Zebra's mining JSON-RPCs.
  Currently Zebra can only send mining rewards to a single fixed address.
  To distribute rewards, use mining software that creates its own distribution transactions,
  a light wallet or the `zcashd` wallet.

Please [join us on Discord](https://discord.gg/na6QZNd) if you'd like to find
out more or get involved!

## Getting Started

You can run Zebra using our Docker image or you can build it manually. Please
see the [System Requirements](https://zebra.zfnd.org/user/requirements.html)
section in the Zebra book for system requirements.

### Docker

This command will run our latest release, and sync it to the tip:

```sh
docker run zfnd/zebra:latest
```

For more information, read our [Docker documentation](https://zebra.zfnd.org/user/docker.html).

### Building Zebra

Building Zebra requires [Rust](https://www.rust-lang.org/tools/install),
[libclang](https://clang.llvm.org/doxygen/group__CINDEX.html), and a C++
compiler.

Zebra is tested with the latest `stable` Rust version. Earlier versions are not
supported or tested. Any Zebra release can start depending on new features in the
latest stable Rust.

Every few weeks, we release a [new Zebra version](https://github.com/ZcashFoundation/zebra/releases).

Below are quick summaries for installing the dependencies on your machine.

[//]: # "The empty line in the `summary` tag below is required for correct Markdown rendering."
<details><summary>

#### General instructions for installing dependencies
</summary>

1. Install [`cargo` and `rustc`](https://www.rust-lang.org/tools/install).

2. Install Zebra's build dependencies:

   - **libclang** is a library that might have different names depending on your
     package manager. Typical names are `libclang`, `libclang-dev`, `llvm`, or
     `llvm-dev`.
   - **clang** or another C++ compiler: `g++` (all platforms) or `Xcode` (macOS).
   - **[`protoc`](https://grpc.io/docs/protoc-installation/)**

> [!NOTE]
> Zebra uses the `--experimental_allow_proto3_optional` flag with `protoc`
> during compilation. This flag was introduced in [Protocol Buffers
> v3.12.0](https://github.com/protocolbuffers/protobuf/releases/tag/v3.12.0)
> released in May 16, 2020, so make sure you're not using a version of `protoc`
> older than 3.12.

</details>

[//]: # "The empty line in the `summary` tag below is required for correct Markdown rendering."
<details><summary>

#### Dependencies on Arch
</summary>

```sh
sudo pacman -S rust clang protobuf
```

Note that the package `clang` includes `libclang` as well as the C++ compiler.

</details>

Once the dependencies are in place, you can build and install Zebra:

```sh
cargo install --locked zebrad
```

You can start Zebra by

```sh
zebrad start
```

See the [Installing Zebra](https://zebra.zfnd.org/user/install.html) and [Running Zebra](https://zebra.zfnd.org/user/run.html)
sections in the book for more details.

#### Optional Configs & Features

##### Initializing Configuration File

```console
zebrad generate -o ~/.config/zebrad.toml
```

The above command places the generated `zebrad.toml` config file in the default preferences directory of Linux. For other OSes default locations [see here](https://docs.rs/dirs/latest/dirs/fn.preference_dir.html).

##### Configuring Progress Bars

Configure `tracing.progress_bar` in your `zebrad.toml` to
[show key metrics in the terminal using progress bars](https://zfnd.org/experimental-zebra-progress-bars/).
When progress bars are active, Zebra automatically sends logs to a file.

There is a known issue where [progress bar estimates become extremely large](https://github.com/console-rs/indicatif/issues/556).

In future releases, the `progress_bar = "summary"` config will show a few key metrics,
and the "detailed" config will show all available metrics. Please let us know which metrics are
important to you!

##### Configuring Mining

Zebra can be configured for mining by passing a `MINER_ADDRESS` and port mapping to Docker.
See the [mining support docs](https://zebra.zfnd.org/user/mining-docker.html) for more details.

##### Custom Build Features

You can also build Zebra with additional [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options):

- `prometheus` for [Prometheus metrics](https://zebra.zfnd.org/user/metrics.html)
- `sentry` for [Sentry monitoring](https://zebra.zfnd.org/user/tracing.html#sentry-production-monitoring)
- `elasticsearch` for [experimental Elasticsearch support](https://zebra.zfnd.org/user/elasticsearch.html)
- `shielded-scan` for [experimental shielded scan support](https://zebra.zfnd.org/user/shielded-scan.html)

You can combine multiple features by listing them as parameters of the `--features` flag:

```sh
cargo install --features="<feature1> <feature2> ..." ...
```

Our full list of experimental and developer features is in [the API documentation](https://docs.rs/zebrad/latest/zebrad/index.html#zebra-feature-flags).

Some debugging and monitoring features are disabled in release builds to increase
performance.

## Known Issues

There are a few bugs in Zebra that we're still working on fixing:

- [The `getpeerinfo` RPC shows current and recent outbound connections](https://github.com/ZcashFoundation/zebra/issues/7893), rather than current inbound and outbound connections.

- [Progress bar estimates can become extremely large](https://github.com/console-rs/indicatif/issues/556). We're waiting on a fix in the progress bar library.

- Zebra currently gossips and connects to [private IP addresses](https://en.wikipedia.org/wiki/IP_address#Private_addresses), we want to [disable private IPs but provide a config (#3117)](https://github.com/ZcashFoundation/zebra/issues/3117) in an upcoming release

- Block download and verification sometimes times out during Zebra's initial sync [#5709](https://github.com/ZcashFoundation/zebra/issues/5709). The full sync still finishes reasonably quickly.

- No Windows support [#3801](https://github.com/ZcashFoundation/zebra/issues/3801). We used to test with Windows Server 2019, but not any more; `zcash_script` has recently been updated to compile with MSVC, we're now waiting on a `zcash_script` release and dependency update, see the issue for details.

- Experimental Tor support is disabled until Zebra upgrades to the latest `arti-client`. This happened due to a Rust dependency conflict ([#5492](https://github.com/ZcashFoundation/zebra/issues/5492)) and is still an issue due to [another dependency conflict](https://github.com/ZcashFoundation/zebra/issues/8328#issuecomment-1969989648).

## Documentation

The Zcash Foundation maintains the following resources documenting Zebra:

- The Zebra Book:
  - [General Introduction](https://zebra.zfnd.org/index.html),
  - [User Documentation](https://zebra.zfnd.org/user.html),
  - [Developer Documentation](https://zebra.zfnd.org/dev.html).

- The [documentation of the public
  APIs](https://docs.rs/zebrad/latest/zebrad/#zebra-crates) for the latest
  releases of the individual Zebra crates.

- The [documentation of the internal APIs](https://doc-internal.zebra.zfnd.org)
  for the `main` branch of the whole Zebra monorepo.

## User support

For bug reports please [open a bug report ticket in the Zebra repository](https://github.com/ZcashFoundation/zebra/issues/new?assignees=&labels=C-bug%2C+S-needs-triage&projects=&template=bug_report.yml&title=%5BUser+reported+bug%5D%3A+).

Alternatively by chat, [Join the Zcash Foundation Discord Server](https://discord.com/invite/aRgNRVwsM8) and find the #zebra-support channel.

## Security

Zebra has a [responsible disclosure policy](https://github.com/ZcashFoundation/zebra/blob/main/SECURITY.md), which we encourage security researchers to follow.

## License

Zebra is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

Some Zebra crates are distributed under the [MIT license only](LICENSE-MIT),
because some of their code was originally from MIT-licensed projects.
See each crate's directory for details.
