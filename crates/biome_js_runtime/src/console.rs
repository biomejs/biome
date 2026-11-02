//! The `console` global available to plugins.
//!
//! Plugins run inside the language server and the CLI, where writing to stdout
//! would corrupt the output, so `console` only records the messages. The host
//! drains them with [`JsConsole::pull_messages`] and reports them like the
//! output of GritQL's `log()`.

use crate::ast::JsAstNode;
use crate::token::JsAstToken;
use boa_engine::object::ObjectInitializer;
use boa_engine::object::builtins::JsArray;
use boa_engine::property::Attribute;
use boa_engine::{Context, JsResult, JsValue, NativeFunction, js_string};
use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;

const METHODS: [&str; 5] = ["log", "info", "warn", "error", "debug"];

pub(crate) struct JsConsole {
    messages: Rc<RefCell<Vec<String>>>,
}

impl JsConsole {
    pub(crate) fn new() -> Self {
        Self {
            messages: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Defines the global `console` object. Every method records its formatted
    /// arguments as one message.
    pub(crate) fn register(&self, context: &mut Context) -> JsResult<()> {
        let mut console = ObjectInitializer::new(context);
        for method in METHODS {
            let messages = self.messages.clone();
            // SAFETY: The closure doesn't capture any GC-managed values.
            let function = unsafe {
                NativeFunction::from_closure(move |_this, args, context| {
                    let message = format_message(args, context)?;
                    messages.borrow_mut().push(message);
                    Ok(JsValue::undefined())
                })
            };
            console.function(function, js_string!(method), 0);
        }
        let console = console.build();

        context.register_global_property(
            js_string!("console"),
            console,
            Attribute::WRITABLE | Attribute::CONFIGURABLE,
        )
    }

    pub(crate) fn pull_messages(&self) -> Vec<String> {
        std::mem::take(&mut self.messages.borrow_mut())
    }
}

/// Arrays deeper than this are printed as `[...]`, which also stops cycles.
const MAX_ARRAY_DEPTH: usize = 3;

/// Joins the arguments with spaces. Strings are written as-is, nodes and tokens
/// as their kind, range, and source text, arrays element by element, and other
/// values with Boa's inspection format.
fn format_message(args: &[JsValue], context: &mut Context) -> JsResult<String> {
    let mut message = String::new();
    for (index, arg) in args.iter().enumerate() {
        if index > 0 {
            message.push(' ');
        }
        if let Some(string) = arg.as_string() {
            message.push_str(&string.to_std_string_lossy());
        } else {
            format_value(&mut message, arg, 0, context)?;
        }
    }
    Ok(message)
}

fn format_value(
    message: &mut String,
    value: &JsValue,
    depth: usize,
    context: &mut Context,
) -> JsResult<()> {
    if let Some(node) = JsAstNode::from_value(value) {
        let node = node.node;
        let _ = write!(message, "{node:?} {:?}", node.text_trimmed().to_string());
    } else if let Some(token) = JsAstToken::from_value(value) {
        let token = token.token;
        let _ = write!(
            message,
            "{:?}@{:?} {:?}",
            token.kind(),
            token.text_range(),
            token.text_trimmed()
        );
    } else if let Some(array) = value
        .as_object()
        .and_then(|object| JsArray::from_object(object).ok())
    {
        if depth >= MAX_ARRAY_DEPTH {
            message.push_str("[...]");
            return Ok(());
        }
        message.push('[');
        for index in 0..array.length(context)? {
            if index > 0 {
                message.push_str(", ");
            }
            let element = array.get(index, context)?;
            format_value(message, &element, depth + 1, context)?;
        }
        message.push(']');
    } else {
        let _ = write!(message, "{}", value.display());
    }
    Ok(())
}
