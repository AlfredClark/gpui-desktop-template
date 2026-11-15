# 开发流程

## 日常循环

```sh
cargo run -p app                                # 运行
cargo fmt --all -- --check                      # 格式门禁
cargo clippy --workspace --all-targets -- -D warnings  # lint 门禁
```

格式配置见 `rustfmt.toml`（edition 2024，`max_width = 100`，全稳定项）；
编辑器缩进与换行见 `.editorconfig`。

## 本地 hook

提交前自动跑两条门禁，任一变红即拦截：

```sh
git config core.hooksPath .githooks   # 每克隆一次配一次，不入库
```

## 提交规范

英文，Conventional Commits，`type(scope): subject`，scope 用 crate 名：

```sh
feat(app): request client-side window decorations
fix(settings): keep selected row by domain id
chore: add git-cliff changelog configuration
```

类型常用 `feat/fix/docs/refactor/test/chore`，breaking 用 `!`
（如 `feat(app)!: ...`）或 `BREAKING CHANGE` footer，会进 changelog 置顶分组。

## Changelog

`cliff.toml` 配 `tag_pattern = "v*"`（workspace 根 tag）。
日常只预览，`CHANGELOG.md` 在发版前统一更新：

```sh
git cliff --prepend CHANGELOG.md --unreleased   # 预览
git cliff --prepend CHANGELOG.md                # 发版前入库（必须 --prepend）
```

## Edition 策略

workspace 经 `[workspace.package] edition = "2024"` 统一，
各 crate 用 `edition.workspace = true`。跟随 stable 主线，不锁旧 edition。
