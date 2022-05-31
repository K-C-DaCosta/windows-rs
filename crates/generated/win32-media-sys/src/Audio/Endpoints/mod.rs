#[repr(C)]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2670689126, data2: 26028, data3: 20390, data4: [138, 228, 18, 60, 120, 184, 147, 19] };
#[cfg(feature = "win32-ui-sys")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 4u32 };
#[cfg(feature = "win32-ui-sys")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 2u32 };
#[cfg(feature = "win32-ui-sys")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 1u32 };
#[cfg(feature = "win32-ui-sys")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 3u32 };
pub type EndpointConnectorType = i32;
pub const eHostProcessConnector: EndpointConnectorType = 0i32;
pub const eOffloadConnector: EndpointConnectorType = 1i32;
pub const eLoopbackConnector: EndpointConnectorType = 2i32;
pub const eKeywordDetectorConnector: EndpointConnectorType = 3i32;
pub const eConnectorCount: EndpointConnectorType = 4i32;
pub type IAudioEndpointFormatControl = *mut ::core::ffi::c_void;
pub type IAudioEndpointLastBufferControl = *mut ::core::ffi::c_void;
pub type IAudioEndpointOffloadStreamMeter = *mut ::core::ffi::c_void;
pub type IAudioEndpointOffloadStreamMute = *mut ::core::ffi::c_void;
pub type IAudioEndpointOffloadStreamVolume = *mut ::core::ffi::c_void;
pub type IAudioEndpointVolume = *mut ::core::ffi::c_void;
pub type IAudioEndpointVolumeCallback = *mut ::core::ffi::c_void;
pub type IAudioEndpointVolumeEx = *mut ::core::ffi::c_void;
pub type IAudioLfxControl = *mut ::core::ffi::c_void;
pub type IAudioMeterInformation = *mut ::core::ffi::c_void;
pub type IHardwareAudioEngineBase = *mut ::core::ffi::c_void;
