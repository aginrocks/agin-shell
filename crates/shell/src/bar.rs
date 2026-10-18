use std::collections::HashSet;

use gpui_kit::{
    App, AppContext, Bounds, Context, DisplayId, Size, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowKind, WindowOptions,
    layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions},
    point, px,
};

use crate::{
    bar::per_output::LayerShellExample,
    constants::{APP_ID, BAR_NAMESPACE},
};

mod per_output;

pub struct BarConfig {
    pub displays: HashSet<DisplayId>,
}

impl BarConfig {
    pub fn display_per_output(&self, cx: &'_ mut App) {
        for display in &self.displays {
            self.display_on_output(cx, *display);
        }
    }

    const fn height() -> u32 {
        const BAR_PADDING: u32 = 12;
        const BAR_CONTENT_HEIGHT: u32 = 24;
        BAR_PADDING * 2 + BAR_CONTENT_HEIGHT
    }

    fn display_on_output(&self, cx: &'_ mut App, id: DisplayId) {
        let options = WindowOptions {
            // window_bounds: todo!(),
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace: BAR_NAMESPACE.to_string(),
                layer: Layer::Top,
                anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
                // exclusive_zone: todo!(),
                // exclusive_edge: todo!(),
                // margin: todo!(),
                keyboard_interactivity: KeyboardInteractivity::None,
                ..Default::default()
            }),
            display_id: Some(id),
            window_background: WindowBackgroundAppearance::Transparent,
            app_id: Some(APP_ID.to_string()),
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(0.), px(0.)),
                size: Size::new(px(1.), px(Self::height() as f32)),
            })),
            window_decorations: Some(WindowDecorations::Client),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| cx.new(LayerShellExample::new));
    }
}
