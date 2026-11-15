# gpui-desktop-template
Desktop application development template based on GPUI.

## Development

```sh
cargo run -p app
```

Quality gate (also enforced by the local pre-commit hook):

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Enable the local hooks after cloning (not stored in the repo):

```sh
git config core.hooksPath .githooks
```

Changelog (see `cliff.toml`):

```sh
git cliff --prepend CHANGELOG.md --unreleased   # preview
git cliff --prepend CHANGELOG.md                # before tagging v*
```
