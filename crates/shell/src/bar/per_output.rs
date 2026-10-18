use std::time::{Duration, SystemTime, UNIX_EPOCH};

use gpui_kit::{
    App, Bounds, Context, FontWeight, Size, Window, WindowBackgroundAppearance, WindowBounds,
    WindowKind, WindowOptions, div, layer_shell::*, point, prelude::*, px, rems, rgba, white,
};

pub struct LayerShellExample;

impl LayerShellExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            loop {
                let _ = this.update(cx, |_, cx| cx.notify());
                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
            }
        })
        .detach();

        LayerShellExample
    }
}

impl Render for LayerShellExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hours = (now / 3600) % 24;
        let minutes = (now / 60) % 60;
        let seconds = now % 60;

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_size(rems(4.5))
            .font_weight(FontWeight::EXTRA_BOLD)
            .text_color(white())
            .bg(rgba(0x0000044))
            .child(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }
}
