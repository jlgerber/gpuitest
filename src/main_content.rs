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
use issue_grp::{issue, group_header};

mod issue_grp;

/// Render the main contents next to the left sidebar
pub fn main_content() -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .child(header())
        .child(active_issues_heading())
        .child(tabs())
        .child(
            div()
                .flex() 
                .flex_col() 
                .gap(px(1.0))
                .child(group_header("ENGINEERING", 3))
                .child(issue(
                    "ENG-124",
                    "Fix authentication redirect",
                    "Backend",
                    "John",
                    "12m",
                    "HIGH",
                    0xd95c5c,
                ))
                .child(issue(
                    "ENG-123", 
                    "IMprove settings page",
                    "Frontend",
                    "Sarah",
                    "1h",
                    "MEDIUM",
                    0xd9a441,
                ))
                .child(issue(
                    "ENG-122", 
                    "Update API documentation",
                    "Documentation",
                    "Alex",
                    "3h",
                    "LOW",
                    0x5b8fd8,
                )),
        )

}

// Create the header at the top of the page 
fn header() -> impl IntoElement {
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
fn active_issues_heading() -> impl IntoElement {
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

// Create the horizontal tabs under the active issues section 
fn tabs() -> impl IntoElement {
    div()
        .h(px(36.0))
        .flex()
        .items_center()
        .gap(px(18.0))
        .border_b_1()
        .border_color(rgb(0x272a2f))
        .child(tab("All", true))
        .child(tab("Assigned to me", false))
        .child(tab("Created by me", false))
}

//
// Contents functions 
//
fn issues() -> impl IntoElement {
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

fn tab(label: &'static str, active:bool) -> impl IntoElement {
    div()
        .h_full()
        .flex()
        .items_center()
        .text_size(px(12.0))
        .text_color(if active {rgb(0xe2e3e5)} else { rgb(0x696d75)})
        .when(active, |this| this.border_b_2().border_color(rgb(0x7c6ff2)))
        .child(label)
}
