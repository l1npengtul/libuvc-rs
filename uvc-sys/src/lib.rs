#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

#[repr(C)]
pub struct uvc_context(libc::c_void);
pub type uvc_context_t = uvc_context;

#[repr(C)]
pub struct uvc_device(libc::c_void);
pub type uvc_device_t = uvc_device;

#[repr(C)]
pub struct uvc_device_handle(libc::c_void);
pub type uvc_device_handle_t = uvc_device_handle;

#[repr(C)]
#[derive(Debug)]
pub struct uvc_stream_ctrl {
    pub bmHint: u16,
    pub bFormatIndex: u8,
    pub bFrameIndex: u8,
    pub dwFrameInterval: u32,
    pub wKeyFrameRate: u16,
    pub wPFrameRate: u16,
    pub wCompQuality: u16,
    pub wCompWindowSize: u16,
    pub wDelay: u16,
    pub dwMaxVideoFrameSize: u32,
    pub dwMaxPayloadTransferSize: u32,
    pub dwClockFrequency: u32,
    pub bmFramingInfo: u8,
    pub bPreferredVersion: u8,
    pub bMinVersion: u8,
    pub bMaxVersion: u8,
    pub bInterfaceNumber: u8,
}
pub type uvc_stream_ctrl_t = uvc_stream_ctrl;

#[repr(C)]
pub struct libusb_context(libc::c_void);

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum uvc_error {
    /** Success (no error) */
    UVC_SUCCESS = 0,
    /** Input/output error */
    UVC_ERROR_IO = -1,
    /** Invalid parameter */
    UVC_ERROR_INVALID_PARAM = -2,
    /** Access denied */
    UVC_ERROR_ACCESS = -3,
    /** No such device */
    UVC_ERROR_NO_DEVICE = -4,
    /** Entity not found */
    UVC_ERROR_NOT_FOUND = -5,
    /** Resource busy */
    UVC_ERROR_BUSY = -6,
    /** Operation timed out */
    UVC_ERROR_TIMEOUT = -7,
    /** Overflow */
    UVC_ERROR_OVERFLOW = -8,
    /** Pipe error */
    UVC_ERROR_PIPE = -9,
    /** System call interrupted */
    UVC_ERROR_INTERRUPTED = -10,
    /** Insufficient memory */
    UVC_ERROR_NO_MEM = -11,
    /** Operation not supported */
    UVC_ERROR_NOT_SUPPORTED = -12,
    /** Device is not UVC-compliant */
    UVC_ERROR_INVALID_DEVICE = -50,
    /** Mode not supported */
    UVC_ERROR_INVALID_MODE = -51,
    /** Resource has a callback (can't use polling and async) */
    UVC_ERROR_CALLBACK_EXISTS = -52,
    /** Undefined error */
    UVC_ERROR_OTHER = -99,
}
pub type uvc_error_t = uvc_error;

#[repr(C)]
pub enum uvc_status_class {
    UVC_STATUS_CLASS_CONTROL = 0x10,
    UVC_STATUS_CALLS_CONTROL_CAMERA = 0x11,
    UVC_STATUS_CLASS_CONTROL_PROCESSING,
}

#[repr(C)]
pub enum uvc_status_attribute {
    UVC_STATUS_ATTRIBUTE_VALUE_CHANGE = 0x00,
    UVC_STATUS_ATTRIBUTE_INFO_CHANGE = 0x01,
    UVC_STATUS_ATTRIBUTE_FAILURE_CHANGE = 0x02,
    UVC_STATUS_ATTRIBUTE_UNKNOWN = 0xff,
}

pub type uvc_status_callback_t = Option<
    unsafe extern "C" fn(
        status_class: uvc_status_class,
        event: libc::c_int,
        selector: libc::c_int,
        status_attribute: uvc_status_attribute,
        data: *mut libc::c_void,
        data_len: libc::size_t,
        user_ptr: *mut libc::c_void,
    ),
>;

#[repr(C)]
pub struct uvc_device_descriptor {
    /** Vendor ID */
    pub idVendor: u16,
    /** Product ID */
    pub idProduct: u16,
    /** UVC compliance level, e.g. 0x0100 (1.0), 0x0110 */
    pub bcdUVC: u16,
    /** Serial number (null if unavailable) */
    pub serialNumber: *const libc::c_char,
    /** Device-reported manufacturer name (or null) */
    pub manufacturer: *const libc::c_char,
    /** Device-reporter product name (or null) */
    pub product: *const libc::c_char,
}
type uvc_device_descriptor_t = uvc_device_descriptor;

#[repr(C)]
pub struct libusb_device_handle(libc::c_void);

#[repr(C)]
pub enum uvc_req_code {
    UVC_RC_UNDEFINED = 0x00,
    UVC_SET_CUR = 0x01,
    UVC_GET_CUR = 0x81,
    UVC_GET_MIN = 0x82,
    UVC_GET_MAX = 0x83,
    UVC_GET_RES = 0x84,
    UVC_GET_LEN = 0x85,
    UVC_GET_INFO = 0x86,
    UVC_GET_DEF = 0x87,
}

#[repr(C)]
pub struct uvc_stream_handle(libc::c_void);
type uvc_stream_handle_t = uvc_stream_handle;

#[repr(C)]
pub enum uvc_frame_format {
    // UVC_FRAME_FORMAT_UNKNOWN = 0,
    /** Any supported format */
    UVC_FRAME_FORMAT_ANY = 0,
    UVC_FRAME_FORMAT_UNCOMPRESSED,
    UVC_FRAME_FORMAT_COMPRESSED,
    /** YUYV/YUV2/YUV422: YUV encoding with one luminance value per pixel and
     * one UV (chrominance) pair for every two pixels.
     */
    UVC_FRAME_FORMAT_YUYV,
    UVC_FRAME_FORMAT_UYVY,
    /** 24-bit RGB */
    UVC_FRAME_FORMAT_RGB,
    UVC_FRAME_FORMAT_BGR,
    /** Motion-JPEG (or JPEG) encoded images */
    UVC_FRAME_FORMAT_MJPEG,
    /** Greyscale images */
    UVC_FRAME_FORMAT_GRAY8,
    UVC_FRAME_FORMAT_GRAY16,
    /* Raw colour mosaic images */
    UVC_FRAME_FORMAT_BY8,
    UVC_FRAME_FORMAT_BA81,
    UVC_FRAME_FORMAT_SGRBG8,
    UVC_FRAME_FORMAT_SGBRG8,
    UVC_FRAME_FORMAT_SRGGB8,
    UVC_FRAME_FORMAT_SBGGR8,
    /** Number of formats understood */
    UVC_FRAME_FORMAT_COUNT,
}

pub type uvc_frame_callback_t =
    Option<unsafe extern "C" fn(frame: *mut uvc_frame, user_ptr: *mut libc::c_void)>;

#[repr(C)]
pub struct uvc_frame {
    /** Image data for this frame */
    pub data: *mut libc::c_void,
    /** Size of image data buffer */
    pub data_bytes: libc::size_t,
    /** Width of image in pixels */
    pub width: u32,
    /** Height of image in pixels */
    pub height: u32,
    /** Pixel data format */
    pub frame_format: uvc_frame_format,
    /** Number of bytes per horizontal line (undefined for compressed format) */
    pub step: libc::size_t,
    /** Frame number (may skip, but is strictly monotonically increasing) */
    pub sequence: u32,
    /** Estimate of system time when the device started capturing the image */
    pub capture_time: libc::timeval,
    /** Handle on the device that produced the image.
     * @warning You must not call any uvc_* functions during a callback. */
    pub source: *mut uvc_device_handle_t,
    /** Is the data buffer owned by the library?
     * If 1, the data buffer can be arbitrarily reallocated by frame conversion functions.
     * If 0, the data buffer will not be reallocated or freed by the library.
     * Set this field to zero if you are supplying the buffer.
     */
    pub library_owns_data: u8,
}
type uvc_frame_t = uvc_frame;

extern "C" {
    // Library initialization/deinitialization
    pub fn uvc_init(ctx: *mut *mut uvc_context_t, usb_ctx: *mut libusb_context) -> uvc_error_t;
    pub fn uvc_exit(ctx: *mut uvc_context_t);

    // Device handling and enumeration
    pub fn uvc_find_device(
        ctx: *mut uvc_context_t,
        dev: *mut *mut uvc_device_t,
        vid: libc::c_int,
        sn: *const libc::c_char,
    ) -> uvc_error_t;
    pub fn uvc_get_bus_number(dev: *mut uvc_device) -> u8;
    pub fn uvc_get_device_address(dev: *mut uvc_device) -> u8;
    pub fn uvc_open(dev: *mut uvc_device, devh: *mut *mut uvc_device_handle_t) -> uvc_error_t;
    pub fn uvc_get_device_descriptor(
        dev: *mut uvc_device_t,
        desc: *mut *mut uvc_device_descriptor_t,
    ) -> uvc_error_t;
    pub fn uvc_free_device_descriptor(desc: *mut uvc_device_descriptor_t);
    pub fn uvc_get_device_list(
        ctx: *mut uvc_context_t,
        list: *mut *mut *mut uvc_device_t,
    ) -> uvc_error_t;
    pub fn uvc_free_device_list(list: *mut *mut uvc_device_t, unref_devices: u8);
    pub fn uvc_get_device(devh: *mut uvc_device_handle_t) -> *mut uvc_device_t;
    pub fn uvc_get_libusb_handle(devh: *mut uvc_device_handle_t) -> *mut libusb_device_handle;
    pub fn uvc_ref_device(dev: *mut uvc_device_t);
    pub fn uvc_unref_device(dev: *mut uvc_device_t);
    pub fn uvc_close(devh: *mut uvc_device_handle);
    pub fn uvc_set_status_callback(
        devh: *mut uvc_device_handle_t,
        cb: uvc_status_callback_t,
        user_ptr: *mut libc::c_void,
    );

    // Diagnostics
    pub fn uvc_perror(err: uvc_error_t, msg: *const libc::c_char);
    pub fn uvc_strerror(err: uvc_error_t) -> *const libc::c_char;
    pub fn uvc_print_stream_ctrl(ctrl: *mut uvc_stream_ctrl_t, stream: *mut libc::FILE);
    pub fn uvc_print_diag(devh: *mut uvc_device_handle_t, stream: *mut libc::FILE);

    // Video capture and processing controls
    pub fn uvc_get_scanning_mode(
        devh: *mut uvc_device_handle_t,
        mode: *mut u8,
        req_code: uvc_req_code,
    ) -> uvc_error_t;
    pub fn uvc_set_scanning_mode(devh: *mut uvc_device_handle_t, mode: u8) -> uvc_error_t;
    pub fn uvc_get_ae_mode(
        devh: *mut uvc_device_handle_t,
        mode: *mut u8,
        req_code: uvc_req_code,
    ) -> uvc_error_t;
    pub fn uvc_set_ae_mode(devh: *mut uvc_device_handle_t, mode: u8) -> uvc_error_t;
    pub fn uvc_get_ae_priority(
        devh: *mut uvc_device_handle_t,
        priority: *mut u8,
        req_code: uvc_req_code,
    ) -> uvc_error_t;
    pub fn uvc_set_ae_priority(devh: *mut uvc_device_handle_t, priority: u8) -> uvc_error_t;
    pub fn uvc_get_exposure_abs(
        devh: *mut uvc_device_handle_t,
        time: *mut u32,
        req_code: uvc_req_code,
    ) -> uvc_error_t;
    pub fn uvc_set_exposure_abs(devh: *mut uvc_device_handle_t, time: u32) -> uvc_error_t;
    pub fn uvc_get_exposure_rel(
        devh: *mut uvc_device_handle_t,
        step: *mut i8,
        req_code: uvc_req_code,
    ) -> uvc_error_t;
    pub fn uvc_set_exposure_rel(devh: *mut uvc_device_handle_t, step: i8) -> uvc_error_t;

    // Streaming control functions
    pub fn uvc_stream_ctrl(
        strmh: *mut uvc_stream_handle_t,
        ctrl: *mut uvc_stream_ctrl_t,
    ) -> uvc_error_t;
    pub fn uvc_probe_stream_ctrl(
        devh: *mut uvc_device_handle_t,
        ctrl: *mut uvc_stream_ctrl_t,
    ) -> uvc_error_t;
    pub fn uvc_get_stream_ctrl_format_size(
        devh: *mut uvc_device_handle_t,
        ctrl: *mut uvc_stream_ctrl_t,
        cf: uvc_frame_format,
        width: libc::c_int,
        height: libc::c_int,
        fps: libc::c_int,
    ) -> uvc_error_t;
    pub fn uvc_start_streaming(
        devh: *mut uvc_device_handle_t,
        ctrl: *mut uvc_stream_ctrl_t,
        cb: uvc_frame_callback_t,
        user_ptr: *mut libc::c_void,
        flags: u8,
    ) -> uvc_error_t;
    #[deprecated(
        note = "The stream type (bulk vs. isochronous) will be determined by the type of interface associated with the uvc_stream_ctrl_t parameter, regardless of whether the caller requests isochronous streaming. Please switch to uvc_start_streaming()."
    )]
    pub fn uvc_start_iso_streaming(
        devh: *mut uvc_device_handle_t,
        ctrl: *mut uvc_stream_ctrl_t,
        cb: uvc_frame_callback_t,
        user_ptr: *mut libc::c_void,
    ) -> uvc_error_t;
    pub fn uvc_stream_open_ctrl(
        devh: *mut uvc_device_handle,
        strmhp: *mut *mut uvc_stream_handle_t,
        ctrl: *mut uvc_stream_ctrl_t,
    ) -> uvc_error_t;
    pub fn uvc_stream_start(
        strmh: *mut uvc_stream_handle_t,
        cb: uvc_frame_callback_t,
        user_ptr: *mut libc::c_void,
        flags: u8,
    ) -> uvc_error_t;
    #[deprecated(
        note = "The stream type (bulk vs. isochronous) will be determined by the type of interface associated with the uvc_stream_ctrl_t parameter, regardless of whether the caller requests isochronous streaming. Please switch to uvc_stream_start()."
    )]
    pub fn uvc_stream_start_iso(
        strmh: *mut uvc_stream_handle_t,
        cb: uvc_frame_callback_t,
        user_ptr: *mut libc::c_void,
    ) -> uvc_error_t;
    pub fn uvc_stream_get_frame(
        strmh: *mut uvc_stream_handle_t,
        frame: *mut *mut uvc_frame_t,
        timeout_us: i32,
    ) -> uvc_error_t;
    pub fn uvc_stop_streaming(devh: *mut uvc_device_handle_t);
    pub fn uvc_stream_stop(strmh: *mut uvc_stream_handle_t) -> uvc_error_t;
    pub fn uvc_stream_close(strmh: *mut uvc_stream_handle_t);

    // Frame processing
    pub fn uvc_mjpeg2rgb(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_allocate_frame(data_bytes: libc::size_t) -> *mut uvc_frame_t;
    pub fn uvc_free_frame(frame: *mut uvc_frame_t);
    pub fn uvc_duplicate_frame(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_yuyv2rgb(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_yuyv2bgr(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_uyvy2rgb(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_uyvy2bgr(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_any2rgb(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
    pub fn uvc_any2bgr(in_: *mut uvc_frame_t, out: *mut uvc_frame_t) -> uvc_error_t;
}
