use super::fast_a::FastA;
use crate::app::helpers::ContentMetadata;
use leptos::prelude::*;

#[component]
pub fn ContentList(items: Vec<ContentMetadata>, route_prefix: &'static str) -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th scope="col">Title</th>
                    <th scope="col">Date</th>
                </tr>
            </thead>
            <tbody>
                {items
                    .into_iter()
                    .map(|item| {
                        let path = format!("{}/{}", route_prefix, item.id);
                        view! {
                            <tr>
                                <th scope="row">
                                    <FastA href=path.clone() class="contrast">
                                        {item.title.clone()}
                                    </FastA>
                                </th>
                                <th scope="row">{item.date.clone()}</th>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}
