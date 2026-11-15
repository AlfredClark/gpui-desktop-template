use gpui_kit::component::ActiveTheme as _;
use gpui_kit::{App, IntoElement, ParentElement as _, RenderOnce, Styled as _, Window, div};

#[derive(IntoElement)]
pub struct HomeView;

impl RenderOnce for HomeView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(cx.theme().muted_foreground)
            .child("首页占位")
    }
}
