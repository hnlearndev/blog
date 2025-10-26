use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

/// Home icon
#[component]
pub fn HomeIcon() -> impl IntoView {
    view! { <Icon icon=i::LuHouse width="27px" height="27px" /> }
}

/// Blog/Article icon
#[component]
pub fn BlogIcon() -> impl IntoView {
    view! { <Icon icon=i::LuNotebookPen width="27px" height="27px" /> }
}

/// Poem icon
#[component]
pub fn PoemIcon() -> impl IntoView {
    view! { <Icon icon=i::LuFileHeart width="27px" height="27px" /> }
}

/// Contact/Mail icon
#[component]
pub fn ContactIcon() -> impl IntoView {
    view! { <Icon icon=i::MdiEmailOutline width="27px" height="27px" /> }
}

/// GitHub icon
#[component]
pub fn GitHubIcon() -> impl IntoView {
    view! { <Icon icon=i::LuGithub width="27px" height="27px" /> }
}

/// LinkedIn icon
#[component]
pub fn LinkedInIcon() -> impl IntoView {
    view! { <Icon icon=i::LuLinkedin width="27px" height="27px" /> }
}

/// Menu/Hamburger icon for mobile
#[component]
pub fn MenuIcon() -> impl IntoView {
    view! { <Icon icon=i::LuMenu width="27px" height="27px" /> }
}

/// Close/X icon
#[component]
pub fn CloseIcon() -> impl IntoView {
    view! { <Icon icon=i::CgClose width="27px" height="27px" /> }
}

/// Project/Portfolio icon
#[component]
pub fn ProjectIcon() -> impl IntoView {
    view! { <Icon icon=i::AiFundProjectionScreenOutlined width="27px" height="27px" /> }
}

/// Resume/Document icon
#[component]
pub fn ResumeIcon() -> impl IntoView {
    view! { <Icon icon=i::TbFileCvOutline width="27px" height="27px" /> }
}

/// Sun icon for light theme
#[component]
pub fn SunIcon() -> impl IntoView {
    view! { <Icon icon=i::LuSun width="20px" height="20px" /> }
}

/// Moon icon for dark theme
#[component]
pub fn MoonIcon() -> impl IntoView {
    view! { <Icon icon=i::LuMoon width="20px" height="20px" /> }
}
