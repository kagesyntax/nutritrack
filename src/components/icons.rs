use dioxus::prelude::*;

macro_rules! icon {
    ($name:ident, $view_box:expr, $($svg:tt)*) => {
        #[component]
        pub fn $name(size: Option<u32>, class: Option<&'static str>) -> Element {
            let s = size.unwrap_or(24);
            let cls = class.unwrap_or("");
            rsx! {
                svg {
                    width: "{s}",
                    height: "{s}",
                    view_box: $view_box,
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "{cls}",
                    $($svg)*
                }
            }
        }
    };
}

icon!(IconHome, "0 0 24 24",
    path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
    polyline { points: "9 22 9 12 15 12 15 22" }
);

icon!(IconEdit3, "0 0 24 24",
    path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
    path { d: "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" }
);

icon!(IconPlus, "0 0 24 24",
    line { x1: "12", y1: "5", x2: "12", y2: "19" }
    line { x1: "5", y1: "12", x2: "19", y2: "12" }
);

icon!(IconX, "0 0 24 24",
    line { x1: "18", y1: "6", x2: "6", y2: "18" }
    line { x1: "6", y1: "6", x2: "18", y2: "18" }
);

icon!(IconSearch, "0 0 24 24",
    circle { cx: "11", cy: "11", r: "8" }
    line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
);

icon!(IconSunrise, "0 0 24 24",
    path { d: "M17 18a5 5 0 0 0-10 0" }
    line { x1: "12", y1: "2", x2: "12", y2: "9" }
    line { x1: "4.22", y1: "10.22", x2: "5.64", y2: "11.64" }
    line { x1: "1", y1: "18", x2: "3", y2: "18" }
    line { x1: "21", y1: "18", x2: "23", y2: "18" }
    line { x1: "18.36", y1: "11.64", x2: "19.78", y2: "10.22" }
    line { x1: "23", y1: "22", x2: "1", y2: "22" }
);

icon!(IconSun, "0 0 24 24",
    circle { cx: "12", cy: "12", r: "5" }
    line { x1: "12", y1: "1", x2: "12", y2: "3" }
    line { x1: "12", y1: "21", x2: "12", y2: "23" }
    line { x1: "4.22", y1: "4.22", x2: "5.64", y2: "5.64" }
    line { x1: "18.36", y1: "18.36", x2: "19.78", y2: "19.78" }
    line { x1: "1", y1: "12", x2: "3", y2: "12" }
    line { x1: "21", y1: "12", x2: "23", y2: "12" }
    line { x1: "4.22", y1: "19.78", x2: "5.64", y2: "18.36" }
    line { x1: "18.36", y1: "5.64", x2: "19.78", y2: "4.22" }
);

icon!(IconMoon, "0 0 24 24",
    path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
);

icon!(IconCoffee, "0 0 24 24",
    path { d: "M18 8h1a4 4 0 0 1 0 8h-1" }
    path { d: "M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z" }
    line { x1: "6", y1: "1", x2: "6", y2: "4" }
    line { x1: "10", y1: "1", x2: "10", y2: "4" }
    line { x1: "14", y1: "1", x2: "14", y2: "4" }
);

icon!(IconClock, "0 0 24 24",
    circle { cx: "12", cy: "12", r: "10" }
    polyline { points: "12 6 12 12 16 14" }
);

icon!(IconBarChart2, "0 0 24 24",
    line { x1: "18", y1: "20", x2: "18", y2: "10" }
    line { x1: "12", y1: "20", x2: "12", y2: "4" }
    line { x1: "6", y1: "20", x2: "6", y2: "14" }
);

icon!(IconSettings, "0 0 24 24",
    circle { cx: "12", cy: "12", r: "3" }
    path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
);

icon!(IconChevronRight, "0 0 24 24",
    polyline { points: "9 18 15 12 9 6" }
);

icon!(IconCalendar, "0 0 24 24",
    rect { x: "3", y: "4", width: "18", height: "18", rx: "2", ry: "2" }
    line { x1: "16", y1: "2", x2: "16", y2: "6" }
    line { x1: "8", y1: "2", x2: "8", y2: "6" }
    line { x1: "3", y1: "10", x2: "21", y2: "10" }
);

icon!(IconMinus, "0 0 24 24",
    line { x1: "5", y1: "12", x2: "19", y2: "12" }
);

icon!(IconArrowLeft, "0 0 24 24",
    line { x1: "19", y1: "12", x2: "5", y2: "12" }
    polyline { points: "12 19 5 12 12 5" }
);
