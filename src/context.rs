use uvc_sys::*;

use device::{Device, DeviceList};
use error::{Error, Result};

use std;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr::NonNull;

unsafe impl<'a> Send for Context<'a> {}
unsafe impl<'a> Sync for Context<'a> {}
#[derive(Debug)]
/// Contains the `libuvc` context
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
    /// Creates a new context
    pub fn new() -> Result<Self> {
        unsafe {
            let mut ctx = std::mem::uninitialized();
            let err = uvc_init(&mut ctx, std::ptr::null_mut()).into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok(Context {
                    ctx: NonNull::new(ctx).unwrap(),
                    _ctx: PhantomData,
                })
            }
        }
    }

    /// Enumerates the available devices
    pub fn devices(&'a self) -> Result<DeviceList<'a>> {
        unsafe {
            let mut list = std::mem::uninitialized();
            let err = uvc_get_device_list(self.ctx.as_ptr(), &mut list).into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(DeviceList::new(NonNull::new(list).unwrap()))
        }
    }

    /// Find a device based on informations about the device
    /// Pass None to all fields to get a default device
    pub fn find_device(
        &'a self,
        vendor_id: Option<c_int>,
        product_id: Option<c_int>,
        serial_number: Option<&str>,
    ) -> Result<Device<'a>> {
        unsafe {
            let mut device = std::mem::uninitialized();
            let cstr = serial_number.map(|v| CString::new(v).unwrap());
            let err = uvc_find_device(
                self.ctx.as_ptr(),
                &mut device,
                vendor_id.unwrap_or(0),
                product_id.unwrap_or(0),
                cstr.map_or(std::ptr::null(), |v| v.as_ptr()),
            ).into();
            if err != Error::Success {
                return Err(err);
            }
            Ok(Device::from_raw(device))
        }
    }
}
