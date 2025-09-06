use leptos::prelude::*;
use lucide_leptos::{Boxes, File, Github, House, Linkedin, Mail, Menu, Moon, NotebookPen, Sun, X};

/// Home icon
#[component]
pub fn HomeIcon() -> impl IntoView {
    view! {
        <House size=27  />
    }
}

/// Blog/Article icon
#[component]
pub fn BlogIcon() -> impl IntoView {
    view! {
        <NotebookPen size=27 />
    }
}

/// Contact/Mail icon
#[component]
pub fn ContactIcon() -> impl IntoView {
    view! {
        <Mail size=27 />
    }
}

/// GitHub icon
#[component]
pub fn GitHubIcon() -> impl IntoView {
    view! {
        <Github size=27 />
    }
}

/// LinkedIn icon
#[component]
pub fn LinkedInIcon() -> impl IntoView {
    view! {
        <Linkedin size=27 />
    }
}

/// Menu/Hamburger icon for mobile
#[component]
pub fn MenuIcon() -> impl IntoView {
    view! {
        <Menu size=27 />
    }
}

/// Close/X icon
#[component]
pub fn CloseIcon() -> impl IntoView {
    view! {
        <X size=27 />
    }
}

/// Project/Portfolio icon
#[component]
pub fn ProjectIcon() -> impl IntoView {
    view! {
        <Boxes size=27 />
    }
}

/// Resume/Document icon
#[component]
pub fn ResumeIcon() -> impl IntoView {
    view! {
        <File size=27 />
    }
}

/// Sun icon for light theme
#[component]
pub fn SunIcon() -> impl IntoView {
    view! {
        <Sun size=20 />
    }
}

/// Moon icon for dark theme
#[component]
pub fn MoonIcon() -> impl IntoView {
    view! {
        <Moon size=20 />
    }
}
