fn main() {
    let input = std::fs::read_to_string("classes.txt").unwrap();
    let trimmed_input = input.trim();
    let lines: Vec<&str> = trimmed_input.split("\n").collect();
    let mut output = String::from("let styles: [(&str, String); NUM] = [");

    for line in lines.clone() {
        let parts: Vec<&str> = line.splitn(2, " ").collect();
        if parts.len() == 2 {
            let value_without_comments = parts[1].split("/*").next().unwrap_or(parts[1]).trim().to_string();
            output.push_str(&format!("(\"{}\", \"{}\"),\n", parts[0].trim(), value_without_comments));
        }
    }

    output.push_str("];");
    let count = lines.len();
    let output = output.replace("NUM", &count.to_string());

    println!("{}", output);
}
