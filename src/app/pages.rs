mod homepage;
mod poempage;
mod postpage;

// Re-export pages for easier access
pub use homepage::HomePage;
pub use poempage::{PoemListPage, SinglePoemPage};
pub use postpage::{PostListPage, SinglePostPage};
