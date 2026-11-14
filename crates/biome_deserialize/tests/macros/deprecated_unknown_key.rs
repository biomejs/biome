use biome_deserialize_macros::Deserializable;

#[derive(Default, Deserializable)]
struct TestStruct {
    #[deserializable(deprecated(reason = "unused"))]
    field: bool,
}

fn main() {}
