# 架构

## Workspace

```text
Cargo.toml            # workspace，edition 经 [workspace.package] 统一
crates/app            # shell：main.rs / app.rs / navigation.rs
crates/home           # feature：首页
crates/settings       # feature：设置
crates/about          # feature：关于
# crates/ui           # 将来时：第二个使用者出现才建（见“通用提取”）
# crates/shared       # 将来时：第二个真实 owner 出现才建（见“通用提取”）
```

## Crate 职责

- `app` 只做组装：`init -> open_window -> Root -> TitleBar + TabBar + Content`。
  不放业务状态，导航选中态（`Destination`）是 shell 唯一持有的跨页状态。
- 每个 feature 对外只暴露 `lib.rs` 的 `pub use`，内部文件可重组。
- feature 内部文件按复杂度取档，见“Feature 内部结构”。

## Feature 内部结构

标准文件角色（按需取用，无对应负担不建空文件）：

| 文件            | 角色                                                     |
| --------------- | -------------------------------------------------------- |
| `lib.rs`        | 唯一出口，只做 `pub use`；内部文件可重组而不改外部导入   |
| `model.rs`      | 纯领域状态与转换，无 `Window`/`Context`，可单元测试      |
| `*_view.rs`     | `Entity` 持有 + `Render` 组合，按 `render_<region>` 分区 |
| `commands.rs`   | Action 定义 + owner 方法；菜单/工具栏/快捷键同调一处     |
| `*_dialog.rs`   | 归属本 feature 的编辑弹窗（如 `rename_dialog.rs`）       |
| `*_delegate.rs` | Table/List 的 delegate；view 不兼任数据模型              |
| `service.rs`    | 仅有 IO 才建；`cx.spawn` + revision 丢弃过期结果         |

按复杂度取三档：

- 静态页（`about` 现状）：`lib.rs + about_view.rs`，`RenderOnce`，无 `model.rs`/`commands.rs`。
- 状态页（`settings` 将来）：`lib.rs/model.rs/settings_view.rs/commands.rs`。
- 集合页（将来）：状态页 + `*_delegate.rs`，必要时加 `*_dialog.rs`。

拆分信号：一个文件要同时理解两种生命周期（如滚动与表单校验）即拆。
命名：view 按产品概念（`ProjectList`、`SettingsView`），处理函数按意图
（`confirm_delete`、`open_project`）；`set_` 只用于 `&mut self` 原地修改，
builder 构造用 `with_`。

## 通用提取：`ui` 与 `shared`

提取门禁：出现第二个真实使用者才提取。单个 feature 内的复用留在 feature 里。

- `crates/ui/`：领域感知的应用组件（如 `page_header`、`empty_state`），
  `RenderOnce` 为主，只依赖 `gpui-kit`，不持业务状态，不依赖任何 feature。
- `crates/shared/`：多 owner 的小能力（如 `persistence`、`recent_items`），
  要求能力有独立名字 + 两个以上 owner，经 `lib.rs` 给稳定 seam，只依赖 `gpui-kit`。
- 可复用但无产品样式的行为属于上游 `gpui-base` 的思想范畴，
  本仓库不自建 base 层；真有此类需求先放 feature 内部，成熟后再议。

依赖铁律：`feature -> ui/shared`，`ui/shared -> gpui-kit`；
`ui` 与 `shared` 互不依赖，更不准依赖 `app` 或 feature。
提前建空 `ui`/`shared` 视为反模式——当前模板没有第二个使用者，所以没有这两个目录。

## 依赖方向

`app -> feature -> gpui-kit`，禁止反向与横向依赖：

- feature 不依赖 shell，不直调兄弟 feature 的 view。
- 跨页协作经 command、event 或下沉到 shared 的小服务。
- 通用 UI 进 `gpui-kit`（或后续的 `ui` crate），业务表达留在 feature。

## 状态归属

| 状态                | 归属                                              |
| ------------------- | ------------------------------------------------- |
| 导航选中页          | `AppView`                                         |
| 输入/表格等保持行为 | 各 view 持有的 `Entity<State>` + `_subscriptions` |
| 纯领域数据与转换    | 各 feature 的 `model.rs`                          |

`render` 只做组合：读状态、派生展示值、拼元素。解析、IO、业务变更放命名方法或 service，经 `cx.spawn` 异步回填时带 revision 丢弃过期结果。
