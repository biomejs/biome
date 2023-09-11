#[cfg(feature = "dhat-heap")]
use crate::features::print_stats;
use crate::language::FormatNode;
use crate::BenchmarkSummary;
use biome_formatter::Printed;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FormatterMeasurement {
    id: String,
    formatting: Duration,
}
pub fn benchmark_format_lib(id: &str, format_node: &FormatNode) -> BenchmarkSummary {
    let formatter_timer = timing::start();
    criterion::black_box(run_format(format_node));
    let formatter_duration = formatter_timer.stop();

    BenchmarkSummary::Formatter(FormatterMeasurement {
        id: id.to_string(),
        formatting: formatter_duration,
    })
}

pub fn run_format(format_node: &FormatNode) -> Printed {
    #[cfg(feature = "dhat-heap")]
    let stats = {
        println!("Start");
        print_stats(dhat::HeapStats::get(), None)
    };

    let formatted = format_node.format_node().unwrap();

    #[cfg(feature = "dhat-heap")]
    let stats = {
        println!("Formatted");
        print_stats(dhat::HeapStats::get(), Some(stats))
    };

    let printed = formatted.print();
    drop(formatted);

    #[cfg(feature = "dhat-heap")]
    {
        println!("Printed");
        print_stats(dhat::HeapStats::get(), Some(stats));
    }

    printed.expect("Document to be valid")
}

impl FormatterMeasurement {
    fn total(&self) -> Duration {
        self.formatting
    }

    pub(crate) fn summary(&self) -> String {
        format!("{}, Formatting: {:?}", self.id, self.total(),)
    }
}

impl Display for FormatterMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tFormatting: {:>10?}", self.formatting);
        let _ = writeln!(f, "\t              ----------");
        let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());

        Ok(())
    }
}
