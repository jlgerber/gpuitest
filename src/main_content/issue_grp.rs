/// Issue Group - Grouping of issues under a tab

use gpui_kit::component::{
    Icon,
    IconName,
    Root,
    Sizable,
    button::{Button, ButtonVariants},
};
use gpui_kit::gpui::prelude::FluentBuilder;
use gpui_kit::*;

pub fn group_header(name: &'static str, count: u32) -> impl IntoElement {
    div()
        .px(px(8.0))
        .py(px(8.0))
        .flex()
        .items_center()
        .gap(px(7.0))
        .child(
            div()
                .text_size(px(10.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x777b83))
                .child(name),
        )
        .child(
            div()
            .text_size(px(10.0))
            .text_color(rgb(0x50545b))
            .child(count.to_string()),
        )
}

fn disk(priority_color: u32) -> impl IntoElement {
    div()
        .size(px(16.0))
        .rounded_full()
        .border_2()
        .border_color(rgb(priority_color))
        .mr(px(11.0))
}

fn label(
    id: &'static str,
    title: &'static str, 
    team: &'static str,
    updated: &'static str
) -> impl IntoElement {
   div()
    .flex_1()
    .flex()
    .flex_col()
    .gap(px(3.0))
    .child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgb(0x5f636b))
                    .child(id),
            )
    )
    .child( 
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .text_size(px(10.0))
            .text_color(rgb(0x656970))
            .child(team)
            .child(".")
            .child(format!("Updated {updated} ago"))
    )

}

fn priority_tag(
    priority: &'static str, 
    priority_color: u32
) -> impl IntoElement {
    div()
        .px(px(7.0))
        .py(px(3.0))
        .rounded(px(4.0))
        .bg(rgb(priority_color))
        .text_color(rgb(0x101113))
        .text_size(px(9.0))
        .font_weight(FontWeight::SEMIBOLD)
        .child(priority)
}

fn assignee_tag(assignee: &'static str) -> impl IntoElement {
    div()
        .ml(px(16.0))
        .size(px(26.0))
        .rounded_full()
        .bg(rgb(0x30343b))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(10.0))
        .text_color(rgb(0xc4c6cb))
        .child(assignee.chars().next().unwrap_or('?').to_string())
}

pub fn issue(
    id: &'static str,
    title: &'static str, 
    team: &'static str,
    assignee: &'static str,
    updated: &'static str,
    priority: &'static str,
    priority_color: u32,
) -> impl IntoElement {
    div()
        .h(px(60.0))
        .flex()
        .items_center()
        .px(px(12.0))
        .rounded(px(7.0))
        .border_1()
        .border_color(rgb(0x141619))
        .hover(|this| this.bg(rgb(0x1a1c20)).border_color(rgb(0x303339)))
        .child(disk(priority_color))
        .child(label(team, id, title, updated))
        .child(priority_tag(priority, priority_color))
        .child(assignee_tag(assignee))
}
