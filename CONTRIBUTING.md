# Contribution guidelines
All contributions are welcome and appreciated.

To ensure that your contribution is accepted, please adhere to the following process:
  1. Open an issue that briefly explains your contribution
  2. If the issue is not straightforward, please describe it as well as you can
  3. In the event that the owner/maintainer(s) of the project disagree or require clarity, please be willing to participate in a civil discussion

Pull requests made with no associated issue will most likely be denied. Please open an issue!

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/dark-fusion/silk/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to keep your pull request as small as possible. It is recommended that you submit one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/dark-fusion/silk/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

#### Prerequisites
To set up your local development environment, all prerequisites must be met:
- Up to date version of [Rust](https://www.rust-lang.org/tools/install) with `rustup` and `cargo`
- [Clippy](https://github.com/rust-lang/rust-clippy)
  - Clippy can be installed using `rustup component add clippy`
- [Cargo Audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
  - Tool for auditing dependencies for known security vulnerabilities

```shell
git clone https://github.com/dark-fusion/silk
cd silk
cargo test
```

### Useful Commands

- Build and run release version:

  ```shell
  cargo build --release && cargo run --release
  ```

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
