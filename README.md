# example-rust-cli

An example Rust CLI used to model project tooling, cross-platform releases, and
shell completion generation.

## Usage

Print the version:

```sh
example-rust-cli version
```

Print a greeting:

```sh
example-rust-cli hello --name Ben
```

Generate a completion script for a supported shell:

```sh
example-rust-cli completion zsh
```

# Install

- [Homebrew](https://brew.sh/):
  `brew install --cask bbkane/tap/example-rust-cli`
- [Scoop](https://scoop.sh/):

```sh
scoop bucket add bbkane https://github.com/bbkane/scoop-bucket
scoop install bbkane/example-rust-cli
```

- Download a macOS, Linux, or Windows executable from
  [GitHub Releases](https://github.com/bbkane/example-rust-cli/releases).
- Build with [GoReleaser](https://goreleaser.com/) after cloning:
  `goreleaser release --snapshot --clean`

# Dev notes

See: [CLI Project Notes | Ben's Corner](https://www.bbkane.com/blog/cli-project-notes)

## Snapshot Testing

Install `cargo-insta`:

```bash
cargo install cargo-insta
```

Run snapshot tests:

```bash
cargo insta test
```

Agents: inspect every `.snap.new` file, then accept the snapshots without opening the review TUI:

```bash
cargo insta accept
```

Humans: use the TUI to review snapshots:

```bash
cargo insta review
```

Delete snapshots that are no longer referenced by tests:

```bash
cargo insta test --unreferenced delete
```