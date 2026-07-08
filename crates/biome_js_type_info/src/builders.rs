use crate::interned_types::{InternedIntersection, InternedUnion, TypeData, TypeDb};

pub(crate) struct IntersectionBuilder<'db> {
    db: &'db dyn TypeDb,
    types: Vec<TypeData<'db>>,
}

impl<'db> IntersectionBuilder<'db> {
    pub(crate) fn new(db: &'db dyn TypeDb) -> Self {
        Self {
            db,
            types: Vec::new(),
        }
    }

    pub(crate) fn add(mut self, ty: TypeData<'db>) -> Self {
        if self.types.as_slice() == [TypeData::NeverKeyword] {
            return self;
        }

        match ty {
            TypeData::NeverKeyword => {
                self.types.clear();
                self.types.push(TypeData::NeverKeyword);
            }
            TypeData::Intersection(intersection) => {
                for ty in intersection.types(self.db) {
                    self = self.add(*ty);
                }
            }
            ty => {
                if ty.is_primitive(self.db)
                    && self
                        .types
                        .iter()
                        .any(|other| *other != ty && other.is_primitive(self.db))
                {
                    self.types.clear();
                    self.types.push(TypeData::NeverKeyword);
                    return self;
                }

                if !self.types.contains(&ty) {
                    self.types.push(ty);
                }
            }
        }

        self
    }

    pub(crate) fn add_all(mut self, types: impl IntoIterator<Item = TypeData<'db>>) -> Self {
        for ty in types {
            self = self.add(ty);
        }

        self
    }

    pub(crate) fn build(mut self) -> TypeData<'db> {
        match self.types.len() {
            0 => TypeData::NeverKeyword,
            1 => self.types.pop().unwrap_or(TypeData::NeverKeyword),
            _ => TypeData::Intersection(InternedIntersection::new(
                self.db,
                self.types.into_boxed_slice(),
            )),
        }
    }
}

pub(crate) struct UnionBuilder<'db> {
    db: &'db dyn TypeDb,
    types: Vec<TypeData<'db>>,
}

impl<'db> UnionBuilder<'db> {
    pub(crate) fn new(db: &'db dyn TypeDb) -> Self {
        Self {
            db,
            types: Vec::new(),
        }
    }

    pub(crate) fn add(mut self, ty: TypeData<'db>) -> Self {
        if self.types.as_slice() == [TypeData::AnyKeyword] {
            return self;
        }

        match ty {
            TypeData::AnyKeyword => {
                self.types.clear();
                self.types.push(TypeData::AnyKeyword);
            }
            TypeData::NeverKeyword => {}
            TypeData::Union(union) => {
                for ty in union.types(self.db) {
                    self = self.add(*ty);
                }
            }
            ty => {
                if !self.types.contains(&ty) {
                    self.types.push(ty);
                }
            }
        }

        self
    }

    pub(crate) fn add_all(mut self, types: impl IntoIterator<Item = TypeData<'db>>) -> Self {
        for ty in types {
            self = self.add(ty);
        }

        self
    }

    pub(crate) fn build(mut self) -> TypeData<'db> {
        self.normalize_boolean_variants();

        match self.types.len() {
            0 => TypeData::NeverKeyword,
            1 => self.types.pop().unwrap_or(TypeData::NeverKeyword),
            _ => TypeData::Union(InternedUnion::new(self.db, self.types.into_boxed_slice())),
        }
    }

    fn normalize_boolean_variants(&mut self) {
        let has_boolean = self.types.iter().any(|ty| *ty == TypeData::Boolean);
        let has_true = self
            .types
            .iter()
            .any(|ty| ty.is_boolean_literal(self.db, true));
        let has_false = self
            .types
            .iter()
            .any(|ty| ty.is_boolean_literal(self.db, false));

        if !(has_boolean || has_true && has_false) {
            return;
        }

        self.types.retain(|ty| {
            !ty.is_boolean_literal(self.db, true) && !ty.is_boolean_literal(self.db, false)
        });

        if !has_boolean {
            self.types.push(TypeData::Boolean);
        }
    }
}
