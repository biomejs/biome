//! Standalone profiler output for CLI traversals.
//!
//! The default finalizer writes this report to the console independently of
//! the diagnostic reporter and its destination.

use std::io;

use biome_analyze::profiling::DisplayProfiles;
use biome_console::markup;
use biome_module_graph::type_inference::profiling::DisplayTypeInferenceProfile;
use camino::{Utf8Path, Utf8PathBuf};

use crate::VERSION;
use crate::reporter::{
    DiagnosticsPayload, Reporter, ReporterVisitor, ReporterWriter, TraversalSummary,
};
use crate::runner::execution::Execution;

/// Reports every enabled profiler after a traversal.
pub(crate) struct ProfilersReporter<'a> {
    pub(crate) execution: &'a dyn Execution,
    pub(crate) summary: TraversalSummary,
    pub(crate) verbose: bool,
}

impl Reporter for ProfilersReporter<'_> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,
        visitor: &mut dyn ReporterVisitor,
    ) -> io::Result<()> {
        visitor.report_summary(writer, self.execution, self.summary, self.verbose)
    }
}

pub(crate) struct ProfilersReporterVisitor {
    working_directory: Option<Utf8PathBuf>,
    rule_profiler: bool,
    type_profiler: bool,
}

impl ProfilersReporterVisitor {
    pub(crate) fn new(
        working_directory: Option<Utf8PathBuf>,
        rule_profiler: bool,
        type_profiler: bool,
    ) -> Self {
        Self {
            working_directory,
            rule_profiler,
            type_profiler,
        }
    }
}

impl ReporterVisitor for ProfilersReporterVisitor {
    fn report_summary(
        &mut self,
        writer: &mut dyn ReporterWriter,
        execution: &dyn Execution,
        _summary: TraversalSummary,
        verbose: bool,
    ) -> io::Result<()> {
        if self.rule_profiler {
            rayon::broadcast(|_| biome_analyze::profiling::flush_thread_profiler());
            let rule_profiles = biome_analyze::profiling::drain_sorted_by_total(true);
            if !rule_profiles.is_empty() {
                writer.log(markup! {{ DisplayProfiles(rule_profiles, None) }});
            }
            biome_analyze::profiling::disable();
        }

        if self.type_profiler
            && let Some(profile) = execution.take_type_inference_profile()
        {
            let display = DisplayTypeInferenceProfile::new(
                &profile,
                self.working_directory.as_deref(),
                VERSION,
            )
            .with_verbose(verbose);
            writer.log(markup! {{ display }});
        }

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        _payload: &DiagnosticsPayload,
        _verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        Ok(())
    }
}
