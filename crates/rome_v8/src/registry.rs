use std::{
    any::{type_name, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};

use anyhow::{ensure, Context, Result};
use v8::{FunctionCallback, MapFnTo};

mod class;
mod enum_;
mod interface;
mod namespace;

pub(crate) use self::enum_::EnumRegistry;
use self::{
    class::ClassBuilder,
    enum_::{EnumBuilder, EnumFactoryBuilder},
    interface::InterfaceBuilder,
    namespace::NamespaceBuilder,
};

#[derive(Default)]
pub(crate) struct TemplateRegistry {
    classes: HashMap<TypeId, Template>,
}

enum Template {
    Interface(v8::Global<v8::ObjectTemplate>),
    Class(v8::Global<v8::FunctionTemplate>),
    Enum(EnumFactoryBuilder),
}

impl TemplateRegistry {
    pub(crate) fn build_class<'scope, 'builder, T: 'static>(
        &'builder mut self,
        scope: &mut v8::HandleScope<'scope, ()>,
        global: v8::Local<'_, v8::ObjectTemplate>,
        name: &str,
    ) -> ClassBuilder<'scope, 'builder, T> {
        self.build_class_with_constructor(scope, global, name, default_constructor)
    }

    pub(crate) fn build_class_with_constructor<'scope, 'builder, T: 'static>(
        &'builder mut self,
        scope: &mut v8::HandleScope<'scope, ()>,
        global: v8::Local<'_, v8::ObjectTemplate>,
        name: &str,
        constructor: impl MapFnTo<FunctionCallback>,
    ) -> ClassBuilder<'scope, 'builder, T> {
        let template = v8::FunctionTemplate::new(scope, constructor);

        let instance_template = template.instance_template(scope);
        instance_template.set_internal_field_count(1);

        let name = v8::String::new(scope, name).expect("failed to allocate string");
        template.set_class_name(name);
        global.set(name.into(), template.into());

        ClassBuilder {
            template,
            entry: &mut self.classes,
            _ty: PhantomData,
        }
    }

    pub(crate) fn build_interface<'scope, 'builder, T: 'static>(
        &'builder mut self,
        scope: &mut v8::HandleScope<'scope, ()>,
    ) -> InterfaceBuilder<'scope, 'builder, T> {
        let template = v8::ObjectTemplate::new(scope);
        template.set_internal_field_count(1);

        InterfaceBuilder {
            template,
            entry: self.classes.entry(TypeId::of::<T>()),
            _ty: PhantomData,
        }
    }

    pub(crate) fn build_namespace<'scope, 'builder>(
        &'builder mut self,
        scope: &mut v8::HandleScope<'scope, ()>,
        global: v8::Local<'_, v8::ObjectTemplate>,
        name: &str,
    ) -> NamespaceBuilder<'scope> {
        let name = v8::String::new(scope, name).expect("failed to allocate string");
        let template = v8::ObjectTemplate::new(scope);
        global.set(name.into(), template.into());
        NamespaceBuilder { template }
    }

    pub(crate) fn build_enum<'scope, 'builder, T: 'static>(
        &'builder mut self,
        scope: &mut v8::HandleScope<'scope, ()>,
        global: v8::Local<'_, v8::ObjectTemplate>,
        name: &str,
    ) -> EnumBuilder<'scope, 'builder, T> {
        let template = v8::FunctionTemplate::new(scope, default_constructor);

        let instance_template = template.instance_template(scope);
        instance_template.set_internal_field_count(1);

        let name = v8::String::new(scope, name).expect("failed to allocate string");
        global.set(name.into(), template.into());
        template.set_class_name(name);

        EnumBuilder {
            template,
            entry: self.classes.entry(TypeId::of::<T>()),
            cache: Vec::new(),
            _ty: PhantomData,
        }
    }

    fn instantiate<'scope, T: 'static>(
        &self,
        scope: &mut v8::HandleScope<'scope>,
        value: T,
    ) -> v8::Local<'scope, v8::Object> {
        self.instantiate_as::<T, T>(scope, value)
    }

    fn instantiate_as<'scope, C: 'static, T: 'static>(
        &self,
        scope: &mut v8::HandleScope<'scope>,
        value: T,
    ) -> v8::Local<'scope, v8::Object> {
        let class = self.classes.get(&TypeId::of::<C>()).unwrap_or_else(|| {
            panic!(
                "could not find class template for type {}",
                type_name::<T>()
            )
        });

        let instance = match class {
            Template::Interface(template) => {
                let template = template.open(scope);
                template
                    .new_instance(scope)
                    .expect("could not instantiate object template")
            }
            Template::Class(template) => {
                let template = template.open(scope);
                let function = template
                    .get_function(scope)
                    .expect("function template is not instantiated in context");
                function
                    .new_instance(scope, &[])
                    .expect("class constructor did not return")
            }
            Template::Enum(_) => {
                let context = scope.get_current_context();
                let registry = context
                    .get_slot::<Rc<EnumRegistry>>(scope)
                    .expect("enum registry slot on isolate is empty")
                    .clone();

                let factory = registry.enums.get(&TypeId::of::<T>()).unwrap_or_else(|| {
                    panic!("could not find enum template for type {}", type_name::<T>())
                });

                return factory(scope, &value);
            }
        };

        store_native(scope, instance, value);
        instance
    }
}

#[repr(C)]
struct NativeData<T> {
    type_id: TypeId,
    obj: RefCell<T>,
    weak: Option<v8::Weak<v8::Object>>,
}

pub(crate) fn instantiate<'scope, T: 'static>(
    scope: &mut v8::HandleScope<'scope>,
    value: T,
) -> Result<v8::Local<'scope, v8::Object>> {
    let registry = scope
        .get_slot::<Rc<TemplateRegistry>>()
        .context("class registry slot on isolate is empty")?
        .clone();

    Ok(registry.instantiate(scope, value))
}

pub(crate) fn instantiate_as<'scope, C: 'static, T: 'static>(
    scope: &mut v8::HandleScope<'scope>,
    value: T,
) -> Result<v8::Local<'scope, v8::Object>> {
    let registry = scope
        .get_slot::<Rc<TemplateRegistry>>()
        .context("class registry slot on isolate is empty")?
        .clone();

    Ok(registry.instantiate_as::<C, T>(scope, value))
}

pub(crate) fn store_native<'s, T: 'static>(
    scope: &mut v8::HandleScope<'s, ()>,
    instance: v8::Local<'s, v8::Object>,
    value: T,
) {
    let value = Box::into_raw(Box::new(NativeData {
        obj: RefCell::new(value),
        type_id: TypeId::of::<T>(),
        weak: None,
    }));

    instance.set_aligned_pointer_in_internal_field(0, value as *mut c_void);

    let weak = v8::Weak::with_finalizer(
        scope,
        instance,
        Box::new(move |_isolate| unsafe {
            drop(Box::from_raw(value));
        }),
    );

    unsafe {
        (*value).weak = Some(weak);
    }
}

fn load_native<T: 'static>(value: v8::Local<v8::Value>) -> Result<&RefCell<T>> {
    let value: v8::Local<v8::Object> = value
        .try_into()
        .context("cannot deserialize a native handle from a non-object value")?;

    let ptr = unsafe { value.get_aligned_pointer_from_internal_field(0) };
    let ptr = ptr as *mut NativeData<T>;

    let handle = unsafe { ptr.as_ref().context("internal pointer is null")? };

    ensure!(
        handle.type_id == TypeId::of::<T>(),
        "handle type ID mismatch while trying to load object of type {}",
        type_name::<T>()
    );

    Ok(&handle.obj)
}

pub(crate) unsafe fn take_native<T: 'static>(value: v8::Local<v8::Value>) -> Result<T> {
    let value: v8::Local<v8::Object> = value
        .try_into()
        .context("cannot deserialize a native handle from a non-object value")?;

    let ptr = value.get_aligned_pointer_from_internal_field(0);
    let ptr = ptr as *mut NativeData<T>;

    let handle = ptr.as_mut().context("internal pointer is null")?;

    ensure!(
        handle.type_id == TypeId::of::<T>(),
        "handle type ID mismatch while trying to load object of type {}",
        type_name::<T>()
    );

    // Clear the finalizer before deallocating the external
    drop(handle.weak.take());

    let data = Box::from_raw(ptr);
    Ok(data.obj.into_inner())
}

pub(crate) fn borrow_native<T: 'static>(value: v8::Local<v8::Value>) -> Result<Ref<T>> {
    Ok(load_native(value)?.borrow())
}

pub(crate) fn borrow_mut_native<T: 'static>(value: v8::Local<v8::Value>) -> Result<RefMut<T>> {
    Ok(load_native(value)?.borrow_mut())
}

fn default_constructor(
    _scope: &mut v8::HandleScope<'_>,
    _args: v8::FunctionCallbackArguments<'_>,
    _res: v8::ReturnValue<'_>,
) {
}
