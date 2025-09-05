use chrono::NaiveDate;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use std::{env, fs, path::Path};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{
        IncludeBackground, append_highlighted_html_for_styled_line, start_highlighted_html_snippet,
    },
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

fn main() {
    // To rerun the build script if contents changes
    println!("cargo::rerun-if-changed=contents");

    // Generate output file path
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("posts_data.rs");

    // Obtain entries for all posts from contents folder
    let posts = posts_list_from_md_files(Path::new("./contents/posts"));

    // Write out the Rust source file with the posts data
    write_out_list(posts, &dest_path);
}

fn write_out_list(posts: Vec<(u32, NaiveDate, String, String)>, dest_path: &Path) {
    let entries: Vec<String> = posts
        .iter()
        .map(|(id, date, title, html)| {
            format!(
                "    (\"{}\", \"{}\", \"{}\", r###\"{}\"###),",
                id,
                date.format("%d-%b-%Y"), // DD-MMM-YYYY eg: 05-Sep-2025
                title,
                html
            )
        })
        .collect();

    fs::write(
        dest_path,
        format!(
            "static POSTS: &[(&str, &str, &str, &str)] = &[\n{}\n];",
            entries.join("\n")
        ),
    )
    .unwrap();
}

// Read markdown files from the specified directory and return a list of posts
// Purposely control types u32 and NaiveDate
fn posts_list_from_md_files(path: &Path) -> Vec<(u32, NaiveDate, String, String)> {
    // Read all markdown files in the directory
    let mut posts: Vec<_> = fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let file_path = entry.ok()?.path();
            let file_name = file_path.file_name()?.to_str()?;
            let parts: Vec<&str> = file_name.trim_end_matches(".md").splitn(3, '-').collect();
            if parts.len() != 3 {
                return None;
            }
            let id = parts[0].parse::<u32>().ok()?;
            let date = NaiveDate::parse_from_str(parts[1], "%Y%m%d").ok()?;
            let markdown = fs::read_to_string(&file_path).ok()?;
            let title = markdown
                .lines()
                .next()
                .and_then(|line| line.strip_prefix("# "))?
                .to_string();
            let html = markdown_to_html(&markdown);
            Some((id, date, title, html))
        })
        .collect();

    // Sort posts by id to ensure consistent order
    posts.sort_by_key(|(id, _, _, _)| *id);

    posts
}

fn markdown_to_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(markdown, options);
    let events = highlight(parser).into_iter();
    let mut html = String::new();
    html::push_html(&mut html, events);

    html
}

fn highlight(parser: Parser) -> Vec<Event> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let plain_text = syntax_set.find_syntax_plain_text();
    let mut syntax = plain_text;
    let theme = &ThemeSet::load_defaults().themes["base16-ocean.dark"];
    let mut events = Vec::new();
    let mut to_highlight = String::new();
    let mut in_code_block = false;

    parser.into_iter().for_each(|event| match event {
        Event::Start(Tag::CodeBlock(kind)) => {
            match kind {
                CodeBlockKind::Fenced(lang) => {
                    syntax = syntax_set.find_syntax_by_token(&lang).unwrap_or(plain_text)
                }
                CodeBlockKind::Indented => {}
            }
            in_code_block = true;
        }
        Event::End(TagEnd::CodeBlock) => {
            if in_code_block {
                // Not using highlighted_html_for_string() because we need to inject the <code> tag
                // for PicoCSS to apply the correct margin
                let mut highlighter = HighlightLines::new(syntax, theme);
                let (mut html, background) = start_highlighted_html_snippet(theme);

                html.push_str("<code>");
                for line in LinesWithEndings::from(&to_highlight) {
                    let regions = highlighter.highlight_line(line, &syntax_set).unwrap();
                    append_highlighted_html_for_styled_line(
                        &regions[..],
                        IncludeBackground::IfDifferent(background),
                        &mut html,
                    )
                    .unwrap();
                }
                html.push_str("</code></pre>\n");

                events.push(Event::Html(CowStr::from(html)));
                to_highlight.clear();
                in_code_block = false;
            }
        }
        Event::Text(t) => {
            if in_code_block {
                to_highlight.push_str(&t);
            } else {
                events.push(Event::Text(t))
            }
        }
        e => {
            events.push(e);
        }
    });

    events
}
