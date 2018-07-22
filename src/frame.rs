use error::{UvcError, UvcResult};

#[derive(Debug)]
pub struct Frame {
    frame: *mut ::uvc_frame,
}

impl Frame {
    pub unsafe fn from_raw(frame: *mut ::uvc_frame) -> Frame {
        assert!(!frame.is_null());
        Frame { frame }
    }

    fn new_with_dimensions(width: u32, height: u32, components: u32) -> Self {
        let frame = unsafe { ::uvc_allocate_frame((width * height * components) as usize) };
        assert!(!frame.is_null());

        Frame { frame: frame }
    }
    pub fn to_rgb(self: &Self) -> UvcResult<Frame> {
        let new_frame = Frame::new_with_dimensions(self.width(), self.height(), 3); // RGB -> 3 bytes

        let err = unsafe { ::uvc_any2rgb(self.frame, new_frame.frame) }.into();
        if err != UvcError::Success {
            Err(err)
        } else {
            Ok(new_frame)
        }
    }

    pub fn to_bytes(self: &Self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts((*self.frame).data as *const u8, (*self.frame).data_bytes)
        }
    }

    pub fn width(self: &Self) -> u32 {
        unsafe { (*self.frame) }.width
    }
    pub fn height(self: &Self) -> u32 {
        unsafe { (*self.frame) }.height
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { ::uvc_free_frame(self.frame) }
    }
}
