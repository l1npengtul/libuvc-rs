use uvc_sys::*;

use error::{UvcError, UvcResult};
use frame::Frame;

use std::marker::PhantomData;

#[derive(Debug)]
pub struct StreamCtrl<'a> {
    pub(crate) ctrl: uvc_stream_ctrl_t,
    pub(crate) _ctrl: PhantomData<&'a uvc_stream_ctrl_t>,
}

struct Vtable<U> {
    func: Box<Fn(&Frame, &mut U)>,
    data: U,
}

pub struct ActiveStream<'a, U: 'a + Send + Sync> {
    devh: &'a ::DeviceHandle<'a>,
    #[allow(unused)]
    vtable: Box<Vtable<U>>,
}

impl<'a, U: 'a + Send + Sync> ActiveStream<'a, U> {
    pub fn stop(self) {
        // Taking ownership of the stream, which drops it
    }
}

impl<'a, U: 'a + Send + Sync> Drop for ActiveStream<'a, U> {
    fn drop(&mut self) {
        unsafe {
            uvc_stop_streaming(self.devh.devh.as_ptr());
        }
    }
}

unsafe extern "C" fn trampoline<F, U>(frame: *mut uvc_frame, userdata: *mut ::std::os::raw::c_void)
where
    F: 'static + Send + Sync + Fn(&Frame, &mut U),
    U: 'static + Send + Sync,
{
    let panic = ::std::panic::catch_unwind(|| {
        if frame.is_null() {
            panic!("Frame is null");
        }
        let frame = Frame::from_raw(frame);

        if userdata.is_null() {
            panic!("Userdata is null");
        }

        let vtable = userdata as *mut Box<Vtable<U>>;

        let func = &(*vtable).func;
        let data = &mut (*vtable).data;

        func(&frame, data);

        ::std::mem::forget(frame); // Not our frame
    });

    if panic.is_err() {
        eprintln!("User defined function panicked");
        ::std::process::abort();
    }
}

impl<'a> StreamCtrl<'a> {
    pub fn start_streaming<F, U>(
        &'a mut self,
        devh: &'a ::DeviceHandle,
        cb: F,
        user_data: U,
    ) -> UvcResult<ActiveStream<'a, U>>
    where
        F: 'static + Send + Sync + Fn(&Frame, &mut U),
        U: 'static + Send + Sync,
    {
        let mut tuple = Box::new(Vtable::<U> {
            func: Box::new(cb),
            data: user_data,
        });

        unsafe {
            let err = uvc_start_streaming(
                devh.devh.as_ptr(),
                &mut self.ctrl,
                Some(trampoline::<F, U>),
                &mut tuple as *mut _ as *mut ::std::os::raw::c_void,
                0,
            ).into();
            if err != UvcError::Success {
                Err(err)
            } else {
                Ok(ActiveStream {
                    devh,
                    vtable: tuple,
                })
            }
        }
    }
}
