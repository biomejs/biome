/// Declares a Boa class wrapping an owned Rust handle and its JavaScript methods.
///
/// Each method entry is `(name, argument_count, |handle, args, context| { ... })`.
/// The count excludes `this`: it sets the function's `length` and the exact number
/// of arguments accepted. The receiver and argument count are checked before the
/// body runs. `handle` is an `Rc` to the native value; the body returns
/// `JsResult<JsValue>`. `constructor_hint` completes the error raised when
/// JavaScript calls the class as a constructor.
macro_rules! js_object {
    ($wrapper:ident, $native:ty, $name:literal, $constructor_hint:literal, $(($method:literal, $arity:literal, |$value:ident, $args:ident, $context:ident| $body:block)),+ $(,)?) => {
        #[derive(Clone, boa_engine::JsData)]
        pub(crate) struct $wrapper(std::rc::Rc<$native>);

        impl boa_engine::Finalize for $wrapper {}

        // SAFETY: The handle owns Rust data that contains no Boa-managed values.
        unsafe impl boa_engine::Trace for $wrapper {
            boa_engine::gc::empty_trace!();
        }

        impl $wrapper {
            pub(crate) fn wrap(
                value: $native,
                context: &mut boa_engine::Context,
            ) -> boa_engine::JsValue {
                let prototype = context.get_global_class::<Self>().map(|class| class.prototype());
                boa_engine::object::JsObject::from_proto_and_data(
                    prototype,
                    Self(std::rc::Rc::new(value)),
                )
                .into()
            }

            fn from_value(
                value: &boa_engine::JsValue,
            ) -> boa_engine::JsResult<std::rc::Rc<$native>> {
                let object = value.as_object().ok_or_else(|| boa_engine::JsNativeError::typ()
                    .with_message(concat!("Call this method on a ", $name, " returned by Biome.")))?;
                let native = object.downcast_ref::<Self>().ok_or_else(|| boa_engine::JsNativeError::typ()
                    .with_message(concat!("Call this method on a ", $name, " returned by Biome.")))?;
                Ok(std::rc::Rc::clone(&native.0))
            }
        }

        impl boa_engine::class::Class for $wrapper {
            const NAME: &'static str = $name;

            fn init(class: &mut boa_engine::class::ClassBuilder<'_>) -> boa_engine::JsResult<()> {
                $(class.method(
                    boa_engine::js_string!($method), $arity,
                    boa_engine::NativeFunction::from_fn_ptr(|this, $args, $context| {
                        let $value = Self::from_value(this)?;
                        let _ = &$context;
                        if $args.len() != $arity {
                            return Err(boa_engine::JsNativeError::typ().with_message(format!(
                                "{}.{}() requires {} argument(s).", $name, $method, $arity,
                            )).into());
                        }
                        $body
                    }),
                );)+
                Ok(())
            }

            fn data_constructor(
                _new_target: &boa_engine::JsValue,
                _args: &[boa_engine::JsValue],
                _context: &mut boa_engine::Context,
            ) -> boa_engine::JsResult<Self> {
                Err(boa_engine::JsNativeError::typ().with_message(concat!(
                    $name, " has no public constructor. ", $constructor_hint,
                )).into())
            }
        }
    };
}

pub(crate) use js_object;
