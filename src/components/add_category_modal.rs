use leptos::prelude::*;

#[component]
pub fn AddCategoryModal() -> impl IntoView {
    view! {
        <div class="modal" id="add-category-modal">
            <h1>
                "Add something that you want to track"
            </h1>
            <form class="row">
                <label for="category-name">
                    "Category name?"
                </label>
                <input type="text" id="cateogry-name" name="category-name" placeholder="eg Meditating, Drinking water, or Smoking cigarettes"/>
                <input type="submit" value="Submit"/>
            </form>
        </div>
    }
}
