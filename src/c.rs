//! The C interface of the `arducam_mipicamera` library.

use std::os::raw::{c_char, c_int, c_void};

pub const IMAGE_ENCODING_I420: u32 = u32::from_le_bytes(*b"I420");
pub const IMAGE_ENCODING_JPEG: u32 = u32::from_le_bytes(*b"JPEG");
pub const IMAGE_ENCODING_RAW_BAYER: u32 = u32::from_le_bytes(*b"RAW ");
pub const IMAGE_ENCODING_BMP: u32 = u32::from_le_bytes(*b"BMP ");
pub const IMAGE_ENCODING_PNG: u32 = u32::from_le_bytes(*b"PNG'");

pub const OUTPUT_FLAG_KEEP_BUFFER_REQUIREMENTS: u32 = 8;
pub const OUTPUT_FLAG_BUFFER_ALLOCATION_USE_MMAL_CORE: u32 = 16;

pub const VIDEO_LEVEL_H264_4: u32 = 28;
pub const VIDEO_LEVEL_H264_41: u32 = 29;
pub const VIDEO_LEVEL_H264_42: u32 = 30;

pub const VIDEO_PROFILE_H264_BASELINE: u32 = 25;
pub const VIDEO_PROFILE_H264_MAIN: u32 = 26;
pub const VIDEO_PROFILE_H264_HIGH: u32 = 28;

pub const VIDEO_INTRA_REFRESH_CYCLIC: u32 = 0;
pub const VIDEO_INTRA_REFRESH_ADAPTIVE: u32 = 1;
pub const VIDEO_INTRA_REFRESH_BOTH: u32 = 2;
pub const VIDEO_INTRA_REFRESH_CYCLIC_MROWS: u32 = 2130706433;

/// Special value signalling that time is not known.
///
/// Timestamps in MMAL are defined as signed 64 bits integer values
/// representing microseconds. However a pre-defined special value is used to
/// signal that a timestamp is not known.
pub const TIME_UNKNOWN: i64 = 1 << 63;

/// Signals that the current payload is the end of the stream of data.
pub const MMAL_BUFFER_HEADER_FLAG_EOS: u32 = 1;
/// Signals that the start of the current payload starts a frame.
pub const MMAL_BUFFER_HEADER_FLAG_FRAME_START: u32 = 2;
/// Signals that the end of the current payload ends a frame.
pub const MMAL_BUFFER_HEADER_FLAG_FRAME_END: u32 = 4;
/// Signals that the current payload contains only complete frames (1 or more).
pub const MMAL_BUFFER_HEADER_FLAG_FRAME: u32 = 6;
/// Signals that the current payload is a keyframe (i.e. self decodable).
pub const MMAL_BUFFER_HEADER_FLAG_KEYFRAME: u32 = 8;
/// Signals a discontinuity in the stream of data (e.g. after a seek).
///
/// Can be used for instance by a decoder to reset its state.
pub const MMAL_BUFFER_HEADER_FLAG_DISCONTINUITY: u32 = 16;
/// Signals a buffer containing some kind of config data for the component
/// (e.g. codec config data).
pub const MMAL_BUFFER_HEADER_FLAG_CONFIG: u32 = 32;
/// Signals an encrypted payload.
pub const MMAL_BUFFER_HEADER_FLAG_ENCRYPTED: u32 = 64;
/// Signals a buffer containing side information.
pub const MMAL_BUFFER_HEADER_FLAG_CODECSIDEINFO: u32 = 128;
/// Signals a buffer which is the snapshot/postview image from a stills capture.
pub const MMAL_BUFFER_HEADER_FLAGS_SNAPSHOT: u32 = 256;
/// Signals a buffer which contains data known to be corrupted.
pub const MMAL_BUFFER_HEADER_FLAG_CORRUPTED: u32 = 512;
/// Signals that a buffer failed to be transmitted.
pub const MMAL_BUFFER_HEADER_FLAG_TRANSMISSION_FAILED: u32 = 1024;
/// Signals the output buffer won't be used, just update reference frames.
pub const MMAL_BUFFER_HEADER_FLAG_DECODEONLY: u32 = 2048;
/// Signals that the end of the current payload ends a NAL.
pub const MMAL_BUFFER_HEADER_FLAG_NAL_END: u32 = 4096;

pub const V4L2_CTRL_CLASS_USER: u32 = 9961472;
pub const V4L2_CID_BASE: u32 = 9963776;
pub const V4L2_CID_ARDUCAM_BASE: u32 = 9967872;
pub const V4L2_CID_ARDUCAM_EXT_TRI: u32 = 9967873;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageFormat {
    pub encoding: u32,
    pub quality: c_int,
}

/// Describes a rectangle
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    /// x coordinate (from left)
    pub x: i32,
    /// y coordinate (from top)
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PreviewParams {
    pub fullscreen: c_int,
    pub opacity: c_int,
    pub window: Rectangle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VideoEncoderState {
    /// Requested codec video encoding (MJPEG or H264)
    pub encoding: u32,
    /// Requested bitrate
    pub bitrate: c_int,
    /// Intra-refresh period (key frame rate)
    pub intraperiod: c_int,
    /// Quantisation parameter - quality. Set bitrate 0 and set this for variable bitrate
    pub quantisation_parameter: c_int,
    /// Insert inline headers to stream (SPS, PPS)
    pub b_inline_headers: c_int,
    /// Not working
    pub immutable_input: c_int,
    /// H264 profile to use for encoding
    pub profile: c_int,
    /// H264 level to use for encoding
    pub level: c_int,
    /// Encoder outputs inline Motion Vectors
    pub inline_motion_vectors: c_int,
    /// What intra refresh type to use. -1 to not set.
    pub intra_refresh_type: c_int,
    /// 0 or 1
    pub add_sps_timing: c_int,
    /// Horizontal slices per frame. Default 1 (off)
    pub slices: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Buffer {
    priv_: *mut c_void,
    pub data: *mut u8,
    /// Allocated size in bytes of payload buffer
    pub alloc_size: u32,
    /// Number of bytes currently used in the payload buffer (starting from offset)
    pub length: u32,
    /// Flags describing properties of a buffer header (see `bufferheaderflags`)
    pub flags: u32,
    /// Presentation timestamp in microseconds.
    ///
    /// `TIME_UNKNOWN` is used when the pts is unknown.
    pub pts: i64,
    /// Field reserved for use by the client
    pub userdata: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fract {
    pub numerator: u32,
    pub denominator: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Format {
    pub mode: c_int,
    pub width: c_int,
    pub height: c_int,
    pub pixelformat: u32,
    pub frameintervals: Fract,
    pub description: *const c_char,
    reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraCtrl {
    pub id: c_int,
    pub desc: *const c_char,
    pub max_value: c_int,
    pub min_value: c_int,
    pub default_value: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraInterface {
    pub i2c_bus: c_int,
    /// mipi interface number
    pub camera_num: c_int,
    /// enable `sda_pins[camera_num]`, disable `sda_pins[camera_num ? 0 : 1]`
    pub sda_pins: [c_int; 2],
    /// enable `scl_pins[camera_num]`, disable `scl_pins[camera_num ? 0 : 1]`
    pub scl_pins: [c_int; 2],
    pub shutdown_pins: [c_int; 2],
    pub led_pins: [c_int; 2],
}

pub type OutputCallback = unsafe extern "C" fn(buffer: *mut Buffer) -> c_int;
pub type CameraInstance = *mut c_void;

#[link(name = "arducam_mipicamera")]
extern "C" {
    /// Initialize camera.
    ///
    /// - `camera_instance`: Pointer of type CameraInstance, use to obtain CameraInstance.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_init_camera(camera_instance: *mut CameraInstance) -> c_int;

    /// Initialize camera.
    ///
    /// - `camera_instance`: Pointer of type CameraInstance, use to obtain CameraInstance.
    /// - `camera_num`: Camera interface num.
    ///
    /// Returns: error code, 0 success, !0 error.
    ///
    /// Some boards have multiple camera interfaces.
    pub fn arducam_init_camera2(
        camera_instance: *mut CameraInstance,
        cam_interface: CameraInterface,
    ) -> c_int;

    /// Set output resolution.
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `width`: Pointer of type int, Used to specify the width and return to the actual width.
    /// - `height`: Pointer of type int, Used to specify the height and return to the actual height.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_resolution(
        camera_instance: CameraInstance,
        width: *mut c_int,
        height: *mut c_int,
    ) -> c_int;

    /// Set sensor mode.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `mode`: Mode index.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_mode(camera_instance: CameraInstance, mode: c_int) -> c_int;

    /// Set lens table path.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `lens_table_path`: the lens table path you choose
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_lens_table(
        camera_instance: CameraInstance,
        lens_table_path: *mut c_char,
    ) -> c_int;

    /// Get the current Format.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `fmt`: Pointer of type struct Format, used to store Format information.
    ///
    /// Returns: ierror code, 0 success, !0 error.
    pub fn arducam_get_format(camera_instance: CameraInstance, fmt: *mut Format) -> c_int;

    /// Set video data output callback.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `encoder_state`: Used to specify encoding parameters. Use default parameters if NULL.
    /// - `callback`: Callback method, this method will be called when there is data return.
    /// - `userdata`: Userdata, which will be a member of the buffer parameter in the callback function.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_video_callback(
        camera_instance: CameraInstance,
        encoder_state: *mut VideoEncoderState,
        callback: OutputCallback,
        userdata: *mut c_void,
    ) -> c_int;

    /// Set raw data output callback.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `callback`: Callback method, this method will be called when there is data return.
    /// - `userdata`: Userdata, which will be a member of the buffer parameter in the callback function.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_raw_callback(
        camera_instance: CameraInstance,
        callback: OutputCallback,
        userdata: *mut c_void,
    ) -> c_int;

    /// Set yuv data output callback.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `callback`: Callback method, this method will be called when there is data return.
    /// - `userdata`: Userdata, which will be a member of the buffer parameter in the callback function.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_yuv_callback(
        camera_instance: CameraInstance,
        callback: OutputCallback,
        userdata: *mut c_void,
    ) -> c_int;

    /// Get single frame data.
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `Format`: The data Format to be obtained.
    /// - `timeout`: This method will return NULL if no data is obtained at this time.
    ///
    /// Returns: Buffer structure pointer containing image data.
    pub fn arducam_capture(
        camera_instance: CameraInstance,
        format: *mut ImageFormat,
        timeout: c_int,
    ) -> *mut Buffer;

    /// Used to release the memory occupied by the buffer.
    ///
    /// - `buffer`: The buffer to be released.
    pub fn arducam_release_buffer(buffer: *mut Buffer);

    /// Turn on image preview
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `preview_params`: Preview parameter,Use default parameters if NULL.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_start_preview(
        camera_instance: CameraInstance,
        preview_params: *mut PreviewParams,
    ) -> c_int;

    /// Turn on image preview
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `preview_params`: Preview parameter,Use default parameters if NULL.
    /// - `lens_table_path`: choose the table path
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_start_preview_fix_lens(
        camera_instance: CameraInstance,
        preview_params: *mut PreviewParams,
        lens_table_path: *mut c_char,
    ) -> c_int;

    /// Turn off image preview
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_stop_preview(camera_instance: CameraInstance) -> c_int;

    /// Release all resources and turn off the camera.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_close_camera(camera_instance: CameraInstance) -> c_int;

    /// Set camera control to default value.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `ctrl_id`: Control id.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_reset_control(camera_instance: CameraInstance, ctrl_id: c_int) -> c_int;

    /// Set camera control.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `ctrl_id`: Control id.
    /// - `value`: Control value.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_set_control(
        camera_instance: CameraInstance,
        ctrl_id: c_int,
        value: c_int,
    ) -> c_int;

    /// Get camera control value.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `ctrl_id`: Control id.
    /// - `value`: Current control value.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_get_control(
        camera_instance: CameraInstance,
        ctrl_id: c_int,
        value: *mut c_int,
    ) -> c_int;

    pub fn arducam_get_gain(
        camera_instance: CameraInstance,
        rgain: *mut c_int,
        bgain: *mut c_int,
    ) -> c_int;

    /// Get the resolution supported by the current camera
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `fmt`: Used to return resolution parameters.
    /// - `index`: Format list index.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_get_support_formats(
        camera_instance: CameraInstance,
        fmt: *mut Format,
        index: c_int,
    ) -> c_int;

    /// Get the control parameters supported by the current camera.
    ///
    /// - `camera_instance`: Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `cam_ctrl`: Used to return control parameters.
    /// - `index`: Control list index.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_get_support_controls(
        camera_instance: CameraInstance,
        cam_ctrl: *mut CameraCtrl,
        index: c_int,
    ) -> c_int;

    /// Write sensor register.
    ///
    /// - `camera_instance`: camera_instance Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `address`: Sensor register address.
    /// - `value`: The value you want to write
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_write_sensor_reg(
        camera_instance: CameraInstance,
        address: u16,
        value: u16,
    ) -> c_int;

    /// Read sensor register.
    ///
    /// - `camera_instance`: camera_instance Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `address`: Sensor register address.
    /// - `value`: The address of the variable that stores the result.
    ///
    /// Returns: error code, 0 success, !0 error.
    pub fn arducam_read_sensor_reg(
        camera_instance: CameraInstance,
        address: u16,
        value: *mut u16,
    ) -> c_int;

    /// Enable/Disable software auto exposure.
    ///
    /// - `camera_instance`: camera_instance Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `enable`: 0 disable, !0 enable.
    ///
    /// Returns: error code, 0 success, !0 error.
    ///
    /// Calling the arducam_set_resolution function will turn off this feature.
    pub fn arducam_software_auto_exposure(camera_instance: CameraInstance, enable: c_int) -> c_int;

    /// Enable/Disable software auto white balance.
    ///
    /// - `camera_instance`: camera_instance Type CameraInstance, Obtained from arducam_init_camera function.
    /// - `enable`: 0 disable, !0 enable.
    ///
    /// Returns: error code, 0 success, !0 error.
    ///
    /// Calling the arducam_set_resolution function will turn off this feature.
    pub fn arducam_software_auto_white_balance(
        camera_instance: CameraInstance,
        enable: c_int,
    ) -> c_int;

    /// Helper function， use to unpack mipi raw10.
    ///
    /// - `buff_in`: Raw10 data buffer.
    /// - `width`: Image width
    /// - `height`: Image height
    ///
    /// Returns: Buffer structure pointer containing image data.
    ///
    /// This function will remove the part that is filled because of the alignment,
    /// for example, the height is 1080, because the height needs 16 is the alignment,
    /// so the actual pixel height is 1088. After passing the height of 1080 using this
    /// function, the actual pixel height of the output is 1080.
    ///
    /// The performance of this function is not very good.
    pub fn arducam_unpack_raw10_to_raw8(
        buff_in: *mut u8,
        width: c_int,
        height: c_int,
    ) -> *mut Buffer;

    /// Helper function， use to unpack mipi raw10.
    ///
    /// - `buff_in`: Raw10 data buffer.
    /// - `width`: Image width
    /// - `height`: Image height
    ///
    /// Returns: Buffer structure pointer containing image data.
    ///
    /// This function will remove the part that is filled because of the alignment,
    /// for example, the height is 1080, because the height needs 16 is the alignment,
    /// so the actual pixel height is 1088. After passing the height of 1080 using this
    /// function, the actual pixel height of the output is 1080.
    ///
    /// The performance of this function is not very good.
    pub fn arducam_unpack_raw10_to_raw16(
        buff_in: *mut u8,
        width: c_int,
        height: c_int,
    ) -> *mut Buffer;

    pub fn arducam_manual_set_awb_compensation(r_gain: u32, b_gain: u32);
}
