use leptos::{
    logging::{debug_error, debug_log},
    prelude::*,
};
use leptos_meta::*;
use leptos_router::{components::*, path};
use reactive_stores::Store;

// Modules
mod components;
mod data;
mod pages;
mod storage;

use data::State;
use storage::initialize;

// Top-Level pages
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let starting_state: State = match initialize() {
        Ok(s) => s,
        Err(e) => {
            debug_error!("Failed to initialize. {}", e);
            State::default()
        }
    };
    debug_log!("Starting state: {:?}", starting_state);
    // App state
    provide_context(Store::new(starting_state));

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="moe often? less?" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
