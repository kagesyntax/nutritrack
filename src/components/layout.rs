use dioxus::prelude::*;

use crate::app::Route;
use crate::components::icons::{IconBarChart2, IconClock, IconEdit3, IconHome, IconSettings};

#[component]
pub fn AppShell() -> Element {
    rsx! {
        div { class: "app-shell",
            Sidebar {}
            main { class: "main-content",
                div { class: "page-transition page-transition-enter",
                    Outlet::<Route> {}
                }
            }
            MobileBottomNav {}
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        nav { class: "sidebar",
            div { class: "flex items-center gap-3 px-3 py-4 mb-8",
                div { class: "logo", "N" }
                span { class: "font-heading font-semibold text-xl text-card-foreground", "NutriTrack" }
            }
            ul { class: "flex flex-col gap-1_5",
                NavLink { to: "/", label: "Dashboard" }
                NavLink { to: "/log", label: "Log" }
                NavLink { to: "/history", label: "History" }
                NavLink { to: "/analytics", label: "Analytics" }
                NavLink { to: "/settings", label: "Settings" }
            }
        }
    }
}

#[component]
fn NavLink(to: &'static str, label: &'static str) -> Element {
    let icon = match to {
        "/" => rsx! { IconHome { size: 24 } },
        "/log" => rsx! { IconEdit3 { size: 24 } },
        "/history" => rsx! { IconClock { size: 24 } },
        "/analytics" => rsx! { IconBarChart2 { size: 24 } },
        "/settings" => rsx! { IconSettings { size: 24 } },
        _ => VNode::empty(),
    };

    rsx! {
        li {
            Link {
                class: "nav-link",
                active_class: "nav-link-active",
                to: to,
                {icon}
                span { "{label}" }
            }
        }
    }
}

#[component]
fn MobileBottomNav() -> Element {
    rsx! {
        nav { class: "mobile-bottom-nav",
            div { class: "flex items-center justify-around py-1 px-2 w-full",
                MobileTab { to: "/", label: "Home" }
                MobileTab { to: "/log", label: "Log" }
                MobileTab { to: "/history", label: "History" }
                MobileTab { to: "/analytics", label: "Analytics" }
                MobileTab { to: "/settings", label: "Settings" }
            }
        }
    }
}

#[component]
fn MobileTab(to: &'static str, label: &'static str) -> Element {
    let icon = match to {
        "/" => rsx! { IconHome { size: 24 } },
        "/log" => rsx! { IconEdit3 { size: 24 } },
        "/history" => rsx! { IconClock { size: 24 } },
        "/analytics" => rsx! { IconBarChart2 { size: 24 } },
        "/settings" => rsx! { IconSettings { size: 24 } },
        _ => VNode::empty(),
    };

    rsx! {
        Link {
            class: "mobile-tab",
            active_class: "mobile-tab-active",
            to: to,
            {icon}
            span { class: "text-xxs leading-tight font-medium", "{label}" }
        }
    }
}
