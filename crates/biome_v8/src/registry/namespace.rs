use v8::{FunctionCallback, MapFnTo};

pub(crate) struct NamespaceBuilder<'scope> {
    pub(super) template: v8::Local<'scope, v8::ObjectTemplate>,
}

impl NamespaceBuilder<'_> {
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
}
