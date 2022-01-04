use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapper: R,
    num_of_reads: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapper: wrapped,
            num_of_reads: 0,
            bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_of_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    // fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    // let n = self.wrapper.read(buf)?;
    // self.num_of_reads += 1;
    // self.bytes_read += n;
    // Ok(n)
    // }
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.wrapper.read(buf).map(|cnt| {
            self.num_of_reads += 1;
            self.bytes_read += cnt;
            cnt
        })
    }
}

pub struct WriteStats<W> {
    wrapper: W,
    num_of_writes: usize,
    bytes_write: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapper: wrapped,
            num_of_writes: 0,
            bytes_write: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_write
    }

    pub fn writes(&self) -> usize {
        self.num_of_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.wrapper.write(buf)?;
        self.num_of_writes += 1;
        self.bytes_write += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapper.flush()
    }
}
