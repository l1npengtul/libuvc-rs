use std::ptr::NonNull;
use std::slice;

use crate::error::{Error, Result};
use crate::formats::FrameFormat;

use crate::uvc_sys::*;

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

    /// Does not initialize any data
    unsafe fn new_with_dimensions(width: u32, height: u32, components: u32) -> Self {
        let frame = uvc_allocate_frame((width * height * components) as usize);

        Frame {
            frame: NonNull::new(frame).unwrap(),
        }
    }

    /// Convert to rgb format
    pub fn to_rgb(&self) -> Result<Frame> {
        let new_frame = unsafe { Frame::new_with_dimensions(self.width(), self.height(), 3) }; // RGB -> 3 bytes

        let err = unsafe {
            match self.format() {
                FrameFormat::MJPEG => uvc_mjpeg2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                FrameFormat::YUYV => uvc_yuyv2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                FrameFormat::UYVY => uvc_uyvy2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                FrameFormat::Any => uvc_any2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                _ => uvc_any2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()),
            }
        }
        .into();

        if err != Error::Success {
            Err(err)
        } else {
            Ok(new_frame)
        }
    }

    /// Convert to bgr format
    pub fn to_bgr(&self) -> Result<Frame> {
        let new_frame = unsafe { Frame::new_with_dimensions(self.width(), self.height(), 3) }; // BGR -> 3 bytes

        let err = unsafe {
            match self.format() {
                FrameFormat::YUYV => uvc_yuyv2bgr(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                FrameFormat::UYVY => uvc_uyvy2bgr(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                FrameFormat::Any => uvc_any2bgr(self.frame.as_ptr(), new_frame.frame.as_ptr()),
                _ => uvc_any2bgr(self.frame.as_ptr(), new_frame.frame.as_ptr()),
            }
        }
        .into();

        if err != Error::Success {
            Err(err)
        } else {
            Ok(new_frame)
        }
    }

    /// Get the raw image data
    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
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

    /// Clones a frame
    pub fn duplicate(&self) -> Result<Frame> {
        unsafe {
            let mut new_frame = Frame::from_raw(uvc_allocate_frame(0));

            let err = uvc_duplicate_frame(self.frame.as_ptr(), new_frame.frame.as_mut()).into();
            if err != Error::Success {
                return Err(err);
            }
            Ok(new_frame)
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { uvc_free_frame(self.frame.as_ptr()) }
    }
}
