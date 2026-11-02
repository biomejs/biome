//! The `console` global available to plugins.
//!
//! Plugins run inside the language server and the CLI, where writing to stdout
//! would corrupt the output, so `console` only records the messages. The host
//! drains them with [`JsConsole::pull_messages`] and reports them like the
//! output of GritQL's `log()`.
//!
//! The object is Boa's `console`, with every method wrapped for three reasons:
//!
//! - Boa prints objects from their own properties, so nodes and tokens, whose
//!   fields are prototype getters, would print as empty objects. The wrappers
//!   replace them with their descriptions first.
//! - Boa keeps its console state borrowed while it converts the arguments,
//!   which can run plugin code such as `toString()`. A console call from that
//!   code would panic on the borrow, so the wrappers reject it with an error.
//! - Boa's console keeps groups, counters, and timers for its lifetime, while
//!   a plugin context serves many files on a worker thread. The wrappers call a
//!   Boa console that is replaced after each invocation that used it.
//!
//! The console and its wrappers are created on the first access to `console`.

use crate::ast::JsAstNode;
use crate::token::JsAstToken;
use boa_engine::gc::{Gc, GcRefCell};
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{FunctionObjectBuilder, JsObject, ObjectInitializer};
use boa_engine::property::{PropertyDescriptor, PropertyKey};
use boa_engine::{
    Context, Finalize, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace, js_string,
};
use boa_runtime::{Console, ConsoleState, Logger};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Arrays nested deeper than this are left to Boa, which also stops cycles.
const MAX_ARRAY_DEPTH: usize = 3;

/// The number of array elements read to describe one argument, across all
/// nesting levels. Matches the limit of Boa's own array display, which keeps
/// large or sparse arrays from hanging the analysis.
const MAX_ARRAY_ELEMENTS: u64 = 1000;

pub(crate) struct JsConsole {
    recorder: ConsoleRecorder,
    /// The Boa console the wrappers call. Created with the wrappers on the
    /// first access to `console`, and replaced after an invocation that used
    /// it so that groups, counters, and timers don't carry over.
    backing: Gc<GcRefCell<Option<JsObject>>>,
}

impl JsConsole {
    /// Defines the global `console` as an accessor that creates the console on
    /// first access and then replaces itself with a data property. Most plugins
    /// never log, so they don't pay for the console's objects in every context.
    pub(crate) fn register(context: &mut Context) -> JsResult<Self> {
        let recorder = ConsoleRecorder::default();
        let backing = Gc::new(GcRefCell::new(None));

        let getter = NativeFunction::from_copy_closure_with_captures(
            |_this, _args, (backing, recorder), context| {
                let console = Self::create(backing, recorder, context)?;
                Self::define_global(console.clone().into(), context)?;
                Ok(console.into())
            },
            (backing.clone(), recorder.clone()),
        );
        let setter = NativeFunction::from_fn_ptr(|_this, args, context| {
            Self::define_global(args.first().cloned().unwrap_or_default(), context)?;
            Ok(JsValue::undefined())
        });
        let descriptor = PropertyDescriptor::builder()
            .get(getter.to_js_function(context.realm()))
            .set(setter.to_js_function(context.realm()))
            .enumerable(false)
            .configurable(true);
        context
            .global_object()
            .define_property_or_throw(Console::NAME, descriptor, context)?;

        Ok(Self { recorder, backing })
    }

    fn define_global(value: JsValue, context: &mut Context) -> JsResult<()> {
        let descriptor = PropertyDescriptor::builder()
            .value(value)
            .writable(true)
            .enumerable(false)
            .configurable(true);
        context
            .global_object()
            .define_property_or_throw(Console::NAME, descriptor, context)?;
        Ok(())
    }

    /// Creates the backing Boa console and the object of wrappers exposed to
    /// plugins.
    fn create(
        backing: &Gc<GcRefCell<Option<JsObject>>>,
        recorder: &ConsoleRecorder,
        context: &mut Context,
    ) -> JsResult<JsObject> {
        let initial = Self::create_backing(recorder, context);
        *backing.borrow_mut() = Some(initial.clone());
        let console = ObjectInitializer::new(context).build();

        for key in initial.own_property_keys(context)? {
            let PropertyKey::String(name) = &key else {
                continue;
            };
            let Some(original) = initial.get(key.clone(), context)?.as_function() else {
                continue;
            };
            let length = original
                .get(js_string!("length"), context)?
                .to_length(context)?;

            let data = DataArgs::of(&name.to_std_string_lossy());
            let wrapper = NativeFunction::from_copy_closure_with_captures(
                move |_this, args, (backing, recorder, name), context| {
                    if recorder.busy.replace(true) {
                        return Err(JsNativeError::typ()
                            .with_message("console was called while another console call was converting its arguments, for example from a toString() method. Call console outside of the code that converts logged values.")
                            .into());
                    }
                    recorder.used.set(true);
                    let result = Self::call_backing(backing, name, args, data, context);
                    recorder.busy.set(false);
                    result
                },
                (backing.clone(), recorder.clone(), name.clone()),
            );
            let wrapper = FunctionObjectBuilder::new(context.realm(), wrapper)
                .name(name.clone())
                .length(length as usize)
                .build();
            console.set(key, wrapper, true, context)?;
        }

        Ok(console)
    }

    fn create_backing(recorder: &ConsoleRecorder, context: &mut Context) -> JsObject {
        let console = Console::init_with_logger(recorder.clone(), context);

        // The timers read `SystemTime`, which panics on wasm32. Deleting an own
        // configurable property of an ordinary object cannot fail.
        #[cfg(target_arch = "wasm32")]
        for method in ["time", "timeLog", "timeEnd"] {
            let _ = console.delete_property_or_throw(js_string!(method), context);
        }

        console
    }

    fn call_backing(
        backing: &Gc<GcRefCell<Option<JsObject>>>,
        name: &JsString,
        args: &[JsValue],
        data: Option<DataArgs>,
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let backing = backing.borrow().clone();
        let method = match &backing {
            Some(backing) => backing.get(name.clone(), context)?,
            None => JsValue::undefined(),
        };
        let (Some(backing), Some(method)) = (backing, method.as_function()) else {
            return Err(JsNativeError::typ()
                .with_message(format!(
                    "console.{} is not available.",
                    name.to_std_string_lossy()
                ))
                .into());
        };
        match data {
            Some(data) => {
                let args = describe_args(args, data, context)?;
                method.call(&backing.into(), &args, context)
            }
            None => method.call(&backing.into(), args, context),
        }
    }

    /// Takes the recorded messages. When the plugin used `console` since the
    /// last call, the wrappers are switched to a fresh Boa console.
    pub(crate) fn pull_messages(&self, context: &mut Context) -> Vec<String> {
        if self.recorder.used.replace(false) {
            *self.backing.borrow_mut() = Some(Self::create_backing(&self.recorder, context));
        }
        std::mem::take(&mut self.recorder.messages.borrow_mut())
    }
}

/// The arguments of a console method that are logged as data.
#[derive(Clone, Copy)]
struct DataArgs {
    /// The index of the first data argument.
    start: usize,
    /// Whether Boa reads the first data argument as a format string when more
    /// arguments follow.
    formatted: bool,
}

impl DataArgs {
    fn of(method: &str) -> Option<Self> {
        let (start, formatted) = match method {
            "log" | "info" | "warn" | "error" | "exception" | "debug" | "trace" | "group"
            | "groupCollapsed" => (0, true),
            "assert" => (1, true),
            "timeLog" => (1, false),
            "dir" | "dirxml" | "table" => (0, false),
            _ => return None,
        };
        Some(Self { start, formatted })
    }
}

/// The [`Logger`] behind the plugins' `console`, shared with the method wrappers.
#[derive(Clone, Default)]
struct ConsoleRecorder {
    messages: Rc<RefCell<Vec<String>>>,
    /// Set while a console method runs.
    busy: Rc<Cell<bool>>,
    /// Set when a console method is called, until the messages are pulled.
    used: Rc<Cell<bool>>,
}

impl Finalize for ConsoleRecorder {}

// SAFETY: `ConsoleRecorder` contains no values managed by Boa's garbage collector.
unsafe impl Trace for ConsoleRecorder {
    boa_engine::gc::empty_trace!();
}

impl ConsoleRecorder {
    /// Records a message, indented by the groups opened with `console.group()`.
    fn record(&self, message: String, state: &ConsoleState) -> JsResult<()> {
        let indent = " ".repeat(state.indent());
        self.messages
            .borrow_mut()
            .push(format!("{indent}{message}"));
        Ok(())
    }
}

impl Logger for ConsoleRecorder {
    /// Records the message and the stack trace as a single message.
    fn trace(&self, msg: String, state: &ConsoleState, context: &mut Context) -> JsResult<()> {
        let mut message = msg;
        for frame in context.stack_trace() {
            message.push_str("\n    at ");
            message.push_str(&frame.code_block().name().to_std_string_lossy());
        }
        self.record(message, state)
    }

    fn log(&self, msg: String, state: &ConsoleState, _context: &mut Context) -> JsResult<()> {
        self.record(msg, state)
    }

    fn info(&self, msg: String, state: &ConsoleState, _context: &mut Context) -> JsResult<()> {
        self.record(msg, state)
    }

    fn warn(&self, msg: String, state: &ConsoleState, _context: &mut Context) -> JsResult<()> {
        self.record(msg, state)
    }

    fn error(&self, msg: String, state: &ConsoleState, _context: &mut Context) -> JsResult<()> {
        self.record(msg, state)
    }
}

/// Replaces the nodes and tokens among the data arguments, including those
/// inside arrays, with their descriptions. Other values are passed through for
/// Boa to format.
fn describe_args(
    args: &[JsValue],
    data: DataArgs,
    context: &mut Context,
) -> JsResult<Vec<JsValue>> {
    let mut described = Vec::with_capacity(args.len());
    for (index, arg) in args.iter().enumerate() {
        let mut budget = MAX_ARRAY_ELEMENTS;
        let description = if index >= data.start {
            describe(arg, 0, &mut budget, context)?
        } else {
            None
        };
        let Some(mut description) = description else {
            described.push(arg.clone());
            continue;
        };
        if data.formatted && index == data.start && args.len() > data.start + 1 {
            description = description.replace('%', "%%");
        }
        described.push(JsString::from(description).into());
    }
    Ok(described)
}

/// Describes a node, a token, or an array containing one. Returns `None` for
/// any other value. Reading array elements consumes `budget`; the elements
/// past it are summarized.
fn describe(
    value: &JsValue,
    depth: usize,
    budget: &mut u64,
    context: &mut Context,
) -> JsResult<Option<String>> {
    if let Some(node) = JsAstNode::from_value(value) {
        return Ok(Some(node.describe()));
    }
    if let Some(token) = JsAstToken::from_value(value) {
        return Ok(Some(token.describe()));
    }
    let Some(array) = value
        .as_object()
        .and_then(|object| JsArray::from_object(object).ok())
    else {
        return Ok(None);
    };
    if depth >= MAX_ARRAY_DEPTH {
        return Ok(None);
    }

    let length = array.length(context)?;
    let read = length.min(*budget);
    *budget -= read;

    let mut elements = Vec::new();
    let mut has_description = false;
    for index in 0..read {
        let element = array.get(index, context)?;
        let description = describe(&element, depth + 1, budget, context)?;
        has_description |= description.is_some();
        elements.push((element, description));
    }
    if !has_description {
        return Ok(None);
    }

    let mut elements: Vec<_> = elements
        .into_iter()
        .map(|(element, description)| description.unwrap_or_else(|| element.display().to_string()))
        .collect();
    if length > read {
        elements.push(format!("... {} more items", length - read));
    }
    Ok(Some(format!("[ {} ]", elements.join(", "))))
}
