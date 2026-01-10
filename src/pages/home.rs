use crate::components::{
    category_title::Title, input::Input, onboarding::Onboarding, output::Output,
};
use crate::data::{State, StateStoreFields, UserLifecycleStage};
use leptos::prelude::*;
use reactive_stores::Store;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let user_interaction_stage = state.interaction_stage();
    let in_onboarding = move || user_interaction_stage.get() == UserLifecycleStage::Onboarding;
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
            <Show
                when=move || { !in_onboarding() }
                fallback=|| view! { <Onboarding/> }
            >
                <div class="container">
                    <Title />
                    <Output />
                    <Input />
                </div>
            </Show>
        </ErrorBoundary>
    }
}
