mod html;
mod stdio;
mod string;
mod termcolor;

use std::{fmt, io};

use crate::fmt::MarkupElements;

#[cfg(not(unix))]
pub(crate) use self::stdio::write_to_color_writer;
#[cfg(unix)]
pub(crate) use self::stdio::write_to_std_stream;
pub use self::{html::HTML, string::StringBuffer, termcolor::Termcolor};

pub trait Write {
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()>;
    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()>;
}
