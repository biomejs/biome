/// Collects debug-only formatter decisions that must be resolved before formatting completes.
#[derive(Debug, Clone, Default)]
pub struct FormatAudit {
    events: Vec<String>,
}

/// Snapshot used to discard audit events from speculative formatting.
#[derive(Copy, Clone)]
pub struct FormatAuditSnapshot {
    len: usize,
}

impl FormatAudit {
    /// Records an audit event.
    pub fn record_event(&mut self, event: impl Into<String>) {
        self.events.push(event.into());
    }

    /// Panics if any unresolved audit events were recorded.
    pub fn assert_no_events(&self) {
        if self.events.is_empty() {
            return;
        }

        let mut message = String::from("formatter audit failed:");
        for event in &self.events {
            message.push_str("\n- ");
            message.push_str(event);
        }

        panic!("{message}");
    }

    pub(crate) fn snapshot(&self) -> FormatAuditSnapshot {
        FormatAuditSnapshot {
            len: self.events.len(),
        }
    }

    pub(crate) fn restore(&mut self, snapshot: FormatAuditSnapshot) {
        self.events.truncate(snapshot.len);
    }
}
