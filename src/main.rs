use std::fs;

// Represents the grammatical category of a single token in MARIE assembly.
// Each variant maps to a CSS class in the generated HTML for syntax highlighting.
#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,    // MARIE instruction mnemonics (LOAD, ADD, HALT, etc.)
    Directive,  // Assembler directives that control memory layout (ORG, DEC, HEX, OCT)
    Number,     // Numeric literals (pure decimal digit sequences)
    Comment,    // Inline comments (entire rest-of-line after a '/')
    Label,      // User-defined symbols that end with a comma (e.g. "Loop,")
    Identifier, // Alphanumeric operands / symbol references
    Unknown,    // Anything that doesn't fit the above categories
}

// Escapes the three characters that have special meaning in HTML so that
// raw assembly text is safe to embed inside a <pre> block without breaking markup.
fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

// Determines the TokenType of a single whitespace-delimited token.
// Classification order matters: keywords and directives are checked first
// (case-insensitively), then structural tokens (labels end with ','),
// then pure-digit numbers, then general identifiers, and finally Unknown.
fn classify_token(token: &str) -> TokenType {
    let upper = token.to_uppercase();

    let keywords = ["LOAD", "STORE", "ADD", "SUBT", "INPUT", "OUTPUT", "HALT", "SKIPCOND", "JUMP"];
    let directives = ["ORG", "DEC", "HEX", "OCT"];

    if keywords.contains(&upper.as_str()) {
        TokenType::Keyword
    } else if directives.contains(&upper.as_str()) {
        TokenType::Directive
    } else if token.ends_with(',') {
        // Labels in MARIE assembly always end with a comma
        TokenType::Label
    } else if token.chars().all(|c| c.is_ascii_digit()) {
        TokenType::Number
    } else if token.chars().all(|c| c.is_alphanumeric() || c == '_') {
        TokenType::Identifier
    } else {
        TokenType::Unknown
    }
}

// Wraps a token in a <span> whose class matches its TokenType.
// The token text is HTML-escaped first so special characters render literally.
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

// Processes one line of assembly source into highlighted HTML.
// The line is split on whitespace; each token is classified and wrapped
// in a coloured <span>. Once a '/' is encountered, the remainder of the
// line is treated as a single comment span regardless of further tokens.
fn highlight_line(line: &str) -> String {
    let mut result = String::new();

    // If the line contains a comment, split it into code part and comment part
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

// Builds a complete, self-contained HTML page around the already-highlighted
// code string. The embedded CSS uses a light background with distinct colours
// per token type for readability.
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
    // Read the MARIE assembly source from disk
    let input = fs::read_to_string("input.mas").expect("Could not read input file");

    // Highlight every line and join them back with newlines so the <pre>
    // block preserves the original line structure of the source file.
    let highlighted: String = input
        .lines()
        .map(highlight_line)
        .collect::<Vec<String>>()
        .join("\n");

    // Wrap the highlighted code in a full HTML document and write it out
    let html = generate_html(&highlighted);

    fs::write("output.html", html).expect("Could not write output file");

    println!("Generated output.html");
}
