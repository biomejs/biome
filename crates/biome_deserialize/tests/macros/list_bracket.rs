use biome_deserialize_macros::Deserializable;

#[derive(Default, Deserializable)]
struct TestStruct {
    #[deserializable[required]]
    field: bool,
}

fn main() {}
