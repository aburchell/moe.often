use crate::components::{
    category_title::Title,
    input::Input,
    output::Output,
    onboarding::Onboarding,
};
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"ERROR"</h1>

                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <Title />
                <Output />
                <Input />
            </div>
        </ErrorBoundary>
    }
}
