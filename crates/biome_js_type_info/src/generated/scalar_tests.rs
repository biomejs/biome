use crate::{InferredType, RawTypeId, TypeData, TypeDb, TypeReference, resolved::InferredTypeData};

include!("scalar_test_types.rs");

#[salsa::db]
#[derive(Default)]
struct TestDb {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, _path: &camino::Utf8Path) -> Option<biome_db::ParsedSource> {
        None
    }
}

#[salsa::db]
impl TypeDb for TestDb {}

#[test]
fn generated_scalars_support_inference_queries() {
    let db = TestDb::default();
    let types = scalar_types();
    let interface = types
        .iter()
        .find_map(|ty| match ty {
            TypeData::Interface(interface) => Some(interface),
            _ => None,
        })
        .unwrap();
    let member_type = |name| {
        let member = interface
            .members
            .iter()
            .find(|member| member.kind.has_name(name))
            .unwrap();
        let TypeReference::Resolved(RawTypeId::Local(id)) = member.ty else {
            panic!("expected a local scalar type")
        };
        let inferred =
            InferredTypeData::from_raw_with_resolver(&db, &types[id.index()], false, &mut |_| {
                panic!("scalar types have no references")
            });
        InferredType::new(&db, inferred)
    };

    assert!(member_type("unchecked").is_return_type_relation_escape_hatch());
    assert!(member_type("uncertain").is_inferred());
    assert!(!member_type("uncertain").is_non_nullish());
    assert!(member_type("impossible").is_inferred());
    assert!(member_type("missing").is_nullish());
    assert!(member_type("integer").is_all_bigint_like());
    assert!(member_type("token").is_non_nullish());
    assert!(member_type("enabled").is_always_truthy());
    assert!(member_type("disabled").is_always_falsy());
    assert!(member_type("negative").is_number_literal(-42.0));
    assert!(!member_type("negative").is_number_literal(42.0));
    assert!(member_type("zero").is_always_falsy());
    assert!(member_type("largeInteger").is_bigint_literal(9_007_199_254_740_993));
    assert!(member_type("negativeInteger").is_bigint_literal(-42));
}
