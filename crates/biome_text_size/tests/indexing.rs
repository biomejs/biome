use biome_text_size::*;

#[test]
fn main() {
    let range = TextRange::default();
    let _ = &""[range];
    let _ = &String::new()[range];
}
