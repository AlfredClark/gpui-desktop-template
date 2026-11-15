# 开发流程

## 日常循环

```sh
cargo run -p app                                # 运行
cargo fmt --all -- --check                      # 格式门禁
dprint check                                     # md/toml/json/yaml 门禁
cargo clippy --workspace --all-targets -- -D warnings  # lint 门禁
dprint fmt                                       # 手动修复 dprint 格式问题
```

格式配置见 `rustfmt.toml`（edition 2024，`max_width = 100`，全稳定项）；
编辑器缩进与换行见 `.editorconfig`。
非 Rust 文件（md/toml/json/yaml）由 `dprint.json` 管（行宽统一 100，
插件版本锁定，md 正文不重排）。安装单二进制并保证在 `PATH` 中：

```sh
curl -fsSL https://dprint.dev/install.sh | sh   # 或 cargo install dprint
```

## 本地 hook

提交前自动跑三条门禁，任一变红即拦截：

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

`cliff.toml` 配 `tag_pattern = "v*"`（workspace 根 tag），`header` 模板供 workflow 回填。
本地只预览（只读，不写文件）：

```sh
git cliff --unreleased
```

`CHANGELOG.md` 不入库，由 release workflow 在发版时生成。

## Edition 策略

workspace 经 `[workspace.package] edition = "2024"` 统一，
各 crate 用 `edition.workspace = true`。跟随 stable 主线，不锁旧 edition。
