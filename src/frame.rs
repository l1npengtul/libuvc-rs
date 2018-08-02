use error::{Error, Result};

use std::ptr::NonNull;

use uvc_sys::*;

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}
#[derive(Debug)]
/// Frame containing the image data
pub struct Frame {
    frame: NonNull<uvc_frame>,
}

impl Frame {
    pub(crate) unsafe fn from_raw(frame: *mut uvc_frame) -> Frame {
        Frame {
            frame: NonNull::new(frame).unwrap(),
        }
    }

    fn new_with_dimensions(width: u32, height: u32, components: u32) -> Self {
        let frame = unsafe { uvc_allocate_frame((width * height * components) as usize) };

        Frame {
            frame: NonNull::new(frame).unwrap(),
        }
    }

    /// Convert to rgb format
    pub fn to_rgb(&self) -> Result<Frame> {
        let new_frame = Frame::new_with_dimensions(self.width(), self.height(), 3); // RGB -> 3 bytes

        let err = unsafe { uvc_any2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()) }.into();
        if err != Error::Success {
            Err(err)
        } else {
            Ok(new_frame)
        }
    }

    /// Get the raw image data
    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                (*self.frame.as_ptr()).data as *const u8,
                (*self.frame.as_ptr()).data_bytes,
            )
        }
    }

    /// Width of the captured frame
    pub fn width(&self) -> u32 {
        unsafe { (*self.frame.as_ptr()) }.width
    }

    /// Heigth of the captured frame
    pub fn height(&self) -> u32 {
        unsafe { (*self.frame.as_ptr()) }.height
    }

    /// Format of the captured frame
    pub fn format(&self) -> FrameFormat {
        unsafe { (*self.frame.as_ptr()) }.frame_format.into()
    }

    /// Monotonically increasing frame number
    pub fn sequence(&self) -> u32 {
        unsafe { (*self.frame.as_ptr()).sequence }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { uvc_free_frame(self.frame.as_ptr()) }
    }
}

#[derive(Debug)]
/// Format of a frame
pub enum FrameFormat {
    Unknown,
    Any,
    Uncompressed,
    Compressed,
    YUYV,
    UYVY,
    RGB,
    BGR,
    MJPEG,
    GRAY8,
    GRAY16,
    BY8,
    BA81,
    SGRBG8,
    SGBRG8,
    SRGGB8,
    SBGGR8,
    Count,
}

#[allow(non_upper_case_globals, unreachable_patterns)]
impl From<uvc_frame_format> for FrameFormat {
    fn from(code: uvc_frame_format) -> Self {
        match code {
            uvc_frame_format_UVC_FRAME_FORMAT_ANY => FrameFormat::Any,
            uvc_frame_format_UVC_FRAME_FORMAT_UNCOMPRESSED => FrameFormat::Uncompressed,
            uvc_frame_format_UVC_FRAME_FORMAT_COMPRESSED => FrameFormat::Compressed,
            uvc_frame_format_UVC_FRAME_FORMAT_YUYV => FrameFormat::YUYV,
            uvc_frame_format_UVC_FRAME_FORMAT_UYVY => FrameFormat::UYVY,
            uvc_frame_format_UVC_FRAME_FORMAT_RGB => FrameFormat::RGB,
            uvc_frame_format_UVC_FRAME_FORMAT_BGR => FrameFormat::BGR,
            uvc_frame_format_UVC_FRAME_FORMAT_MJPEG => FrameFormat::MJPEG,
            uvc_frame_format_UVC_FRAME_FORMAT_GRAY8 => FrameFormat::GRAY8,
            uvc_frame_format_UVC_FRAME_FORMAT_GRAY16 => FrameFormat::GRAY16,
            uvc_frame_format_UVC_FRAME_FORMAT_BY8 => FrameFormat::BY8,
            uvc_frame_format_UVC_FRAME_FORMAT_BA81 => FrameFormat::BA81,
            uvc_frame_format_UVC_FRAME_FORMAT_SGRBG8 => FrameFormat::SGRBG8,
            uvc_frame_format_UVC_FRAME_FORMAT_SGBRG8 => FrameFormat::SGBRG8,
            uvc_frame_format_UVC_FRAME_FORMAT_SRGGB8 => FrameFormat::SRGGB8,
            uvc_frame_format_UVC_FRAME_FORMAT_SBGGR8 => FrameFormat::SBGGR8,

            uvc_frame_format_UVC_FRAME_FORMAT_COUNT => FrameFormat::Count,
            uvc_frame_format_UVC_FRAME_FORMAT_UNKNOWN => FrameFormat::Unknown, // unreachable
            _ => FrameFormat::Unknown,
        }
    }
}

impl Into<uvc_frame_format> for FrameFormat {
    fn into(self: FrameFormat) -> uvc_frame_format {
        match self {
            FrameFormat::Any => uvc_frame_format_UVC_FRAME_FORMAT_ANY,
            FrameFormat::Uncompressed => uvc_frame_format_UVC_FRAME_FORMAT_UNCOMPRESSED,
            FrameFormat::Compressed => uvc_frame_format_UVC_FRAME_FORMAT_COMPRESSED,
            FrameFormat::YUYV => uvc_frame_format_UVC_FRAME_FORMAT_YUYV,
            FrameFormat::UYVY => uvc_frame_format_UVC_FRAME_FORMAT_UYVY,
            FrameFormat::RGB => uvc_frame_format_UVC_FRAME_FORMAT_RGB,
            FrameFormat::BGR => uvc_frame_format_UVC_FRAME_FORMAT_BGR,
            FrameFormat::MJPEG => uvc_frame_format_UVC_FRAME_FORMAT_MJPEG,
            FrameFormat::GRAY8 => uvc_frame_format_UVC_FRAME_FORMAT_GRAY8,
            FrameFormat::GRAY16 => uvc_frame_format_UVC_FRAME_FORMAT_GRAY16,
            FrameFormat::BY8 => uvc_frame_format_UVC_FRAME_FORMAT_BY8,
            FrameFormat::BA81 => uvc_frame_format_UVC_FRAME_FORMAT_BA81,
            FrameFormat::SGRBG8 => uvc_frame_format_UVC_FRAME_FORMAT_SGRBG8,
            FrameFormat::SGBRG8 => uvc_frame_format_UVC_FRAME_FORMAT_SGBRG8,
            FrameFormat::SRGGB8 => uvc_frame_format_UVC_FRAME_FORMAT_SRGGB8,
            FrameFormat::SBGGR8 => uvc_frame_format_UVC_FRAME_FORMAT_SBGGR8,
            FrameFormat::Count => uvc_frame_format_UVC_FRAME_FORMAT_COUNT,
            FrameFormat::Unknown => uvc_frame_format_UVC_FRAME_FORMAT_UNKNOWN,
        }
    }
}
