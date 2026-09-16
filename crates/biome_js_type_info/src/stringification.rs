//! Classification of values converted to strings by JavaScript operations.
//!
//! String conversion depends on the operation, compound-type composition,
//! configured ignored names, and whether an object inherits the default object
//! conversion hooks. [`StringificationAnalyzer`] owns those rules and evaluates
//! nested types iteratively so recursive types cannot consume the Rust stack.

use crate::{
    TypeDb,
    interned_types::{Literal, TypeData, TypeMember},
};
use rustc_hash::FxHashSet;

const MAX_STRINGIFICATION_DEPTH: usize = 1024;

/// JavaScript operation that triggers string conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringificationMode {
    /// An array or tuple is converted by `Array.prototype.join`.
    Join,
    /// Ordinary conversion through object conversion hooks and primitives.
    ToString,
}

/// Whether string conversion produces a useful representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringificationUsefulness {
    /// No possible type is known to require the base object representation.
    Always,
    /// Some possible types use the base object representation.
    Sometimes,
    /// Every possible type uses the base object representation.
    Never,
}

pub(crate) struct StringificationAnalyzer<'db, 'names> {
    db: &'db dyn TypeDb,
    ignored_type_names: &'names [&'names str],
}

impl<'db, 'names> StringificationAnalyzer<'db, 'names> {
    pub(crate) fn new(db: &'db dyn TypeDb, ignored_type_names: &'names [&'names str]) -> Self {
        Self {
            db,
            ignored_type_names,
        }
    }

    pub(crate) fn analyze(
        &self,
        root: TypeData<'db>,
        mode: StringificationMode,
    ) -> StringificationUsefulness {
        let mut active = FxHashSet::default();
        let mut frames = vec![StringificationFrame::Enter {
            data: root,
            mode,
            depth: 0,
        }];
        let mut results = Vec::new();

        while let Some(frame) = frames.pop() {
            match frame {
                StringificationFrame::Enter { data, mode, depth } => {
                    if depth >= MAX_STRINGIFICATION_DEPTH || !active.insert(data) {
                        results.push(StringificationUsefulness::Always);
                        continue;
                    }

                    let step = if let TypeData::Generic(generic) = data {
                        generic.constraint(self.db).map_or(
                            StringificationStep::Complete(StringificationUsefulness::Always),
                            |constraint| StringificationStep::Children {
                                combination: StringificationCombination::Single,
                                children: vec![(constraint, mode)],
                            },
                        )
                    } else if matches!(mode, StringificationMode::ToString) {
                        match self.is_ignored_type(data) {
                            None | Some(true) => {
                                StringificationStep::Complete(StringificationUsefulness::Always)
                            }
                            Some(false) if self.is_safe_type(data) => {
                                StringificationStep::Complete(StringificationUsefulness::Always)
                            }
                            Some(false) => self.unignored_step(data, mode, depth),
                        }
                    } else {
                        self.unignored_step(data, mode, depth)
                    };

                    match step {
                        StringificationStep::Complete(result) => {
                            active.remove(&data);
                            results.push(result);
                        }
                        StringificationStep::Children {
                            combination,
                            children,
                        } => {
                            frames.push(StringificationFrame::Combine {
                                data,
                                combination,
                                child_count: children.len(),
                            });
                            frames.extend(children.into_iter().rev().map(|(data, mode)| {
                                StringificationFrame::Enter {
                                    data,
                                    mode,
                                    depth: depth + 1,
                                }
                            }));
                        }
                    }
                }
                StringificationFrame::Combine {
                    data,
                    combination,
                    child_count,
                } => {
                    let first_child = results.len() - child_count;
                    let result = combination.combine(&results[first_child..]);
                    results.truncate(first_child);
                    results.push(result);
                    active.remove(&data);
                }
            }
        }

        results.pop().unwrap_or(StringificationUsefulness::Always)
    }

    fn unignored_step(
        &self,
        data: TypeData<'db>,
        mode: StringificationMode,
        depth: usize,
    ) -> StringificationStep<'db> {
        match data {
            TypeData::Union(union) => StringificationStep::Children {
                combination: StringificationCombination::Union,
                children: union
                    .types(self.db)
                    .iter()
                    .copied()
                    .map(|ty| (ty, mode))
                    .collect(),
            },
            TypeData::Intersection(intersection) => StringificationStep::Children {
                combination: StringificationCombination::Intersection,
                children: intersection
                    .types(self.db)
                    .iter()
                    .copied()
                    .map(|ty| (ty, mode))
                    .collect(),
            },
            TypeData::Tuple(tuple) => StringificationStep::Children {
                combination: StringificationCombination::Tuple,
                children: tuple
                    .elements(self.db)
                    .iter()
                    .map(|element| (element.ty, StringificationMode::ToString))
                    .collect(),
            },
            TypeData::InstanceOf(instance) if instance.ty(self.db).is_array_class(self.db) => {
                instance.type_parameters(self.db).first().map_or(
                    StringificationStep::Complete(StringificationUsefulness::Always),
                    |element| StringificationStep::Children {
                        combination: StringificationCombination::Single,
                        children: vec![(*element, StringificationMode::ToString)],
                    },
                )
            }
            TypeData::InstanceOf(instance) => StringificationStep::Children {
                combination: StringificationCombination::Single,
                children: vec![(instance.ty(self.db), mode)],
            },
            _ if matches!(mode, StringificationMode::Join) => {
                StringificationStep::Complete(StringificationUsefulness::Always)
            }
            _ => StringificationStep::Complete(
                self.base_stringification(data, depth + 1).into_usefulness(),
            ),
        }
    }

    fn is_safe_type(&self, data: TypeData<'db>) -> bool {
        match data {
            TypeData::AnyKeyword
            | TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Function(_)
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Undefined
            | TypeData::Unknown
            | TypeData::UnknownKeyword
            | TypeData::NeverKeyword
            | TypeData::VoidKeyword => true,
            TypeData::Literal(literal) => matches!(
                literal.literal(self.db),
                Literal::BigInt(_)
                    | Literal::Boolean(_)
                    | Literal::Number(_)
                    | Literal::String(_)
                    | Literal::Template(_)
            ),
            _ => false,
        }
    }

    fn is_ignored_type(&self, root: TypeData<'db>) -> Option<bool> {
        let mut seen = FxHashSet::default();
        let mut pending = vec![root];
        let mut remaining_steps = MAX_STRINGIFICATION_DEPTH;

        while let Some(data) = pending.pop() {
            if !seen.insert(data) {
                continue;
            }
            if remaining_steps == 0 {
                return None;
            }
            remaining_steps -= 1;

            let name = match data {
                TypeData::Class(class) => class.name(self.db).as_ref().map(|name| name.text()),
                TypeData::Generic(generic) => Some(generic.name(self.db).text()),
                TypeData::Interface(interface) => Some(interface.name(self.db).text()),
                TypeData::Literal(literal)
                    if matches!(literal.literal(self.db), Literal::RegExp(_)) =>
                {
                    Some("RegExp")
                }
                TypeData::TypeofValue(value) => Some(value.identifier(self.db).text()),
                _ => None,
            };
            if name.is_some_and(|name| self.ignored_type_names.contains(&name)) {
                return Some(true);
            }

            match data {
                TypeData::Class(class) => pending.extend(class.extends(self.db)),
                TypeData::Generic(generic) => pending.extend(generic.constraint(self.db)),
                TypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                TypeData::Interface(interface) => {
                    pending.extend(interface.extends(self.db).iter().copied());
                }
                TypeData::MergedReference(reference) => {
                    pending.extend(reference.targets(self.db));
                }
                TypeData::TypeOperator(operator) => pending.push(operator.ty(self.db)),
                TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(self.db)),
                TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(self.db)),
                _ => {}
            }
        }

        Some(false)
    }

    fn base_stringification(&self, root: TypeData<'db>, root_depth: usize) -> BaseStringification {
        let mut active = FxHashSet::default();
        let mut frames = vec![BaseStringificationFrame::Enter {
            data: root,
            depth: root_depth,
        }];
        let mut results = Vec::new();

        while let Some(frame) = frames.pop() {
            match frame {
                BaseStringificationFrame::Enter { data, depth } => {
                    if depth >= MAX_STRINGIFICATION_DEPTH || !active.insert(data) {
                        results.push(BaseStringification::Suppress);
                        continue;
                    }

                    let step = match data {
                        TypeData::Class(class) => {
                            if class
                                .members(self.db)
                                .iter()
                                .any(TypeMember::is_custom_stringification_member)
                            {
                                BaseStringificationStep::Complete(BaseStringification::Suppress)
                            } else if let Some(base) = class.extends(self.db) {
                                BaseStringificationStep::Children(vec![base])
                            } else {
                                BaseStringificationStep::Complete(BaseStringification::Report)
                            }
                        }
                        TypeData::InstanceOf(instance) => {
                            BaseStringificationStep::Children(vec![instance.ty(self.db)])
                        }
                        TypeData::Interface(interface) => {
                            if interface
                                .members(self.db)
                                .iter()
                                .any(TypeMember::is_custom_stringification_member)
                            {
                                BaseStringificationStep::Complete(BaseStringification::Suppress)
                            } else if interface.extends(self.db).is_empty() {
                                BaseStringificationStep::Complete(BaseStringification::Report)
                            } else {
                                BaseStringificationStep::Children(
                                    interface.extends(self.db).to_vec(),
                                )
                            }
                        }
                        TypeData::Literal(literal) => match literal.literal(self.db) {
                            Literal::Object(members) => BaseStringificationStep::Complete(
                                if members
                                    .iter()
                                    .any(TypeMember::is_custom_stringification_member)
                                {
                                    BaseStringification::Suppress
                                } else {
                                    BaseStringification::Report
                                },
                            ),
                            Literal::RegExp(_) => {
                                BaseStringificationStep::Complete(BaseStringification::Report)
                            }
                            _ => BaseStringificationStep::Complete(BaseStringification::Suppress),
                        },
                        TypeData::MergedReference(reference) => {
                            BaseStringificationStep::Children(reference.targets(self.db).collect())
                        }
                        TypeData::Object(object) => BaseStringificationStep::Complete(
                            if object
                                .members(self.db)
                                .iter()
                                .any(TypeMember::is_custom_stringification_member)
                            {
                                BaseStringification::Suppress
                            } else {
                                BaseStringification::Report
                            },
                        ),
                        TypeData::ObjectKeyword => {
                            BaseStringificationStep::Complete(BaseStringification::Report)
                        }
                        TypeData::TypeofValue(value) => {
                            BaseStringificationStep::Children(vec![value.ty(self.db)])
                        }
                        _ => BaseStringificationStep::Complete(BaseStringification::Unknown),
                    };

                    match step {
                        BaseStringificationStep::Complete(result) => {
                            active.remove(&data);
                            results.push(result);
                        }
                        BaseStringificationStep::Children(children) => {
                            frames.push(BaseStringificationFrame::Combine {
                                data,
                                child_count: children.len(),
                            });
                            frames.extend(children.into_iter().rev().map(|data| {
                                BaseStringificationFrame::Enter {
                                    data,
                                    depth: depth + 1,
                                }
                            }));
                        }
                    }
                }
                BaseStringificationFrame::Combine { data, child_count } => {
                    let first_child = results.len() - child_count;
                    let result = results[first_child..]
                        .iter()
                        .copied()
                        .fold(BaseStringification::Unknown, BaseStringification::merge);
                    results.truncate(first_child);
                    results.push(result);
                    active.remove(&data);
                }
            }
        }

        results.pop().unwrap_or(BaseStringification::Unknown)
    }
}

enum StringificationFrame<'db> {
    Enter {
        data: TypeData<'db>,
        mode: StringificationMode,
        depth: usize,
    },
    Combine {
        data: TypeData<'db>,
        combination: StringificationCombination,
        child_count: usize,
    },
}

enum StringificationStep<'db> {
    Complete(StringificationUsefulness),
    Children {
        combination: StringificationCombination,
        children: Vec<(TypeData<'db>, StringificationMode)>,
    },
}

#[derive(Clone, Copy)]
enum StringificationCombination {
    Single,
    Union,
    Intersection,
    Tuple,
}

impl StringificationCombination {
    fn combine(self, children: &[StringificationUsefulness]) -> StringificationUsefulness {
        use StringificationUsefulness::{Always, Never, Sometimes};

        match self {
            Self::Single => children.first().copied().unwrap_or(Always),
            Self::Union => {
                let Some(first) = children.first() else {
                    return Always;
                };
                if children.iter().all(|result| result == first) {
                    *first
                } else {
                    Sometimes
                }
            }
            Self::Intersection => {
                if children.contains(&Always) {
                    Always
                } else {
                    Never
                }
            }
            Self::Tuple => {
                if children.contains(&Never) {
                    Never
                } else if children.contains(&Sometimes) {
                    Sometimes
                } else {
                    Always
                }
            }
        }
    }
}

enum BaseStringificationFrame<'db> {
    Enter {
        data: TypeData<'db>,
        depth: usize,
    },
    Combine {
        data: TypeData<'db>,
        child_count: usize,
    },
}

enum BaseStringificationStep<'db> {
    Complete(BaseStringification),
    Children(Vec<TypeData<'db>>),
}

/// Internal result of checking object conversion hooks and inherited types.
#[derive(Clone, Copy)]
enum BaseStringification {
    /// The type definitely uses the base object representation.
    Report,
    /// The diagnostic must be suppressed due to a custom hook, cycle, or limit.
    Suppress,
    /// The type shape does not determine which representation is used.
    Unknown,
}

impl BaseStringification {
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Suppress, _) | (_, Self::Suppress) => Self::Suppress,
            (Self::Report, _) | (_, Self::Report) => Self::Report,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        }
    }

    fn into_usefulness(self) -> StringificationUsefulness {
        match self {
            Self::Report => StringificationUsefulness::Never,
            Self::Suppress | Self::Unknown => StringificationUsefulness::Always,
        }
    }
}
