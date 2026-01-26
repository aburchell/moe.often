use crate::components::{add_category_modal::AddCategoryModal, onboarding::Onboarding};
use crate::data::{State, StateStoreFields, UserLifecycleStage};
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn AlternateScreen() -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let user_interaction_stage = state.interaction_stage();
    view! {
        {move || match user_interaction_stage.get() {
            UserLifecycleStage::AddingCategoryModal => view! { <AddCategoryModal /> }.into_any(),
            UserLifecycleStage::Onboarding => view!{ <Onboarding /> }.into_any(),
            _ => view! {}.into_any()}
        }
    }
}
