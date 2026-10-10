use std::borrow::Cow;
use anyhow::Result;
use rust_embed::RustEmbed;

use gpui_kit::component::{
    Icon,
    IconName,
    Root,
    Sizable,
    button::{Button, ButtonVariants},
};
use gpui_kit::gpui::prelude::FluentBuilder;
use gpui_kit::*;
use app::App;

pub mod sidebar;
pub mod main_content;
pub mod app;

//
// ASSETs
//
#[derive(RustEmbed)]
#[folder = "assets"]
struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(file) = AppAssets::get(path) {
            return Ok(Some(file.data));
        }

        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        // grab the assets in the assets folder
        let mut files = AppAssets::iter()
            .filter(|file| file.starts_with(path))
            .map(SharedString::from)
            .collect::<Vec<_>>();
        // add the assets that ship with gpui_kit
        files.extend(gpui_kit::assets::Assets.list(path)?);

        Ok(files)
    }
}
fn main() {
    let app = gpui_kit::application().with_assets(AppAssets);

    app.run(|cx| {
        gpui_kit::init(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds{
                    origin: point(px(0.0),px(0.0)),
                    size: size(px(1200.0),px(760.0)),
                })),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| App::new());
                cx.new(|cx| Root::new(view, window, cx))
            }
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}

