use std::ffi::CStr;
use std::fmt;

/// Result type of functions in this crate
pub type Result<T> = ::std::result::Result<T, Error>;

/// Error codes from `libusb`
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error {
    Success,
    Access,
    Busy,
    CallbackExists,
    Interrupted,
    InvalidDevice,
    InvalidMode,
    InvalidParam,
    IO,
    NotFound,
    NotSupported,
    NoDevice,
    NoMem,
    Other,
    Overflow,
    Pipe,
    Timeout,
    Unknown(::uvc_error_t),
}

impl From<::uvc_error_t> for Error {
    fn from(code: ::uvc_error_t) -> Self {
        match code {
            ::uvc_error_UVC_SUCCESS => Error::Success,
            ::uvc_error_UVC_ERROR_ACCESS => Error::Access,
            ::uvc_error_UVC_ERROR_BUSY => Error::Busy,
            ::uvc_error_UVC_ERROR_CALLBACK_EXISTS => Error::CallbackExists,
            ::uvc_error_UVC_ERROR_INTERRUPTED => Error::Interrupted,
            ::uvc_error_UVC_ERROR_INVALID_DEVICE => Error::InvalidDevice,
            ::uvc_error_UVC_ERROR_INVALID_MODE => Error::InvalidMode,
            ::uvc_error_UVC_ERROR_INVALID_PARAM => Error::InvalidParam,
            ::uvc_error_UVC_ERROR_IO => Error::IO,
            ::uvc_error_UVC_ERROR_NOT_FOUND => Error::NotFound,
            ::uvc_error_UVC_ERROR_NOT_SUPPORTED => Error::NotSupported,
            ::uvc_error_UVC_ERROR_NO_DEVICE => Error::NoDevice,
            ::uvc_error_UVC_ERROR_NO_MEM => Error::NoMem,
            ::uvc_error_UVC_ERROR_OTHER => Error::Other,
            ::uvc_error_UVC_ERROR_OVERFLOW => Error::Overflow,
            ::uvc_error_UVC_ERROR_PIPE => Error::Pipe,
            ::uvc_error_UVC_ERROR_TIMEOUT => Error::Timeout,
            x => Error::Unknown(x),
        }
    }
}

impl Into<::uvc_error_t> for Error {
    fn into(self) -> ::uvc_error_t {
        match self {
            Error::Success => ::uvc_error_UVC_SUCCESS,
            Error::Access => ::uvc_error_UVC_ERROR_ACCESS,
            Error::Busy => ::uvc_error_UVC_ERROR_BUSY,
            Error::CallbackExists => ::uvc_error_UVC_ERROR_CALLBACK_EXISTS,
            Error::Interrupted => ::uvc_error_UVC_ERROR_INTERRUPTED,
            Error::InvalidDevice => ::uvc_error_UVC_ERROR_INVALID_DEVICE,
            Error::InvalidMode => ::uvc_error_UVC_ERROR_INVALID_MODE,
            Error::InvalidParam => ::uvc_error_UVC_ERROR_INVALID_PARAM,
            Error::IO => ::uvc_error_UVC_ERROR_IO,
            Error::NotFound => ::uvc_error_UVC_ERROR_NOT_FOUND,
            Error::NotSupported => ::uvc_error_UVC_ERROR_NOT_SUPPORTED,
            Error::NoDevice => ::uvc_error_UVC_ERROR_NO_DEVICE,
            Error::NoMem => ::uvc_error_UVC_ERROR_NO_MEM,
            Error::Other => ::uvc_error_UVC_ERROR_OTHER,
            Error::Overflow => ::uvc_error_UVC_ERROR_OVERFLOW,
            Error::Pipe => ::uvc_error_UVC_ERROR_PIPE,
            Error::Timeout => ::uvc_error_UVC_ERROR_TIMEOUT,
            Error::Unknown(x) => x,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strerror = unsafe { ::uvc_strerror((*self).into()) };
        if strerror.is_null() {
            return write!(f, "Unknown error");
        }
        let strerr = unsafe { CStr::from_ptr(strerror) }.to_str().unwrap();
        write!(f, "{}", strerr)
    }
}
impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        None
    }
}
