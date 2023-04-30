use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::formats::{FrameFormat, StreamFormat};
use crate::streaming::StreamHandle;
use uvc_sys::*;

unsafe impl<'a> Send for DeviceList<'a> {}
unsafe impl<'a> Sync for DeviceList<'a> {}
#[derive(Debug)]
/// List of camera devices, iterate to get the device(s)
pub struct DeviceList<'a> {
    start: *mut *mut uvc_device,
    list: NonNull<*mut uvc_device>,
    _ph: PhantomData<&'a &'a uvc_device>,
}

impl<'a> Drop for DeviceList<'a> {
    fn drop(&mut self) {
        unsafe { uvc_free_device_list(self.start, false as u8) }
    }
}

impl<'a> DeviceList<'a> {
    pub(crate) fn new(list: NonNull<*mut uvc_device>) -> Self {
        Self {
            start: list.as_ptr(),
            list,
            _ph: PhantomData,
        }
    }
}

impl<'a> Iterator for DeviceList<'a> {
    type Item = Device<'a>;

    fn next(&mut self) -> Option<Device<'a>> {
        let item = self.list.as_ptr();
        if unsafe { (*item).is_null() } {
            return None;
        }

        let device = unsafe { Device::from_raw(*item) };
        self.list = unsafe { NonNull::new(self.list.as_ptr().add(1)).unwrap() };

        Some(device)
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
            let mut devh = std::mem::MaybeUninit::uninit();
            let err = uvc_open(self.dev.as_ptr(), devh.as_mut_ptr()).into();
            Error::cvt(err)?;
            Ok(DeviceHandle {
                devh: NonNull::new(devh.assume_init()).unwrap(),
                _devh: PhantomData,
            })
        }
    }
    /// Get the description of a device
    pub fn description(&self) -> Result<DeviceDescription> {
        unsafe {
            let mut desc = std::mem::MaybeUninit::uninit();
            let err = uvc_get_device_descriptor(self.dev.as_ptr(), desc.as_mut_ptr()).into();
            Error::cvt(err)?;

            let desc = desc.assume_init();

            let vendor_id = (*desc).idVendor;
            let product_id = (*desc).idProduct;
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
                vendor_id,
                product_id,
                bcd_uvc,
                serial_number,
                manufacturer,
                product,
            });

            uvc_free_device_descriptor(desc);

            descp
        }
    }

    /// Bus number of which this device is connected
    #[must_use]
    pub fn bus_number(&self) -> u8 {
        unsafe { uvc_get_bus_number(self.dev.as_ptr()) }
    }

    /// Device address within the bus
    #[must_use]
    pub fn device_address(&self) -> u8 {
        unsafe { uvc_get_device_address(self.dev.as_ptr()) }
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

impl<'a, 'b> DeviceHandle<'a> {
    /// List all supported formats
    #[must_use]
    pub fn supported_formats(&self) -> FormatDescriptors<'a> {
        unsafe {
            let format_descs = uvc_get_format_descs(self.devh.as_ptr());

            FormatDescriptors {
                head: format_descs,
                _ph: PhantomData,
            }
        }
    }

    /// Iterates over all available formats to select the best format.
    ///
    /// f should compare (x, y) and return the preferred format.
    pub fn get_preferred_format<F>(&self, f: F) -> Option<StreamFormat>
    where
        F: Fn(StreamFormat, StreamFormat) -> StreamFormat,
    {
        let mut pref_format = None;
        for i in self.supported_formats() {
            for j in i.supported_formats() {
                for k in j.intervals() {
                    let format = StreamFormat {
                        width: u32::from(j.width()),
                        height: u32::from(j.height()),
                        fps: 10_000_000 / *k,
                        format: match j.subtype() {
                            DescriptionSubtype::FormatMJPEG | DescriptionSubtype::FrameMJPEG => {
                                FrameFormat::MJPEG
                            }
                            DescriptionSubtype::FormatUncompressed
                            | DescriptionSubtype::FrameUncompressed => FrameFormat::Uncompressed,
                            _ => FrameFormat::Any,
                        },
                    };
                    pref_format = Some(pref_format.map_or(format, |x| f(x, format)));
                }
            }
        }
        pref_format
    }

    /// Creates a stream handle
    pub fn get_stream_handle_with_format_size_and_fps(
        &'a self,
        format: FrameFormat,
        width: u32,
        height: u32,
        fps: u32,
    ) -> Result<StreamHandle<'a>> {
        unsafe {
            let mut handle = std::mem::MaybeUninit::uninit();
            let err = uvc_get_stream_ctrl_format_size(
                self.devh.as_ptr(),
                handle.as_mut_ptr(),
                format.into(),
                width as i32,
                height as i32,
                fps as i32,
            )
            .into();
            Error::cvt(err)?;
            Ok(StreamHandle {
                handle: handle.assume_init(),
                devh: self,
            })
        }
    }

    /// Creates a stream handle
    pub fn get_stream_handle_with_format(
        &'a self,
        format: StreamFormat,
    ) -> Result<StreamHandle<'a>> {
        self.get_stream_handle_with_format_size_and_fps(
            format.format,
            format.width,
            format.height,
            format.fps,
        )
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
    pub vendor_id: u16,
    pub product_id: u16,
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

#[derive(Debug, PartialEq)]
/// Describes what frame or format is supported
pub enum DescriptionSubtype {
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

impl From<uvc_vs_desc_subtype> for DescriptionSubtype {
    fn from(x: uvc_vs_desc_subtype) -> DescriptionSubtype {
        #[allow(non_upper_case_globals)]
        match x {
            uvc_vs_desc_subtype_UVC_VS_UNDEFINED => DescriptionSubtype::Undefined,
            uvc_vs_desc_subtype_UVC_VS_INPUT_HEADER => DescriptionSubtype::InputHeader,
            uvc_vs_desc_subtype_UVC_VS_OUTPUT_HEADER => DescriptionSubtype::OutputHeader,
            uvc_vs_desc_subtype_UVC_VS_STILL_IMAGE_FRAME => DescriptionSubtype::StillImageFrame,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_UNCOMPRESSED => {
                DescriptionSubtype::FormatUncompressed
            }
            uvc_vs_desc_subtype_UVC_VS_FRAME_UNCOMPRESSED => DescriptionSubtype::FrameUncompressed,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_MJPEG => DescriptionSubtype::FormatMJPEG,
            uvc_vs_desc_subtype_UVC_VS_FRAME_MJPEG => DescriptionSubtype::FrameMJPEG,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_MPEG2TS => DescriptionSubtype::FormatMPEG2TS,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_DV => DescriptionSubtype::FormatDV,
            uvc_vs_desc_subtype_UVC_VS_COLORFORMAT => DescriptionSubtype::ColorFormat,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_FRAME_BASED => DescriptionSubtype::FormatFrameBased,
            uvc_vs_desc_subtype_UVC_VS_FRAME_FRAME_BASED => DescriptionSubtype::FrameFrameBased,
            uvc_vs_desc_subtype_UVC_VS_FORMAT_STREAM_BASED => DescriptionSubtype::FormatStreamBased,
            _ => DescriptionSubtype::Undefined,
        }
    }
}

impl<'a> FormatDescriptor<'a> {
    #[must_use]
    pub fn supported_formats(&self) -> FrameDescriptors {
        FrameDescriptors {
            head: unsafe { (*self.format_desc.as_ptr()).frame_descs },
            _ph: PhantomData,
        }
    }

    #[must_use]
    pub fn subtype(&self) -> DescriptionSubtype {
        unsafe { (*self.format_desc.as_ptr()).bDescriptorSubtype }.into()
    }
}

unsafe impl<'a> Send for FormatDescriptors<'a> {}
unsafe impl<'a> Sync for FormatDescriptors<'a> {}
/// Iterate to get a `FormatDescriptor`
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
    #[must_use]
    pub fn width(&self) -> u16 {
        unsafe { (*self.frame_desc.as_ptr()).wWidth }
    }
    #[must_use]
    pub fn height(&self) -> u16 {
        unsafe { (*self.frame_desc.as_ptr()).wHeight }
    }
    /// Type of frame
    #[must_use]
    pub fn subtype(&self) -> DescriptionSubtype {
        unsafe { (*self.frame_desc.as_ptr()).bDescriptorSubtype }.into()
    }
    /// Time in 100ns
    #[must_use]
    pub fn intervals(&self) -> &[u32] {
        unsafe {
            let intervals: *const u32 = (*self.frame_desc.as_ptr()).intervals;
            if intervals.is_null() {
                return &[];
            }
            let mut len = 0;
            loop {
                let x = *intervals.add(len);
                if x == 0 {
                    return slice::from_raw_parts::<'a>(intervals, len);
                }
                len += 1;
            }
        }
    }

    /// Duration between captures
    #[must_use]
    pub fn intervals_duration(&self) -> Vec<Duration> {
        let times = self.intervals();
        let mut durations = Vec::with_capacity(times.len());

        for i in times {
            durations.push(Duration::from_nanos(u64::from(*i) * 100));
        }

        durations
    }
}

unsafe impl<'a> Send for FrameDescriptors<'a> {}
unsafe impl<'a> Sync for FrameDescriptors<'a> {}
/// Iterate to get a `FrameDescriptor`
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
