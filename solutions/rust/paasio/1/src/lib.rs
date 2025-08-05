use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    source: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            source: wrapped,
            reads: 0,
            bytes_through: 0,
        }
        
    }

    pub fn get_ref(&self) -> &R {
        &self.source
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.source.read(buf)?;
        self.reads += 1;
        self.bytes_through += bytes;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    source: W,
    writes: usize,
    bytes_through: usize,

}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            source: wrapped,
            writes: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.source
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.source.write(buf)?;
        self.writes += 1;
        self.bytes_through += bytes;
        
        Ok(bytes)

    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
