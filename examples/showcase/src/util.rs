macro_rules! snippet {
    ($file:literal, $start:literal:$end:literal) => {{
        let content = include_str!($file)
            .lines()
            .skip($start as usize)
            .take(($end - $start) as usize)
            .collect::<Vec<_>>()
            .join("\n");
        crate::util::hightlight_snippet(&content)
    }};
}

// TODO: Html highlighting on macro inside
pub fn hightlight_snippet(content: &str) -> String {
    let ss = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let syntax = ss
        .find_syntax_by_name("Rust")
        .expect("failed to load rust syntax reference for highlighting");
    let ts = syntect::highlighting::ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    format!(
        "<style>pre{{white-space: pre-wrap;word-wrap: break-word;}}</style>{}",
        syntect::html::highlighted_html_for_string(content, &ss, syntax, theme)
    )
}
