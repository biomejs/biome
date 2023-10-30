use std::rc::Rc;

use anyhow::Result;

use crate::registry::TemplateRegistry;

mod js;
mod rowan_idl;
// mod css;
// mod json;

pub(crate) fn setup_context<'scope>(
    scope: &mut v8::HandleScope<'scope, ()>,
) -> Result<v8::Local<'scope, v8::ObjectTemplate>> {
    let global = v8::ObjectTemplate::new(scope);
    let mut registry = TemplateRegistry::default();

    rowan_idl::register_interfaces(scope, global, &mut registry);
    js::register_interfaces(scope, global, &mut registry);
    // css::register_interfaces(scope, global, &mut registry);
    // json::register_interfaces(scope, global, &mut registry);

    scope.set_slot(Rc::new(registry));

    Ok(global)
}
