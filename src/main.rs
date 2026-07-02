mod app;
mod components;
mod routes;
mod state;
mod utils;

fn main() {
    dioxus::launch(app::App);
}
