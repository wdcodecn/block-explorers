# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


### Miscellaneous Tasks

- Update cliff link
- Add CHANGELOG.md scripts ([#26](https://github.com/foundry-rs/compilers/issues/26))

## [0.1.3](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.3) - 2024-01-05

### Bug Fixes

- Dont force trailing url slash ([#25](https://github.com/foundry-rs/compilers/issues/25))

### Miscellaneous Tasks

- Release 0.1.3

### Other

- Add `getcontractcreation` binding ([#24](https://github.com/foundry-rs/compilers/issues/24))
- Fix deserialization error resulting from Blockscout omitting "OptimizationRuns" field when optimization was not used ([#23](https://github.com/foundry-rs/compilers/issues/23))
- Fix deserialization failure when fetching contract source_code from blockscout ([#22](https://github.com/foundry-rs/compilers/issues/22))

## [0.1.2](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.2) - 2023-12-08

### Bug Fixes

- Sanitize all source entries ([#19](https://github.com/foundry-rs/compilers/issues/19))

### Miscellaneous Tasks

- 0.1.2 ([#20](https://github.com/foundry-rs/compilers/issues/20))

## [0.1.1](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.1) - 2023-11-23

### Dependencies

- Bump Alloy

### Miscellaneous Tasks

- Release 0.1.1
- [meta] Add CODEOWNERS

## [0.1.0](https://github.com/foundry-rs/block-explorers/releases/tag/v0.1.0) - 2023-11-15

### Bug Fixes

- Add licensing ([#4](https://github.com/foundry-rs/compilers/issues/4))
- [features] Remove ethers-solc for foundry-compilers ([#3](https://github.com/foundry-rs/compilers/issues/3))

### Dependencies

- Bump ethers ([#9](https://github.com/foundry-rs/compilers/issues/9))

### Documentation

- Add CHANGELOG.md

### Features

- Remove Ethers ([#14](https://github.com/foundry-rs/compilers/issues/14))
- Repo improvements ([#11](https://github.com/foundry-rs/compilers/issues/11))
- Alloy migration ([#2](https://github.com/foundry-rs/compilers/issues/2))
- [`CI`] Enable ci/cd ([#1](https://github.com/foundry-rs/compilers/issues/1))
- Repo init

### Miscellaneous Tasks

- [meta] Update configs ([#15](https://github.com/foundry-rs/compilers/issues/15))
- Remove RawAbi and LosslessAbi usage ([#12](https://github.com/foundry-rs/compilers/issues/12))
- Enable more lints ([#13](https://github.com/foundry-rs/compilers/issues/13))
- Remove default feats from openssl ([#7](https://github.com/foundry-rs/compilers/issues/7))
- Patch ethers to be in sync w/ foundry ([#6](https://github.com/foundry-rs/compilers/issues/6))
- Clippy ([#5](https://github.com/foundry-rs/compilers/issues/5))

### Other

- Update README.md

### Styling

- Update rustfmt config ([#16](https://github.com/foundry-rs/compilers/issues/16))

<!-- generated by git-cliff -->