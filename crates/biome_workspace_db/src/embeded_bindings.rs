use biome_rowan::{TextRange, TokenText};

#[salsa::db]
pub(crate) trait BindingsDb: biome_db::Db {
    fn bindings(&self) -> Vec<Vec<EmbeddedBinding>>;
}

#[salsa::input]
#[derive(Debug)]
pub struct EmbeddedBinding {
    /// The range of the binding
    pub(crate) range: TextRange,
    /// The text of the binding
    #[returns(clone)]
    pub(crate) text: TokenText,
    /// Optionally, the source of the binding. It represents the path of the import/dynamic import.
    #[returns(ref)]
    pub(crate) source: Option<TokenText>,
}

#[salsa::interned]
pub struct InternedBinding {
    #[returns(ref)]
    name: String,
}

#[salsa::tracked]
pub(crate) fn get_binding_by_name<'db>(
    db: &'db dyn BindingsDb,
    binding_name: InternedBinding<'db>,
) -> Option<EmbeddedBinding> {
    for bindings in db.bindings() {
        for binding in bindings {
            if binding.text(db).text() == binding_name.name(db) {
                return Some(binding);
            }
        }
    }
    None
}

#[salsa::tracked]
pub(crate) fn get_binding_with_source<'db>(
    db: &'db dyn BindingsDb,
    binding_name: InternedBinding<'db>,
) -> Option<EmbeddedBinding> {
    for bindings in db.bindings() {
        for binding in bindings {
            if binding.text(db).text() == binding_name.name(db) && binding.source(db).is_some() {
                return Some(binding);
            }
        }
    }
    None
}
