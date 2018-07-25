use error::{UvcError, UvcResult};

use std::ptr::NonNull;

#[derive(Debug)]
pub struct Frame {
    frame: NonNull<::uvc_frame>,
}

impl Frame {
    pub(crate) unsafe fn from_raw(frame: *mut ::uvc_frame) -> Frame {
        Frame {
            frame: NonNull::new(frame).unwrap(),
        }
    }

    fn new_with_dimensions(width: u32, height: u32, components: u32) -> Self {
        let frame = unsafe { ::uvc_allocate_frame((width * height * components) as usize) };

        Frame {
            frame: NonNull::new(frame).unwrap(),
        }
    }
    pub fn to_rgb(self: &Self) -> UvcResult<Frame> {
        let new_frame = Frame::new_with_dimensions(self.width(), self.height(), 3); // RGB -> 3 bytes

        let err = unsafe { ::uvc_any2rgb(self.frame.as_ptr(), new_frame.frame.as_ptr()) }.into();
        if err != UvcError::Success {
            Err(err)
        } else {
            Ok(new_frame)
        }
    }

    pub fn to_bytes(self: &Self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                (*self.frame.as_ptr()).data as *const u8,
                (*self.frame.as_ptr()).data_bytes,
            )
        }
    }

    pub fn width(self: &Self) -> u32 {
        unsafe { (*self.frame.as_ptr()) }.width
    }
    pub fn height(self: &Self) -> u32 {
        unsafe { (*self.frame.as_ptr()) }.height
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { ::uvc_free_frame(self.frame.as_ptr()) }
    }
}
