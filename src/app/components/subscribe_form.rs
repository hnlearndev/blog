//! Minimal newsletter subscription form component

#[cfg(feature = "hydrate")]
use crate::shared::dto::SubscribeResponse;

use garde::Validate;
use leptos::prelude::*;
use leptos::{ev, task::spawn_local};
use serde::{Deserialize, Serialize};

/// Subscriber data with validation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SubscriberData {
    #[garde(email)]
    pub email: String,

    #[garde(ip)]
    pub ip_address: Option<String>,
}

/// Form states
#[derive(Debug, Clone, PartialEq)]
enum FormState {
    Idle,
    Loading,
    Success(String),
    Error(String),
}

/// Get user's public IP address from external service
async fn get_ip() -> Option<String> {
    #[cfg(feature = "hydrate")]
    {
        use gloo_net::http::Request;
        Request::get("https://api.ipify.org?format=text")
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()
            .map(|ip| ip.trim().to_string())
    }

    #[cfg(not(feature = "hydrate"))]
    {
        None
    }
}

/// Submit subscription to backend API
async fn submit_subscription(data: SubscriberData) -> Result<String, String> {
    // Validate email using garde
    data.validate()
        .map_err(|e| format!("Validation error: {}", e))?;

    #[cfg(feature = "hydrate")]
    {
        use gloo_net::http::Request;

        match Request::post("/api/subscribe")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "email": data.email,
                "ip_address": data.ip_address,
                "location": None::<String>,
                "user_agent": None::<String>
            }))
            .map_err(|e| format!("Request creation failed: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?
        {
            response if response.ok() => match response.json::<SubscribeResponse>().await {
                Ok(resp) => Ok(resp.message),
                Err(_) => Ok("Successfully subscribed!".to_string()),
            },
            response => match response.json::<SubscribeResponse>().await {
                Ok(resp) => Err(resp.message),
                Err(_) => Err(format!("HTTP error: {}", response.status())),
            },
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // Server-side fallback
        Ok("Successfully subscribed! Check your email for confirmation.".to_string())
    }
}

/// Minimal subscription form component
#[component]
pub fn SubscribeForm() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (form_state, set_form_state) = signal(FormState::Idle);

    let do_submit = move || {
        let email_value = email.get().trim().to_string();

        if email_value.is_empty() {
            return;
        }

        set_form_state(FormState::Loading);

        spawn_local(async move {
            let subscriber_data = SubscriberData {
                email: email_value,
                ip_address: get_ip().await,
            };

            match submit_subscription(subscriber_data).await {
                Ok(msg) => {
                    set_form_state(FormState::Success(msg));
                    set_email(String::new());
                }
                Err(err) => {
                    set_form_state(FormState::Error(err));
                }
            }
        });
    };

    let reset_form = move |_| {
        set_form_state(FormState::Idle);
    };

    view! {
        <div class="subscribe-form" on:submit=move |ev: ev::SubmitEvent| ev.prevent_default()>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <input
                    type="email"
                    placeholder="Enter your email"
                    value=email
                    on:input=move |ev| set_email(event_target_value(&ev))
                    disabled=move || matches!(form_state.get(), FormState::Loading)
                    required
                    style="flex: 1; min-width: 0;"
                />
                <button
                    type="button"
                    on:click=move |_| do_submit()
                    disabled=move || matches!(form_state.get(), FormState::Loading)
                    style="white-space: nowrap;"
                >
                    {move || match form_state.get() {
                        FormState::Loading => "Subscribing...",
                        _ => "Subscribe"
                    }}
                </button>
            </div>

            {move || match form_state.get() {
                FormState::Success(msg) => view! {
                    <div class="success">
                        <p>{msg}</p>
                        <button on:click=reset_form>"Subscribe another"</button>
                    </div>
                }.into_any(),
                FormState::Error(msg) => view! {
                    <div class="error">
                        <p>{msg}</p>
                        <button on:click=reset_form>"Try again"</button>
                    </div>
                }.into_any(),
                FormState::Idle | FormState::Loading => view! { <div></div> }.into_any(),
            }}
        </div>
    }
}
