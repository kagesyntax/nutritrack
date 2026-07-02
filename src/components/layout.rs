use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::{
    FiBarChart2, FiClock, FiEdit3, FiHome, FiSettings,
};
use dioxus_free_icons::Icon;

use crate::app::Route;

#[component]
pub fn AppShell() -> Element {
    rsx! {
        div { class: "flex h-screen bg-background text-foreground",
            Sidebar {}
            main { class: "flex-1 overflow-y-auto bg-background pb-20 md:pb-0",
                Outlet::<Route> {}
            }
            MobileBottomNav {}
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        nav { class: "hidden md:flex w-64 bg-card border-r border-border p-6 flex-col shrink-0 h-screen",
            div { class: "flex items-center gap-3 px-3 py-4 mb-8",
                div { class: "w-10 h-10 rounded-xl bg-primary flex items-center justify-center text-white text-sm font-bold",
                    "N"
                }
                span { class: "font-heading font-semibold text-xl text-card-foreground", "NutriTrack" }
            }
            ul { class: "flex flex-col gap-1.5",
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
        "/" => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiHome } },
        "/log" => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiEdit3 } },
        "/history" => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiClock } },
        "/analytics" => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiBarChart2 } },
        "/settings" => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiSettings } },
        _ => VNode::empty(),
    };

    rsx! {
        li {
            Link {
                class: "flex items-center gap-3 px-3 py-2.5 rounded-lg text-muted-foreground hover:bg-muted hover:text-foreground transition-colors duration-150",
                active_class: "bg-primary/10 text-primary font-medium",
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
        nav { class: "fixed bottom-0 left-0 right-0 bg-card border-t border-border md:hidden z-50",
            div { class: "flex items-center justify-around py-1 px-2",
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
        "/" => rsx! { Icon { width: 20, height: 20, fill: "currentColor", icon: FiHome } },
        "/log" => rsx! { Icon { width: 20, height: 20, fill: "currentColor", icon: FiEdit3 } },
        "/history" => rsx! { Icon { width: 20, height: 20, fill: "currentColor", icon: FiClock } },
        "/analytics" => rsx! { Icon { width: 20, height: 20, fill: "currentColor", icon: FiBarChart2 } },
        "/settings" => rsx! { Icon { width: 20, height: 20, fill: "currentColor", icon: FiSettings } },
        _ => VNode::empty(),
    };

    rsx! {
        Link {
            class: "flex flex-col items-center gap-0.5 px-3 py-1.5 rounded-lg text-muted-foreground transition-colors duration-150 min-h-[48px] min-w-[48px] justify-center",
            active_class: "text-primary",
            to: to,
            {icon}
            span { class: "text-[10px] leading-tight font-medium", "{label}" }
        }
    }
}
