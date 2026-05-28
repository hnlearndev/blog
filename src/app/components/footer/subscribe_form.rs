//! Newsletter subscription form using rust-ui AutoForm

use autoform::AutoForm;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::components::ui::auto_form::AutoForm;
use crate::app::components::ui::button::Button;
use crate::app::hooks::use_form::use_form;

/// Subscriber data with validation and auto form generation
#[derive(Debug, AutoForm, Validate, Serialize, Deserialize, Clone, Default)]
pub struct SubscriberData {
    #[autoform(placeholder = "you@example.com", field_type = "email")]
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
}

/// Form states
#[derive(Debug, Clone, PartialEq)]
enum FormState {
    Idle,
    Loading,
    Success(String),
    Error(String),
}

/// Submit subscription to backend API
async fn submit_subscription(_data: SubscriberData) -> Result<String, String> {
    // Database feature is not yet available
    Ok("Thank you for your interest! The newsletter subscription feature is currently not available as we are still setting up the database. Please check back later.".to_string())
}

/// Newsletter subscription form component using rust-ui AutoForm
#[component]
pub fn SubscribeForm() -> impl IntoView {
    let form = use_form::<SubscriberData>();
    let (form_state, set_form_state) = signal(FormState::Idle);

    let on_submit = Callback::new(move |data: SubscriberData| {
        set_form_state.set(FormState::Loading);
        spawn_local(async move {
            match submit_subscription(data).await {
                Ok(msg) => {
                    set_form_state.set(FormState::Success(msg));
                    form.reset();
                }
                Err(err) => {
                    set_form_state.set(FormState::Error(err));
                }
            }
        });
    });

    let reset_form = move |_: leptos::ev::MouseEvent| {
        set_form_state.set(FormState::Idle);
        form.reset();
    };

    view! {
        <div class="subscribe-form">
            {move || match form_state.get() {
                FormState::Success(msg) => {
                    view! {
                        <div class="success">
                            <p>{msg}</p>
                            <button on:click=reset_form>"Subscribe another"</button>
                        </div>
                    }
                        .into_any()
                }
                _ => {
                    view! {
                        <AutoForm form=form on_submit=on_submit>
                            <Button attr:r#type="submit">
                                {move || match form_state.get() {
                                    FormState::Loading => "Subscribing...",
                                    _ => "Subscribe",
                                }}
                            </Button>
                        </AutoForm>
                    }
                        .into_any()
                }
            }}
            {move || match form_state.get() {
                FormState::Error(msg) => {
                    view! {
                        <div class="error">
                            <p>{msg}</p>
                            <button on:click=reset_form>"Try again"</button>
                        </div>
                    }
                        .into_any()
                }
                _ => view! { <div></div> }.into_any(),
            }}
        </div>
    }
}
