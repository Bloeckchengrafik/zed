use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

struct Empty;

impl Render for Empty {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |wnd, cx| {
                println!("Window handle: {:?}", wnd.window_handle());
                println!("Display handle: {:?}", wnd.display_handle());
                cx.new(|_| Empty)
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
