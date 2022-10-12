use v8::inspector::{StringView, V8InspectorClientBase, V8InspectorClientImpl, V8StackTrace};

pub(crate) struct DebugClient {
    base: V8InspectorClientBase,
}

impl DebugClient {
    pub(crate) fn new() -> Self {
        Self {
            base: V8InspectorClientBase::new::<Self>(),
        }
    }
}

impl V8InspectorClientImpl for DebugClient {
    fn base(&self) -> &V8InspectorClientBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut V8InspectorClientBase {
        &mut self.base
    }

    fn console_api_message(
        &mut self,
        _context_group_id: i32,
        _level: i32,
        message: &StringView,
        _url: &StringView,
        _line_number: u32,
        _column_number: u32,
        _stack_trace: &mut V8StackTrace,
    ) {
        println!("{message}");
    }
}
