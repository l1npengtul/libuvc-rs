use uvc_sys::*;

use error::{UvcError, UvcResult};

pub struct StreamCtrl {
    pub ctrl: uvc_stream_ctrl_t,
}

struct Vtable<U: Send + Sync> {
    func: Box<Fn(&uvc_frame, &mut U)>,
    data: U,
}

pub struct ActiveStream<'a, U: 'a + Send + Sync> {
    devh: &'a ::DeviceHandle,
    #[allow(unused)]
    vtable: Box<Vtable<U>>,
}

impl<'a, U: 'a + Send + Sync> Drop for ActiveStream<'a, U> {
    fn drop(&mut self) {
        unsafe {
            uvc_stop_streaming(self.devh.devh);
        }
    }
}

unsafe extern "C" fn trampoline<F, U>(frame: *mut uvc_frame, tuple: *mut ::std::os::raw::c_void)
where
    F: 'static + Send + Sync + Fn(&uvc_frame, &mut U),
    U: 'static + Send + Sync,
{
    if frame.is_null() {
        println!("Frame is null");
        ::std::process::abort();
    }
    let frame = &*frame;

    if tuple.is_null() {
        println!("tuple is null");
        ::std::process::abort();
    }

    let panic = ::std::panic::catch_unwind(|| {
        let vtable = tuple as *mut Box<Vtable<U>>;

        let func = &(*vtable).func;
        let data = &mut (*vtable).data;

        func(frame, data);
    });

    if panic.is_err() {
        ::std::process::abort();
    }
}

impl<'a> StreamCtrl {
    pub fn start_streaming<F, U>(
        &mut self,
        devh: &'a ::DeviceHandle,
        cb: F,
        user_data: U,
    ) -> UvcResult<ActiveStream<'a, U>>
    where
        F: 'static + Send + Sync + Fn(&uvc_frame, &mut U),
        U: 'static + Send + Sync,
    {
        let mut tuple = Box::new(Vtable::<U> {
            func: Box::new(cb),
            data: user_data,
        });

        unsafe {
            let err = uvc_start_streaming(
                devh.devh,
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
