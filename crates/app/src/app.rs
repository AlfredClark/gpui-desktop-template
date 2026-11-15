use about::AboutView;
use gpui_kit::component::tab::{Tab, TabBar};
use gpui_kit::component::{v_flex, ActiveTheme as _, TitleBar};
use gpui_kit::{div, Context, IntoElement, ParentElement as _, Render, Styled as _, Window};
use home::HomeView;
use settings::SettingsView;

use crate::navigation::Destination;

pub struct AppView {
    selected: Destination,
}

impl AppView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            selected: Destination::Home,
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.selected {
            Destination::Home => HomeView.into_any_element(),
            Destination::Settings => SettingsView.into_any_element(),
            Destination::About => AboutView.into_any_element(),
        };

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(TitleBar::new().child(div().child("GPUI Desktop")))
            .child(
                TabBar::new("main-tabs")
                    .selected_index(self.selected.index())
                    .on_click(cx.listener(|this, ix, _window, cx| {
                        this.selected = Destination::from_index(*ix);
                        cx.notify();
                    }))
                    .children(
                        Destination::ALL
                            .iter()
                            .map(|dest| Tab::new().label(dest.label())),
                    ),
            )
            .child(div().flex_1().min_h_0().p_4().child(content))
    }
}
