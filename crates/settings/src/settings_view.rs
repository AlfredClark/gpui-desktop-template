use gpui_kit::component::ActiveTheme as _;
use gpui_kit::{div, App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window};

#[derive(IntoElement)]
pub struct SettingsView;

impl RenderOnce for SettingsView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(cx.theme().muted_foreground)
            .child("设置占位")
    }
}
