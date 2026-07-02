use dioxus::prelude::*;

use crate::components::layout::AppShell;
use crate::routes::analytics::Analytics;
use crate::routes::dashboard::Dashboard;
use crate::routes::history::History;
use crate::routes::log::Log;
use crate::routes::meal_detail::MealDetail;
use crate::routes::settings::Settings;
use crate::state::models::DayLog;
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

    ensure_today_exists(logs);

    use_effect(move || {
        persistence::save_logs(&logs.read());
        persistence::save_settings(&settings.read());
    });

    use_context_provider(|| logs);
    use_context_provider(|| settings);

    rsx! {
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
