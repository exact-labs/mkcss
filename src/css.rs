use log;
use macros_rs::ternary;
use scraper::{Html, Selector};
use std::{collections::HashSet, error::Error, fs, fs::File, include_str, io::Write, path::Path, path::PathBuf};

pub fn get_classes(document: &Html) -> HashSet<String> {
    let mut classes = HashSet::new();

    for element in document.select(&Selector::parse("*").unwrap()) {
        if let Some(class_attr) = element.value().attr("class") {
            let element_classes: Vec<_> = class_attr.split_whitespace().collect();
            for elem_class in element_classes {
                classes.insert(String::from(elem_class));
            }
        }
    }
    classes
}

pub fn create_stylesheet(classes: Vec<&String>, add_reset: bool) -> String {
    let mut css_content = String::from(ternary!(add_reset, include_str!("reset.css"), ""));

    let sans = r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji""#;
    let serif = r#"font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif;"#;
    let mono = r#"font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#;

    let styles: [(&str, &str); 23] = [
        ("text-xs", "font-size: 10px"),
        ("text-sm", "font-size: 12px"),
        ("text-md", "font-size: 14px"),
        ("text-lg", "font-size: 18px"),
        ("text-xl", "font-size: 24px"),
        ("text-left", "text-align: left"),
        ("text-center", "text-align: center"),
        ("text-right", "text-align: right"),
        ("text-justify", "text-align: justify"),
        ("text-start", "text-align: start"),
        ("text-end", "text-align: end"),
        ("font-thin", "font-weight: 100"),
        ("font-extralight", "font-weight: 200"),
        ("font-light", "font-weight: 300"),
        ("font-normal", "font-weight: 400"),
        ("font-medium", "font-weight: 500"),
        ("font-semibold", "font-weight: 600"),
        ("font-bold", "font-weight: 700"),
        ("font-extrabold", "font-weight: 800"),
        ("font-black", "font-weight: 900"),
        ("font-sans", sans),
        ("font-serif", serif),
        ("font-mono", mono),
    ];

    let margin_styles: [(&str, &str); 7] = [
        ("m", "margin"),
        ("mt", "margin-top"),
        ("mr", "margin-right"),
        ("mb", "margin-bottom"),
        ("ml", "margin-left"),
        ("mx", "margin-left margin-right"),
        ("my", "margin-top margin-bottom"),
    ];

    for class in &classes {
        let parts: Vec<&str> = class.split('-').collect();
        let name = if class.starts_with("-") && parts.len() > 1 { parts[1] } else { parts[0] };

        match name {
            "text" => {
                if let Ok(size) = parts.get(1).unwrap_or(&"").parse::<u32>() {
                    css_content.push_str(&format!(".{} {{ font-size: {}px }}\n", class, size));
                } else if let Some(&(_, value)) = styles.iter().find(|(key, _)| key == *&class) {
                    css_content.push_str(&format!(".{} {{ {} }}\n", class, value));
                }
            }
            "color" => {
                css_content.push_str(&format!(".{} {{ color: #{} !important }}\n", class, parts.get(1).unwrap_or(&"")));
            }
            "font" => {
                if let Ok(weight) = parts.get(1).unwrap_or(&"").parse::<u32>() {
                    css_content.push_str(&format!(".{} {{ font-weight: {} }}\n", class, weight));
                } else if let Some(&(_, value)) = styles.iter().find(|(key, _)| key == *&class) {
                    css_content.push_str(&format!(".{} {{ {} }}\n", class, value));
                }
            }
            _ => {
                let property_value = parts
                    .get(if class.starts_with("-") { 2 } else { 1 })
                    .map(|&s| if class.starts_with("-") { format!("-{}", s) } else { s.to_string() })
                    .unwrap_or_else(String::new);

                if let Some(&(_, property)) = margin_styles.iter().find(|(prop_name, _)| prop_name == &name) {
                    let sign = if name.starts_with("-") { "-" } else { "" };
                    let declarations: Vec<String> = property.split_whitespace().map(|p| format!("{}: {}{}px", p, sign, property_value)).collect();
                    css_content.push_str(&format!(".{} {{ {} }}\n", class, declarations.join("; ")));
                }
            }
        }
    }

    css_content
}

pub fn minify(css: &str) -> String {
    let mut output = String::new();
    let mut iter = css.chars().peekable();
    let mut in_rule = false;

    while let Some(c) = iter.next() {
        match c {
            ':' => {
                in_rule = true;
                output.push(c);
            }
            ';' => {
                in_rule = false;
                output.push(c);
            }
            '/' if iter.peek() == Some(&'*') => {
                while let Some(c) = iter.next() {
                    if c == '*' {
                        if let Some('/') = iter.peek() {
                            iter.next();
                            break;
                        }
                    }
                }
            }

            space if space.is_whitespace() => {
                if in_rule {
                    output.push(' ');
                }
            }

            _ => output.push(c),
        }
    }

    output
}

pub fn write(path: &str, add_reset: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let document = Html::parse_document(&contents);
    let mut classes = HashSet::new();

    for class in get_classes(&document) {
        classes.insert(class);
    }

    let mut sorted_classes: Vec<_> = classes.iter().collect();
    sorted_classes.sort();

    let css_content = create_stylesheet(sorted_classes, add_reset);
    let selector = Selector::parse(r#"link[mkcss]"#).unwrap();
    let link = document.select(&selector).next();

    let parent = match Path::new(path).parent() {
        Some(parent_path) => parent_path,
        None => Path::new(path),
    };

    match link {
        Some(link) => {
            let href = link.value().attr("href").unwrap();
            let mut base_path = PathBuf::from(parent);

            base_path.push(&href);
            log::info!("export: {}", base_path.display());

            let dir_path = base_path.parent().expect("Failed to get directory path");
            fs::create_dir_all(dir_path)?;

            let mut file = File::create(base_path)?;
            write!(
                file,
                "{}\n/* generated by mkcss v{} (https://github.com/exact-labs/mkcss) */",
                minify(&css_content),
                env!("CARGO_PKG_VERSION")
            )?;

            Ok(println!("Stylesheet has been created successfully."))
        }
        None => Ok(println!("No 'util' attribute found. Program will exit.")),
    }
}
