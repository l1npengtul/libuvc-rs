use uvc_sys::*;

use device::Device;
use error::{UvcError, UvcResult};

#[derive(Debug)]
pub struct Context {
    ctx: *mut uvc_context,
}

impl Context {
    pub fn new() -> UvcResult<Context> {
        unsafe {
            let mut ctx = ::std::mem::uninitialized();
            let err = uvc_init(&mut ctx, ::std::ptr::null_mut()).into();
            if err != UvcError::Success {
                Err(err)
            } else {
                Ok(Context { ctx })
            }
        }
    }

    pub fn get_devices(&self) -> UvcResult<Vec<Device>> {
        unsafe {
            let mut list = ::std::mem::uninitialized();
            let err = uvc_get_device_list(self.ctx, &mut list).into();
            if err != UvcError::Success {
                return Err(err);
            }

            let mut devices = Vec::new();
            let mut len = 0;
            loop {
                let walker = list.offset(len);
                let dev = *walker;
                if dev.is_null() {
                    break;
                } else {
                    devices.push(Device::from_raw(dev));
                    len += 1;
                }
            }
            uvc_free_device_list(list, false as u8);

            Ok(devices)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            uvc_exit(self.ctx);
        }
    }
}
