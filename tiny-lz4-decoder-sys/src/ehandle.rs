#[derive(Debug)]
pub(crate) struct LZ4Error(String);

impl std::fmt::Display for LZ4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "lz4: {}", &self.0)
    }
}

impl std::error::Error for LZ4Error {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&dyn (std::error::Error)> {
        None
    }
}

pub(crate) fn handle_error(code: crate::ffi::LZ4FErrorCode) -> Result<usize, std::io::Error> {
    unsafe {
        if crate::ffi::LZ4F_isError(code) != 0 {
            let error_name = crate::ffi::LZ4F_getErrorName(code);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                LZ4Error(
                    std::str::from_utf8(std::ffi::CStr::from_ptr(error_name).to_bytes())
                        .unwrap()
                        .to_string(),
                ),
            ));
        }
    }

    Ok(code)
}
