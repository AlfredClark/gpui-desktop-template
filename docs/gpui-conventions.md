# gpui-kit 用法约定

只记录本仓库已验证的做法。完整规则见 `gpui-kit` skill 的 coding-guides，
界面决策见 `gpui-kit-design-guides`。单个组件的权威文档：
`https://gpui-kit.com/component/{name}.md`（如 `title-bar.md`、`tabs.md`）。

## 无边框窗口

自建标题栏不手写拖拽与三键，全部交给 `TitleBar`：

```rust
open_window(
    WindowOptions {
        window_decorations: Some(WindowDecorations::Client),
        ..TitleBar::window_options()
    },
    cx,
    |window, cx| cx.new(|cx| AppView::new(window, cx)),
)
```

`TitleBar::window_options()` 只配 `titlebar`，Linux 下必须再请求
`WindowDecorations::Client`，否则窗口保持服务端装饰，
`TitleBar` 的三键会自动隐藏（避免与系统栏重复）。

## Tabs 导航

`TabBar` 受控使用：`selected_index(usize)` + `on_click` 回写 owner 并 `cx.notify()`。
`Tab::new().label(..)`，ID 用 domain（`"main-tabs"`），不用 index 派生。

## 占位页

纯展示页用 `RenderOnce`：

```rust
#[derive(IntoElement)]
pub struct HomeView;

impl RenderOnce for HomeView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(cx.theme().muted_foreground)
            .child("首页占位")
    }
}
```

需要 `use gpui_kit::component::ActiveTheme as _;` 才能调用 `cx.theme()`。

## 主题与布局

- 颜色/圆角/间距走 `cx.theme()` 语义 token，布局走 rem helpers
 （`p_4/gap_3/text_sm`，`size_full/flex_1/min_h_0`）。
- 纵向用 `v_flex()`，内容区 `flex_1().min_h_0()` 让出收缩权。
- 选中态画在元素自身表面，不加前缘色条。

## 测试

交互覆盖用 UI integration test：`#[gpui_kit::test]` 起真窗口，
模拟输入后断言状态与渲染结果，而非调内部方法。
