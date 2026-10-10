/// Pieces holds individual functions

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

pub fn project_item(label: &'static str, color: u32) -> impl IntoElement {
    div()
        .h(px(29.0))
        .flex()
        .items_center()
        .px(px(8.0))
        .rounded(px(6.0))
        .hover(|this| this.bg(rgb(0x1d2024)))
        .child(
            div().size(px(8.0)).rounded_full().bg(rgb(color))
        )
        .child(
            div()
                .ml(px(10.0))
                .text_size(px(12.0))
                .text_color(rgb(0x858990))
                .child(label),
        )
}

pub fn workspace_elem() -> impl IntoElement {
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
        )

}

pub fn info() -> impl IntoElement {
    div()
        .mt_auto()
        .h(px(40.0))
        .flex()
        .items_center()
        .px(px(6.0))
        .rounded(px(7.0))
        .hover(|this| this.bg(rgb(0x1d2024)))
        .child(img("user.svg").size(px(26.0)).rounded_full())
        .child(
            div()
                .ml(px(8.0))
                .flex_1()
                .flex()
                .flex_col()
                .justify_center()
                .gap(px(1.0))
                .child(
                    div() 
                        .text_size(px(11.0))
                        .text_color(rgb(0xd0d2d6))
                        .child("Jgerber"),
                )
                .child(
                    div()
                        .text_size(px(9.0))
                        .text_color(rgb(0x656970))
                        .child("Personal")
                ),

        )
        .child(
            Icon::new(IconName::EllipsisVertical)
                .size(px(14.0))
                .text_color(rgb(0x656070)),
        )
}

pub  fn search_box() -> impl IntoElement {
    div()
        .h(px(32.0))
        .flex()
        .items_center()
        .px(px(8.0))
        .rounded(px(7.0))
        .bg(rgb(0x181a1d))
        .border_1()
        .border_color(rgb(0x272a2f))
        .child(
            Icon::new(IconName::Search)
                .size(px(14.0))
                .text_color(rgb(0x777b83)),
        )
        .child(
            div()
                .ml(px(8.0))
                .flex_1()
                .text_color(rgb(0x777b83))
                .text_size(px(12.0))
                .child("Search"),
        )
        .child(
            div()
                .px(px(5.0))
                .py(px(2.0))
                .rounded(px(4.0))
                .border_1()
                .border_color(rgb(0x303339))
                .text_color(rgb(0x696d75))
                .text_size(px(10.0))
                .child("⌘K"),
        )
}

