# 架构

## Workspace

```text
Cargo.toml            # workspace，edition 经 [workspace.package] 统一
crates/app            # shell：main.rs / app.rs / navigation.rs
crates/home           # feature：首页
crates/settings       # feature：设置
crates/about          # feature：关于
```

## Crate 职责

- `app` 只做组装：`init -> open_window -> Root -> TitleBar + TabBar + Content`。
  不放业务状态，导航选中态（`Destination`）是 shell 唯一持有的跨页状态。
- 每个 feature 对外只暴露 `lib.rs` 的 `pub use`，内部文件可重组。
- 无状态页（如 `about`）只有 `RenderOnce` view，不建 `model.rs`/`commands.rs`；
  有状态页才加 `model.rs`（纯领域状态，可单元测试）与 `commands.rs`（Action + 处理方法）。

## 依赖方向

`app -> feature -> gpui-kit`，禁止反向与横向依赖：

- feature 不依赖 shell，不直调兄弟 feature 的 view。
- 跨页协作经 command、event 或下沉到 shared 的小服务。
- 通用 UI 进 `gpui-kit`（或后续的 `ui` crate），业务表达留在 feature。

## 状态归属

| 状态 | 归属 |
| --- | --- |
| 导航选中页 | `AppView` |
| 输入/表格等保持行为 | 各 view 持有的 `Entity<State>` + `_subscriptions` |
| 纯领域数据与转换 | 各 feature 的 `model.rs` |

`render` 只做组合：读状态、派生展示值、拼元素。解析、IO、业务变更放命名方法或 service，经 `cx.spawn` 异步回填时带 revision 丢弃过期结果。
