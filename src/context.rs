use uvc_sys::*;

use device::Device;

pub struct Context {
    ctx: *mut uvc_context,
}

impl Context {
    pub fn new() -> Result<Context, ::UvcError> {
        unsafe {
            let mut ctx = ::std::mem::uninitialized();
            let err = uvc_init(&mut ctx, ::std::ptr::null_mut());
            if err != uvc_error::UVC_SUCCESS {
                return Err(err);
            }
            Ok(Context { ctx })
        }
    }

    pub fn get_devices(&self) -> Result<Vec<Device>, ::UvcError> {
        unsafe {
            let mut list = ::std::mem::uninitialized();
            let err = uvc_get_device_list(self.ctx, &mut list);
            if err != uvc_error::UVC_SUCCESS {
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
                    devices.push(Device { dev });
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
