#[link(name = "windows")]
extern "system" {
    pub fn EnableMouseInPointer(fenable: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-ui-sys"))]
    pub fn GetPointerDevice(device: ::win32_foundation_sys::HANDLE, pointerdevice: *mut super::super::Controls::POINTER_DEVICE_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerDeviceCursors(device: ::win32_foundation_sys::HANDLE, cursorcount: *mut u32, devicecursors: *mut super::super::Controls::POINTER_DEVICE_CURSOR_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerDeviceProperties(device: ::win32_foundation_sys::HANDLE, propertycount: *mut u32, pointerproperties: *mut super::super::Controls::POINTER_DEVICE_PROPERTY) -> ::win32_foundation_sys::BOOL;
    pub fn GetPointerDeviceRects(device: ::win32_foundation_sys::HANDLE, pointerdevicerect: *mut ::win32_foundation_sys::RECT, displayrect: *mut ::win32_foundation_sys::RECT) -> ::win32_foundation_sys::BOOL;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-ui-sys"))]
    pub fn GetPointerDevices(devicecount: *mut u32, pointerdevices: *mut super::super::Controls::POINTER_DEVICE_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: *mut POINTER_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn GetPointerInputTransform(pointerid: u32, historycount: u32, inputtransform: *mut INPUT_TRANSFORM) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetPointerType(pointerid: u32, pointertype: *mut super::super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, propertiescount: u32, pproperties: *const super::super::Controls::POINTER_DEVICE_PROPERTY, pvalues: *mut i32) -> ::win32_foundation_sys::BOOL;
    pub fn GetUnpredictedMessagePos() -> u32;
    pub fn InitializeTouchInjection(maxcount: u32, dwmode: TOUCH_FEEDBACK_MODE) -> ::win32_foundation_sys::BOOL;
    #[cfg(all(feature = "win32-ui-sys", feature = "win32-ui-sys"))]
    pub fn InjectSyntheticPointerInput(device: super::super::Controls::HSYNTHETICPOINTERDEVICE, pointerinfo: *const super::super::Controls::POINTER_TYPE_INFO, count: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-ui-sys")]
    pub fn InjectTouchInput(count: u32, contacts: *const POINTER_TOUCH_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn IsMouseInPointerEnabled() -> ::win32_foundation_sys::BOOL;
    pub fn SkipPointerFrameMessages(pointerid: u32) -> ::win32_foundation_sys::BOOL;
}
#[repr(C)]
pub struct INPUT_INJECTION_VALUE {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
impl ::core::marker::Copy for INPUT_INJECTION_VALUE {}
impl ::core::clone::Clone for INPUT_INJECTION_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INPUT_TRANSFORM {
    pub Anonymous: INPUT_TRANSFORM_0,
}
impl ::core::marker::Copy for INPUT_TRANSFORM {}
impl ::core::clone::Clone for INPUT_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union INPUT_TRANSFORM_0 {
    pub Anonymous: INPUT_TRANSFORM_0_0,
    pub m: [f32; 16],
}
impl ::core::marker::Copy for INPUT_TRANSFORM_0 {}
impl ::core::clone::Clone for INPUT_TRANSFORM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INPUT_TRANSFORM_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl ::core::marker::Copy for INPUT_TRANSFORM_0_0 {}
impl ::core::clone::Clone for INPUT_TRANSFORM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type POINTER_BUTTON_CHANGE_TYPE = i32;
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = 0i32;
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 1i32;
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 2i32;
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 3i32;
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 4i32;
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 5i32;
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 6i32;
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 7i32;
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 8i32;
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 9i32;
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 10i32;
pub type POINTER_FLAGS = u32;
pub const POINTER_FLAG_NONE: POINTER_FLAGS = 0u32;
pub const POINTER_FLAG_NEW: POINTER_FLAGS = 1u32;
pub const POINTER_FLAG_INRANGE: POINTER_FLAGS = 2u32;
pub const POINTER_FLAG_INCONTACT: POINTER_FLAGS = 4u32;
pub const POINTER_FLAG_FIRSTBUTTON: POINTER_FLAGS = 16u32;
pub const POINTER_FLAG_SECONDBUTTON: POINTER_FLAGS = 32u32;
pub const POINTER_FLAG_THIRDBUTTON: POINTER_FLAGS = 64u32;
pub const POINTER_FLAG_FOURTHBUTTON: POINTER_FLAGS = 128u32;
pub const POINTER_FLAG_FIFTHBUTTON: POINTER_FLAGS = 256u32;
pub const POINTER_FLAG_PRIMARY: POINTER_FLAGS = 8192u32;
pub const POINTER_FLAG_CONFIDENCE: POINTER_FLAGS = 16384u32;
pub const POINTER_FLAG_CANCELED: POINTER_FLAGS = 32768u32;
pub const POINTER_FLAG_DOWN: POINTER_FLAGS = 65536u32;
pub const POINTER_FLAG_UPDATE: POINTER_FLAGS = 131072u32;
pub const POINTER_FLAG_UP: POINTER_FLAGS = 262144u32;
pub const POINTER_FLAG_WHEEL: POINTER_FLAGS = 524288u32;
pub const POINTER_FLAG_HWHEEL: POINTER_FLAGS = 1048576u32;
pub const POINTER_FLAG_CAPTURECHANGED: POINTER_FLAGS = 2097152u32;
pub const POINTER_FLAG_HASTRANSFORM: POINTER_FLAGS = 4194304u32;
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct POINTER_INFO {
    pub pointerType: super::super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: POINTER_FLAGS,
    pub sourceDevice: ::win32_foundation_sys::HANDLE,
    pub hwndTarget: ::win32_foundation_sys::HWND,
    pub ptPixelLocation: ::win32_foundation_sys::POINT,
    pub ptHimetricLocation: ::win32_foundation_sys::POINT,
    pub ptPixelLocationRaw: ::win32_foundation_sys::POINT,
    pub ptHimetricLocationRaw: ::win32_foundation_sys::POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for POINTER_INFO {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for POINTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct POINTER_PEN_INFO {
    pub pointerInfo: POINTER_INFO,
    pub penFlags: u32,
    pub penMask: u32,
    pub pressure: u32,
    pub rotation: u32,
    pub tiltX: i32,
    pub tiltY: i32,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for POINTER_PEN_INFO {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for POINTER_PEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct POINTER_TOUCH_INFO {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: u32,
    pub touchMask: u32,
    pub rcContact: ::win32_foundation_sys::RECT,
    pub rcContactRaw: ::win32_foundation_sys::RECT,
    pub orientation: u32,
    pub pressure: u32,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for POINTER_TOUCH_INFO {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for POINTER_TOUCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOUCH_FEEDBACK_MODE = u32;
pub const TOUCH_FEEDBACK_DEFAULT: TOUCH_FEEDBACK_MODE = 1u32;
pub const TOUCH_FEEDBACK_INDIRECT: TOUCH_FEEDBACK_MODE = 2u32;
pub const TOUCH_FEEDBACK_NONE: TOUCH_FEEDBACK_MODE = 3u32;
