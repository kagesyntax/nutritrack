use dioxus::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use crate::components::layout::AppShell;
use crate::routes::analytics::Analytics;
use crate::routes::dashboard::Dashboard;
use crate::routes::history::History;
use crate::routes::log::Log;
use crate::routes::meal_detail::MealDetail;
use crate::routes::settings::Settings;
use crate::state::models::{DayLog, ThemeMode};
use crate::state::persistence;
use crate::utils::todays_date;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
    #[route("/")]
    Dashboard {},
    #[route("/log")]
    Log {},
    #[route("/meal/:id")]
    MealDetail { id: String },
    #[route("/history")]
    History {},
    #[route("/analytics")]
    Analytics {},
    #[route("/settings")]
    Settings {},
}

#[component]
pub fn App() -> Element {
    let logs = use_signal(|| persistence::load_logs());
    let settings = use_signal(|| persistence::load_settings());
    let mut system_dark = use_signal(|| false);

    ensure_today_exists(logs);

    use_effect(move || {
        persistence::save_logs(&logs.read());
        persistence::save_settings(&settings.read());
    });

    // Keep the system theme listener closure alive
    let mut listener = use_signal(|| None::<Closure<dyn FnMut()>>);

    // Listen to system color scheme preference (runs once on mount)
    use_effect(move || {
        let window = web_sys::window();
        let mql = window
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
            .flatten();
        if let Some(mql) = mql {
            system_dark.set(mql.matches());
            let mut sd = system_dark.clone();
            let cb = Closure::wrap(Box::new(move || {
                if let Some(Some(mql)) = web_sys::window()
                    .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
                {
                    sd.set(mql.matches());
                }
            }) as Box<dyn FnMut()>);
            mql.set_onchange(Some(cb.as_ref().unchecked_ref()));
            listener.set(Some(cb));
        }
    });

    // Apply theme attribute when settings or system preference changes
    use_effect(move || {
        let theme = settings.read().theme_mode;
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = doc.document_element() {
                match theme {
                    ThemeMode::Light => {
                        let _ = html.set_attribute("data-theme", "light");
                    }
                    ThemeMode::Dark => {
                        let _ = html.set_attribute("data-theme", "dark");
                    }
                    ThemeMode::System => {
                        if system_dark() {
                            let _ = html.set_attribute("data-theme", "dark");
                        } else {
                            let _ = html.set_attribute("data-theme", "light");
                        }
                    }
                }
            }
        }
    });

    use_context_provider(|| logs);
    use_context_provider(|| settings);

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}

fn ensure_today_exists(mut logs: Signal<Vec<DayLog>>) {
    let today = todays_date();
    let has_today = logs.read().iter().any(|l| l.date == today);
    if !has_today {
        logs.write().push(DayLog::new(today));
    }
}
