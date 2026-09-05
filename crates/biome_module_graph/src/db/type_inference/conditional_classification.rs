//! Fast conditional classification for expressions whose raw type is enough
//! to answer the question without resolving complete local type tables.

use crate::ModuleDb;
use crate::js_module_info::{JsExport, JsModuleInfo, JsOwnExport};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::type_inference::CaseLiteral;
use biome_js_syntax::numbers::canonicalize_js_bigint_literal;
use biome_js_type_info::interned_types::ConditionalType;
use biome_js_type_info::{
    GlobalTypeId, ImportSymbol, Literal, RawTypeData, RawTypeId, ScopeId, TypeId,
    TypeImportQualifier, TypeMember, TypeReference, TypeReferenceQualifier, TypeofExpression,
    global_types,
};
use biome_rowan::{Text, TextRange};
use camino::Utf8PathBuf;
use rustc_hash::FxHashSet;

const MAX_CONDITIONAL_CLASSIFICATION_STEPS: usize = 1024;

#[derive(Clone, Copy)]
enum MemberMode {
    Value,
    Instance,
}

struct RawReference {
    module: ModuleInfo,
    js_info: JsModuleInfo,
    reference: TypeReference,
}

struct ConditionalClassifier<'db> {
    db: &'db dyn ModuleDb,
    seen_local_types: FxHashSet<(Utf8PathBuf, TypeId)>,
    seen_imports: FxHashSet<(Utf8PathBuf, ImportSymbol)>,
    steps: usize,
}

pub(in crate::db) fn classify_expression_conditional(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    expression: TextRange,
) -> Option<ConditionalType> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let reference = js_info.raw_expressions.get(&expression)?.clone();
    let mut classifier = ConditionalClassifier {
        db,
        seen_local_types: FxHashSet::default(),
        seen_imports: FxHashSet::default(),
        steps: 0,
    };
    classifier.classify_reference(RawReference {
        module,
        js_info,
        reference,
    })
}

pub(in crate::db) fn classify_expression_case_literal(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    expression: TextRange,
    literal: &CaseLiteral,
) -> Option<bool> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let reference = js_info.raw_expressions.get(&expression)?.clone();
    let mut classifier = ConditionalClassifier {
        db,
        seen_local_types: FxHashSet::default(),
        seen_imports: FxHashSet::default(),
        steps: 0,
    };
    classifier.could_equal_reference(
        RawReference {
            module,
            js_info,
            reference,
        },
        literal,
    )
}

impl<'db> ConditionalClassifier<'db> {
    fn classify_reference(&mut self, reference: RawReference) -> Option<ConditionalType> {
        if !self.step() {
            return None;
        }

        match &reference.reference {
            TypeReference::Resolved(RawTypeId::Local(type_id)) => {
                self.classify_local_type(&reference, *type_id)
            }
            TypeReference::Resolved(RawTypeId::Global(global_id)) => {
                self.classify_global_type(*global_id)
            }
            TypeReference::Qualifier(qualifier) => {
                let reference = self.resolve_qualifier(&reference, qualifier)?;
                self.classify_reference(reference)
            }
            TypeReference::Import(import) => self.classify_import(import),
        }
    }

    fn classify_local_type(
        &mut self,
        reference: &RawReference,
        type_id: TypeId,
    ) -> Option<ConditionalType> {
        let key = (reference.module.path(self.db).clone(), type_id);
        if !self.seen_local_types.insert(key.clone()) {
            return None;
        }

        let result = reference
            .js_info
            .raw_types
            .get(type_id.index())
            .cloned()
            .and_then(|raw| self.classify_raw_type(reference, &raw));

        self.seen_local_types.remove(&key);
        result
    }

    fn could_equal_reference(
        &mut self,
        reference: RawReference,
        literal: &CaseLiteral,
    ) -> Option<bool> {
        if !self.step() {
            return None;
        }

        match &reference.reference {
            TypeReference::Resolved(RawTypeId::Local(type_id)) => {
                let raw = reference.js_info.raw_types.get(type_id.index())?.clone();
                self.could_equal_raw_type(&reference, &raw, literal)
            }
            TypeReference::Resolved(RawTypeId::Global(_)) => None,
            TypeReference::Qualifier(qualifier) => {
                let reference = self.resolve_qualifier(&reference, qualifier)?;
                self.could_equal_reference(reference, literal)
            }
            TypeReference::Import(import) => {
                if import.type_only {
                    return None;
                }
                if matches!(&import.symbol, ImportSymbol::All) {
                    return Some(false);
                }
                let reference = self.resolve_import_reference(import)?;
                self.could_equal_reference(reference, literal)
            }
        }
    }

    fn could_equal_raw_type(
        &mut self,
        source: &RawReference,
        raw: &RawTypeData,
        literal: &CaseLiteral,
    ) -> Option<bool> {
        match raw {
            RawTypeData::Unknown | RawTypeData::AnyKeyword | RawTypeData::UnknownKeyword => {
                Some(true)
            }
            RawTypeData::NeverKeyword => Some(false),
            RawTypeData::BigInt
            | RawTypeData::Class(_)
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Global
            | RawTypeData::ImportNamespace(_)
            | RawTypeData::Interface(_)
            | RawTypeData::Module(_)
            | RawTypeData::Namespace(_)
            | RawTypeData::Object(_)
            | RawTypeData::ObjectKeyword
            | RawTypeData::Symbol
            | RawTypeData::ThisKeyword
            | RawTypeData::Tuple(_) => Some(false),
            RawTypeData::Boolean => Some(matches!(literal, CaseLiteral::Boolean(_))),
            RawTypeData::Number => Some(matches!(literal, CaseLiteral::Number(_))),
            RawTypeData::String => Some(matches!(literal, CaseLiteral::String(_))),
            RawTypeData::Null | RawTypeData::Undefined | RawTypeData::VoidKeyword => {
                Some(matches!(literal, CaseLiteral::Null))
            }
            RawTypeData::Literal(value) => Some(raw_literal_could_equal(value, literal)),
            RawTypeData::Reference(reference) => self.could_equal_reference(
                RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: reference.clone(),
                },
                literal,
            ),
            RawTypeData::TypeofType(reference) => self.could_equal_reference(
                RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: reference.as_ref().clone(),
                },
                literal,
            ),
            RawTypeData::TypeofValue(value) => {
                let reference = if value.ty.is_unknown() {
                    TypeReferenceQualifier::from_path(
                        value.scope_id.unwrap_or(ScopeId::GLOBAL),
                        value.identifier.clone(),
                    )
                    .into()
                } else {
                    value.ty.clone()
                };
                self.could_equal_reference(
                    RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference,
                    },
                    literal,
                )
            }
            RawTypeData::Generic(generic) => {
                if generic.constraint.is_unknown() {
                    Some(true)
                } else {
                    self.could_equal_reference(
                        RawReference {
                            module: source.module,
                            js_info: source.js_info.clone(),
                            reference: generic.constraint.clone(),
                        },
                        literal,
                    )
                }
            }
            RawTypeData::InstanceOf(instance) => self.could_equal_reference(
                RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: instance.ty.clone(),
                },
                literal,
            ),
            RawTypeData::Union(union) => {
                self.could_equal_union(source, union.types().iter(), literal)
            }
            RawTypeData::Intersection(_) => Some(false),
            RawTypeData::MergedReference(_)
            | RawTypeData::Conditional
            | RawTypeData::TypeOperator(_) => None,
            RawTypeData::TypeofExpression(_) => Some(true),
        }
    }

    fn could_equal_union<'a>(
        &mut self,
        source: &RawReference,
        references: impl IntoIterator<Item = &'a TypeReference>,
        literal: &CaseLiteral,
    ) -> Option<bool> {
        let mut could_equal = false;
        for reference in references {
            match self.could_equal_reference(
                RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: reference.clone(),
                },
                literal,
            )? {
                true => could_equal = true,
                false => {}
            }
        }
        Some(could_equal)
    }

    fn classify_global_type(&self, global_id: GlobalTypeId) -> Option<ConditionalType> {
        global_types(self.db)
            .get(global_id)
            .conditional_type_shallow(self.db)
    }

    fn classify_raw_type(
        &mut self,
        source: &RawReference,
        raw: &RawTypeData,
    ) -> Option<ConditionalType> {
        match raw {
            RawTypeData::Unknown
            | RawTypeData::AnyKeyword
            | RawTypeData::Conditional
            | RawTypeData::NeverKeyword
            | RawTypeData::ThisKeyword
            | RawTypeData::UnknownKeyword => Some(ConditionalType::Anything),
            RawTypeData::BigInt
            | RawTypeData::Boolean
            | RawTypeData::Interface(_)
            | RawTypeData::Number
            | RawTypeData::String => Some(ConditionalType::NonNullish),
            RawTypeData::Class(_)
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Global
            | RawTypeData::ImportNamespace(_)
            | RawTypeData::Module(_)
            | RawTypeData::Namespace(_)
            | RawTypeData::Object(_)
            | RawTypeData::ObjectKeyword
            | RawTypeData::Symbol
            | RawTypeData::Tuple(_) => Some(ConditionalType::Truthy),
            RawTypeData::Null | RawTypeData::Undefined | RawTypeData::VoidKeyword => {
                Some(ConditionalType::Nullish)
            }
            RawTypeData::Literal(literal) => Some(match literal.as_ref() {
                Literal::BigInt(text) => match canonicalize_js_bigint_literal(text.text()) {
                    Some(text) if text == "0n" => ConditionalType::FalsyButNotNullish,
                    Some(_) => ConditionalType::Truthy,
                    None => ConditionalType::Anything,
                },
                Literal::Boolean(boolean) => {
                    if boolean.as_bool() {
                        ConditionalType::Truthy
                    } else {
                        ConditionalType::FalsyButNotNullish
                    }
                }
                Literal::Number(number) => match number.to_f64() {
                    Some(number) if number == 0. || number.is_nan() => {
                        ConditionalType::FalsyButNotNullish
                    }
                    Some(_) => ConditionalType::Truthy,
                    None => ConditionalType::Anything,
                },
                Literal::Object(_) | Literal::RegExp(_) => ConditionalType::Truthy,
                Literal::String(string) => {
                    if string.as_str().is_empty() {
                        ConditionalType::FalsyButNotNullish
                    } else {
                        ConditionalType::Truthy
                    }
                }
                Literal::Template(_) => ConditionalType::Anything,
            }),
            RawTypeData::Reference(reference) => self.classify_reference(RawReference {
                module: source.module,
                js_info: source.js_info.clone(),
                reference: reference.clone(),
            }),
            RawTypeData::TypeofType(reference) => self.classify_reference(RawReference {
                module: source.module,
                js_info: source.js_info.clone(),
                reference: reference.as_ref().clone(),
            }),
            RawTypeData::TypeofValue(value) => {
                if value.ty.is_unknown() {
                    let qualifier = TypeReferenceQualifier::from_path(
                        value.scope_id.unwrap_or(ScopeId::GLOBAL),
                        value.identifier.clone(),
                    );
                    self.classify_reference(RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: qualifier.into(),
                    })
                } else {
                    self.classify_reference(RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: value.ty.clone(),
                    })
                }
            }
            RawTypeData::Generic(generic) => (!generic.constraint.is_unknown())
                .then(|| {
                    self.classify_reference(RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: generic.constraint.clone(),
                    })
                })
                .flatten(),
            RawTypeData::InstanceOf(instance) => self.classify_reference(RawReference {
                module: source.module,
                js_info: source.js_info.clone(),
                reference: instance.ty.clone(),
            }),
            RawTypeData::MergedReference(reference) => self.merge_references(
                source,
                [
                    reference.ty.as_ref(),
                    reference.value_ty.as_ref(),
                    reference.namespace_ty.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ),
            RawTypeData::Union(union) => self.merge_references(source, union.types().iter()),
            RawTypeData::Intersection(intersection) => {
                self.merge_references(source, intersection.types().iter())
            }
            RawTypeData::TypeOperator(_) => None,
            RawTypeData::TypeofExpression(expression) => {
                self.classify_expression_type(source, expression.as_ref())
            }
        }
    }

    fn classify_expression_type(
        &mut self,
        source: &RawReference,
        expression: &TypeofExpression,
    ) -> Option<ConditionalType> {
        match expression {
            TypeofExpression::Addition(_)
            | TypeofExpression::BitwiseNot(_)
            | TypeofExpression::UnaryMinus(_) => Some(ConditionalType::NonNullish),
            TypeofExpression::New(_) | TypeofExpression::Typeof(_) => Some(ConditionalType::Truthy),
            TypeofExpression::StaticMember(expression) => {
                let member = self.member_reference(
                    source,
                    &expression.object,
                    expression.member.text(),
                    MemberMode::Value,
                )?;
                self.classify_reference(member)
            }
            TypeofExpression::OptionalChainStaticMember(_)
            | TypeofExpression::Call(_)
            | TypeofExpression::Await(_)
            | TypeofExpression::Destructure(_)
            | TypeofExpression::Index(_)
            | TypeofExpression::OptionalChainIndex(_)
            | TypeofExpression::IterableValueOf(_)
            | TypeofExpression::Super(_)
            | TypeofExpression::This(_) => None,
            TypeofExpression::Conditional(expression) => {
                let consequent = self.classify_reference(RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: expression.consequent.clone(),
                })?;
                let alternate = self.classify_reference(RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: expression.alternate.clone(),
                })?;
                let merged = consequent.merged_with(alternate);

                if merged.is_truthy() || merged.is_falsy() || merged.is_non_nullish() {
                    return Some(merged);
                }

                let test = self.classify_reference(RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: expression.test.clone(),
                })?;
                if test.is_truthy() {
                    Some(consequent)
                } else if test.is_falsy() {
                    Some(alternate)
                } else {
                    Some(merged)
                }
            }
            TypeofExpression::LogicalAnd(expression) => {
                let left = self.classify_reference(RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: expression.left.clone(),
                })?;
                if left.is_truthy() {
                    self.classify_reference(RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: expression.right.clone(),
                    })
                } else if left.is_falsy() {
                    Some(left)
                } else {
                    None
                }
            }
            TypeofExpression::LogicalOr(expression) => {
                let left = self.classify_reference(RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: expression.left.clone(),
                })?;
                if left.is_truthy() {
                    Some(left)
                } else if left.is_falsy() {
                    self.classify_reference(RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: expression.right.clone(),
                    })
                } else {
                    None
                }
            }
            TypeofExpression::NullishCoalescing(expression) => {
                let left = self.classify_reference(RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: expression.left.clone(),
                })?;
                if left.is_non_nullish() {
                    Some(left)
                } else if left.is_nullish() {
                    self.classify_reference(RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: expression.right.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }

    fn merge_references<'a>(
        &mut self,
        source: &RawReference,
        references: impl IntoIterator<Item = &'a TypeReference>,
    ) -> Option<ConditionalType> {
        let mut merged = ConditionalType::Unknown;
        let mut found = false;

        for reference in references {
            let conditional = self.classify_reference(RawReference {
                module: source.module,
                js_info: source.js_info.clone(),
                reference: reference.clone(),
            })?;
            merged = if found {
                merged.merged_with(conditional)
            } else {
                conditional
            };
            found = true;
        }

        Some(if found {
            merged
        } else {
            ConditionalType::Anything
        })
    }

    fn resolve_qualifier(
        &mut self,
        source: &RawReference,
        qualifier: &TypeReferenceQualifier,
    ) -> Option<RawReference> {
        if !qualifier.type_parameters.is_empty() {
            return None;
        }

        let mut path = qualifier.path.iter();
        let identifier = path.next()?;
        let mut scope = source
            .js_info
            .semantic_model
            .scope_from_id(qualifier.scope_id);
        let binding = loop {
            if let Some(binding) = scope.get_binding(identifier.text()) {
                break binding;
            }
            scope = scope.parent()?;
        };

        let range = binding.syntax().text_trimmed_range();
        let mut reference = RawReference {
            module: source.module,
            js_info: source.js_info.clone(),
            reference: source.js_info.raw_binding_types.get(&range)?.clone(),
        };
        for member in path {
            let raw_reference = reference.reference.clone();
            reference = self.member_reference(
                &reference,
                &raw_reference,
                member.text(),
                MemberMode::Value,
            )?;
        }
        Some(reference)
    }

    fn member_reference(
        &mut self,
        source: &RawReference,
        reference: &TypeReference,
        name: &str,
        mode: MemberMode,
    ) -> Option<RawReference> {
        if let TypeReference::Import(import) = reference {
            return self.import_member_reference(import, name, mode);
        }

        let raw = self.raw_type(source, reference)?;
        match raw {
            RawTypeData::Reference(reference) => {
                self.member_reference(source, &reference, name, mode)
            }
            RawTypeData::TypeofType(reference) => {
                self.member_reference(source, reference.as_ref(), name, mode)
            }
            RawTypeData::TypeofValue(value) => {
                let reference = if value.ty.is_unknown() {
                    TypeReferenceQualifier::from_path(
                        value.scope_id.unwrap_or(ScopeId::GLOBAL),
                        value.identifier,
                    )
                    .into()
                } else {
                    value.ty
                };
                self.member_reference(source, &reference, name, mode)
            }
            RawTypeData::TypeofExpression(expression) => {
                let TypeofExpression::StaticMember(expression) = expression.as_ref() else {
                    return None;
                };
                let member = self.member_reference(
                    source,
                    &expression.object,
                    expression.member.text(),
                    MemberMode::Value,
                )?;
                let member_reference = member.reference.clone();
                self.member_reference(&member, &member_reference, name, mode)
            }
            RawTypeData::InstanceOf(instance) => {
                self.member_reference(source, &instance.ty, name, MemberMode::Instance)
            }
            RawTypeData::Class(class) => {
                find_member(&class.members, name, mode).map(|member| RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: member.ty.clone(),
                })
            }
            RawTypeData::Interface(interface) => {
                find_member(&interface.members, name, MemberMode::Instance).map(|member| {
                    RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: member.ty.clone(),
                    }
                })
            }
            RawTypeData::Object(object) => find_member(&object.members, name, MemberMode::Instance)
                .map(|member| RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: member.ty.clone(),
                }),
            RawTypeData::Module(module) => find_member(&module.members, name, MemberMode::Value)
                .map(|member| RawReference {
                    module: source.module,
                    js_info: source.js_info.clone(),
                    reference: member.ty.clone(),
                }),
            RawTypeData::Namespace(namespace) => {
                find_member(&namespace.members, name, MemberMode::Value).map(|member| {
                    RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: member.ty.clone(),
                    }
                })
            }
            RawTypeData::Literal(literal) => {
                let Literal::Object(object) = literal.as_ref() else {
                    return None;
                };
                find_member(object.members(), name, MemberMode::Instance).map(|member| {
                    RawReference {
                        module: source.module,
                        js_info: source.js_info.clone(),
                        reference: member.ty.clone(),
                    }
                })
            }
            RawTypeData::Union(_)
            | RawTypeData::Intersection(_)
            | RawTypeData::MergedReference(_) => None,
            RawTypeData::Unknown
            | RawTypeData::Global
            | RawTypeData::BigInt
            | RawTypeData::Boolean
            | RawTypeData::Null
            | RawTypeData::Number
            | RawTypeData::String
            | RawTypeData::Symbol
            | RawTypeData::Undefined
            | RawTypeData::Conditional
            | RawTypeData::ImportNamespace(_)
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Tuple(_)
            | RawTypeData::Generic(_)
            | RawTypeData::TypeOperator(_)
            | RawTypeData::AnyKeyword
            | RawTypeData::NeverKeyword
            | RawTypeData::ObjectKeyword
            | RawTypeData::ThisKeyword
            | RawTypeData::UnknownKeyword
            | RawTypeData::VoidKeyword => None,
        }
    }

    fn raw_type(
        &mut self,
        source: &RawReference,
        reference: &TypeReference,
    ) -> Option<RawTypeData> {
        if !self.step() {
            return None;
        }

        match reference {
            TypeReference::Resolved(RawTypeId::Local(type_id)) => {
                source.js_info.raw_types.get(type_id.index()).cloned()
            }
            TypeReference::Resolved(RawTypeId::Global(_)) | TypeReference::Import(_) => None,
            TypeReference::Qualifier(qualifier) => {
                let reference = self.resolve_qualifier(source, qualifier)?;
                let raw_reference = reference.reference.clone();
                self.raw_type(&reference, &raw_reference)
            }
        }
    }

    fn classify_import(&mut self, import: &TypeImportQualifier) -> Option<ConditionalType> {
        if import.type_only {
            return None;
        }

        if matches!(&import.symbol, ImportSymbol::All) {
            return Some(ConditionalType::Truthy);
        }

        let reference = self.resolve_import_reference(import)?;
        self.classify_reference(reference)
    }

    fn resolve_import_reference(&mut self, import: &TypeImportQualifier) -> Option<RawReference> {
        let (module, _) = self.module_for_import(import)?;
        self.resolve_export_reference(module, &import.symbol)
    }

    fn import_member_reference(
        &mut self,
        import: &TypeImportQualifier,
        name: &str,
        mode: MemberMode,
    ) -> Option<RawReference> {
        if import.type_only {
            return None;
        }

        let (module, _) = self.module_for_import(import)?;
        if matches!(&import.symbol, ImportSymbol::All) {
            return self.resolve_export_reference(
                module,
                &ImportSymbol::Named(Text::from(name.to_string())),
            );
        }

        let reference = self.resolve_import_reference(import)?;
        let raw_reference = reference.reference.clone();
        self.member_reference(&reference, &raw_reference, name, mode)
    }

    fn module_for_import(
        &self,
        import: &TypeImportQualifier,
    ) -> Option<(ModuleInfo, JsModuleInfo)> {
        let path = import.resolved_path.as_path()?;
        let module = self.db.module_for_path(path)?;
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return None;
        };
        Some((module, js_info))
    }

    fn resolve_export_reference(
        &mut self,
        module: ModuleInfo,
        symbol: &ImportSymbol,
    ) -> Option<RawReference> {
        let name = match symbol {
            ImportSymbol::Default => Text::from("default"),
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::All => return None,
        };
        let key = (module.path(self.db).clone(), symbol.clone());
        if !self.seen_imports.insert(key.clone()) {
            return None;
        }

        let result = match module.kind(self.db) {
            ModuleInfoKind::Js(js_info) => js_info
                .exports
                .get(&name)
                .cloned()
                .and_then(|export| self.raw_reference_for_export(module, &js_info, export)),
            ModuleInfoKind::Css(_) | ModuleInfoKind::Html(_) => None,
        };

        self.seen_imports.remove(&key);
        result
    }

    fn raw_reference_for_export(
        &mut self,
        module: ModuleInfo,
        js_info: &JsModuleInfo,
        export: JsExport,
    ) -> Option<RawReference> {
        match export {
            JsExport::Own(own) => match own {
                JsOwnExport::Binding(range) => {
                    js_info
                        .raw_binding_types
                        .get(&range)
                        .map(|reference| RawReference {
                            module,
                            js_info: js_info.clone(),
                            reference: reference.clone(),
                        })
                }
                JsOwnExport::Type(type_id) => Some(RawReference {
                    module,
                    js_info: js_info.clone(),
                    reference: TypeReference::Resolved(RawTypeId::Local(type_id)),
                }),
                JsOwnExport::Namespace(reexport) => Some(RawReference {
                    module,
                    js_info: js_info.clone(),
                    reference: TypeReference::from(TypeImportQualifier {
                        symbol: ImportSymbol::All,
                        resolved_path: reexport.import.resolved_path.clone(),
                        type_only: false,
                    }),
                }),
            },
            JsExport::Reexport(reexport) => {
                let (target, _) = self.module_for_import(&TypeImportQualifier {
                    symbol: reexport.import.symbol.clone(),
                    resolved_path: reexport.import.resolved_path.clone(),
                    type_only: false,
                })?;
                self.resolve_export_reference(target, &reexport.import.symbol)
            }
            JsExport::OwnType(_) | JsExport::ReexportType(_) => None,
        }
    }

    fn step(&mut self) -> bool {
        self.steps += 1;
        self.steps <= MAX_CONDITIONAL_CLASSIFICATION_STEPS
    }
}

fn raw_literal_could_equal(value: &Literal, literal: &CaseLiteral) -> bool {
    match (value, literal) {
        (Literal::Boolean(value), CaseLiteral::Boolean(expected)) => value.as_bool() == *expected,
        (Literal::Number(value), CaseLiteral::Number(expected)) => {
            value.to_f64() == Some(f64::from_bits(*expected))
        }
        (Literal::String(value), CaseLiteral::String(expected)) => {
            value.as_str() == expected.text()
        }
        (
            Literal::Object(_) | Literal::RegExp(_) | Literal::Template(_) | Literal::BigInt(_),
            _,
        ) => false,
        _ => false,
    }
}

fn find_member<'a>(
    members: &'a [TypeMember],
    name: &str,
    mode: MemberMode,
) -> Option<&'a TypeMember> {
    let member = members.iter().find(|member| {
        member.has_name(name)
            && match mode {
                MemberMode::Value => member.is_static(),
                MemberMode::Instance => !member.is_static(),
            }
    })?;
    (!member.is_getter()).then_some(member)
}
