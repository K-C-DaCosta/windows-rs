#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
pub type AdvancedPhotoCaptureSettings = *mut ::core::ffi::c_void;
pub type AdvancedPhotoControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Hdr: Self = Self(2i32);
    pub const LowLight: Self = Self(3i32);
}
impl ::core::marker::Copy for AdvancedPhotoMode {}
impl ::core::clone::Clone for AdvancedPhotoMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioDeviceController = *mut ::core::ffi::c_void;
pub type AudioDeviceModule = *mut ::core::ffi::c_void;
pub type AudioDeviceModuleNotificationEventArgs = *mut ::core::ffi::c_void;
pub type AudioDeviceModulesManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioDeviceRole {}
impl ::core::clone::Clone for AudioDeviceRole {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: Self = Self(0i32);
    pub const Macro: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoFocusRange {}
impl ::core::clone::Clone for AutoFocusRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CallControl = *mut ::core::ffi::c_void;
pub type CallControlEventHandler = *mut ::core::ffi::c_void;
pub type CameraOcclusionInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: Self = Self(0i32);
    pub const CameraHardware: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraOcclusionKind {}
impl ::core::clone::Clone for CameraOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CameraOcclusionState = *mut ::core::ffi::c_void;
pub type CameraOcclusionStateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: Self = Self(0i32);
    pub const Streaming: Self = Self(1i32);
    pub const BlockedForPrivacy: Self = Self(2i32);
    pub const Shutdown: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraStreamState {}
impl ::core::clone::Clone for CameraStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Macro: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const Sport: Self = Self(4i32);
    pub const Snow: Self = Self(5i32);
    pub const Night: Self = Self(6i32);
    pub const Beach: Self = Self(7i32);
    pub const Sunset: Self = Self(8i32);
    pub const Candlelight: Self = Self(9i32);
    pub const Landscape: Self = Self(10i32);
    pub const NightPortrait: Self = Self(11i32);
    pub const Backlit: Self = Self(12i32);
}
impl ::core::marker::Copy for CaptureSceneMode {}
impl ::core::clone::Clone for CaptureSceneMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CaptureUse {}
impl ::core::clone::Clone for CaptureUse {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Cloudy: Self = Self(2i32);
    pub const Daylight: Self = Self(3i32);
    pub const Flash: Self = Self(4i32);
    pub const Fluorescent: Self = Self(5i32);
    pub const Tungsten: Self = Self(6i32);
    pub const Candlelight: Self = Self(7i32);
}
impl ::core::marker::Copy for ColorTemperaturePreset {}
impl ::core::clone::Clone for ColorTemperaturePreset {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DefaultAudioCaptureDeviceChangedEventArgs = *mut ::core::ffi::c_void;
pub type DefaultAudioRenderDeviceChangedEventArgs = *mut ::core::ffi::c_void;
pub type DialRequestedEventArgs = *mut ::core::ffi::c_void;
pub type DialRequestedEventHandler = *mut ::core::ffi::c_void;
pub type DigitalWindowBounds = *mut ::core::ffi::c_void;
pub type DigitalWindowCapability = *mut ::core::ffi::c_void;
pub type DigitalWindowControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for DigitalWindowMode {}
impl ::core::clone::Clone for DigitalWindowMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExposureCompensationControl = *mut ::core::ffi::c_void;
pub type ExposureControl = *mut ::core::ffi::c_void;
pub type ExposurePriorityVideoControl = *mut ::core::ffi::c_void;
pub type FlashControl = *mut ::core::ffi::c_void;
pub type FocusControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Continuous: Self = Self(2i32);
    pub const Manual: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusMode {}
impl ::core::clone::Clone for FocusMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const AutoMacro: Self = Self(2i32);
    pub const AutoNormal: Self = Self(3i32);
    pub const AutoInfinity: Self = Self(4i32);
    pub const AutoHyperfocal: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusPreset {}
impl ::core::clone::Clone for FocusPreset {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FocusSettings = *mut ::core::ffi::c_void;
pub type HdrVideoControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for HdrVideoMode {}
impl ::core::clone::Clone for HdrVideoMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAdvancedPhotoCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedPhotoControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDeviceProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, propertyvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController10 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CameraOcclusionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LowLagPhotoSequence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LowLagPhoto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SceneModeControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TorchControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlashControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WhiteBalanceControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegionsOfInterestControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CaptureUse) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryUse: unsafe extern "system" fn(this: *mut *mut Self, value: CaptureUse) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Devices_Core")]
    pub VariablePhotoSequenceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    VariablePhotoSequenceController: usize,
    pub PhotoConfirmationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExposurePriorityVideoControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DesiredOptimization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureOptimization) -> ::windows_sys::core::HRESULT,
    pub SetDesiredOptimization: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCaptureOptimization) -> ::windows_sys::core::HRESULT,
    pub HdrVideoControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpticalImageStabilizationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AdvancedPhotoControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyById: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyById: usize,
    pub SetDevicePropertyById: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, propertyvalue: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut *mut Self, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyByExtendedId: usize,
    pub SetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut *mut Self, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoTemporalDenoisingControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InfraredTorchControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PanelBasedOptimizationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DigitalWindowControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMuted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Muted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVolumePercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub VolumePercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioDeviceModule {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendCommandAsync: unsafe extern "system" fn(this: *mut *mut Self, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendCommandAsync: usize,
}
#[repr(C)]
pub struct IAudioDeviceModuleNotificationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Module: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub NotificationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    NotificationData: usize,
}
#[repr(C)]
pub struct IAudioDeviceModulesManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ModuleNotificationReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveModuleNotificationReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllById: unsafe extern "system" fn(this: *mut *mut Self, moduleid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllById: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
}
#[repr(C)]
pub struct IAudioDeviceModulesManagerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICallControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IndicateNewIncomingCall: unsafe extern "system" fn(this: *mut *mut Self, enableringer: bool, callerid: ::windows_sys::core::HSTRING, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IndicateNewOutgoingCall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IndicateActiveCall: unsafe extern "system" fn(this: *mut *mut Self, calltoken: u64) -> ::windows_sys::core::HRESULT,
    pub EndCall: unsafe extern "system" fn(this: *mut *mut Self, calltoken: u64) -> ::windows_sys::core::HRESULT,
    pub HasRinger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HangUpRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHangUpRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DialRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDialRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RedialRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRedialRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub KeypadPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeypadPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub AudioTransferRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioTransferRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioTransferRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioTransferRequested: usize,
}
#[repr(C)]
pub struct ICallControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICameraOcclusionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOcclusionKindSupported: unsafe extern "system" fn(this: *mut *mut Self, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[repr(C)]
pub struct ICameraOcclusionState {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOccluded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOcclusionKind: unsafe extern "system" fn(this: *mut *mut Self, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICameraOcclusionStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDefaultAudioDeviceChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDeviceRole) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDialRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDigitalWindowBounds {
    pub base__: ::windows_sys::core::IInspectable,
    pub NormalizedOriginTop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNormalizedOriginTop: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub NormalizedOriginLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNormalizedOriginLeft: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDigitalWindowCapability {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinScaleValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxScaleValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinScaleValueWithoutUpsampling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedFieldOfViewLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedFieldOfViewLimit: usize,
}
#[repr(C)]
pub struct IDigitalWindowControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows_sys::core::HRESULT,
    pub CurrentMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DigitalWindowMode) -> ::windows_sys::core::HRESULT,
    pub GetBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, digitalwindowmode: DigitalWindowMode) -> ::windows_sys::core::HRESULT,
    pub ConfigureWithBounds: unsafe extern "system" fn(this: *mut *mut Self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCapabilities: usize,
    pub GetCapabilityForSize: unsafe extern "system" fn(this: *mut *mut Self, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExposureCompensationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[repr(C)]
pub struct IExposureControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, shutterduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[repr(C)]
pub struct IExposurePriorityVideoControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlashControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlashControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AssistantLightSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AssistantLightEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAssistantLightEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPresets: usize,
    pub Preset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusPreset) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: FocusPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPresetWithCompletionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: FocusPreset, completebeforefocus: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetWithCompletionOptionAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, focus: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusAsync: usize,
}
#[repr(C)]
pub struct IFocusControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FocusChangedSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub WaitForFocusSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusModes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusDistances: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusDistances: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusRanges: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusMode) -> ::windows_sys::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureFocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnlockAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LockAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockAsync: usize,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: FocusMode) -> ::windows_sys::core::HRESULT,
    pub AutoFocusRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutoFocusRange) -> ::windows_sys::core::HRESULT,
    pub SetAutoFocusRange: unsafe extern "system" fn(this: *mut *mut Self, value: AutoFocusRange) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Foundation")]
    pub Distance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Distance: usize,
    #[cfg(feature = "Foundation")]
    pub SetDistance: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDistance: usize,
    pub WaitForFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetWaitForFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisableDriverFallback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDisableDriverFallback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHdrVideoControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HdrVideoMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: HdrVideoMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInfraredTorchControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub CurrentMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InfraredTorchMode) -> ::windows_sys::core::HRESULT,
    pub SetCurrentMode: unsafe extern "system" fn(this: *mut *mut Self, value: InfraredTorchMode) -> ::windows_sys::core::HRESULT,
    pub MinPower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxPower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PowerStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Power: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPower: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIsoSpeedControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedPresets: usize,
    #[cfg(feature = "deprecated")]
    pub Preset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsoSpeedPreset) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Preset: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: IsoSpeedPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPresetAsync: usize,
}
#[repr(C)]
pub struct IIsoSpeedControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, isospeed: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
}
#[repr(C)]
pub struct IKeypadPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TelephonyKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TelephonyKey) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILowLagPhotoControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILowLagPhotoSequenceControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxPastPhotos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub PastPhotoLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPastPhotoLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaDeviceControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryGetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: *mut bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaDeviceControlCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Default: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AutoModeSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetAvailableMediaStreamProperties: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetAvailableMediaStreamProperties: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetMediaStreamProperties: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetMediaStreamProperties: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub SetMediaStreamPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    SetMediaStreamPropertiesAsync: usize,
}
#[repr(C)]
pub struct IMediaDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAudioCaptureSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAudioRenderSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetVideoCaptureSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDefaultAudioCaptureId: unsafe extern "system" fn(this: *mut *mut Self, role: AudioDeviceRole, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDefaultAudioRenderId: unsafe extern "system" fn(this: *mut *mut Self, role: AudioDeviceRole, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioRenderDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioRenderDeviceChanged: usize,
}
#[repr(C)]
pub struct IModuleCommandResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SendCommandStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
}
#[repr(C)]
pub struct IOpticalImageStabilizationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut OpticalImageStabilizationMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: OpticalImageStabilizationMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPanelBasedOptimizationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Panel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Panel: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPanel: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPanel: usize,
}
#[repr(C)]
pub struct IPhotoConfirmationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PixelFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, format: super::MediaProperties::MediaPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPixelFormat: usize,
}
#[repr(C)]
pub struct IRedialRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRegionOfInterest {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoFocusEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoFocusEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoExposureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
}
#[repr(C)]
pub struct IRegionOfInterest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RegionOfInterestType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: RegionOfInterestType) -> ::windows_sys::core::HRESULT,
    pub BoundsNormalized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBoundsNormalized: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRegionsOfInterestControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxRegions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsAsync: unsafe extern "system" fn(this: *mut *mut Self, regions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsWithLockAsync: unsafe extern "system" fn(this: *mut *mut Self, regions: *mut ::core::ffi::c_void, lockvalues: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsWithLockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearRegionsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearRegionsAsync: usize,
    pub AutoFocusSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoWhiteBalanceSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoExposureSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneModeControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CaptureSceneMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, scenemode: CaptureSceneMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[repr(C)]
pub struct ITorchControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Brightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Hue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BacklightCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Tilt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Zoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub TrySetPowerlineFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TrySetPowerlineFrequency: usize,
    #[cfg(feature = "Media_Capture")]
    pub TryGetPowerlineFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryGetPowerlineFrequency: usize,
}
#[repr(C)]
pub struct IVideoDeviceControllerGetDevicePropertyResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoTemporalDenoisingControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoTemporalDenoisingMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: VideoTemporalDenoisingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWhiteBalanceControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Preset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ColorTemperaturePreset) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: ColorTemperaturePreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, temperature: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[repr(C)]
pub struct IZoomControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IZoomControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ZoomTransitionMode) -> ::windows_sys::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IZoomSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ZoomTransitionMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: ZoomTransitionMode) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
pub type InfraredTorchControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const AlternatingFrameIllumination: Self = Self(2i32);
}
impl ::core::marker::Copy for InfraredTorchMode {}
impl ::core::clone::Clone for InfraredTorchMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsoSpeedControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IsoSpeedPreset(pub i32);
#[cfg(feature = "deprecated")]
impl IsoSpeedPreset {
    pub const Auto: Self = Self(0i32);
    pub const Iso50: Self = Self(1i32);
    pub const Iso80: Self = Self(2i32);
    pub const Iso100: Self = Self(3i32);
    pub const Iso200: Self = Self(4i32);
    pub const Iso400: Self = Self(5i32);
    pub const Iso800: Self = Self(6i32);
    pub const Iso1600: Self = Self(7i32);
    pub const Iso3200: Self = Self(8i32);
    pub const Iso6400: Self = Self(9i32);
    pub const Iso12800: Self = Self(10i32);
    pub const Iso25600: Self = Self(11i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for IsoSpeedPreset {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IsoSpeedPreset {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeypadPressedEventArgs = *mut ::core::ffi::c_void;
pub type KeypadPressedEventHandler = *mut ::core::ffi::c_void;
pub type LowLagPhotoControl = *mut ::core::ffi::c_void;
pub type LowLagPhotoSequenceControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: Self = Self(0i32);
    pub const Hyperfocal: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for ManualFocusDistance {}
impl ::core::clone::Clone for ManualFocusDistance {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: Self = Self(0i32);
    pub const Lost: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Focused: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaCaptureFocusState {}
impl ::core::clone::Clone for MediaCaptureFocusState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: Self = Self(0i32);
    pub const Quality: Self = Self(1i32);
    pub const Latency: Self = Self(2i32);
    pub const Power: Self = Self(3i32);
    pub const LatencyThenQuality: Self = Self(4i32);
    pub const LatencyThenPower: Self = Self(5i32);
    pub const PowerAndQuality: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaCaptureOptimization {}
impl ::core::clone::Clone for MediaCaptureOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: Self = Self(0i32);
    pub const ReleaseHardwareResources: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCapturePauseBehavior {}
impl ::core::clone::Clone for MediaCapturePauseBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaDeviceControl = *mut ::core::ffi::c_void;
pub type MediaDeviceControlCapabilities = *mut ::core::ffi::c_void;
pub type ModuleCommandResult = *mut ::core::ffi::c_void;
pub type OpticalImageStabilizationControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for OpticalImageStabilizationMode {}
impl ::core::clone::Clone for OpticalImageStabilizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PanelBasedOptimizationControl = *mut ::core::ffi::c_void;
pub type PhotoConfirmationControl = *mut ::core::ffi::c_void;
pub type RedialRequestedEventArgs = *mut ::core::ffi::c_void;
pub type RedialRequestedEventHandler = *mut ::core::ffi::c_void;
pub type RegionOfInterest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: Self = Self(0i32);
    pub const Face: Self = Self(1i32);
}
impl ::core::marker::Copy for RegionOfInterestType {}
impl ::core::clone::Clone for RegionOfInterestType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RegionsOfInterestControl = *mut ::core::ffi::c_void;
pub type SceneModeControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for SendCommandStatus {}
impl ::core::clone::Clone for SendCommandStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
    pub const A: Self = Self(12i32);
    pub const B: Self = Self(13i32);
    pub const C: Self = Self(14i32);
    pub const D: Self = Self(15i32);
}
impl ::core::marker::Copy for TelephonyKey {}
impl ::core::clone::Clone for TelephonyKey {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TorchControl = *mut ::core::ffi::c_void;
pub type VideoDeviceController = *mut ::core::ffi::c_void;
pub type VideoDeviceControllerGetDevicePropertyResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerGetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const BufferTooSmall: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(5i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for VideoDeviceControllerGetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for VideoDeviceControllerSetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerSetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VideoTemporalDenoisingControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoTemporalDenoisingMode {}
impl ::core::clone::Clone for VideoTemporalDenoisingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WhiteBalanceControl = *mut ::core::ffi::c_void;
pub type ZoomControl = *mut ::core::ffi::c_void;
pub type ZoomSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: Self = Self(0i32);
    pub const Direct: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl ::core::marker::Copy for ZoomTransitionMode {}
impl ::core::clone::Clone for ZoomTransitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
