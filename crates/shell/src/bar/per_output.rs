use std::time::{Duration, SystemTime, UNIX_EPOCH};

use gpui_kit::{
    App, Bounds, Context, FontWeight, Size, Window, WindowBackgroundAppearance, WindowBounds,
    WindowKind, WindowOptions, div, layer_shell::*, point, prelude::*, px, red, rems, rgba, white,
};

use crate::{bar::widget::BarWidget, settings::Settings};

pub struct PerOutputBar;

impl Render for PerOutputBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = cx.global::<Settings>();
        let bar_margins = &settings.bar.margins;

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .pt(px(bar_margins.top as f32))
            .pl(px(bar_margins.left as f32))
            .pb(px(bar_margins.bottom as f32))
            .pr(px(bar_margins.right as f32))
            .child(BarWidget::new("aaa").child("test"))
    }
}
