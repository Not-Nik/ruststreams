#[cfg(test)]
mod tests {
    use crate::Stream;
    use std::io::{Write, Read};

    #[test]
    fn simple() {
        let mut stream = Stream { buf: vec![0u8; 0] };
        stream.write_item(42);
        stream.write_item(69);

        let read = stream.read_item();

        assert_eq!(read.is_some(), true);
        assert_eq!(read.unwrap(), 42);

        let read = stream.read_item();

        assert_eq!(read.is_some(), true);
        assert_eq!(read.unwrap(), 69);
    }

    #[test]
    fn io_test() {
        let mut stream = Stream::new();
        let read = &mut [0, 0];

        stream.write(&mut [42, 69]);
        stream.read(read);

        assert_eq!(read, &mut [42, 69]);
    }

    #[test]
    fn readme() {
        let mut stream = Stream::new();
        stream.write_item(42);
        assert_eq!(stream.read_item().expect(""), 42);
    }
}

use std::vec::Vec;
use std::io::{Read, Result, Error, ErrorKind, Write};

pub struct Stream<T>
{
    buf: Vec<T>
}

impl<T> Stream<T>
{
    pub fn new() -> Stream<T> {
        Stream { buf: vec![] }
    }

    pub fn read_item(&mut self) -> Option<T>
    {
        self.buf.reverse();
        let res = self.buf.pop();
        self.buf.reverse();
        res
    }

    pub fn write_item(&mut self, item: T)
    {
        self.buf.push(item);
    }
}

impl Read for Stream<u8>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let x = 0;
        for x in 0..buf.len()
        {
            let item = self.read_item();
            if item.is_some() {
                buf[x] = item.unwrap();
            } else {
                return Err(Error::new(ErrorKind::Other, "Read error"));
            }
        };
        Result::Ok(x)
    }
}

impl Write for Stream<u8>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let x = 0;
        for x in 0..buf.len()
        {
            self.write_item(buf[x]);
        };
        Result::Ok(x)
    }

    fn flush(&mut self) -> Result<()> {
        Result::Ok(())
    }
}
