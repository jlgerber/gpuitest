/// main content
///


use gpui_kit::component::{
    Icon,
    IconName,
    Root,
    Sizable,
    button::{Button, ButtonVariants},
};
use gpui_kit::gpui::prelude::FluentBuilder;
use gpui_kit::*;

pub  mod issue_grp;
// Create the header at the top of the page 
pub fn header() -> impl IntoElement {
    div()
        .h(px(56.0))
        .flex()
        .items_center()
        .px(px(28.0))
        .border_b_1()
        .border_color(rgb(0x272a2f))
        .child(
            div()
                .text_color(rgb(0x686c734))
                .text_size(px(12.0))
                .child("Issues")
        )
        .child(div().mx(px(8.0)).text_color(rgb(0x44474d)).child("/"))
        .child(div().text_size(px(12.0)).child("Active"))
        .child(div().flex_1())
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .child(Button::new("filter").ghost().label("Filter").small())
                .child(Button::new("sort").ghost().label("Sort").small())
                .child(Button::new("new").primary().label("New issue").small())
        )
}

// create the Active Issues heading below the header section 
pub fn active_issues_heading() -> impl IntoElement {
    div()
        //.flex_1()
        .px(px(44.0))
        .py(px(30.0))
        .flex()
        .flex_col()
        //.border_1()
        //.border_color(rgb(0xcccccc))
        .gap(px(24.0))
        .child(issues())
}
//
// Contents functions 
//
pub  fn issues() -> impl IntoElement {
    div()
        .flex()
        .items_end()
        .child(
            div()
                .flex_1()
                .child(
                    div()
                        .text_size(px(23.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("Active Issues")
                )
                .child(
                    div()
                        .mt(px(5.0))
                        .text_size(px(12.0))
                        .text_color(rgb(0x777b83))
                        .child("Issues curerntly being worked on")
                ),
        )
        .child(Button::new("more").ghost().label("...").xsmall())
}

