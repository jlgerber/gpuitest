
use gpui_kit::prelude::*;
use gpui_kit::gpui::{div, rgb, Window, Context, px, FontWeight };
use gpui_kit::component::{IconName, Icon};

use crate::sidebar::{
     project_item,
     search_box,
     info,
     workspace_elem
    
};

use crate::main_content::issue_grp::{issue, group_header}; //, header, active_issues_heading};
use crate::main_content::{ header, active_issues_heading};

//
// App and State
//
#[derive(Clone, Copy, PartialEq, Eq)]
enum Navigation {
    Inbox,
    MyIssues,
    ActiveIssues,
    Projects,
    Views,
}

impl Navigation {
    
    fn title(self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
            Self::MyIssues => "MyIssues",
            Self::ActiveIssues => "ActiveIssues",
            Self::Projects => "Projects",
            Self::Views => "Views",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::Inbox => "Recently updated issues",
            Self::MyIssues => "Issues currently assigned to me",
            Self::ActiveIssues => "Issues currently being worked on",
            Self::Projects => "Issues across every project",
            Self::Views => "Issues included in your saved view",
        }
    }
}

#[derive(Clone,Copy)]
pub struct Issue {
    id: &'static str,
    title: &'static str,
    group: &'static str,
    team: &'static str,
    assignee: &'static str,
    updated: &'static str,
    priority: &'static str,
    priority_color: i32,
    assigned_to_me: bool,
    created_by_me: bool,
    completed: bool,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum IssueTab {
    All,
    AssignedToMe,
    CreatedByMe,
}

pub struct App {
    navigation: Navigation,
    active_tab: IssueTab,
    selected_issue: Option<&'static str>,
    issues: Vec<Issue>,
}

impl App {

    fn tab(
        &self, 
        label: &'static str, 
        active:bool
    ) -> impl IntoElement {
        div()
            .h_full()
            .flex()
            .items_center()
            .text_size(px(12.0))
            .text_color(if active {rgb(0xe2e3e5)} else { rgb(0x696d75)})
            .when(active, |this| this.border_b_2().border_color(rgb(0x7c6ff2)))
            .child(label)
    }

    // Create the horizontal tabs under the active issues section 
    fn tabs(&self) -> impl IntoElement {
        div()
            .h(px(36.0))
            .flex()
            .items_center()
            .gap(px(18.0))
            .border_b_1()
            .border_color(rgb(0x272a2f))
            .child(self.tab("All", true))
            .child(self.tab("Assigned to me", false))
            .child(self.tab("Created by me", false))
    }


    /// Render the main contents next to the left sidebar
    fn main_content(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .child(header())
            .child(active_issues_heading())
            .child(self.tabs())
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


    /// Left Sidebar in the main window 
    fn sidebar(&self, cx: &Context<Self>) -> impl IntoElement {
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
            .child(workspace_elem())
            .child(search_box())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(1.0))
                    .child(self.sidebar_item(IconName::Inbox, "Inbox", Some("3"), false))
                    .child(self.sidebar_item(IconName::User, "My Issues", Some("7"), false))
                    .child(self.sidebar_item(IconName::CircleX, "Active Issues", None, true))
                    .child(self.sidebar_item(IconName::FolderOpen, "Projects", None, false))
                    .child(self.sidebar_item(IconName::PanelLeft, "Views", None, false)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .px(px(8.0))
                            .pt(px(4.0))
                            .pb(px(4.0))
                            .text_size(px(10.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x656970))
                            .child("PROJECTS"),
                    )
                    .child(project_item("Website", 0x7c6ff2))
                    .child(project_item("Mobile", 0x4e9f7a))
                    .child(project_item("API", 0xd9a441)),
            )
            .child(info())
    }


    fn sidebar_item(
        &self,
        icon: IconName,
        label: &'static str,
        count: Option<&'static str>,
        active: bool,
    ) -> impl IntoElement {
        div()
            .h(px(31.0))
            .flex()
            .items_center()
            .px(px(8.0))
            .rounded(px(6.0))
            .when(active, |this| this.bg(rgb(0x202227)))
            .hover(|this| this.bg(rgb(0x1d2024)))
            .child(Icon::new(icon).size(px(14.0)).text_color(if active {
                rgb(0xd9dbe0)
            } else {
                rgb(0x777b83)
            }))
            .child(
                div()
                    .ml(px(9.0))
                    .flex_1()
                    .text_size(px(12.0))
                    .text_color(if active { rgb(0xe0e1e4) } else {rgb(0x858990)})
                    .child(label),
            )
            .when_some(count, |this,count|{
                this.child(
                    div()
                        .text_size(px(10.0))
                        .text_color(rgb(0x656970))
                        .child(count),
                )
            })
    }



    pub fn new() -> Self {
        Self {
            navigation: Navigation::ActiveIssues,
            active_tab: IssueTab::All,
            selected_issue: None,
            issues: vec![
                Issue {
                    id: "ENG-124",
                    title: "Fix atuthentication redirect",
                    group: "ENGINEERING",
                    team: "Backend",
                    assignee: "John",
                    updated: "12m",
                    priority: "HIGH",
                    priority_color: 0xd95c5c,
                    assigned_to_me: true,
                    created_by_me: false,
                    completed: false,
                },
                Issue {
                    id: "ENG-123",
                    title: "Improve settings page",
                    group: "ENGINEERING",
                    team: "Frontend",
                    assignee: "Sarah",
                    updated: "1h",
                    priority: "MEDIUM",
                    priority_color: 0xd9a441,
                    assigned_to_me: false,
                    created_by_me: true,
                    completed: false,
                },
                Issue {
                    id: "ENG-122",
                    title: "Update API documentation",
                    group: "ENGINEERING",
                    team: "Documentation",
                    assignee: "Alex",
                    updated: "3h",
                    priority: "LOW",
                    priority_color: 0x5b8fd8,
                    assigned_to_me: true,
                    created_by_me: true,
                    completed: false,
                },
                Issue {
                    id: "PRO-087",
                    title: "Redesign onboarding flow",
                    group: "PRODUCT",
                    team: "Product",
                    assignee: "Maya",
                    updated: "5h",
                    priority: "MEDIUM",
                    priority_color: 0xd9a441,
                    assigned_to_me: false,
                    created_by_me: false,
                    completed: false,
                },
                Issue {
                    id: "PRO-086",
                    title: "Add keyboard shortcuts",
                    group: "PRODUCT",
                    team: "Desktop",
                    assignee: "David",
                    updated: "1d",
                    priority: "LOW",
                    priority_color: 0x5b8fd8,
                    assigned_to_me: true,
                    created_by_me: false,
                    completed: false,
                },
            ]
        }
    }
}


impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x0f1912))
            .text_color(rgb(0xe8e9ea))
            .child(
                div()
                .size_full()
                .flex()
                .flex_row()
                .child(self.sidebar(cx))
                .child(self.main_content(cx)),
            )
    }
}

