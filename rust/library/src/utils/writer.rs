use std::fmt::Display;
use std::io::{stdout, BufWriter, Error as IoError, Stdout, Write};
#[derive(Debug)]
pub struct Writer<W: Write> {
    writer: BufWriter<W>,
}
impl Writer<Stdout> {
    pub fn stdout() -> Self {
        Self::new(stdout())
    }
}
impl<W: Write> Writer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }
    pub fn write<M, T: Writable<M>>(&mut self, val: T) -> Result<(), IoError> {
        val.write_to(&mut self.writer)
    }
    pub fn flush(&mut self) -> Result<(), IoError> {
        self.writer.flush()
    }
}
pub trait Writable<Mode> {
    fn write_to<W: Write>(self, w: &mut W) -> Result<(), IoError>;
}
#[non_exhaustive]
pub struct Single;
impl<T: Display> Writable<Single> for T {
    fn write_to<W: Write>(self, w: &mut W) -> Result<(), IoError> {
        writeln!(w, "{}", self)
    }
}
#[non_exhaustive]
pub struct Many;
impl<I> Writable<Many> for I
where
    I: Iterator,
    I::Item: Display,
{
    fn write_to<W: Write>(mut self, w: &mut W) -> Result<(), IoError> {
        if let Some(x) = self.next() {
            write!(w, "{}", x)?;
        } else {
            return Ok(());
        }
        for x in self {
            write!(w, " {}", x)?;
        }
        writeln!(w)
    }
}
#[non_exhaustive]
pub struct Array;
impl<T: Display> Writable<Array> for &[T] {
    fn write_to<W: Write>(self, w: &mut W) -> Result<(), IoError> {
        self.iter().write_to(w)
    }
}
