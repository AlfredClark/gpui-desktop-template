mod app;
mod navigation;

use app::AppView;
use gpui_kit::component::TitleBar;
use gpui_kit::{application, init, open_window, AppContext as _, WindowDecorations, WindowOptions};

fn main() {
    application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            init(cx);
            open_window(
                WindowOptions {
                    window_decorations: Some(WindowDecorations::Client),
                    ..TitleBar::window_options()
                },
                cx,
                |window, cx| cx.new(|cx| AppView::new(window, cx)),
            )
            .expect("failed to open window");
        });
}
