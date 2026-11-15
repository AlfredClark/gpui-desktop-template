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

## CI 与 Release

`.github/workflows/ci.yml`：push/PR 到 main 自动跑 `lint`（fmt + dprint）
与 `validate`（仅 ubuntu：clippy/check/test），分支保护只需盯 `Passed` 门禁。
工具链 pin 在 `rust-toolchain.toml`（1.99.1），workflow 内 `dtolnay/rust-toolchain` 同值引用，两处保持一致。
部分第三方 action（如 `upload/download-artifact`、`softprops/action-gh-release`）
上游仍是 Node 20 构建，runner 会给出 deprecation 警告但不影响执行，等上游发布 node24 构建后再升级。

发版流程：

1. 把 `[workspace.package] version` bump 到目标版，commit。
2. 打 tag `vX.Y.Z` 并 push（workflow 也支持手动输入版本号触发）。
3. `release.yml` 校验 `app` 版 == tag 版 → 三 OS 打包
   （linux `tar.gz` + `.deb`、mac `.dmg`、windows `zip` + Inno setup）
   → git-cliff 生成 notes → publish release。

打包均为未签名占位产物：应用名/标识（`GpuiDesktop`、`com.example.gpuidesktop`、
`gpui-desktop`）、图标（`assets/icons/icon-256.png` 占位图）发货前替换；
正式分发再接入签名/notary。本地复刻版本校验：

```sh
cargo metadata --format-version 1 --no-deps | python3 -c "import json,sys; print([p['version'] for p in json.load(sys.stdin)['packages'] if p['name']=='app'][0])"
```

## Edition 策略

workspace 经 `[workspace.package] edition = "2024"` 统一，
各 crate 用 `edition.workspace = true`。跟随 stable 主线，不锁旧 edition。
