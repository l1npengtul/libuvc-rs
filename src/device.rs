use streaming;
use uvc_sys::*;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::NonNull;

use error::{Error, Result};
use frame::FrameFormat;

unsafe impl<'a> Send for DeviceList<'a> {}
unsafe impl<'a> Sync for DeviceList<'a> {}
#[derive(Debug)]
/// List of camera devices, iterate to get the device(s)
pub struct DeviceList<'a> {
    list: NonNull<*mut uvc_device>,
    _ph: PhantomData<&'a &'a uvc_device>,

    reached_end: bool,
    index: usize,
}

impl<'a> Drop for DeviceList<'a> {
    fn drop(&mut self) {
        unsafe { uvc_free_device_list(self.list.as_ptr(), false as u8) }
    }
}

impl<'a> DeviceList<'a> {
    pub(crate) fn new(list: NonNull<*mut uvc_device>) -> Self {
        Self {
            list,
            _ph: PhantomData,
            reached_end: false,
            index: 0,
        }
    }
}

impl<'a> Iterator for DeviceList<'a> {
    type Item = Device<'a>;

    fn next(&mut self) -> Option<Device<'a>> {
        if self.reached_end {
            return None;
        }

        let item = unsafe { self.list.as_ptr().offset(self.index as isize) };
        if item.is_null() {
            self.reached_end = true;
            return None;
        }

        self.index += 1;
        Some(unsafe { Device::from_raw(*item) })
    }
}

unsafe impl<'a> Send for Device<'a> {}
unsafe impl<'a> Sync for Device<'a> {}
#[derive(Debug)]
/// Device that can be opened
pub struct Device<'a> {
    dev: NonNull<uvc_device>,
    _dev: PhantomData<&'a uvc_device>,
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        unsafe { uvc_unref_device(self.dev.as_ptr()) };
    }
}

impl<'a> Device<'a> {
    pub(crate) unsafe fn from_raw(dev: *mut uvc_device) -> Self {
        Device {
            dev: NonNull::new(dev).unwrap(),
            _dev: PhantomData,
        }
    }
    /// Create handle to a device
    pub fn open(&'a self) -> Result<DeviceHandle<'a>> {
        unsafe {
            let mut devh = ::std::mem::uninitialized();
            let err = uvc_open(self.dev.as_ptr(), &mut devh).into();
            match err {
                Error::Success => Ok(DeviceHandle {
                    devh: NonNull::new(devh).unwrap(),
                    _devh: PhantomData,
                }),
                err => Err(err),
            }
        }
    }
    /// Get the description of a device
    pub fn description(&self) -> Result<DeviceDescription> {
        unsafe {
            let mut desc = ::std::mem::uninitialized();
            let err = uvc_get_device_descriptor(self.dev.as_ptr(), &mut desc).into();
            if err != Error::Success {
                return Err(err);
            }

            let id_vendor = (*desc).idVendor;
            let id_product = (*desc).idProduct;
            let bcd_uvc = (*desc).bcdUVC;

            let serial_number_c_str = (*desc).serialNumber;
            let serial_number = if serial_number_c_str.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(serial_number_c_str)
                        .to_owned()
                        .into_string()
                        .unwrap(),
                )
            };
            let manufacturer_c_str = (*desc).manufacturer;
            let manufacturer = if manufacturer_c_str.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(manufacturer_c_str)
                        .to_owned()
                        .into_string()
                        .unwrap(),
                )
            };
            let product_c_str = (*desc).product;
            let product = if product_c_str.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(product_c_str)
                        .to_owned()
                        .into_string()
                        .unwrap(),
                )
            };
            let descp = Ok(DeviceDescription {
                id_vendor,
                id_product,
                bcd_uvc,
                serial_number,
                manufacturer,
                product,
            });

            uvc_free_device_descriptor(desc);

            descp
        }
    }
}

unsafe impl<'a> Send for DeviceHandle<'a> {}
unsafe impl<'a> Sync for DeviceHandle<'a> {}
#[derive(Debug)]
/// Open handle to a device
pub struct DeviceHandle<'a> {
    pub(crate) devh: NonNull<uvc_device_handle>,
    _devh: PhantomData<&'a uvc_device_handle>,
}

impl<'a> DeviceHandle<'a> {
    /// List all supported formats
    pub fn supported_formats(&self) -> FormatDescriptors<'a> {
        unsafe {
            let format_descs = uvc_get_format_descs(self.devh.as_ptr());

            FormatDescriptors {
                head: format_descs,
                _ph: PhantomData,
            }
        }
    }

    /// Creates a stream handle
    pub fn get_stream_ctrl_with_format_size_and_fps(
        &self,
        format: FrameFormat,
        width: u32,
        height: u32,
        fps: u32,
    ) -> Result<streaming::StreamCtrl<'a>> {
        unsafe {
            let mut ctrl = ::std::mem::uninitialized();
            let err = uvc_get_stream_ctrl_format_size(
                self.devh.as_ptr(),
                &mut ctrl,
                format.into(),
                width as i32,
                height as i32,
                fps as i32,
            ).into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok(::StreamCtrl {
                    ctrl,
                    _ctrl: PhantomData,
                })
            }
        }
    }
}

impl<'a> Drop for DeviceHandle<'a> {
    fn drop(&mut self) {
        unsafe {
            uvc_close(self.devh.as_ptr());
        }
    }
}

#[derive(Debug)]
/// Describes the device
pub struct DeviceDescription {
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_uvc: u16,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

unsafe impl<'a> Send for FormatDescriptor<'a> {}
unsafe impl<'a> Sync for FormatDescriptor<'a> {}
/// Describes possible formats
pub struct FormatDescriptor<'a> {
    format_desc: NonNull<uvc_format_desc_t>,
    _ph: PhantomData<&'a uvc_format_desc_t>,
}

#[derive(Debug)]
pub enum FormatDescriptionSubtype {
    Undefined,
    InputHeader,
    OutputHeader,
    StillImageFrame,
    FormatUncompressed,
    FrameUncompressed,
    FormatMJPEG,
    FrameMJPEG,
    FormatMPEG2TS,
    FormatDV,
    ColorFormat,
    FormatFrameBased,
    FrameFrameBased,
    FormatStreamBased,
}

impl<'a> FormatDescriptor<'a> {
    pub fn supported_formats(&self) -> FrameDescriptors {
        FrameDescriptors {
            head: unsafe { (*self.format_desc.as_ptr()).frame_descs },
            _ph: PhantomData,
        }
    }

    pub fn subtype(&self) -> FormatDescriptionSubtype {
        #[allow(non_upper_case_globals)]
        match unsafe { (*self.format_desc.as_ptr()).bDescriptorSubtype } {
            uvc_vs_desc_subtype_UVC_VS_UNDEFINED => FormatDescriptionSubtype::Undefined,
            uvc_vs_desc_subtype_UVC_VS_INPUT_HEADER => FormatDescriptionSubtype::InputHeader,
            uvc_vs_desc_subtype_UVC_VS_OUTPUT_HEADER => FormatDescriptionSubtype::OutputHeader,
            uvc_vs_desc_subtype_UVC_VS_STILL_IMAGE_FRAME => {
                FormatDescriptionSubtype::StillImageFrame
            }
            uvc_vs_desc_subtype_UVC_VS_FORMAT_UNCOMPRESSED => {
                FormatDescriptionSubtype::FormatUncompressed
            }
            uvc_vs_desc_subtype_UVC_VS_FRAME_UNCOMPRESSED => {
                FormatDescriptionSubtype::FrameUncompressed
            }
            uvc_vs_desc_subtype_UVC_VS_FORMAT_MJPEG => FormatDescriptionSubtype::FormatMJPEG,
            uvc_vs_desc_subtype_UVC_VS_FRAME_MJPEG => FormatDescriptionSubtype::FrameMJPEG,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_MPEG2TS => FormatDescriptionSubtype::FormatMPEG2TS,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_DV => FormatDescriptionSubtype::FormatDV,
            uvc_vs_desc_subtype_UVC_VS_COLORFORMAT => FormatDescriptionSubtype::ColorFormat,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_FRAME_BASED => {
                FormatDescriptionSubtype::FormatFrameBased
            }
            uvc_vs_desc_subtype_UVC_VS_FRAME_FRAME_BASED => {
                FormatDescriptionSubtype::FrameFrameBased
            }
            uvc_vs_desc_subtype_UVC_VS_FORMAT_STREAM_BASED => {
                FormatDescriptionSubtype::FormatStreamBased
            }
            _ => panic!("This enum value is not valid"),
        }
    }
}

unsafe impl<'a> Send for FormatDescriptors<'a> {}
unsafe impl<'a> Sync for FormatDescriptors<'a> {}
/// Iterate to get a FormatDescriptor
pub struct FormatDescriptors<'a> {
    head: *const uvc_format_desc_t,
    _ph: PhantomData<&'a uvc_format_desc_t>,
}

impl<'a> Iterator for FormatDescriptors<'a> {
    type Item = FormatDescriptor<'a>;

    fn next(&mut self) -> Option<FormatDescriptor<'a>> {
        match NonNull::new(self.head as *mut _) {
            None => None,
            Some(x) => {
                let current = FormatDescriptor {
                    format_desc: x,
                    _ph: PhantomData,
                };
                self.head = unsafe { (*self.head).next };
                Some(current)
            }
        }
    }
}

unsafe impl<'a> Send for FrameDescriptor<'a> {}
unsafe impl<'a> Sync for FrameDescriptor<'a> {}
#[derive(Debug)]
/// Describes possible frames
pub struct FrameDescriptor<'a> {
    frame_desc: NonNull<uvc_frame_desc_t>,
    _ph: PhantomData<&'a uvc_frame_desc_t>,
}

impl<'a> FrameDescriptor<'a> {
    pub fn width(&self) -> u16 {
        unsafe { (*self.frame_desc.as_ptr()).wWidth }
    }
    pub fn height(&self) -> u16 {
        unsafe { (*self.frame_desc.as_ptr()).wHeight }
    }
    /// Time in 100ns
    pub fn intervals(&self) -> &[u32] {
        unsafe {
            let intervals = (*self.frame_desc.as_ptr()).intervals;
            let mut len = 0;
            loop {
                let x = *intervals.offset(len);
                if x == 0 {
                    return ::std::slice::from_raw_parts::<'a>(intervals, len as usize);
                }
                len += 1;
            }
        }
    }

    /// Duration between captures
    pub fn intervals_duration(&self) -> Vec<::std::time::Duration> {
        let times = self.intervals();
        let mut dur = Vec::with_capacity(times.len());

        for i in times {
            dur.push(::std::time::Duration::from_nanos(*i as u64 * 100));
        }

        return dur;
    }
}

unsafe impl<'a> Send for FrameDescriptors<'a> {}
unsafe impl<'a> Sync for FrameDescriptors<'a> {}
/// Iterate to get a FrameDescriptor
pub struct FrameDescriptors<'a> {
    head: *mut uvc_frame_desc_t,
    _ph: PhantomData<&'a uvc_frame_desc_t>,
}

impl<'a> Iterator for FrameDescriptors<'a> {
    type Item = FrameDescriptor<'a>;

    fn next(&mut self) -> Option<FrameDescriptor<'a>> {
        match NonNull::new(self.head) {
            None => None,
            Some(x) => {
                let current = FrameDescriptor {
                    frame_desc: x,
                    _ph: PhantomData,
                };
                unsafe { self.head = (*self.head).next };
                Some(current)
            }
        }
    }
}
