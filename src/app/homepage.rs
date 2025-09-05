use super::helpers::{FastA, get_post_metadata_list};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Willian Nguyen - Home" />
        <h1>"Hi there, I'm Willian 👋"</h1>
        <p>"I am a curious learner with a genuine love for new technologies."</p>
        <p>"Anything that I learn and find interesting, I will write about it on my blog."</p>
        
        <PostList />
    }
}

#[component]
fn PostList() -> impl IntoView {
    let posts = get_post_metadata_list();

    view! {
        <table>
            <thead>
                <tr>
                    <th scope="col">Title</th>
                    <th scope="col">Date</th>
                </tr>
            </thead>
            <tbody>
                {posts
                    .into_iter()
                    .map(|post| {
                        let path = format!("/posts/{}", post.id);
                        view! {
                            <tr>
                                <th scope="row">
                                    <FastA href=path.clone() class="contrast">
                                        {post.title}
                                    </FastA>
                                </th>
                                <th scope="row">{post.date}</th>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}
