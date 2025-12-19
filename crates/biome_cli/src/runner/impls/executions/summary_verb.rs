use biome_console::fmt::Formatter;
use biome_console::{MarkupBuf, fmt, markup};
use std::io;
use std::time::Duration;

pub(crate) struct SummaryVerbExecution;

impl SummaryVerbExecution {
    /// Prints "<verb> <files> in <duration>"
    pub(crate) fn summary_verb(&self, verb: &str, files: usize, duration: &Duration) -> MarkupBuf {
        let files = Files(files);

        markup!(
            {verb}" "{files} " in " {duration}"."
        )
        .to_owned()
    }
}

struct Files(usize);

impl fmt::Display for Files {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup!({self.0} " "))?;
        if self.0 == 1 {
            fmt.write_str("file")
        } else {
            fmt.write_str("files")
        }
    }
}
