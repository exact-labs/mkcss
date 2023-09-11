use log;
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

pub fn create_stylesheet(classes: Vec<&String>) -> String {
    let mut css_content = String::from(include_str!("reset.css"));

    let styles: [(&str, &str); 8] = [
        ("text-xs", "font-size: 10px"),
        ("text-sm", "font-size: 15px"),
        ("text-md", "font-size: 20px"),
        ("text-lg", "font-size: 25px"),
        ("text-xl", "font-size: 30px"),
        ("font-light", "font-weight: 300"),
        ("font-bold", "font-weight: 800"),
        ("font-extrabold", "font-weight: 900"),
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
        let property_value = parts
            .get(if class.starts_with("-") { 2 } else { 1 })
            .map(|&s| if class.starts_with("-") { format!("-{}", s) } else { s.to_string() })
            .unwrap_or_else(String::new);

        if let Some(&(_, property)) = margin_styles.iter().find(|(prop_name, _)| prop_name == &name) {
            let sign = if name.starts_with("-") { "-" } else { "" };
            let declarations: Vec<String> = property.split_whitespace().map(|p| format!("{}: {}{}px", p, sign, property_value)).collect();
            css_content.push_str(&format!(".{} {{ {} }}\n", class, declarations.join("; ")));
        }
        if let Some(&(_, value)) = styles.iter().find(|(key, _)| key == *&class) {
            css_content.push_str(&format!(".{} {{ {} }}\n", class, value));
        }
    }

    css_content
}

pub fn minify(css: &str) -> String {
    let mut output = String::new();
    let mut iter = css.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
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

            space if space.is_whitespace() => (),
            _ => output.push(c),
        }
    }

    output
}

pub fn write(path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let document = Html::parse_document(&contents);
    let mut classes = HashSet::new();

    for class in get_classes(&document) {
        classes.insert(class);
    }

    let mut sorted_classes: Vec<_> = classes.iter().collect();
    sorted_classes.sort();

    let css_content = create_stylesheet(sorted_classes);
    let selector = Selector::parse(r#"link[util]"#).unwrap();
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
            let mut file = File::create(href)?;

            write!(file, "{}", minify(&css_content))?;
            Ok(println!("Stylesheet has been created successfully."))
        }
        None => Ok(println!("No 'util' attribute found. Program will exit.")),
    }
}