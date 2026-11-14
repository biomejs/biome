use biome_deserialize_macros::Deserializable;

#[derive(Default, Deserializable)]
enum TestEnum {
    #[default]
    #[deserializable(rename = 1)]
    Variant,
}

fn main() {}
