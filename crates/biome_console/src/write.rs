mod html;
mod string;
mod termcolor;

use std::{fmt, io};

use crate::fmt::MarkupElements;

pub use self::{html::HTML, string::StringBuffer, termcolor::Termcolor};

pub trait Write {
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()>;
    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()>;
}
