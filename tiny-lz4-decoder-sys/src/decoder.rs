use crate::ffi::{
    LZ4FDecompressionContext, LZ4F_createDecompressionContext, LZ4F_decompress,
    LZ4F_freeDecompressionContext, LZ4_VERSION_NUMBER,
};

use std::io::{Error, ErrorKind, Read};

const BUFFER_SIZE: usize = 32 * 1024;

struct DecoderContext {
    c: LZ4FDecompressionContext,
}

pub struct Decoder<R> {
    c: DecoderContext,
    r: R,
    buf: Box<[u8]>,
    pos: usize,
    len: usize,
    next: usize,
}

impl<R: Read> Decoder<R> {
    pub fn new(r: R) -> Result<Decoder<R>, Error> {
        Ok(Decoder {
            r,
            c: DecoderContext::new()?,
            buf: vec![0; BUFFER_SIZE].into_boxed_slice(),
            pos: BUFFER_SIZE,
            len: BUFFER_SIZE,
            next: 11,
        })
    }

    pub fn reader(&self) -> &R {
        &self.r
    }

    pub fn finish(self) -> (R, Result<(), Error>) {
        (
            self.r,
            match self.next {
                0 => Ok(()),
                _ => Err(Error::new(
                    ErrorKind::Interrupted,
                    "`fn finish` can't be called before `fn reader`",
                )),
            },
        )
    }
}

impl<R: Read> Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.next == 0 || buf.is_empty() {
            return Ok(0);
        }
        let mut dst_offset: usize = 0;
        while dst_offset == 0 {
            if self.pos >= self.len {
                let need = if self.buf.len() < self.next {
                    self.buf.len()
                } else {
                    self.next
                };
                self.len = self.r.read(&mut self.buf[0..need])?;
                if self.len == 0 {
                    break;
                }
                self.pos = 0;
                self.next -= self.len;
            }
            while (dst_offset < buf.len()) && (self.pos < self.len) {
                let mut src_size = self.len - self.pos;
                let mut dst_size = buf.len() - dst_offset;

                let len = crate::ehandle::handle_error(unsafe {
                    LZ4F_decompress(
                        self.c.c,
                        buf[dst_offset..].as_mut_ptr(),
                        &mut dst_size,
                        self.buf[self.pos..].as_ptr(),
                        &mut src_size,
                        std::ptr::null(),
                    )
                })?;

                self.pos += src_size;
                dst_offset += dst_size;

                if len == 0 {
                    self.next = 0;
                    return Ok(dst_offset);
                } else if self.next < len {
                    self.next = len;
                }
            }
        }

        Ok(dst_offset)
    }
}

impl DecoderContext {
    fn new() -> Result<DecoderContext, Error> {
        let mut context = LZ4FDecompressionContext(std::ptr::null_mut());
        crate::ehandle::handle_error(unsafe {
            LZ4F_createDecompressionContext(&mut context, LZ4_VERSION_NUMBER)
        })?;
        Ok(DecoderContext { c: context })
    }
}

impl Drop for DecoderContext {
    fn drop(&mut self) {
        unsafe { LZ4F_freeDecompressionContext(self.c) };
    }
}
