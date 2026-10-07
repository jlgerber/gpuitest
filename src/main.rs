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
struct App;

impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x0f1912))
            .text_color(rgb(0xe8e9ea))
            .child(
                div()
                .size_full()
                .flex()
                .flex_row()
                .child(sidebar())
                .child(main_content()),
            )
    }
}

fn sidebar() -> impl IntoElement {
    div()
        .w(px(232.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(rgb(0x131416))
        .border_r_1()
        .border_color(rgb(0x272a2f))
        .px(px(10.0))
        .py(px(10.0))
        .gap(px(14.0))
        .child(
            div()
                .h(px(34.0))
                .flex()
                .items_center()
                .px(px(7.0))
                .rounded(px(7.0))
                .hover(|this| this.bg(rgb(0x1d2024)))
                .child(img("org.svg").size(px(22.0)).rounded(px(6.0)))
                .child(
                    div()
                        .ml(px(9.0))
                        .flex_1()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child("SMBL"),
                )
                .child(
                    Icon::new(IconName::ChevronDown)
                    .size(px(14.0))
                    .text_color(rgb(0x777b83)),
                ),
        )
}

fn main_content() -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
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
                let view = cx.new(|_| App);
                cx.new(|cx| Root::new(view, window, cx))
            }
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}

