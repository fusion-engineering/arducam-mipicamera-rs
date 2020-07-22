//! Rust wrapper for the [ArduCAM MIPI Camera library](https://github.com/ArduCAM/MIPI_Camera/tree/master/RPI).

pub mod c;

use std::mem::MaybeUninit;

/// Interface to a camera.
pub struct Camera {
    ptr: c::CameraInstance,
}

/// Buffer returned by [`Camera::capture`].
pub struct Buffer {
    ptr: *mut c::Buffer,
}

pub use c::CameraInterface;
pub use c::Format;
pub use c::Fract;

// TODO: Wrap Format struct:
//   pub struct Format {
//       mode: i32,
//       width: i32,
//       height: i32,
//       pixel_format: ???,
//       frame_intervals: ???,
//       description: &'static CStr,
//   }

/// Image encoding format.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Encoding {
    I420 = c::IMAGE_ENCODING_I420,
    Jpeg = c::IMAGE_ENCODING_JPEG,
    RawBayer = c::IMAGE_ENCODING_RAW_BAYER,
    Bmp = c::IMAGE_ENCODING_BMP,
    Png = c::IMAGE_ENCODING_PNG,
}

impl Camera {
    /// Initialize a camera.
    ///
    /// Optionally, specific interface settings can be given.
    pub fn init(interface: Option<CameraInterface>) -> Result<Self, ()> {
        let mut ptr: c::CameraInstance = std::ptr::null_mut();
        let r = if let Some(interface) = interface {
            unsafe { c::arducam_init_camera2(&mut ptr, interface) }
        } else {
            unsafe { c::arducam_init_camera(&mut ptr) }
        };
        if r == 0 {
            Ok(Self { ptr })
        } else {
            Err(())
        }
    }

    /// Set the output resolution.
    pub fn set_resolution(&mut self, mut width: i32, mut height: i32) -> Result<(i32, i32), ()> {
        unsafe {
            to_result(c::arducam_set_resolution(self.ptr, &mut width, &mut height))?;
            Ok((width, height))
        }
    }

    /// Set the mode of the sensor.
    pub fn set_mode(&mut self, mode: i32) -> Result<(), ()> {
        unsafe { to_result(c::arducam_set_mode(self.ptr, mode)) }
    }

    /// Get the current output format.
    pub fn get_format(&mut self) -> Result<Format, ()> {
        unsafe {
            let mut format = MaybeUninit::uninit();
            to_result(c::arducam_get_format(self.ptr, format.as_mut_ptr()))?;
            Ok(format.assume_init())
        }
    }

    /// Capture a single frame.
    pub fn capture(
        &mut self,
        timeout: i32,
        encoding: Encoding,
        quality: i32,
    ) -> Result<Buffer, ()> {
        unsafe {
            let mut format = c::ImageFormat {
                encoding: encoding as u32,
                quality,
            };
            let buffer = c::arducam_capture(self.ptr, &mut format, timeout);
            if buffer.is_null() {
                return Err(());
            }
            Ok(Buffer::from_raw_pointer(buffer))
        }
    }

    /// Set a camera control to default value.
    pub fn reset_control(&mut self, ctrl_id: i32) -> Result<(), ()> {
        unsafe { to_result(c::arducam_reset_control(self.ptr, ctrl_id)) }
    }

    /// Set a camera control to the specified value.
    pub fn set_control(&mut self, ctrl_id: i32, value: i32) -> Result<(), ()> {
        unsafe { to_result(c::arducam_set_control(self.ptr, ctrl_id, value)) }
    }

    /// Read the current value of a camera control.
    pub fn get_control(&mut self, ctrl_id: i32) -> Result<i32, ()> {
        unsafe {
            let mut value = 0;
            to_result(c::arducam_get_control(self.ptr, ctrl_id, &mut value))?;
            Ok(value)
        }
    }

    /// Enable or disable software auto exposure.
    pub fn arducam_software_auto_exposure(&mut self, enable: bool) -> Result<(), ()> {
        unsafe { to_result(c::arducam_software_auto_exposure(self.ptr, enable as i32)) }
    }

    /// Enable or disable software auto white balance.
    pub fn arducam_software_auto_white_balance(&mut self, enable: bool) -> Result<(), ()> {
        unsafe {
            to_result(c::arducam_software_auto_white_balance(
                self.ptr,
                enable as i32,
            ))
        }
    }

    /// The raw pointer to the camera instance, as used by the C interface.
    pub fn raw_pointer(&self) -> c::CameraInstance {
        self.ptr
    }

    // TODO:
    //  - start_preview
    //  - stop_preview
    //  - set_video_callback
    //  - set_raw_callback
    //  - get_support_formats
    //  - get_support_controls
    //  - manual_set_awb_compensation
    //  - read_sensor_reg
    //  - write_sensor_reg
    //  - set_lens_table
    //  - set_yuv_callback
    //  - start_preview_fix_lens
    //  - get_gain
}

impl Drop for Camera {
    fn drop(&mut self) {
        unsafe { to_result(c::arducam_close_camera(self.ptr)).unwrap() };
    }
}

impl Buffer {
    /// The data contained in the buffer.
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.raw_buffer().data, self.raw_buffer().length as usize)
        }
    }

    /// The presentation timestamp of the buffer.
    ///
    /// Returns `None` when the buffer timestamp is set to `TIME_UNKNOWN`.
    pub fn timestamp(&self) -> Option<i64> {
        match self.raw_buffer().pts {
            c::TIME_UNKNOWN => None,
            t => Some(t),
        }
    }

    /// Access to the raw [`Buffer`][c::Buffer] structure.
    pub fn raw_buffer(&self) -> &c::Buffer {
        unsafe { &*self.ptr }
    }

    /// Mutable access to the raw [`Buffer`][c::Buffer] structure.
    pub unsafe fn raw_buffer_mut(&mut self) -> &mut c::Buffer {
        &mut *self.ptr
    }

    /// The raw pointer to the [`Buffer`][c::Buffer] structure, as used by the C interface.
    pub fn raw_pointer(&self) -> *mut c::Buffer {
        self.ptr
    }

    /// Take ownership of a raw [`Buffer`][c::Buffer].
    ///
    /// The pointer must be valid and non-null.
    pub unsafe fn from_raw_pointer(ptr: *mut c::Buffer) -> Self {
        Self { ptr }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { c::arducam_release_buffer(self.ptr) };
    }
}

fn to_result(r: i32) -> Result<(), ()> {
    if r == 0 {
        Ok(())
    } else {
        Err(())
    }
}
