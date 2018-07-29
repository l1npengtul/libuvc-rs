use uvc_sys::*;

use device::DeviceList;
use error::{UvcError, UvcResult};

use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Context<'a> {
    ctx: NonNull<uvc_context>,
    _ctx: PhantomData<&'a uvc_context>,
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe {
            uvc_exit(self.ctx.as_ptr());
        }
    }
}

impl<'a> Context<'a> {
    pub fn new() -> UvcResult<Self> {
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

    pub fn devices(&'a self) -> UvcResult<DeviceList<'a>> {
        unsafe {
            let mut list = ::std::mem::uninitialized();
            let err = uvc_get_device_list(self.ctx.as_ptr(), &mut list).into();
            if err != UvcError::Success {
                return Err(err);
            }

            Ok(DeviceList::new(NonNull::new(list).unwrap()))
        }
    }
}
