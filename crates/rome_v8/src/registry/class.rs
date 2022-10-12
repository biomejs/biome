use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use v8::{FunctionCallback, MapFnTo};

use super::Template;

pub(crate) struct ClassBuilder<'scope, 'builder, T> {
    pub(super) template: v8::Local<'scope, v8::FunctionTemplate>,
    pub(super) entry: &'builder mut HashMap<TypeId, Template>,
    pub(super) _ty: PhantomData<T>,
}

impl<T: 'static> ClassBuilder<'_, '_, T> {
    pub(crate) fn extends<P: 'static>(self, scope: &mut v8::HandleScope<'_, ()>) -> Self {
        let parent = match self.entry.get(&TypeId::of::<P>()) {
            Some(Template::Class(parent)) => parent,
            Some(_) => panic!("type {} is not registered as a class", type_name::<P>()),
            None => panic!("class {} is not in the registry", type_name::<P>()),
        };

        let parent = v8::Local::new(scope, parent);
        self.template.inherit(parent);
        self
    }

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
        self.entry.insert(
            TypeId::of::<T>(),
            Template::Class(v8::Global::new(scope, self.template)),
        );
    }
}
