use uvc_sys::*;

use device::Device;
use error::{UvcError, UvcResult};

use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Context<'a> {
    ctx: NonNull<uvc_context>,
    _ctx: PhantomData<&'a uvc_context>,
}

impl<'a> Context<'a> {
    pub fn new() -> UvcResult<Context<'a>> {
        unsafe {
            let mut ctx = ::std::mem::uninitialized();
            let err = uvc_init(&mut ctx, ::std::ptr::null_mut()).into();
            if err != UvcError::Success {
                Err(err)
            } else {
                Ok(Context {
                    ctx: NonNull::new(ctx).unwrap(),
                    _ctx: PhantomData,
                })
            }
        }
    }

    pub fn get_devices(&'a self) -> UvcResult<Vec<Device<'a>>> {
        unsafe {
            let mut list = ::std::mem::uninitialized();
            let err = uvc_get_device_list(self.ctx.as_ptr(), &mut list).into();
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

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe {
            uvc_exit(self.ctx.as_ptr());
        }
    }
}
