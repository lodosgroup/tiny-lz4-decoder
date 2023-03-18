pub const LZ4_VERSION_NUMBER: u32 = 10904;

pub type LZ4FErrorCode = usize;

#[repr(C)]
pub struct LZ4FDecompressOptions {
    pub stable_dst: std::os::raw::c_uint,
    pub reserved: [std::os::raw::c_uint; 3],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LZ4FDecompressionContext(pub *mut std::ffi::c_void);
unsafe impl Send for LZ4FDecompressionContext {}

extern "C" {
    pub fn LZ4F_isError(code: usize) -> std::os::raw::c_uint;

    pub fn LZ4F_getErrorName(code: usize) -> *const std::os::raw::c_char;

    pub fn LZ4F_createDecompressionContext(
        ctx: &mut LZ4FDecompressionContext,
        version: std::os::raw::c_uint,
    ) -> LZ4FErrorCode;

    pub fn LZ4F_freeDecompressionContext(ctx: LZ4FDecompressionContext) -> LZ4FErrorCode;

    pub fn LZ4F_decompress(
        ctx: LZ4FDecompressionContext,
        dstBuffer: *mut u8,
        dstSizePtr: &mut usize,
        srcBuffer: *const u8,
        srcSizePtr: &mut usize,
        optionsPtr: *const LZ4FDecompressOptions,
    ) -> LZ4FErrorCode;
}
