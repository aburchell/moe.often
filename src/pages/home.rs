use crate::components::alternate_screen::AlternateScreen;
use crate::components::{category_title::Title, input::Input, output::Output};
use crate::data::{State, StateStoreFields, UserLifecycleStage};
use leptos::prelude::*;
use reactive_stores::Store;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let user_interaction_stage = state.interaction_stage();
    let is_main_screen = move || user_interaction_stage.get() == UserLifecycleStage::Regular;
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
                when=move || { is_main_screen() }
                fallback=|| view! { <AlternateScreen/> }>
                <div class="container">
                    <Title />
                    <Output />
                    <Input />
                </div>
            </Show>
        </ErrorBoundary>
    }
}
