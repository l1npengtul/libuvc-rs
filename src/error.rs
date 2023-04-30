use std::ffi::CStr;
use std::fmt;
use std::num::NonZeroI32;

/// Result type of functions in this crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error codes from `libusb`
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Error(NonZeroI32);

const fn make_err(x: uvc_sys::uvc_error_t) -> Error {
    Error(unsafe { NonZeroI32::new_unchecked(x) })
}
impl Error {
    pub const ACCESS: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_ACCESS);
    pub const BUSY: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_BUSY);
    pub const CALLBACK_EXISTS: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_CALLBACK_EXISTS);
    pub const INTERRUPTED: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_INTERRUPTED);
    pub const INVALID_DEVICE: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_INVALID_DEVICE);
    pub const INVALID_MODE: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_INVALID_MODE);
    pub const INVALID_PARAM: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_INVALID_PARAM);
    pub const IO: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_IO);
    pub const NOT_FOUND: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_NOT_FOUND);
    pub const NOT_SUPPORTED: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_NOT_SUPPORTED);
    pub const NO_DEVICE: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_NO_DEVICE);
    pub const NO_MEM: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_NO_MEM);
    pub const OTHER: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_OTHER);
    pub const OVERFLOW: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_OVERFLOW);
    pub const PIPE: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_PIPE);
    pub const TIMEOUT: Self = make_err(uvc_sys::uvc_error_UVC_ERROR_TIMEOUT);
}

// impl From<uvc_sys::uvc_error_t> for Error {
//     fn from(code: uvc_sys::uvc_error_t) -> Self {
//         Error(code)
//     }
// }

impl Into<uvc_sys::uvc_error_t> for Error {
    fn into(self) -> uvc_sys::uvc_error_t {
        self.0.get()
    }
}

impl Error {
    pub const fn from_code(x: i32) -> Option<Self> {
        match NonZeroI32::new(x) {
            Some(e) => Some(Self(e)),
            None => None,
        }
    }
    pub const fn code(self) -> i32 {
        self.0.get()
    }
    pub(crate) fn cvt(x: i32) -> Result<()> {
        match Self::from_code(x) {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strerror = unsafe { uvc_sys::uvc_strerror(self.0.get()) };
        if strerror.is_null() {
            return write!(f, "Unknown error");
        }
        let strerr = unsafe { CStr::from_ptr(strerror) }.to_str().unwrap();
        write!(f, "{}", strerr)
    }
}
impl std::error::Error for Error {}

fn _test_matching() {
    match Error::ACCESS {
        Error::ACCESS => {}
        Error::BUSY => {}
        _ => {}
    }
}
