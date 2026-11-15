# AGENTS.md

gpui-kit 桌面应用模板（Rust）。唯一依赖 `gpui-kit`，应用经 `gpui_kit::*` 使用 GPUI。

## 结构地图

```text
crates/app      # shell：窗口、TitleBar、TabBar 导航，不放业务逻辑
crates/home | settings | about  # feature：各页 model + view，扁平 crate
```

依赖只向下：`app -> feature -> gpui-kit`。feature 之间不互调，跨页走 command/event/shared（见 `docs/architecture.md`）。

## 常用命令

```sh
cargo run -p app                                          # 运行
cargo fmt --all -- --check                                # 格式
cargo clippy --workspace --all-targets -- -D warnings     # lint
git cliff --prepend CHANGELOG.md --unreleased             # changelog 预览
```

提交格式 `type(scope): subject`（英文，scope 用 crate 名），见 `docs/workflow.md`。

## 硬约束

1. 不编造 API：先 `grep` 本地 `~/.cargo/registry` 源码或查 `https://gpui-kit.com/component/{name}.md`，再写代码。
2. `RenderOnce` 做纯展示，`Entity<T>` 持有跨帧状态；`render` 里不建 Entity、不做 IO。
3. 重复元素用 domain `ElementId`（如 `("delete-project", project.id)`），禁用 index/random。
4. 样式只读 `cx.theme()` + rem helpers（`p_4/gap_3/text_sm`），无裸 hex/`px`。
5. `Button` 做应用命令，`Link` 只做外部 URL。
6. 动 skill 前先读对应指南：编码问题读 `gpui-kit` skill 的 coding-guides，界面问题先读 `gpui-kit-design-guides`。

## 文档索引

- `docs/architecture.md`：crate 划分与状态归属
- `docs/gpui-conventions.md`：本仓库已验证的 gpui-kit 用法
- `docs/workflow.md`：开发循环、hook、提交、changelog
