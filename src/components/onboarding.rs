use leptos::prelude::*;

#[component]
pub fn Onboarding() -> impl IntoView {
    view! {
        <div id="onboarding-container" class="container">
            <section>
                <h2>
                    "This is a personal, private data-logger."
                </h2>
                <p>
                    "Free. No account. No ads. Your data never leaves your device."
                </p>
            </section>
            <section>
                <h3>"Keep track of anything."</h3>

                <ul class="examples" id="categories-examples">
                    <li>"Mood"</li>
                    <li>"Smoking"</li>
                    <li>"Eating healthy"</li>
                    <li>"Doing pushups"</li>
                    <li>"Meditating"</li>
                </ul>
                <h3>"Choose what information you log."</h3>
                <ul class="examples" id="inputs-examples">
                    <li>"Occurances -- when, eg you drink water"</li>
                    <li>"Choice -- pick some options to choose from, eg Good or Bad"</li>
                    <li>"Quantity -- how many, eg number of pushups completed"</li>
                    <li>"Continuous -- pick a point along some scale, eg from Happy to Sad"</li>
                </ul>
                <h3>"Make sense of your logged data."</h3>
                <ul class="examples" id="outputs-examples">
                    <li>"A short summary"</li>
                    <li>"A full set of statistics"</li>
                    <li>"A graph with all your data."</li>
                </ul>
            </section>
            <div id="hint">"Add a category to start keeping track of!"</div>
            <button id="add-category-button" type="button">"+"</button>
        </div>
    }
}
