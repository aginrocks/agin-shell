use std::collections::HashSet;

use anyhow::Result;
use gpui_kit::{
    App, AppContext, Bounds, Context, DisplayId, Pixels, Size, WindowBackgroundAppearance,
    WindowBounds, WindowDecorations, WindowKind, WindowOptions,
    layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions},
    point, px,
};

use crate::{
    bar::per_output::PerOutputBar,
    constants::{APP_ID, BAR_NAMESPACE},
    settings::Settings,
};

mod per_output;
mod widget;

pub struct BarConfig {
    pub displays: HashSet<DisplayId>,
}

impl BarConfig {
    pub fn display_per_output(&self, cx: &'_ mut App) -> Result<()> {
        for display in &self.displays {
            Self::display_on_output(cx, *display)?;
        }
        Ok(())
    }

    fn height(cx: &App) -> Pixels {
        let settings = cx.global::<Settings>();
        px(
            (settings.bar.margins.top + settings.bar.margins.bottom + settings.bar.inner_height)
                as f32,
        )
    }

    pub fn watch_for_output_changes(&mut self) {}

    #[tracing::instrument(skip(cx))]
    fn display_on_output(cx: &'_ mut App, id: DisplayId) -> Result<()> {
        let height = Self::height(cx);
        let options = WindowOptions {
            // window_bounds: todo!(),
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace: BAR_NAMESPACE.to_string(),
                layer: Layer::Top,
                anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
                exclusive_zone: Some(height),
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
                size: Size::new(px(1.), height),
            })),
            window_decorations: Some(WindowDecorations::Client),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| cx.new(|_| PerOutputBar))?;

        Ok(())
    }
}
