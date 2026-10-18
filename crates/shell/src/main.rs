use std::collections::HashSet;

use color_eyre::Result;
use gpui_kit::{application, assets::Assets};
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::bar::BarConfig;

mod bar;
mod constants;
mod settings;

pub fn init_tracing(filter: LevelFilter) -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(filter.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_tracing(LevelFilter::INFO)?;

    let app = application().with_assets(Assets {});
    app.run(move |cx| {
        gpui_tokio::init(cx);

        let displays = cx.displays();
        dbg!(&displays);

        let mut displays_set = HashSet::new();
        for display in displays {
            displays_set.insert(display.id());
        }

        let config = BarConfig {
            displays: displays_set,
        };
        config.display_per_output(cx);
    });

    Ok(())
}
