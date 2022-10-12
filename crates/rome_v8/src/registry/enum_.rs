use std::{
    any::{type_name, Any, TypeId},
    collections::{hash_map, HashMap},
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};

use anyhow::{Context, Result};
use rome_rowan::{RawSyntaxKind, SyntaxKind};
use v8::{FunctionCallback, MapFnTo};

use super::{Template, TemplateRegistry};

pub(crate) struct EnumBuilder<'scope, 'builder, T> {
    pub(super) template: v8::Local<'scope, v8::FunctionTemplate>,
    pub(super) entry: hash_map::Entry<'builder, TypeId, Template>,
    pub(super) cache: Vec<Option<&'static str>>,
    pub(super) _ty: PhantomData<T>,
}

impl<T: Debug + SyntaxKind + 'static> EnumBuilder<'_, '_, T> {
    pub(crate) fn variant(mut self, name: &'static str, value: T) -> Self {
        let RawSyntaxKind(value) = value.to_raw();
        let index = value as usize;
        if self.cache.len() <= index {
            self.cache.resize(index + 1, None);
        }

        self.cache[index] = Some(name);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn method(
        self,
        scope: &mut v8::HandleScope<'_, ()>,
        name: &str,
        callback: impl MapFnTo<FunctionCallback>,
    ) -> Self {
        let prototype_template = self.template.prototype_template(scope);
        let key = v8::String::new(scope, name).expect("failed to allocate string");
        let value = v8::FunctionTemplate::new(scope, callback);
        prototype_template.set(key.into(), value.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn method_static(
        self,
        scope: &mut v8::HandleScope<'_, ()>,
        name: &str,
        callback: impl MapFnTo<FunctionCallback>,
    ) -> Self {
        let key = v8::String::new(scope, name).expect("failed to allocate string");
        let value = v8::FunctionTemplate::new(scope, callback);
        self.template.set(key.into(), value.into());
        self
    }

    pub(crate) fn finish(self, scope: &mut v8::HandleScope<'_, ()>) {
        let cache = self.cache;
        let template = v8::Global::new(scope, self.template);

        let factory = Template::Enum(Box::new(move |scope| {
            let template = v8::Local::new(scope, &template);
            let function = template.get_function(scope).unwrap();

            let cache: Vec<_> = cache
                .iter()
                .enumerate()
                .map(|(value, key)| {
                    let key = v8::String::new(scope, (*key)?).unwrap();

                    let instance = function.new_instance(scope, &[]).unwrap();
                    function.set(scope, key.into(), instance.into());

                    let value = v8::Number::new(scope, value as f64);
                    instance.set_internal_field(0, value.into());

                    Some(v8::Global::new(scope, instance))
                })
                .collect();

            Box::new(move |scope, value| {
                let value = value.downcast_ref::<T>().unwrap();
                let RawSyntaxKind(index) = value.to_raw();

                let entry = cache
                    .get(index as usize)
                    .and_then(Option::as_ref)
                    .unwrap_or_else(|| {
                        panic!(
                            "enum cache for type {} is missing an entry for value {value:?}",
                            type_name::<T>()
                        )
                    });

                v8::Local::new(scope, entry)
            })
        }));

        self.entry.or_insert(factory);
    }
}

pub(super) type EnumFactoryBuilder = Box<dyn Fn(&mut v8::HandleScope) -> EnumFactory>;

type EnumFactory = Box<
    dyn for<'scope> Fn(&mut v8::HandleScope<'scope>, &dyn Any) -> v8::Local<'scope, v8::Object>,
>;

#[derive(Default)]
pub(crate) struct EnumRegistry {
    pub(super) enums: HashMap<TypeId, EnumFactory>,
}

impl EnumRegistry {
    pub(crate) fn install(scope: &mut v8::HandleScope<'_>) -> Result<()> {
        let registry = scope
            .get_slot::<Rc<TemplateRegistry>>()
            .context("class registry slot on isolate is empty")?
            .clone();

        let mut enums = HashMap::new();

        for (key, value) in &registry.classes {
            let factory_builder = if let Template::Enum(value) = value {
                value
            } else {
                continue;
            };

            enums.insert(*key, factory_builder(scope));
        }

        let context = scope.get_current_context();
        context.set_slot(scope, Rc::new(Self { enums }));

        Ok(())
    }
}
