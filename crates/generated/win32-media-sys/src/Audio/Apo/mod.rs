pub const APOERR_ALREADY_INITIALIZED: ::windows_core_sys::HRESULT = -2005073919i32;
pub const APOERR_ALREADY_UNLOCKED: ::windows_core_sys::HRESULT = -2005073914i32;
pub const APOERR_APO_LOCKED: ::windows_core_sys::HRESULT = -2005073910i32;
pub const APOERR_BUFFERS_OVERLAP: ::windows_core_sys::HRESULT = -2005073915i32;
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2005073917i32;
pub const APOERR_INVALID_APO_CLSID: ::windows_core_sys::HRESULT = -2005073916i32;
pub const APOERR_INVALID_COEFFCOUNT: ::windows_core_sys::HRESULT = -2005073909i32;
pub const APOERR_INVALID_COEFFICIENT: ::windows_core_sys::HRESULT = -2005073908i32;
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows_core_sys::HRESULT = -2005073911i32;
pub const APOERR_INVALID_CURVE_PARAM: ::windows_core_sys::HRESULT = -2005073907i32;
pub const APOERR_INVALID_INPUTID: ::windows_core_sys::HRESULT = -2005073906i32;
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows_core_sys::HRESULT = -2005073912i32;
pub const APOERR_NOT_INITIALIZED: ::windows_core_sys::HRESULT = -2005073918i32;
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows_core_sys::HRESULT = -2005073913i32;
#[repr(C)]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: super::IMMDeviceCollection,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for APOInitSystemEffects {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_core_sys::GUID,
    pub InitializeForDiscoveryOnly: ::win32_foundation_sys::BOOL,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for APOInitSystemEffects2 {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "win32-system-sys", feature = "win32-ui-sys"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub pServiceProvider: ::win32_system_sys::Com::IServiceProvider,
    pub pDeviceCollection: super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_core_sys::GUID,
    pub InitializeForDiscoveryOnly: ::win32_foundation_sys::BOOL,
}
#[cfg(all(feature = "win32-system-sys", feature = "win32-ui-sys"))]
impl ::core::marker::Copy for APOInitSystemEffects3 {}
#[cfg(all(feature = "win32-system-sys", feature = "win32-ui-sys"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APO_BUFFER_FLAGS = i32;
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = 0i32;
pub const BUFFER_VALID: APO_BUFFER_FLAGS = 1i32;
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = 2i32;
pub type APO_CONNECTION_BUFFER_TYPE = i32;
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = 0i32;
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = 1i32;
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = 2i32;
#[repr(C)]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: IAudioMediaType,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APO_FLAG = i32;
pub const APO_FLAG_NONE: APO_FLAG = 0i32;
pub const APO_FLAG_INPLACE: APO_FLAG = 1i32;
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = 2i32;
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = 4i32;
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = 8i32;
pub const APO_FLAG_MIXER: APO_FLAG = 16i32;
pub const APO_FLAG_DEFAULT: APO_FLAG = 14i32;
pub type APO_LOG_LEVEL = i32;
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = 0i32;
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = 1i32;
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = 2i32;
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = 3i32;
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = 4i32;
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = 5i32;
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for APO_NOTIFICATION {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for APO_NOTIFICATION_0 {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APO_NOTIFICATION_TYPE = i32;
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = 0i32;
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = 1i32;
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = 2i32;
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = 3i32;
#[repr(C)]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows_core_sys::GUID,
    pub Flags: APO_FLAG,
    pub szFriendlyName: [u16; 256],
    pub szCopyrightInfo: [u16; 256],
    pub u32MajorVersion: u32,
    pub u32MinorVersion: u32,
    pub u32MinInputConnections: u32,
    pub u32MaxInputConnections: u32,
    pub u32MinOutputConnections: u32,
    pub u32MaxOutputConnections: u32,
    pub u32MaxInstances: u32,
    pub u32NumAPOInterfaces: u32,
    pub iidAPOInterfaceList: [::windows_core_sys::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[repr(C)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub propertyStore: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AUDIO_FLOW_TYPE = i32;
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = 0i32;
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = 1i32;
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows_core_sys::GUID,
    pub canSetState: ::win32_foundation_sys::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
    pub propertyStoreContext: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub propertyStoreContext: ::windows_core_sys::GUID,
    pub propertyStoreType: super::__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
    pub propertyStore: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AUDIO_SYSTEMEFFECT_STATE = i32;
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = 0i32;
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = 1i32;
#[repr(C)]
#[cfg(feature = "win32-ui-sys")]
pub struct AudioFXExtensionParams {
    pub AddPageParam: ::win32_foundation_sys::LPARAM,
    pub pwstrEndpointID: ::windows_core_sys::PWSTR,
    pub pFxProperties: ::win32_ui_sys::Shell::PropertiesSystem::IPropertyStore,
}
#[cfg(feature = "win32-ui-sys")]
impl ::core::marker::Copy for AudioFXExtensionParams {}
#[cfg(feature = "win32-ui-sys")]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAudioConstriction = i32;
pub const eAudioConstrictionOff: EAudioConstriction = 0i32;
pub const eAudioConstriction48_16: EAudioConstriction = 1i32;
pub const eAudioConstriction44_16: EAudioConstriction = 2i32;
pub const eAudioConstriction14_14: EAudioConstriction = 3i32;
pub const eAudioConstrictionMute: EAudioConstriction = 4i32;
pub type FNAPONOTIFICATIONCALLBACK = ::core::option::Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT>;
pub type IApoAcousticEchoCancellation = *mut ::core::ffi::c_void;
pub type IApoAuxiliaryInputConfiguration = *mut ::core::ffi::c_void;
pub type IApoAuxiliaryInputRT = *mut ::core::ffi::c_void;
pub type IAudioDeviceModulesClient = *mut ::core::ffi::c_void;
pub type IAudioMediaType = *mut ::core::ffi::c_void;
pub type IAudioProcessingObject = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectConfiguration = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectLoggingService = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectNotifications = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectRT = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectRTQueueService = *mut ::core::ffi::c_void;
pub type IAudioProcessingObjectVBR = *mut ::core::ffi::c_void;
pub type IAudioSystemEffects = *mut ::core::ffi::c_void;
pub type IAudioSystemEffects2 = *mut ::core::ffi::c_void;
pub type IAudioSystemEffects3 = *mut ::core::ffi::c_void;
pub type IAudioSystemEffectsCustomFormats = *mut ::core::ffi::c_void;
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_APO_SWFallback_ProcessingModes: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 13u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_EndpointEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 15u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 18u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 17u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 16u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_ModeEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 14u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 20u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 19u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_CompositeFX_StreamEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 13u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 10u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 7u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_Association: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 0u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_EndpointEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 7u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_FriendlyName: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 4u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 10u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 9u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 8u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_ModeEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 6u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_Offload_ModeEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 12u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_Offload_StreamEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 11u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_PostMixEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 2u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_PreMixEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 1u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_StreamEffectClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 5u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_FX_UserInterfaceClsid: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 3u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 9u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 12u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 6u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 8u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 11u32 };
#[cfg(feature = "win32-ui-sys")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui_sys::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 5u32 };
pub const SID_AudioProcessingObjectLoggingService: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2340423855, data2: 2553, data3: 17774, data4: [161, 115, 189, 181, 132, 153, 188, 231] };
pub const SID_AudioProcessingObjectRTQueue: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1166809631, data2: 26777, data3: 19474, data4: [153, 172, 226, 230, 172, 37, 49, 4] };
#[repr(C)]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows_core_sys::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl ::core::marker::Copy for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::clone::Clone for UNCOMPRESSEDAUDIOFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
