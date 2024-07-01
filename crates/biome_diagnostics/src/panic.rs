use std::panic::UnwindSafe;

#[derive(Default, Debug)]
pub struct PanicError {
    pub info: String,
    pub backtrace: Option<std::backtrace::Backtrace>,
}

thread_local! {
    static LAST_PANIC: std::cell::Cell<Option<PanicError>> = const { std::cell::Cell::new(None) };
}

impl std::fmt::Display for PanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = f.write_fmt(format_args!("{}\n", self.info));
        if let Some(backtrace) = &self.backtrace {
            f.write_fmt(format_args!("Backtrace: {backtrace}"))
        } else {
            r
        }
    }
}

/// Take and set a specific panic hook before calling `f` inside a `catch_unwind`, then
/// return the old set_hook.
///
/// If `f` panicks am `Error` with the panic message plus backtrace will be returned.
pub fn catch_unwind<F, R>(f: F) -> Result<R, PanicError>
where
    F: FnOnce() -> R + UnwindSafe,
{
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| {
        let info = info.to_string();
        let backtrace = std::backtrace::Backtrace::capture();
        LAST_PANIC.with(|cell| {
            cell.set(Some(PanicError {
                info,
                backtrace: Some(backtrace),
            }))
        })
    }));

    let result = std::panic::catch_unwind(f)
        .map_err(|_| LAST_PANIC.with(|cell| cell.take()).unwrap_or_default());

    std::panic::set_hook(prev);

    result
}
