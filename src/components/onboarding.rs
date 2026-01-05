use leptos::prelude::*;

#[component]
pub fn Onboarding() -> impl IntoView {
    view! {
        <h1>
            "This is a personal, private data-logger."
        </h1>
        <p>
            "Free always. No account. No ads.
            Your data never leaves your device."
        </p>
        <p>
            <h3>"Keep track of anything."</h3>
            // TODO make examples cycle through
            //      focused list item, with others in
            //      (wrapping) background, above and below
                 
            <ul class="examples" id="categories-examples">
                <li>"Mood"</li>
                <li>"Smoking"</li>
                <li>"Eating healthy"</li>
                <li>"Doing pushups"</li>
                <li>"Meditating"</li>
            </ul>
        </p>
        <p>
            <h3>"Choose what information you log."</h3>
            <ul class="examples" id="inputs-examples">
                <li>"Occurances -- when, eg you drink water"</li>
                <li>"Choice -- pick some options to choose from, eg Good or Bad"</li>
                <li>"Quantity -- how many, eg number of pushups completed"</li>
                <li>"Continuous -- pick a point along some scale, eg from Happy to Sad"</li>
            </ul>
        </p>
        <p>
            <h3>"Make sense of your logged data."</h3>
            <ul class="examples" id="outputs-examples">
                <li>"A short summary"</li>
                <li>"A full set of statistics"</li>
                <li>"A graph with all your data."</li>
            </ul>
        </p>

        <button>"Add your first category and start now!"</button>
    }
}
