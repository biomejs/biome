use crate::TypeDb;
use crate::interned_types::{Literal, TypeData};

/// A Salsa-backed type value returned by type inference.
#[derive(Clone, Copy)]
pub struct InferredType<'db> {
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
}

impl<'db> InferredType<'db> {
    pub const fn new(db: &'db dyn TypeDb, data: TypeData<'db>) -> Self {
        Self { db, data }
    }

    pub fn is_number_literal(self, value: f64) -> bool {
        matches!(
            self.data,
            TypeData::Literal(literal)
                if matches!(literal.literal(self.db), Literal::Number(number) if number.to_f64() == Some(value))
        )
    }

    pub fn is_number_or_number_literal(self) -> bool {
        matches!(self.data, TypeData::Number)
            || matches!(
                self.data,
                TypeData::Literal(literal)
                    if matches!(literal.literal(self.db), Literal::Number(_))
            )
    }

    pub fn is_bigint_literal(self, value: i64) -> bool {
        matches!(
            self.data,
            TypeData::Literal(literal)
                if matches!(literal.literal(self.db), Literal::BigInt(number) if number.text().trim_end_matches('n').parse() == Ok(value))
        )
    }

    pub fn is_string_or_string_literal(self) -> bool {
        matches!(self.data, TypeData::String)
            || matches!(
                self.data,
                TypeData::Literal(literal)
                    if matches!(literal.literal(self.db), Literal::String(_))
            )
    }

    pub fn is_regexp_literal_without_global_flag(self) -> bool {
        matches!(
        self.data,
        TypeData::Literal(literal)
            if matches!(literal.literal(self.db), Literal::RegExp(regexp) if !regexp.flags.contains('g'))
        )
    }

    pub fn is_array(self) -> bool {
        matches!(self.data, TypeData::InstanceOf(instance) if instance.ty(self.db).is_array_class(self.db))
    }

    pub fn is_array_of_promise(self) -> bool {
        let TypeData::InstanceOf(instance) = self.data else {
            return false;
        };

        instance.ty(self.db).is_array_class(self.db)
            && instance
                .type_parameters(self.db)
                .first()
                .is_some_and(|ty| is_promise_instance(self.db, *ty))
    }

    pub fn is_promise_instance(self) -> bool {
        is_promise_instance(self.db, self.data)
    }

    pub fn has_promise_variant(self) -> bool {
        match self.data {
            TypeData::Union(union) => union
                .types(self.db)
                .iter()
                .any(|ty| is_promise_instance(self.db, *ty)),
            _ => false,
        }
    }

    pub const fn is_null(self) -> bool {
        matches!(self.data, TypeData::Null)
    }

    pub const fn is_undefined(self) -> bool {
        matches!(self.data, TypeData::Undefined)
    }
}

fn is_promise_instance<'db>(db: &'db dyn TypeDb, mut data: TypeData<'db>) -> bool {
    while let TypeData::InstanceOf(instance) = data {
        data = instance.ty(db);
        if data.is_promise_class(db) {
            return true;
        }
    }

    false
}
