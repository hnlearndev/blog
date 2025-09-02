use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="Willian Nguyen - About" />
        <h1>About</h1>
        <p>
            "I am a curious learner with a genuine love for new technologies."
        </p>
        <p>
            "Anything that I learn and find interesting, I will write about it on my blog. You can check it out "
            <a target="_blank" href="https://github.com/hnlearndev">
                here
            </a> ". Hope you find them useful!"
        </p>
        <p>
            <ul>
                <li>
                    <a target="_blank" href="mailto:hieunt.hello@gmail.com">
                        Email
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://github.com/hnlearndev">
                        GitHub
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://linkedin.com/in/hieunthello">
                        LinkedIn
                    </a>
                </li>
            </ul>
        </p>
    }
}
