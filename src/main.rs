use std::fs;

#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,
    Directive,
    Number,
    Comment,
    Label,
    Identifier,
    Unknown,
}

fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

fn classify_token(token: &str) -> TokenType {
    let upper = token.to_uppercase();

    let keywords = ["LOAD", "STORE", "ADD", "SUBT", "INPUT", "OUTPUT", "HALT", "SKIPCOND", "JUMP"];
    let directives = ["ORG", "DEC", "HEX", "OCT"];

    if keywords.contains(&upper.as_str()) {
        TokenType::Keyword
    } else if directives.contains(&upper.as_str()) {
        TokenType::Directive
    } else if token.ends_with(',') {
        TokenType::Label
    } else if token.chars().all(|c| c.is_ascii_digit()) {
        TokenType::Number
    } else if token.chars().all(|c| c.is_alphanumeric() || c == '_') {
        TokenType::Identifier
    } else {
        TokenType::Unknown
    }
}

fn token_to_html(token: &str, token_type: TokenType) -> String {
    let escaped = escape_html(token);

    let class = match token_type {
        TokenType::Keyword => "keyword",
        TokenType::Directive => "directive",
        TokenType::Number => "number",
        TokenType::Comment => "comment",
        TokenType::Label => "label",
        TokenType::Identifier => "identifier",
        TokenType::Unknown => "unknown",
    };

    format!(r#"<span class="{}">{}</span>"#, class, escaped)
}

fn highlight_line(line: &str) -> String {
    let mut result = String::new();

    if let Some(comment_start) = line.find('/') {
        let code_part = &line[..comment_start];
        let comment_part = &line[comment_start..];

        for token in code_part.split_whitespace() {
            let token_type = classify_token(token);
            result.push_str(&token_to_html(token, token_type));
            result.push(' ');
        }

        result.push_str(&token_to_html(comment_part.trim(), TokenType::Comment));
    } else {
        for token in line.split_whitespace() {
            let token_type = classify_token(token);
            result.push_str(&token_to_html(token, token_type));
            result.push(' ');
        }
    }

    result
}

fn generate_html(highlighted_code: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>MARIE.js Highlighter</title>
<style>
body {{ background: #000000; color: #d4d4d4; font-family: monospace; padding: 20px; }}
.keyword {{ color: #ffdd00; }}
.directive {{ color: #ff89f5; }}
.number {{ color: #7774f8; }}
.comment {{ color: #72b852; }}
.label {{ color: #c0b2ff; }}
.identifier {{ color: #79d0ff; }}
.unknown {{ color: #ffffff; }}
pre {{ white-space: pre-wrap; }}
</style>
</head>
<body>
<pre>{}</pre>
</body>
</html>"#, highlighted_code)
}

fn main() {
    let input = fs::read_to_string("input.mas").expect("Could not read input file");

    let highlighted: String = input
        .lines()
        .map(highlight_line)
        .collect::<Vec<String>>()
        .join("\n");

    let html = generate_html(&highlighted);

    fs::write("output.html", html).expect("Could not write output file");

    println!("Generated output.html");
}
