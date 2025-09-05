use super::subscribe_form::SubscribeForm;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer-enhanced">
            <div class="container">
                <div class="newsletter-container">
                    <label for="newsletter-email" class="newsletter-label">
                        "NEWSLETTER SUBSCRIBE"
                    </label>
                    <div class="newsletter-form-row" style="display: flex; align-items: center; gap: 0.5rem;">
                        <SubscribeForm />
                    </div>
                </div>
            </div>
        </footer>
    }
}
