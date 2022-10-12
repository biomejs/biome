use std::{any::TypeId, collections::hash_map, marker::PhantomData};

use anyhow::Result;
use v8::{FunctionCallback, MapFnTo};

use crate::support::UnitType;

use super::{load_native, Template};

pub(crate) struct InterfaceBuilder<'scope, 'builder, T> {
    pub(super) template: v8::Local<'scope, v8::ObjectTemplate>,
    pub(super) entry: hash_map::Entry<'builder, TypeId, Template>,
    pub(super) _ty: PhantomData<T>,
}

impl<T> InterfaceBuilder<'_, '_, T> {
    pub(crate) fn method(
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

    pub(crate) fn iterable<'s, F>(self, scope: &mut v8::HandleScope<'_, ()>, callback: F) -> Self
    where
        T: Iterator + 'static,
        F: UnitType + Fn(T::Item, &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>>,
    {
        let _ = callback;

        let key = v8::String::new(scope, "next").expect("failed to allocate string");
        let value = v8::FunctionTemplate::new(scope, iterator_next::<T, F>);
        self.template.set(key.into(), value.into());

        let key = v8::Symbol::get_iterator(scope);
        let value = v8::FunctionTemplate::new(scope, iterable_iterator);
        self.template.set(key.into(), value.into());
        self
    }

    pub(crate) fn finish(self, scope: &mut v8::HandleScope<'_, ()>) {
        self.entry
            .or_insert_with(|| Template::Interface(v8::Global::new(scope, self.template)));
    }
}

fn iterable_iterator(
    _scope: &mut v8::HandleScope<'_>,
    args: v8::FunctionCallbackArguments<'_>,
    mut res: v8::ReturnValue<'_>,
) {
    res.set(args.this().into());
}

fn iterator_next<'s, T, F>(
    scope: &'_ mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue<'_>,
) where
    T: Iterator + 'static,
    F: UnitType + Fn(T::Item, &mut v8::HandleScope<'s>) -> Result<v8::Local<'s, v8::Value>>,
{
    let iter = args.this().into();
    let iter = load_native::<T>(iter).unwrap();

    let item = {
        let mut iter = iter.borrow_mut();
        iter.next()
    };

    let result = v8::Object::new(scope);

    let key = v8::String::new(scope, "done").unwrap();
    let value = v8::Boolean::new(scope, item.is_none());
    result.set(scope, key.into(), value.into());

    if let Some(item) = item {
        let key = v8::String::new(scope, "value").unwrap();
        let value = (F::get())(item, scope).unwrap();
        result.set(scope, key.into(), value);
    }

    res.set(result.into());
}
