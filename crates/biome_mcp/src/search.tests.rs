use super::*;

#[test]
fn shortens_long_lines_around_the_match() {
    let line = format!("{}target();{}", "a, ".repeat(100), " b,".repeat(100));
    let offset = line.find("target").unwrap();

    let shortened = shorten_line(&line, offset);

    assert!(shortened.starts_with("…target();"), "{shortened}");
    assert!(shortened.ends_with('…'), "{shortened}");
    assert_eq!(shortened.chars().count(), MAX_LINE_CHARS + 2);
}
