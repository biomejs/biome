use biome_deserialize_macros::Deserializable;

#[derive(Default, Deserializable)]
struct TestStruct {
    #[deserializable]
    field: bool,
}

fn main() {}
