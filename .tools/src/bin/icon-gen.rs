use heck::CamelCase;
use scraper::{Html, Selector};
use std::io::Write;

// lazy_static! {
//     static ref LIST_ITEM_REGEX = Regex::new(r"");
// }

struct Icon {
    name: String,
    unicode: String,
    link: String,
}

// fn main() {
//     let args = std::env::args().collect::<Vec<_>>();
//     let path = args.last().expect("Input .html path required");

//     println!("Reading file: {}", path);
//     let contents = std::fs::read_to_string(path).expect("failed to read file");
//     println!("...{} lines", contents.lines().count());
//     let contents = contents.replace("\n", " ");

//     let list_item_regex = Regex::new(
//         r#"<li class="(?P<cls>[A-z0-9-\s]*">)"#,
//         //       # ... with a link
//         // <i class=\"fas (fa-[a-z-])\">   # ... and an icon
//     )
//     .unwrap();

//     let mut icons = Vec::with_capacity(1001);
//     for captures in list_item_regex.captures_iter(&contents) {
//         // TODO
//         // icons.push(Icon {
//         //     name: String::new(),
//         //     unicode: String::new(),
//         //     link: String::new(),
//         // })
//         icons.push(format!("{:#?}", captures.name("cls")));
//     }

//     println!("found: {} items", icons.len());
//     let output = icons.join("\n\n");
//     let mut file = std::fs::File::create("./tmp/out").expect("Failed to open output file");
//     file.write_all(output.as_bytes())
//         .expect("Failed to write file")
// }

const TARGET_FILE: &str = "./src/components/icon_kind.rs";

const BEFORE: &str = r#"
///! Automatically generated code. Do not manually edit
use std::convert::AsRef;
use strum::AsRefStr;

impl IconKind {
    pub fn name(&self) -> String {
        let kind = *self;
        let name = if let IconKind::Custom(name) = kind {
            name
        } else {
            kind.as_ref()
        };
        format!("fa-{}", name)
    }
}

/// A Fontawesome free/solid icon.
///
/// There are a lot (>1000) possible icons here,
#[derive(Debug, AsRefStr, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    #[strum(disabled)]
    Custom(&'static str),
"#;

const AFTER: &str = r#"
}
"#;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let path = args.last().expect("Input .html path required");

    let icon_defs = parse_icons(path)
        .into_iter()
        .map(
            |Icon {
                 name,
                 unicode,
                 link,
             }: Icon| {
                let variant_name = name.to_camel_case();
                let class_name = format!("fa-{}", name);
                format!(
                    "
    /// {}: \\u{}
    ///
    /// https://fontawesome.com{}
    #[strum(serialize = \"{}\")]
    {},
                    ",
                    name, unicode, link, class_name, variant_name
                )
            },
        )
        .collect::<String>();

    let output = format!("{}{}{}", BEFORE, icon_defs, AFTER);

    let mut file = std::fs::File::create(TARGET_FILE).expect("Failed to open output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write file")
}

fn parse_icons(path: &str) -> Vec<Icon> {
    println!("Reading file: {}", path);
    let contents = std::fs::read_to_string(path).expect("failed to read file");
    println!("...{} lines", contents.lines().count());

    let doc = Html::parse_document(&contents);

    let wrapping_selector = Selector::parse("#results-icons>ul>li").unwrap();
    let href_selector = Selector::parse("div>a").unwrap();
    // let icon_selector = Selector::parse("div>a>i").unwrap();
    let name_selector = Selector::parse("div>div>span").unwrap();
    let unicode_selector = Selector::parse("div>div>span>span").unwrap();

    let mut items = Vec::with_capacity(1001);
    // println!("selection: {:?}", doc.select(&wrapping_selector));
    for element in doc.select(&wrapping_selector) {
        let link = element
            .select(&href_selector)
            .next()
            .expect("no anchor")
            .value()
            .attr("href")
            .expect("no href")
            .to_string();

        let name = element
            .select(&name_selector)
            .next()
            .expect("no name span")
            .text()
            .collect::<String>();

        let unicode = element
            .select(&unicode_selector)
            .last()
            .expect("no unicode span")
            .text()
            .collect::<String>();

        items.push(Icon {
            name,
            link,
            unicode,
        });
    }

    println!("found: {} items", items.len());
    items
}
