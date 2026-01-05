use leptos::prelude::*;
use crate::components:: {
    summary::Summary,
    statistics::Statistics,
    logs::Logs,
};

#[component]
pub fn Output() -> impl IntoView {
    view! {
        <div id="output" class="output-container">
            <Summary />
            <Statistics />
            <Logs />
        </div>
    }
}
