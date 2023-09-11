use biome_formatter::token::string::ToAsciiLowercaseCow;

fn to_ascii_lowercase() {
    let _ = str::to_ascii_lowercase(iai::black_box("lowercase"));
    let _ = str::to_ascii_lowercase(iai::black_box("upperCASE"));
}

fn to_ascii_lowercase_cow() {
    let _ = str::to_ascii_lowercase_cow(iai::black_box("lowercase"));
    let _ = str::to_ascii_lowercase_cow(iai::black_box("upperCASE"));
}

iai::main!(to_ascii_lowercase, to_ascii_lowercase_cow);
