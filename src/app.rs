
use gpui_kit::prelude::*;
use gpui_kit::gpui::{div, rgb, Window, Context};

use crate::sidebar::{
    sidebar 
};
use crate::main_content::main_content;

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

