/// Normalize HTML for comparison, preserving whitespace inside `<pre>` blocks.
/// Matches the normalization in `xtask/coverage/src/markdown/commonmark.rs`.
pub fn normalize_html(html: &str) -> String {
    let mut result = Vec::new();
    let mut in_pre = false;

    for line in html.lines() {
        if line.contains("<pre") {
            in_pre = true;
        }
        if in_pre {
            result.push(line.to_string());
        } else {
            result.push(line.trim_end().to_string());
        }
        if line.contains("</pre>") {
            in_pre = false;
        }
    }

    result.join("\n").trim().to_string() + "\n"
}
