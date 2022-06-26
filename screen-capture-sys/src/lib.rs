#![allow(
  non_upper_case_globals,
  non_camel_case_types,
  non_snake_case,
  improper_ctypes
)]

use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: c_int,
  pub y: c_int
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MousePoint {
  pub Position: Point,
  pub HotSpot: Point
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Window {
  // c_ulonglong could potentially be wrong for 32 bit
  pub Handle: c_ulonglong,
  pub Position: Point,
  pub Size: Point,
  pub Name: [c_char; 128usize]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Monitor {
  pub Id: c_int,
  pub Index: c_int,
  pub Adapter: c_int,
  pub Height: c_int,
  pub Width: c_int,
  pub OriginalHeight: c_int,
  pub OriginalWidth: c_int,
  pub OffsetX: c_int,
  pub OffsetY: c_int,
  pub OriginalOffsetX: c_int,
  pub OriginalOffsetY: c_int,
  pub Name: [c_char; 128usize],
  pub Scaling: f32
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageRect {
  pub left: c_int,
  pub top: c_int,
  pub right: c_int,
  pub bottom: c_int
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageBGRA {
  pub B: c_uchar,
  pub G: c_uchar,
  pub R: c_uchar,
  pub A: c_uchar
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
  pub Bounds: ImageRect,
  pub RowStrideInBytes: c_int,
  pub isContiguous: bool,
  pub Data: *const ImageBGRA
}

pub type SCL_IScreenCaptureManagerWrapperRef = *mut c_void;
pub type SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef = *mut c_void;
pub type SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef = *mut c_void;

pub type SCL_ScreenCaptureCallback = Option<
  unsafe extern "C" fn(img: *const Image, monitor: *const Monitor) -> c_int
>;

pub type SCL_ScreenCaptureCallbackWithContext = Option<
  unsafe extern "C" fn(
    img: *const Image,
    monitor: *const Monitor,
    context: *mut c_void
  ) -> c_int
>;

pub type SCL_MouseCaptureCallback = Option<
  unsafe extern "C" fn(img: *const Image, mouse: *const MousePoint) -> c_int
>;

pub type SCL_MouseCaptureCallbackWithContext = Option<
  unsafe extern "C" fn(
    img: *const Image,
    mouse: *const MousePoint,
    context: *mut c_void
  ) -> c_int
>;

pub type SCL_WindowCaptureCallback = Option<
  unsafe extern "C" fn(img: *const Image, monitor: *const Window) -> c_int
>;

pub type SCL_WindowCaptureCallbackWithContext = Option<
  unsafe extern "C" fn(
    img: *const Image,
    monitor: *const Window,
    context: *mut c_void
  ) -> c_int
>;

pub type SCL_WindowCallback = Option<
  unsafe extern "C" fn(buffer: *mut Window, buffersize: c_int) -> c_int
>;

pub type SCL_MonitorCallback = Option<
  unsafe extern "C" fn(buffer: *mut Monitor, buffersize: c_int) -> c_int
>;

pub type SCL_WindowCallbackWithContext = Option<
  unsafe extern "C" fn(buffer: *mut Window, buffersize: c_int, context: *mut c_void) -> c_int
>;

pub type SCL_MonitorCallbackWithContext = Option<
  unsafe extern "C" fn(buffer: *mut Monitor, buffersize: c_int, context: *mut c_void) -> c_int
>;

extern "C" {
  pub fn SCL_GetWindows(windows: *mut Window, windows_size: c_int) -> c_int;

  pub fn SCL_GetMonitors(monitors: *mut Monitor, monitors_size: c_int) -> c_int;

  pub fn SCL_IsMonitorInsideBounds(
    monitors: *mut Monitor,
    monitorsize: c_int,
    monitor: *mut Monitor
  ) -> c_int;

  pub fn SCL_MonitorOnNewFrame(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef,
    cb: SCL_ScreenCaptureCallback
  );

  pub fn SCL_MonitorOnNewFrameWithContext(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef,
    cb: SCL_ScreenCaptureCallbackWithContext
  );

  pub fn SCL_MonitorOnFrameChanged(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef,
    cb: SCL_ScreenCaptureCallback
  );

  pub fn SCL_MonitorOnFrameChangedWithContext(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef,
    cb: SCL_ScreenCaptureCallbackWithContext
  );

  pub fn SCL_MonitorOnMouseChanged(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef,
    cb: SCL_MouseCaptureCallback
  );

  pub fn SCL_MonitorOnMouseChangedWithContext(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef,
    cb: SCL_MouseCaptureCallbackWithContext
  );

  pub fn SCL_MonitorStartCapturing(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef
  ) -> SCL_IScreenCaptureManagerWrapperRef;

  pub fn SCL_FreeIScreenCaptureManagerWrapper(ptr: SCL_IScreenCaptureManagerWrapperRef);

  pub fn SCL_SetFrameChangeInterval(ptr: SCL_IScreenCaptureManagerWrapperRef, milliseconds: c_int);

  pub fn SCL_SetMouseChangeInterval(ptr: SCL_IScreenCaptureManagerWrapperRef, milliseconds: c_int);

  pub fn SCL_PauseCapturing(ptr: SCL_IScreenCaptureManagerWrapperRef);

  pub fn SCL_IsPaused(ptr: SCL_IScreenCaptureManagerWrapperRef) -> c_int;

  pub fn SCL_Resume(ptr: SCL_IScreenCaptureManagerWrapperRef);

  pub fn SCL_WindowOnNewFrame(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef,
    cb: SCL_WindowCaptureCallback
  );

  pub fn SCL_WindowOnNewFrameWithContext(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef,
    cb: SCL_WindowCaptureCallbackWithContext
  );

  pub fn SCL_WindowOnFrameChanged(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef,
    cb: SCL_WindowCaptureCallback
  );

  pub fn SCL_WindowOnFrameChangedWithContext(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef,
    cb: SCL_WindowCaptureCallbackWithContext
  );

  pub fn SCL_WindowOnMouseChanged(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef,
    cb: SCL_MouseCaptureCallback
  );

  pub fn SCL_WindowOnMouseChangedWithContext(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef,
    cb: SCL_MouseCaptureCallbackWithContext
  );

  pub fn SCL_WindowStartCapturing(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef
  ) -> SCL_IScreenCaptureManagerWrapperRef;

  pub fn SCL_Utility_CopyToContiguous(
    destination: *mut c_uchar,
    image: *const Image
  ) -> *mut c_uchar;

  pub fn SCL_CreateMonitorCaptureConfiguration(
    monitorstocapture: SCL_MonitorCallback
  ) -> SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef;

  pub fn SCL_CreateMonitorCaptureConfigurationWithContext(
    monitorstocapture: SCL_MonitorCallbackWithContext,
    context: *mut c_void
  ) -> SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef;

  pub fn SCL_FreeMonitorCaptureConfiguration(
    ptr: SCL_ICaptureConfigurationScreenCaptureCallbackWrapperRef
  );

  pub fn SCL_CreateWindowCaptureConfiguration(
    windowstocapture: SCL_WindowCallback
  ) -> SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef;

  pub fn SCL_CreateWindowCaptureConfigurationWithContext(
    windowstocapture: SCL_WindowCallbackWithContext,
    context: *mut c_void
  ) -> SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef;

  pub fn SCL_FreeWindowCaptureConfiguration(
    ptr: SCL_ICaptureConfigurationWindowCaptureCallbackWrapperRef
  );
}
