use std::error::Error;
use std::ffi::CStr;
use std::fmt;

pub type UvcResult<T> = Result<T, UvcError>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UvcError {
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

impl From<::uvc_error_t> for UvcError {
    fn from(code: ::uvc_error_t) -> Self {
        match code {
            ::uvc_error_UVC_SUCCESS => UvcError::Success,
            ::uvc_error_UVC_ERROR_ACCESS => UvcError::Access,
            ::uvc_error_UVC_ERROR_BUSY => UvcError::Busy,
            ::uvc_error_UVC_ERROR_CALLBACK_EXISTS => UvcError::CallbackExists,
            ::uvc_error_UVC_ERROR_INTERRUPTED => UvcError::Interrupted,
            ::uvc_error_UVC_ERROR_INVALID_DEVICE => UvcError::InvalidDevice,
            ::uvc_error_UVC_ERROR_INVALID_MODE => UvcError::InvalidMode,
            ::uvc_error_UVC_ERROR_INVALID_PARAM => UvcError::InvalidParam,
            ::uvc_error_UVC_ERROR_IO => UvcError::IO,
            ::uvc_error_UVC_ERROR_NOT_FOUND => UvcError::NotFound,
            ::uvc_error_UVC_ERROR_NOT_SUPPORTED => UvcError::NotSupported,
            ::uvc_error_UVC_ERROR_NO_DEVICE => UvcError::NoDevice,
            ::uvc_error_UVC_ERROR_NO_MEM => UvcError::NoMem,
            ::uvc_error_UVC_ERROR_OTHER => UvcError::Other,
            ::uvc_error_UVC_ERROR_OVERFLOW => UvcError::Overflow,
            ::uvc_error_UVC_ERROR_PIPE => UvcError::Pipe,
            ::uvc_error_UVC_ERROR_TIMEOUT => UvcError::Timeout,
            x => UvcError::Unknown(x),
        }
    }
}

impl Into<::uvc_error_t> for UvcError {
    fn into(self) -> ::uvc_error_t {
        match self {
            UvcError::Success => ::uvc_error_UVC_SUCCESS,
            UvcError::Access => ::uvc_error_UVC_ERROR_ACCESS,
            UvcError::Busy => ::uvc_error_UVC_ERROR_BUSY,
            UvcError::CallbackExists => ::uvc_error_UVC_ERROR_CALLBACK_EXISTS,
            UvcError::Interrupted => ::uvc_error_UVC_ERROR_INTERRUPTED,
            UvcError::InvalidDevice => ::uvc_error_UVC_ERROR_INVALID_DEVICE,
            UvcError::InvalidMode => ::uvc_error_UVC_ERROR_INVALID_MODE,
            UvcError::InvalidParam => ::uvc_error_UVC_ERROR_INVALID_PARAM,
            UvcError::IO => ::uvc_error_UVC_ERROR_IO,
            UvcError::NotFound => ::uvc_error_UVC_ERROR_NOT_FOUND,
            UvcError::NotSupported => ::uvc_error_UVC_ERROR_NOT_SUPPORTED,
            UvcError::NoDevice => ::uvc_error_UVC_ERROR_NO_DEVICE,
            UvcError::NoMem => ::uvc_error_UVC_ERROR_NO_MEM,
            UvcError::Other => ::uvc_error_UVC_ERROR_OTHER,
            UvcError::Overflow => ::uvc_error_UVC_ERROR_OVERFLOW,
            UvcError::Pipe => ::uvc_error_UVC_ERROR_PIPE,
            UvcError::Timeout => ::uvc_error_UVC_ERROR_TIMEOUT,
            UvcError::Unknown(x) => x,
        }
    }
}

impl fmt::Display for UvcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strerror = unsafe { ::uvc_strerror((*self).into()) };
        if strerror.is_null() {
            return write!(f, "Unknown error");
        }
        let strerr = unsafe { CStr::from_ptr(strerror) }.to_str().unwrap();
        write!(f, "{}", strerr)
    }
}
impl Error for UvcError {
    fn cause(&self) -> Option<&Error> {
        None
    }
}
