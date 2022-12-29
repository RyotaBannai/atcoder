use std::io::{stdin, BufReader, Bytes, Error as IoError, Read, Stdin};
use std::str::{self, FromStr, Utf8Error};

#[macro_export]
macro_rules! scan {
    ($scanner:expr) => { $scanner.next().expect("failed to read token") };
    ($scanner:expr, $type:ty) => { $scanner .next::<$type>() .expect(concat!("failed to read token of type ", stringify!($type))) };
    ($scanner:expr, $type:ty; $n:expr) => { (0..$n).map(|_| scan!($scanner)).collect::<Vec<$type>>() };
    ($scanner:expr, $($type:ty),+) => { ($( scan!($scanner, $type), )+) };
}

#[derive(Debug)]
pub struct Scanner<R: Read> {
    bytes: Bytes<BufReader<R>>,
    buf: Vec<u8>,
}
#[derive(Debug)]
pub enum ScanError<T: FromStr> {
    Io(IoError),
    Parse(T::Err, String),
    NonUtf8(Utf8Error, Vec<u8>),
}
impl Scanner<Stdin> {
    pub fn stdin() -> Self {
        Self::new(stdin())
    }
}
impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            bytes: BufReader::new(reader).bytes(),
            buf: Vec::new(),
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn next<T: FromStr>(&mut self) -> Result<T, ScanError<T>> {
        assert!(self.buf.is_empty());
        for b in &mut self.bytes {
            let b = b.map_err(ScanError::Io)?;
            if b.is_ascii_whitespace() {
                if self.buf.is_empty() {
                    continue;
                } else {
                    break;
                }
            } else {
                self.buf.push(b);
            }
        }
        match str::from_utf8(&self.buf) {
            Err(err) => Err(ScanError::NonUtf8(err, std::mem::take(&mut self.buf))),
            Ok(s) => {
                let ret = s.parse().map_err(|err| ScanError::Parse(err, s.to_owned()));
                self.buf.clear();
                ret
            }
        }
    }
}
