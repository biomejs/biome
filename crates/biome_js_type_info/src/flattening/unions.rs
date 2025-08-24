use rustc_hash::FxHashSet;

use crate::{ResolvedTypeData, TypeData, TypeReference, TypeResolver};

impl TypeData {
    /// Returns an iterator over the variants of this type, while deduplicating
    /// variants and flattening nested unions in the process.
    ///
    /// Returns an iterator that yields no elements if the type is not a union.
    pub fn flattened_union_variants(
        &self,
        resolver: &dyn TypeResolver,
    ) -> impl Iterator<Item = TypeReference> {
        if let Self::Union(union) = self {
            UnionVariantIterator::new(union.types().to_vec(), resolver)
        } else {
            UnionVariantIterator::new_empty(resolver)
        }
    }
}

impl ResolvedTypeData<'_> {
    /// Returns an iterator over the variants of this resolved union, while
    /// deduplicating variants and flattening nested unions in the process.
    ///
    /// Returns an iterator that yields no elements if the resolved data is not
    /// a union.
    pub fn flattened_union_variants(
        self,
        resolver: &dyn TypeResolver,
    ) -> impl Iterator<Item = TypeReference> {
        if let TypeData::Union(union) = self.to_data() {
            UnionVariantIterator::new(union.into_types(), resolver)
        } else {
            UnionVariantIterator::new_empty(resolver)
        }
    }
}

struct UnionVariantIterator<'a> {
    resolver: &'a dyn TypeResolver,
    unions: Vec<Vec<TypeReference>>,
    previous_references: FxHashSet<TypeReference>,
}

impl<'a> UnionVariantIterator<'a> {
    fn new(mut variants: Vec<TypeReference>, resolver: &'a dyn TypeResolver) -> Self {
        // Reverse in-place so we can efficiently pop them later.
        variants.reverse();

        Self {
            resolver,
            unions: vec![variants],
            previous_references: Default::default(),
        }
    }

    fn new_empty(resolver: &'a dyn TypeResolver) -> Self {
        Self {
            resolver,
            unions: Vec::new(),
            previous_references: Default::default(),
        }
    }
}

impl<'a> Iterator for UnionVariantIterator<'a> {
    type Item = TypeReference;

    fn next(&mut self) -> Option<Self::Item> {
        let mut union = self.unions.last_mut()?;
        loop {
            let reference = match union.pop() {
                Some(reference) => reference,
                None => {
                    self.unions.pop();
                    union = self.unions.last_mut()?;
                    continue;
                }
            };

            if self.previous_references.insert(reference.clone()) {
                if let Some(ty) = self.resolver.resolve_and_get(&reference)
                    && ty.is_union()
                    && let TypeData::Union(nested) = ty.to_data()
                {
                    self.unions
                        .push(nested.types().iter().rev().cloned().collect());
                    union = self.unions.last_mut()?;
                    continue;
                }

                return Some(reference);
            }
        }
    }
}
