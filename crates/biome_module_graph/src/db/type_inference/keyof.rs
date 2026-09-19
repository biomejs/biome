use super::{
    apply_substitutions_to_root_body, normalize_structural_type, resolve_local_type_on_demand,
    substitutions_for_instance,
};
use crate::ModuleDb;
use biome_js_type_info::interned_types::{
    InternedLiteral, Literal, TypeData, TypeMember, TypeMemberKind,
};
use biome_js_type_info::literal::{NumberLiteral, StringLiteral, encode_js_string_content};
use biome_js_type_info::{TypeDb, TypeOperator};
use rustc_hash::FxHashSet;

const MAX_KEYOF_STEPS: usize = 1024;
// This limit guards Rust stack recursion. The step budget remains larger so
// wide, shallow object shapes can still be projected completely.
const MAX_KEYOF_DEPTH: usize = 64;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ClassSide {
    Static,
    Instance,
    All,
}

enum KeyProjection<'db> {
    Exact(Vec<TypeData<'db>>),
    Universal,
    /// The identity element for `keyof` over a union, contributed by `never`.
    UnionIdentity,
}

struct KeyofEvaluator<'db> {
    db: &'db dyn ModuleDb,
    active: FxHashSet<(TypeData<'db>, ClassSide)>,
    active_key_types: FxHashSet<TypeData<'db>>,
    remaining_steps: usize,
    depth: usize,
}

/// Evaluates a `keyof` operand when its complete property-key shape is known.
///
/// `None` means that the complete key set cannot be determined within the
/// traversal limits, including because the operand contains unresolved or
/// unsupported information.
pub(in crate::db) fn evaluate_keyof<'db>(
    db: &'db dyn ModuleDb,
    operand: TypeData<'db>,
) -> Option<TypeData<'db>> {
    let mut evaluator = KeyofEvaluator {
        db,
        active: FxHashSet::default(),
        active_key_types: FxHashSet::default(),
        remaining_steps: MAX_KEYOF_STEPS,
        depth: 0,
    };
    match evaluator.collect(operand, ClassSide::Instance)? {
        KeyProjection::Exact(keys) => Some(TypeData::union_from_types(db, keys)),
        KeyProjection::Universal | KeyProjection::UnionIdentity => {
            Some(TypeData::union_from_types(
                db,
                vec![TypeData::String, TypeData::Number, TypeData::Symbol],
            ))
        }
    }
}

impl<'db> KeyofEvaluator<'db> {
    fn collect(
        &mut self,
        raw_ty: TypeData<'db>,
        class_side: ClassSide,
    ) -> Option<KeyProjection<'db>> {
        self.with_depth(|evaluator| evaluator.collect_inner(raw_ty, class_side))
    }

    fn collect_inner(
        &mut self,
        raw_ty: TypeData<'db>,
        class_side: ClassSide,
    ) -> Option<KeyProjection<'db>> {
        let ty = resolve_local_type_on_demand(self.db, raw_ty).expand_canonical_global(self.db);
        if self.remaining_steps == 0 || !self.active.insert((ty, class_side)) {
            return None;
        }
        self.remaining_steps -= 1;

        let result = match ty {
            TypeData::AnyKeyword => Some(KeyProjection::Universal),
            TypeData::NeverKeyword => Some(KeyProjection::UnionIdentity),
            TypeData::UnknownKeyword | TypeData::ObjectKeyword => {
                Some(KeyProjection::Exact(Vec::new()))
            }
            TypeData::Class(class) => {
                let mut members = self.collect_members(class.members(self.db), class_side)?;
                if matches!(class_side, ClassSide::Static) {
                    members = union_projections(
                        members,
                        KeyProjection::Exact(vec![TypeData::Literal(InternedLiteral::new(
                            self.db,
                            Literal::String(StringLiteral::from(biome_rowan::Text::new_static(
                                "prototype",
                            ))),
                        ))]),
                    );
                }
                let inherited = match class.extends(self.db) {
                    Some(ty) => self.collect(ty, class_side)?,
                    None => KeyProjection::Exact(Vec::new()),
                };
                Some(union_projections(members, inherited))
            }
            TypeData::Interface(interface) => {
                let mut result = self.collect_members(interface.members(self.db), class_side)?;
                for parent in interface.extends(self.db) {
                    result = union_projections(result, self.collect(*parent, class_side)?);
                }
                Some(result)
            }
            TypeData::Object(object) => {
                if object.has_unknown_members(self.db) {
                    None
                } else {
                    let members = self.collect_members(object.members(self.db), class_side)?;
                    let inherited = match object.prototype(self.db) {
                        Some(ty) => self.collect(ty, class_side)?,
                        None => KeyProjection::Exact(Vec::new()),
                    };
                    Some(union_projections(members, inherited))
                }
            }
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::Object(members) => {
                    Some(self.collect_members(members, ClassSide::Instance)?)
                }
                Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::RegExp(_)
                | Literal::String(_)
                | Literal::Template(_) => None,
            },
            TypeData::Module(module) => {
                Some(self.collect_members(module.members(self.db), ClassSide::All)?)
            }
            TypeData::Namespace(namespace) => {
                Some(self.collect_members(namespace.members(self.db), ClassSide::All)?)
            }
            TypeData::InstanceOf(instance) => {
                let target = resolve_local_type_on_demand(self.db, instance.ty(self.db));
                let target = apply_instance_substitutions(
                    self.db,
                    target,
                    instance.type_parameters(self.db),
                );
                self.collect(target, class_side)
            }
            TypeData::Union(union) => {
                let mut result = None;
                for ty in union.types(self.db) {
                    let projection = self.collect(*ty, class_side)?;
                    if matches!(projection, KeyProjection::Universal) {
                        result = Some(KeyProjection::Universal);
                        break;
                    }
                    result = Some(match result {
                        None => projection,
                        Some(current) => intersect_projections(self.db, current, projection)?,
                    });
                }
                result.or(Some(KeyProjection::Exact(Vec::new())))
            }
            TypeData::Intersection(intersection) => {
                let mut result = KeyProjection::Exact(Vec::new());
                for ty in intersection.types(self.db) {
                    let projection = self.collect(*ty, class_side)?;
                    if matches!(projection, KeyProjection::UnionIdentity) {
                        result = KeyProjection::Universal;
                        break;
                    }
                    result = union_projections(result, projection);
                }
                Some(result)
            }
            TypeData::TypeofType(typeof_type) => {
                let target = resolve_local_type_on_demand(self.db, typeof_type.ty(self.db));
                let expanded_target = target.expand_canonical_global(self.db);
                let side = if matches!(expanded_target, TypeData::Class(_)) {
                    ClassSide::Static
                } else {
                    class_side
                };
                self.collect(target, side)
            }
            TypeData::TypeofValue(typeof_value) => {
                let target = resolve_local_type_on_demand(self.db, typeof_value.ty(self.db));
                let expanded_target = target.expand_canonical_global(self.db);
                // A class value exposes its constructor/static side. A value
                // produced by that constructor exposes the instance side,
                // even though its target still refers back to the class.
                let side = if matches!(expanded_target, TypeData::Class(_)) {
                    ClassSide::Static
                } else if matches!(expanded_target, TypeData::InstanceOf(_)) {
                    ClassSide::Instance
                } else {
                    class_side
                };
                self.collect(target, side)
            }
            TypeData::TypeOperator(operator) => match operator.operator(self.db) {
                TypeOperator::Readonly => self.collect(operator.ty(self.db), class_side),
                TypeOperator::Unique => None,
                TypeOperator::Keyof => None,
            },
            TypeData::IndexedAccess(_) => {
                let normalized = normalize_structural_type(self.db, ty, |ty| {
                    resolve_local_type_on_demand(self.db, ty)
                })
                .ok()?;
                if matches!(normalized, TypeData::Object(_)) {
                    self.collect(normalized, class_side)
                } else {
                    None
                }
            }
            TypeData::Generic(_)
            | TypeData::Unknown
            | TypeData::Global
            | TypeData::GlobalType(_)
            | TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Undefined
            | TypeData::Conditional
            | TypeData::Constructor(_)
            | TypeData::Function(_)
            | TypeData::Tuple(_)
            | TypeData::Local(_)
            | TypeData::MergedReference(_)
            | TypeData::TypeofExpression(_)
            | TypeData::ThisKeyword
            | TypeData::VoidKeyword => None,
        };

        self.active.remove(&(ty, class_side));
        result
    }

    fn with_depth<T>(&mut self, operation: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        if self.depth >= MAX_KEYOF_DEPTH {
            return None;
        }
        self.depth += 1;
        let result = operation(self);
        self.depth -= 1;
        result
    }

    fn collect_members(
        &mut self,
        members: &[TypeMember<'db>],
        class_side: ClassSide,
    ) -> Option<KeyProjection<'db>> {
        let mut keys = Vec::new();
        for member in members {
            if member.kind.is_constructor() || member.kind.is_call_signature() {
                continue;
            }
            if member
                .kind
                .computed_value_type()
                .is_some_and(|ty| ty == TypeData::Unknown)
            {
                return None;
            }
            if matches!(class_side, ClassSide::Static) && !member.kind.is_static() {
                continue;
            }
            if matches!(class_side, ClassSide::Instance) && member.kind.is_static() {
                continue;
            }
            if self.remaining_steps == 0 {
                return None;
            }
            self.remaining_steps -= 1;
            let projection = self.member_key_projection(&member.kind)?;
            match projection {
                KeyProjection::Exact(mut member_keys) => keys.append(&mut member_keys),
                KeyProjection::Universal => return Some(KeyProjection::Universal),
                KeyProjection::UnionIdentity => {}
            }
        }
        Some(KeyProjection::Exact(keys))
    }

    fn member_key_projection(&mut self, kind: &TypeMemberKind<'db>) -> Option<KeyProjection<'db>> {
        if let TypeMemberKind::GetterNumber(number)
        | TypeMemberKind::ConstAssertedGetterNumber(number)
        | TypeMemberKind::NamedNumber(number)
        | TypeMemberKind::NamedOptionalNumber(number)
        | TypeMemberKind::ConstAssertedNamedNumber(number)
        | TypeMemberKind::ConstAssertedNamedOptionalNumber(number)
        | TypeMemberKind::NamedStaticNumber(number)
        | TypeMemberKind::ConstAssertedNamedStaticNumber(number) = kind
        {
            return numeric_key_projection(self.db, number);
        }
        if let Some(key_type) = kind.computed_value_type() {
            return self.computed_key_type_projection(key_type);
        }
        if let Some(key_type) = kind.index_signature_type() {
            return self.key_type_projection(key_type);
        }

        kind.name()
            .map(|name| KeyProjection::Exact(vec![encode_string_key(self.db, name.text())]))
    }

    fn computed_key_type_projection(
        &mut self,
        raw_key_type: TypeData<'db>,
    ) -> Option<KeyProjection<'db>> {
        self.with_depth(|evaluator| evaluator.computed_key_type_projection_inner(raw_key_type))
    }

    fn computed_key_type_projection_inner(
        &mut self,
        raw_key_type: TypeData<'db>,
    ) -> Option<KeyProjection<'db>> {
        let key_type =
            resolve_local_type_on_demand(self.db, raw_key_type).expand_canonical_global(self.db);
        if self.remaining_steps == 0 || !self.active_key_types.insert(key_type) {
            return None;
        }
        self.remaining_steps -= 1;

        let result = if let TypeData::Literal(literal) = key_type {
            match literal.literal(self.db) {
                Literal::String(value) => Some(KeyProjection::Exact(vec![encode_string_key(
                    self.db,
                    value.decoded()?.text(),
                )])),
                Literal::Number(value) => numeric_key_projection(self.db, value),
                Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Object(_)
                | Literal::RegExp(_)
                | Literal::Template(_) => None,
            }
        } else if let TypeData::Union(union) = key_type {
            let mut result = KeyProjection::Exact(Vec::new());
            for ty in union.types(self.db) {
                result = union_projections(result, self.computed_key_type_projection(*ty)?);
            }
            Some(result)
        } else if let TypeData::TypeofType(typeof_type) = key_type {
            self.computed_key_type_projection(typeof_type.ty(self.db))
        } else if let TypeData::TypeofValue(typeof_value) = key_type {
            self.computed_key_type_projection(typeof_value.ty(self.db))
        } else {
            None
        };
        self.active_key_types.remove(&key_type);
        result
    }

    fn key_type_projection(&mut self, raw_key_type: TypeData<'db>) -> Option<KeyProjection<'db>> {
        self.with_depth(|evaluator| evaluator.key_type_projection_inner(raw_key_type))
    }

    fn key_type_projection_inner(
        &mut self,
        raw_key_type: TypeData<'db>,
    ) -> Option<KeyProjection<'db>> {
        let key_type =
            resolve_local_type_on_demand(self.db, raw_key_type).expand_canonical_global(self.db);
        if self.remaining_steps == 0 || !self.active_key_types.insert(key_type) {
            return None;
        }
        self.remaining_steps -= 1;

        let result = if let TypeData::Literal(literal) = key_type {
            match literal.literal(self.db) {
                Literal::String(value) => Some(KeyProjection::Exact(vec![encode_string_key(
                    self.db,
                    value.decoded()?.text(),
                )])),
                Literal::Number(value) => numeric_key_projection(self.db, value),
                Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Object(_)
                | Literal::RegExp(_)
                | Literal::Template(_) => None,
            }
        } else if matches!(key_type, TypeData::String) {
            Some(KeyProjection::Exact(vec![
                TypeData::String,
                TypeData::Number,
            ]))
        } else if matches!(key_type, TypeData::Number) {
            Some(KeyProjection::Exact(vec![TypeData::Number]))
        } else if matches!(key_type, TypeData::Symbol) {
            Some(KeyProjection::Exact(vec![TypeData::Symbol]))
        } else if let TypeData::Union(union) = key_type {
            let mut result = KeyProjection::Exact(Vec::new());
            for ty in union.types(self.db) {
                result = union_projections(result, self.key_type_projection(*ty)?);
            }
            Some(result)
        } else if matches!(key_type, TypeData::AnyKeyword) {
            Some(KeyProjection::Universal)
        } else if matches!(key_type, TypeData::NeverKeyword) {
            Some(KeyProjection::UnionIdentity)
        } else if let TypeData::TypeofType(typeof_type) = key_type {
            self.key_type_projection(typeof_type.ty(self.db))
        } else if let TypeData::TypeofValue(typeof_value) = key_type {
            self.key_type_projection(typeof_value.ty(self.db))
        } else {
            None
        };
        self.active_key_types.remove(&key_type);
        result
    }
}

fn encode_string_key<'db>(db: &'db dyn TypeDb, value: &str) -> TypeData<'db> {
    let encoded = encode_js_string_content(value);
    TypeData::Literal(InternedLiteral::new(
        db,
        Literal::String(StringLiteral::from(encoded)),
    ))
}

fn numeric_key_projection<'db>(
    db: &'db dyn TypeDb,
    number: &NumberLiteral,
) -> Option<KeyProjection<'db>> {
    number.to_property_key()?;
    Some(KeyProjection::Exact(vec![TypeData::Literal(
        InternedLiteral::new(db, Literal::Number(number.clone())),
    )]))
}

fn union_projections<'db>(
    left: KeyProjection<'db>,
    right: KeyProjection<'db>,
) -> KeyProjection<'db> {
    match (left, right) {
        (KeyProjection::Universal, _) | (_, KeyProjection::Universal) => KeyProjection::Universal,
        (KeyProjection::UnionIdentity, projection) | (projection, KeyProjection::UnionIdentity) => {
            projection
        }
        (KeyProjection::Exact(mut left), KeyProjection::Exact(right)) => {
            for key in right {
                if !left.contains(&key) {
                    left.push(key);
                }
            }
            KeyProjection::Exact(left)
        }
    }
}

fn intersect_projections<'db>(
    db: &'db dyn TypeDb,
    left: KeyProjection<'db>,
    right: KeyProjection<'db>,
) -> Option<KeyProjection<'db>> {
    Some(match (left, right) {
        (KeyProjection::Universal, projection) | (projection, KeyProjection::Universal) => {
            projection
        }
        (KeyProjection::UnionIdentity, projection) | (projection, KeyProjection::UnionIdentity) => {
            projection
        }
        (KeyProjection::Exact(left), KeyProjection::Exact(right)) => {
            let mut keys = Vec::new();
            for left in left {
                for right in &right {
                    match intersect_key_types(db, left, *right) {
                        Err(()) => return None,
                        Ok(Some(key)) if !keys.contains(&key) => keys.push(key),
                        Ok(None | Some(_)) => {}
                    }
                }
            }
            KeyProjection::Exact(keys)
        }
    })
}

fn intersect_key_types<'db>(
    db: &'db dyn TypeDb,
    left: TypeData<'db>,
    right: TypeData<'db>,
) -> Result<Option<TypeData<'db>>, ()> {
    if let (TypeData::Literal(left_id), TypeData::Literal(right_id)) = (left, right)
        && let (Literal::Number(left), Literal::Number(right)) =
            (left_id.literal(db), right_id.literal(db))
    {
        if left.to_property_key().is_none() || right.to_property_key().is_none() {
            return Err(());
        }
        if left == right
            || left
                .to_f64()
                .is_some_and(|value| right.to_f64() == Some(value))
        {
            return Ok(Some(TypeData::Literal(left_id)));
        }
        return Ok(None);
    }

    if left == right {
        return Ok(Some(left));
    }

    if let (TypeData::String, TypeData::Literal(right)) = (left, right)
        && matches!(right.literal(db), Literal::String(_))
    {
        return Ok(Some(TypeData::Literal(right)));
    }
    if let (TypeData::Literal(left), TypeData::String) = (left, right)
        && matches!(left.literal(db), Literal::String(_))
    {
        return Ok(Some(TypeData::Literal(left)));
    }
    if let (TypeData::Number, TypeData::Literal(right)) = (left, right)
        && let Literal::Number(number) = right.literal(db)
    {
        if number.to_property_key().is_none() {
            return Err(());
        }
        return Ok(Some(TypeData::Literal(right)));
    }
    if let (TypeData::Literal(left), TypeData::Number) = (left, right)
        && let Literal::Number(number) = left.literal(db)
    {
        if number.to_property_key().is_none() {
            return Err(());
        }
        return Ok(Some(TypeData::Literal(left)));
    }
    if let (TypeData::Literal(left_id), TypeData::Literal(right_id)) = (left, right)
        && let (Literal::String(left), Literal::String(right)) =
            (left_id.literal(db), right_id.literal(db))
        && left.as_str() == right.as_str()
    {
        return Ok(Some(TypeData::Literal(left_id)));
    }

    Ok(None)
}

fn apply_instance_substitutions<'db>(
    db: &'db dyn ModuleDb,
    target: TypeData<'db>,
    type_parameters: &[TypeData<'db>],
) -> TypeData<'db> {
    let substitutions = substitutions_for_instance(db, target, type_parameters, &[]);
    let body = if let TypeData::InstanceOf(instance) = target {
        instance.ty(db)
    } else {
        target
    };
    apply_substitutions_to_root_body(db, body, &substitutions)
}

#[cfg(test)]
mod tests {
    use super::{KeyProjection, evaluate_keyof, intersect_projections};
    use crate::{ModuleDb, ModuleInfo};
    use biome_js_type_info::TypeDb;
    use biome_js_type_info::interned_types::{
        InternedLiteral, InternedObject, Literal, TypeData, TypeMember, TypeMemberKind,
    };
    use biome_js_type_info::literal::{NumberLiteral, StringLiteral};
    use biome_languages::LanguageDb;
    use biome_rowan::Text;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl TypeDb for TestDb {}

    #[salsa::db]
    impl LanguageDb for TestDb {
        fn source_from_index(&self, _index: usize) -> Option<biome_languages::DocumentFileSource> {
            None
        }
    }

    #[salsa::db]
    impl ModuleDb for TestDb {
        fn module_for_path(&self, _path: &camino::Utf8Path) -> Option<ModuleInfo> {
            None
        }

        fn for_each_module(&self, _f: &mut dyn FnMut(ModuleInfo)) {}
    }

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    fn string_literal<'db>(db: &'db TestDb, value: &'static str) -> TypeData<'db> {
        TypeData::Literal(InternedLiteral::new(
            db,
            Literal::String(StringLiteral::from(Text::new_static(value))),
        ))
    }

    fn number_literal<'db>(db: &'db TestDb, value: &'static str) -> TypeData<'db> {
        TypeData::Literal(InternedLiteral::new(
            db,
            Literal::Number(NumberLiteral::new(Text::new_static(value))),
        ))
    }

    #[test]
    fn broad_string_intersection_keeps_every_literal_key() {
        let db = TestDb::default();
        let first = string_literal(&db, "first");
        let second = string_literal(&db, "second");
        let result = intersect_projections(
            &db,
            KeyProjection::Exact(vec![TypeData::String]),
            KeyProjection::Exact(vec![first, second]),
        )
        .expect("literal string intersection should be determinate");

        assert!(matches!(result, KeyProjection::Exact(keys) if keys == vec![first, second]));
    }

    #[test]
    fn broad_computed_key_is_incomplete() {
        let db = TestDb::default();
        let members = vec![TypeMember {
            kind: TypeMemberKind::ComputedValue(TypeData::String),
            ty: TypeData::Unknown,
        }]
        .into_boxed_slice();
        let object = TypeData::Object(InternedObject::new(&db, None, members, false));

        assert!(evaluate_keyof(&db, object).is_none());
    }

    #[test]
    fn numeric_literal_intersection_uses_numeric_identity() {
        let db = TestDb::default();
        let hexadecimal = number_literal(&db, "0x1");
        let decimal = number_literal(&db, "1");
        let result = intersect_projections(
            &db,
            KeyProjection::Exact(vec![hexadecimal]),
            KeyProjection::Exact(vec![decimal]),
        )
        .expect("equivalent numeric literals should be determinate");

        assert!(matches!(result, KeyProjection::Exact(keys) if keys == vec![hexadecimal]));
    }

    #[test]
    fn unparseable_numeric_literal_intersection_is_indeterminate() {
        let db = TestDb::default();
        let hexadecimal = number_literal(&db, "0x10000000000000000");
        let decimal = number_literal(&db, "18446744073709551616");
        assert!(
            intersect_projections(
                &db,
                KeyProjection::Exact(vec![hexadecimal]),
                KeyProjection::Exact(vec![decimal]),
            )
            .is_none()
        );
    }

    #[test]
    fn member_budget_failure_is_indeterminate() {
        let db = TestDb::default();
        let members = (0..super::MAX_KEYOF_STEPS)
            .map(|index| TypeMember {
                kind: TypeMemberKind::Named(Text::new_owned(
                    format!("member{index}").into_boxed_str(),
                )),
                ty: TypeData::Unknown,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let object = TypeData::Object(InternedObject::new(&db, None, members, false));

        assert!(evaluate_keyof(&db, object).is_none());
    }

    #[test]
    fn shallow_prototype_chain_succeeds() {
        let db = TestDb::default();
        let mut prototype: Option<TypeData<'_>> = None;

        for _ in 0..8 {
            let object =
                TypeData::Object(InternedObject::new(&db, prototype, Box::default(), false));
            prototype = Some(object);
        }

        let root = TypeData::Object(InternedObject::new(
            &db,
            prototype,
            vec![TypeMember {
                kind: TypeMemberKind::Named(Text::new_static("root")),
                ty: TypeData::Unknown,
            }]
            .into_boxed_slice(),
            false,
        ));

        assert!(evaluate_keyof(&db, root).is_some());
    }

    #[test]
    fn prototype_chain_budget_failure_is_indeterminate() {
        let db = TestDb::default();
        let mut prototype: Option<TypeData<'_>> = None;

        for _ in 0..=super::MAX_KEYOF_DEPTH {
            let object =
                TypeData::Object(InternedObject::new(&db, prototype, Box::default(), false));
            prototype = Some(object);
        }

        assert!(evaluate_keyof(&db, prototype.expect("prototype chain has a root")).is_none());
    }
}
