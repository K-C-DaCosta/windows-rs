#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMProcessConfigXMLFiltered(pszxmlin: ::windows_sys::core::PCWSTR, rgszallowedcspnodes: *const ::windows_sys::core::PWSTR, dwnumallowedcspnodes: u32, pbstrxmlout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_WPD_NAMESPACE_EXTENSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 897084732, data2: 45173, data3: 18873, data4: [136, 221, 2, 152, 118, 225, 28, 1] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type DELETE_OBJECT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type DEVICE_RADIO_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_MTPBTH_IsConnected: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 3927062522, data2: 22685, data3: 17522, data4: [132, 228, 10, 190, 54, 253, 98, 239] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_DEVICE_ALREADY_OPENED: ::windows_sys::core::HRESULT = -2144731135i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_DEVICE_IS_HUNG: ::windows_sys::core::HRESULT = -2144731130i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_DEVICE_NOT_OPEN: ::windows_sys::core::HRESULT = -2144731134i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: ::windows_sys::core::HRESULT = -2144731133i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: ::windows_sys::core::HRESULT = -2144730934i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: ::windows_sys::core::HRESULT = -2144731132i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: ::windows_sys::core::HRESULT = -2144730933i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_NOT_COMMITED: ::windows_sys::core::HRESULT = -2144731131i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SERVICE_ALREADY_OPENED: ::windows_sys::core::HRESULT = -2144730936i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: ::windows_sys::core::HRESULT = -2144730932i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SERVICE_NOT_OPEN: ::windows_sys::core::HRESULT = -2144730935i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: ::windows_sys::core::HRESULT = -2144731035i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SMS_INVALID_RECIPIENT: ::windows_sys::core::HRESULT = -2144731036i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: ::windows_sys::core::HRESULT = -2144731034i32;
pub const EnumBthMtpConnectors: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2706833737, data2: 58949, data3: 20291, data4: [139, 13, 64, 155, 6, 29, 178, 252] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FACILITY_WPD: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
pub const GUID_DEVINTERFACE_WPD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1791129720, data2: 42746, data3: 16725, data4: [186, 133, 249, 143, 73, 29, 79, 51] };
pub const GUID_DEVINTERFACE_WPD_PRIVATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121377679, data2: 19949, data3: 18871, data4: [189, 211, 250, 190, 40, 102, 18, 17] };
pub const GUID_DEVINTERFACE_WPD_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2666811264, data2: 15716, data3: 16966, data4: [166, 170, 32, 111, 50, 141, 30, 220] };
#[repr(C)]
pub struct IConnectionRequestCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnComplete: unsafe extern "system" fn(this: *mut *mut Self, hrstatus: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumPortableDeviceConnectors {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, crequested: u32, pconnectors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cconnectors: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumPortableDeviceObjectIDs {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cobjects: u32, pobjids: *mut ::windows_sys::core::PWSTR, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cobjects: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaRadioManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRadioInstances: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSystemRadioStateChange: unsafe extern "system" fn(this: *mut *mut Self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaRadioManagerNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnInstanceAdd: unsafe extern "system" fn(this: *mut *mut Self, pradioinstance: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInstanceRemove: unsafe extern "system" fn(this: *mut *mut Self, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInstanceRemove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInstanceRadioChange: unsafe extern "system" fn(this: *mut *mut Self, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInstanceRadioChange: usize,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
#[repr(C)]
pub struct IPortableDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pcallback: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, ppszcookie: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, pszcookie: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPnPDeviceID: unsafe extern "system" fn(this: *mut *mut Self, ppszpnpdeviceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceCapabilities {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSupportedCommands: unsafe extern "system" fn(this: *mut *mut Self, ppcommands: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetCommandOptions: unsafe extern "system" fn(this: *mut *mut Self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetCommandOptions: usize,
    pub GetFunctionalCategories: unsafe extern "system" fn(this: *mut *mut Self, ppcategories: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFunctionalObjects: unsafe extern "system" fn(this: *mut *mut Self, category: *const ::windows_sys::core::GUID, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSupportedContentTypes: unsafe extern "system" fn(this: *mut *mut Self, category: *const ::windows_sys::core::GUID, ppcontenttypes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSupportedFormats: unsafe extern "system" fn(this: *mut *mut Self, contenttype: *const ::windows_sys::core::GUID, ppformats: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetFixedPropertyAttributes: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetFixedPropertyAttributes: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSupportedEvents: unsafe extern "system" fn(this: *mut *mut Self, ppevents: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEventOptions: unsafe extern "system" fn(this: *mut *mut Self, event: *const ::windows_sys::core::GUID, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceConnector {
    pub base__: ::windows_sys::core::IUnknown,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Devices_Properties")]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Devices_Properties")]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))]
    SetProperty: usize,
    pub GetPnPID: unsafe extern "system" fn(this: *mut *mut Self, ppwszpnpid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceContent {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumObjects: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pszparentobjectid: ::windows_sys::core::PCWSTR, pfilter: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Transfer: unsafe extern "system" fn(this: *mut *mut Self, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateObjectWithPropertiesOnly: unsafe extern "system" fn(this: *mut *mut Self, pvalues: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateObjectWithPropertiesAndData: unsafe extern "system" fn(this: *mut *mut Self, pvalues: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateObjectWithPropertiesAndData: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, dwoptions: u32, pobjectids: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetObjectIDsFromPersistentUniqueIDs: unsafe extern "system" fn(this: *mut *mut Self, ppersistentuniqueids: *mut ::core::ffi::c_void, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, pobjectids: *mut ::core::ffi::c_void, pszdestinationfolderobjectid: ::windows_sys::core::PCWSTR, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, pobjectids: *mut ::core::ffi::c_void, pszdestinationfolderobjectid: ::windows_sys::core::PCWSTR, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceContent2 {
    pub base__: IPortableDeviceContent,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateObjectWithPropertiesAndData: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, pproperties: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateObjectWithPropertiesAndData: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPortableDeviceDataStream {
    pub base__: super::super::System::Com::IStream,
    pub GetObjectID: unsafe extern "system" fn(this: *mut *mut Self, ppszobjectid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceDispatchFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceDispatch: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, ppdevicedispatch: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceDispatch: usize,
}
#[repr(C)]
pub struct IPortableDeviceEventCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnEvent: unsafe extern "system" fn(this: *mut *mut Self, peventparameters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceKeyCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcelems: *const u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetAt: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDevices: unsafe extern "system" fn(this: *mut *mut Self, ppnpdeviceids: *mut ::windows_sys::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RefreshDeviceList: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetDeviceFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, pdevicefriendlyname: ::windows_sys::core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceDescription: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, pdevicedescription: ::windows_sys::core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, pdevicemanufacturer: ::windows_sys::core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, pszdevicepropertyname: ::windows_sys::core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPrivateDevices: unsafe extern "system" fn(this: *mut *mut Self, ppnpdeviceids: *mut ::windows_sys::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDevicePropVariantCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcelems: *const u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Add: usize,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pvt: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, vt: u16) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceProperties {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSupportedProperties: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetPropertyAttributes: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetPropertyAttributes: usize,
    pub GetValues: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, pkeys: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValues: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, pvalues: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, pkeys: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDevicePropertiesBulk {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueueGetValuesByObjectList: unsafe extern "system" fn(this: *mut *mut Self, pobjectids: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub QueueGetValuesByObjectFormat: unsafe extern "system" fn(this: *mut *mut Self, pguidobjectformat: *const ::windows_sys::core::GUID, pszparentobjectid: ::windows_sys::core::PCWSTR, dwdepth: u32, pkeys: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub QueueSetValuesByObjectList: unsafe extern "system" fn(this: *mut *mut Self, pobjectvalues: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDevicePropertiesBulkCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStart: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *const ::windows_sys::core::GUID, presults: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnEnd: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *const ::windows_sys::core::GUID, hrstatus: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceResources {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSupportedResources: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetResourceAttributes: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetResourceAttributes: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetStream: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, pszobjectid: ::windows_sys::core::PCWSTR, pkeys: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResource: unsafe extern "system" fn(this: *mut *mut Self, presourceattributes: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResource: usize,
}
#[repr(C)]
pub struct IPortableDeviceService {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pszpnpserviceid: ::windows_sys::core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Methods: unsafe extern "system" fn(this: *mut *mut Self, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetServiceObjectID: unsafe extern "system" fn(this: *mut *mut Self, ppszserviceobjectid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPnPServiceID: unsafe extern "system" fn(this: *mut *mut Self, ppszpnpserviceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pcallback: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, ppszcookie: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, pszcookie: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceServiceActivation {
    pub base__: ::windows_sys::core::IUnknown,
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, pszpnpserviceid: ::windows_sys::core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelOpenAsync: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceServiceCapabilities {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSupportedMethods: unsafe extern "system" fn(this: *mut *mut Self, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSupportedMethodsByFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMethodAttributes: unsafe extern "system" fn(this: *mut *mut Self, method: *const ::windows_sys::core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetMethodParameterAttributes: unsafe extern "system" fn(this: *mut *mut Self, method: *const ::windows_sys::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetMethodParameterAttributes: usize,
    pub GetSupportedFormats: unsafe extern "system" fn(this: *mut *mut Self, ppformats: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFormatAttributes: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetFormatPropertyAttributes: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetFormatPropertyAttributes: usize,
    pub GetSupportedEvents: unsafe extern "system" fn(this: *mut *mut Self, ppevents: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEventAttributes: unsafe extern "system" fn(this: *mut *mut Self, event: *const ::windows_sys::core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetEventParameterAttributes: unsafe extern "system" fn(this: *mut *mut Self, event: *const ::windows_sys::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetEventParameterAttributes: usize,
    pub GetInheritedServices: unsafe extern "system" fn(this: *mut *mut Self, dwinheritancetype: u32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFormatRenderingProfiles: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID, pprenderingprofiles: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSupportedCommands: unsafe extern "system" fn(this: *mut *mut Self, ppcommands: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetCommandOptions: unsafe extern "system" fn(this: *mut *mut Self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetCommandOptions: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceServiceManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDeviceServices: unsafe extern "system" fn(this: *mut *mut Self, pszpnpdeviceid: ::windows_sys::core::PCWSTR, guidservicecategory: *const ::windows_sys::core::GUID, pservices: *mut ::windows_sys::core::PWSTR, pcservices: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceForService: unsafe extern "system" fn(this: *mut *mut Self, pszpnpserviceid: ::windows_sys::core::PCWSTR, ppszpnpdeviceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceServiceMethodCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnComplete: unsafe extern "system" fn(this: *mut *mut Self, hrstatus: ::windows_sys::core::HRESULT, presults: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceServiceMethods {
    pub base__: ::windows_sys::core::IUnknown,
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, method: *const ::windows_sys::core::GUID, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InvokeAsync: unsafe extern "system" fn(this: *mut *mut Self, method: *const ::windows_sys::core::GUID, pparameters: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceServiceOpenCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnComplete: unsafe extern "system" fn(this: *mut *mut Self, hrstatus: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceUnitsStream {
    pub base__: ::windows_sys::core::IUnknown,
    pub SeekInUnits: unsafe extern "system" fn(this: *mut *mut Self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceValues {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcelt: *const u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetStringValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetStringValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetStringValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetStringValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetUnsignedIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetUnsignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetUnsignedIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetUnsignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetSignedIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetSignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSignedIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetUnsignedLargeIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetUnsignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetUnsignedLargeIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetUnsignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetSignedLargeIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetSignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSignedLargeIntegerValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetFloatValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetFloatValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetFloatValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetFloatValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetErrorValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetErrorValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetErrorValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetErrorValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetKeyValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetKeyValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetKeyValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetBoolValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetBoolValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetBoolValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetBoolValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIUnknownValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIUnknownValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIUnknownValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIUnknownValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetGuidValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetGuidValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetGuidValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetGuidValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetBufferValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetBufferValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetBufferValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetBufferValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDeviceValuesValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDeviceValuesValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDeviceValuesValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDeviceValuesValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub RemoveValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    RemoveValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CopyValuesFromPropertyStore: unsafe extern "system" fn(this: *mut *mut Self, pstore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CopyValuesFromPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CopyValuesToPropertyStore: unsafe extern "system" fn(this: *mut *mut Self, pstore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CopyValuesToPropertyStore: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPortableDeviceValuesCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcelems: *const u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPortableDeviceWebControl {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetDeviceFromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetDeviceFromId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetDeviceFromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: *mut ::core::ffi::c_void, perrorhandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetDeviceFromIdAsync: usize,
}
#[repr(C)]
pub struct IRadioInstance {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRadioManagerSignature: unsafe extern "system" fn(this: *mut *mut Self, pguidsignature: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInstanceSignature: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInstanceSignature: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFriendlyName: usize,
    pub GetRadioState: unsafe extern "system" fn(this: *mut *mut Self, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows_sys::core::HRESULT,
    pub SetRadioState: unsafe extern "system" fn(this: *mut *mut Self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMultiComm: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMultiComm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAssociatingDevice: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAssociatingDevice: usize,
}
#[repr(C)]
pub struct IRadioInstanceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcinstance: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, ppradioinstance: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWpdSerializer {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIPortableDeviceValuesFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WriteIPortableDeviceValuesToBuffer: unsafe extern "system" fn(this: *mut *mut Self, dwoutputbufferlength: u32, presults: *mut ::core::ffi::c_void, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetBufferFromIPortableDeviceValues: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSerializedSize: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_3GPP2File: &str = "3GPP2File";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_3GPPFile: &str = "3GPPFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AACFile: &str = "AACFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AIFFFile: &str = "AIFFFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AMRFile: &str = "AMRFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ASFFile: &str = "ASFFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ASXPlaylist: &str = "ASXPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ATSCTSFile: &str = "ATSCTSFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AVCHDFile: &str = "AVCHDFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AVIFile: &str = "AVIFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractActivity: &str = "AbstractActivity";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractActivityOccurrence: &str = "AbstractActivityOccurrence";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractAudioAlbum: &str = "AbstractAudioAlbum";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractAudioPlaylist: &str = "AbstractAudioPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractAudioVideoAlbum: &str = "AbstractAudioVideoAlbum";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractChapteredProduction: &str = "AbstractChapteredProduction";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractContact: &str = "AbstractContact";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractContactGroup: &str = "AbstractContactGroup";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractDocument: &str = "AbstractDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractImageAlbum: &str = "AbstractImageAlbum";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMediacast: &str = "AbstractMediacast";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMessage: &str = "AbstractMessage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMessageFolder: &str = "AbstractMessageFolder";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMultimediaAlbum: &str = "AbstractMultimediaAlbum";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractNote: &str = "AbstractNote";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractTask: &str = "AbstractTask";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractVideoAlbum: &str = "AbstractVideoAlbum";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractVideoPlaylist: &str = "AbstractVideoPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults: &str = "AnchorResults";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults_Anchor: &str = "Anchor";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults_AnchorState: &str = "AnchorState";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults_ResultObjectID: &str = "ResultObjectID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncKnowledge: &str = "AnchorSyncKnowledge";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc: &str = "AnchorSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_BeginSync: &str = "BeginSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_CurrentAnchor: &str = "AnchorCurrentAnchor";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_EndSync: &str = "EndSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_FilterType: &str = "FilterType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_GetChangesSinceAnchor: &str = "GetChangesSinceAnchor";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_KnowledgeObjectID: &str = "AnchorKnowledgeObjectID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_LastSyncProxyID: &str = "AnchorLastSyncProxyID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_LocalOnlyDelete: &str = "LocalOnlyDelete";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_ProviderVersion: &str = "AnchorProviderVersion";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_ReplicaID: &str = "AnchorReplicaID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_SyncFormat: &str = "SyncFormat";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_VersionProps: &str = "AnchorVersionProps";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Association: &str = "Association";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudibleFile: &str = "AudibleFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioBitDepth: &str = "AudioBitDepth";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioBitRate: &str = "AudioBitRate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioBlockAlignment: &str = "AudioBlockAlignment";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioFormatCode: &str = "AudioFormatCode";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_Channels: &str = "Channels";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_Lyrics: &str = "Lyrics";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_BMPImage: &str = "BMPImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CIFFImage: &str = "CIFFImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Accepted: &str = "Accepted";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_BeginDateTime: &str = "BeginDateTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_BusyStatus: &str = "BusyStatus";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Declined: &str = "Declined";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_EndDateTime: &str = "EndDateTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Location: &str = "Location";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_PatternDuration: &str = "PatternDuration";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_PatternStartTime: &str = "PatternStartTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_ReminderOffset: &str = "ReminderOffset";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Tentative: &str = "Tentative";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_TimeZone: &str = "TimeZone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarSvc: &str = "Calendar";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarSvc_SyncWindowEnd: &str = "SyncWindowEnd";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarSvc_SyncWindowStart: &str = "SyncWindowStart";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_AnniversaryDate: &str = "AnniversaryDate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Assistant: &str = "Assistant";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Birthdate: &str = "Birthdate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressCity: &str = "BusinessAddressCity";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressCountry: &str = "BusinessAddressCountry";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressFull: &str = "BusinessAddressFull";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressLine2: &str = "BusinessAddressLine2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressPostalCode: &str = "BusinessAddressPostalCode";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressRegion: &str = "BusinessAddressRegion";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressStreet: &str = "BusinessAddressStreet";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessEmail: &str = "BusinessEmail";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessEmail2: &str = "BusinessEmail2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessFax: &str = "BusinessFax";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessPhone: &str = "BusinessPhone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessPhone2: &str = "BusinessPhone2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessWebAddress: &str = "BusinessWebAddress";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Children: &str = "Children";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Email: &str = "Email";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_FamilyName: &str = "FamilyName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Fax: &str = "Fax";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_GivenName: &str = "GivenName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_IMAddress: &str = "IMAddress";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_IMAddress2: &str = "IMAddress2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_IMAddress3: &str = "IMAddress3";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_MiddleNames: &str = "MiddleNames";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_MobilePhone: &str = "MobilePhone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_MobilePhone2: &str = "MobilePhone2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Organization: &str = "Organization";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressCity: &str = "OtherAddressCity";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressCountry: &str = "OtherAddressCountry";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressFull: &str = "OtherAddressFull";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressLine2: &str = "OtherAddressLine2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressPostalCode: &str = "OtherAddressPostalCode";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressRegion: &str = "OtherAddressRegion";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressStreet: &str = "OtherAddressStreet";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherEmail: &str = "OtherEmail";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherPhone: &str = "OtherPhone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Pager: &str = "Pager";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressCity: &str = "PersonalAddressCity";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressCountry: &str = "PersonalAddressCountry";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressFull: &str = "PersonalAddressFull";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressLine2: &str = "PersonalAddressLine2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressPostalCode: &str = "PersonalAddressPostalCode";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressRegion: &str = "PersonalAddressRegion";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressStreet: &str = "PersonalAddressStreet";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalEmail: &str = "PersonalEmail";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalEmail2: &str = "PersonalEmail2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalFax: &str = "PersonalFax";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalPhone: &str = "PersonalPhone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalPhone2: &str = "PersonalPhone2";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalWebAddress: &str = "PersonalWebAddress";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Phone: &str = "Phone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PhoneticFamilyName: &str = "PhoneticFamilyName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PhoneticGivenName: &str = "PhoneticGivenName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PhoneticOrganization: &str = "PhoneticOrganization";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Ringtone: &str = "Ringtone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Role: &str = "Role";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Spouse: &str = "Spouse";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Suffix: &str = "Suffix";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Title: &str = "Title";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_WebAddress: &str = "WebAddress";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactSvc_SyncWithPhoneOnly: &str = "FilterType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactsSvc: &str = "Contacts";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DPOFDocument: &str = "DPOFDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DVBTSFile: &str = "DVBTSFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceExecutable: &str = "DeviceExecutable";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataCAB: &str = "DeviceMetadataCAB";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataObj_ContentID: &str = "ContentID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataObj_DefaultCAB: &str = "DefaultCAB";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataSvc: &str = "Metadata";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceScript: &str = "DeviceScript";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_EXIFImage: &str = "EXIFImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ExcelDocument: &str = "ExcelDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FLACFile: &str = "FLACFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FirmwareFile: &str = "FirmwareFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FlashPixImage: &str = "FlashPixImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncKnowledge: &str = "FullEnumSyncKnowledge";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc: &str = "FullEnumSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_BeginSync: &str = "BeginSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_EndSync: &str = "EndSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_FilterType: &str = "FilterType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_KnowledgeObjectID: &str = "FullEnumKnowledgeObjectID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_LastSyncProxyID: &str = "FullEnumLastSyncProxyID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_LocalOnlyDelete: &str = "LocalOnlyDelete";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_ProviderVersion: &str = "FullEnumProviderVersion";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_ReplicaID: &str = "FullEnumReplicaID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_SyncFormat: &str = "SyncFormat";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_VersionProps: &str = "FullEnumVersionProps";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GIFImage: &str = "GIFImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_AllowedFolderContents: &str = "AllowedFolderContents";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_AssociationDesc: &str = "AssociationDesc";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_AssociationType: &str = "AssociationType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Copyright: &str = "Copyright";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Corrupt: &str = "Corrupt";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DRMStatus: &str = "DRMStatus";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateAccessed: &str = "DateAccessed";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateAdded: &str = "DateAdded";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateAuthored: &str = "DateAuthored";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateCreated: &str = "DateCreated";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateModified: &str = "DateModified";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateRevised: &str = "DateRevised";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Description: &str = "Description";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Hidden: &str = "Hidden";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Keywords: &str = "Keywords";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_LanguageLocale: &str = "LanguageLocale";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Name: &str = "Name";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_NonConsumable: &str = "NonConsumable";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectFileName: &str = "ObjectFileName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectFormat: &str = "ObjectFormat";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectID: &str = "ObjectID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectSize: &str = "ObjectSize";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ParentID: &str = "ParentID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_PersistentUID: &str = "PersistentUID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_PropertyBag: &str = "PropertyBag";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ProtectionStatus: &str = "ProtectionStatus";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ReferenceParentID: &str = "ReferenceParentID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_StorageID: &str = "StorageID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_SubDescription: &str = "SubDescription";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_SyncID: &str = "SyncID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_SystemObject: &str = "SystemObject";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_TimeToLive: &str = "TimeToLive";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_HDPhotoImage: &str = "HDPhotoImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_HTMLDocument: &str = "HTMLDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_HintsSvc: &str = "Hints";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ICalendarActivity: &str = "ICalendar";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_Aperature: &str = "Aperature";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_Exposure: &str = "Exposure";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_ISOSpeed: &str = "ISOSpeed";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_ImageBitDepth: &str = "ImageBitDepth";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_IsColorCorrected: &str = "IsColorCorrected";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_IsCropped: &str = "IsCropped";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JFIFImage: &str = "JFIFImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JP2Image: &str = "JP2Image";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JPEGXRImage: &str = "JPEGXRImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JPXImage: &str = "JPXImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_M3UPlaylist: &str = "M3UPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MHTDocument: &str = "MHTDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MP3File: &str = "MP3File";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPEG2File: &str = "MPEG2File";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPEG4File: &str = "MPEG4File";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPEGFile: &str = "MPEGFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPLPlaylist: &str = "MPLPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_AlbumArtist: &str = "AlbumArtist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_AlbumName: &str = "AlbumName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Artist: &str = "Artist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_AudioEncodingProfile: &str = "AudioEncodingProfile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BitRateType: &str = "BitRateType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BookmarkByte: &str = "BookmarkByte";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BookmarkObject: &str = "BookmarkObject";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BookmarkTime: &str = "BookmarkTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BufferSize: &str = "BufferSize";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Composer: &str = "Composer";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Credits: &str = "Credits";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_DateOriginalRelease: &str = "DateOriginalRelease";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Duration: &str = "Duration";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Editor: &str = "Editor";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_EffectiveRating: &str = "EffectiveRating";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_EncodingProfile: &str = "EncodingProfile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_EncodingQuality: &str = "EncodingQuality";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Genre: &str = "Genre";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_GeographicOrigin: &str = "GeographicOrigin";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Height: &str = "Height";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_MediaType: &str = "MediaType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_MediaUID: &str = "MediaUID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Mood: &str = "Mood";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Owner: &str = "Owner";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_ParentalRating: &str = "ParentalRating";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Producer: &str = "Producer";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_SampleRate: &str = "SampleRate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_SkipCount: &str = "SkipCount";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_SubscriptionContentID: &str = "SubscriptionContentID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Subtitle: &str = "Subtitle";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_TotalBitRate: &str = "TotalBitRate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Track: &str = "Track";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_URLLink: &str = "URLLink";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_URLSource: &str = "URLSource";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_UseCount: &str = "UseCount";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_UserRating: &str = "UserRating";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_WebMaster: &str = "WebMaster";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Width: &str = "Width";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_BCC: &str = "BCC";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Body: &str = "Body";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_CC: &str = "CC";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Category: &str = "Category";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternDayOfMonth: &str = "PatternDayOfMonth";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternDayOfWeek: &str = "PatternDayOfWeek";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternDeleteDates: &str = "PatternDeleteDates";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternInstance: &str = "PatternInstance";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternMonthOfYear: &str = "PatternMonthOfYear";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternOriginalDateTime: &str = "PatternOriginalDateTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternPeriod: &str = "PatternPeriod";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternType: &str = "PatternType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternValidEndDate: &str = "PatternValidEndDate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternValidStartDate: &str = "PatternValidStartDate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Priority: &str = "Priority";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Read: &str = "Read";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_ReceivedTime: &str = "ReceivedTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Sender: &str = "Sender";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Subject: &str = "Subject";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_To: &str = "To";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageSvc: &str = "Message";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_NotesSvc: &str = "Notes";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_OGGFile: &str = "OGGFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PCDImage: &str = "PCDImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PICTImage: &str = "PICTImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PNGImage: &str = "PNGImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PSLPlaylist: &str = "PSLPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PowerPointDocument: &str = "PowerPointDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_QCELPFile: &str = "QCELPFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_RingtonesSvc: &str = "Ringtones";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_RingtonesSvc_DefaultRingtone: &str = "DefaultRingtone";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Services_ServiceDisplayName: &str = "ServiceDisplayName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Services_ServiceIcon: &str = "ServiceIcon";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Services_ServiceLocale: &str = "ServiceLocale";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc: &str = "Status";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_BatteryLife: &str = "BatteryLife";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_ChargingState: &str = "ChargingState";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_MissedCalls: &str = "MissedCalls";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_NetworkName: &str = "NetworkName";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_NetworkType: &str = "NetworkType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_NewPictures: &str = "NewPictures";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_Roaming: &str = "Roaming";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_SignalStrength: &str = "SignalStrength";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_StorageCapacity: &str = "StorageCapacity";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_StorageFreeSpace: &str = "StorageFreeSpace";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_TextMessages: &str = "TextMessages";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_VoiceMail: &str = "VoiceMail";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncObj_LastAuthorProxyID: &str = "LastAuthorProxyID";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_BeginSync: &str = "BeginSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_EndSync: &str = "EndSync";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_FilterType: &str = "FilterType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_LocalOnlyDelete: &str = "LocalOnlyDelete";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_SyncFormat: &str = "SyncFormat";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_SyncObjectReferences: &str = "SyncObjectReferences";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TIFFEPImage: &str = "TIFFEPImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TIFFITImage: &str = "TIFFITImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TIFFImage: &str = "TIFFImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_BeginDate: &str = "BeginDate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_Complete: &str = "Complete";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_EndDate: &str = "EndDate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_ReminderDateTime: &str = "ReminderDateTime";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TasksSvc: &str = "Tasks";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TasksSvc_SyncActiveOnly: &str = "FilterType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TextDocument: &str = "TextDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Undefined: &str = "Undefined";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedAudio: &str = "UndefinedAudio";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedCollection: &str = "UndefinedCollection";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedDocument: &str = "UndefinedDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedVideo: &str = "UndefinedVideo";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UnknownImage: &str = "UnknownImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VCalendar1Activity: &str = "VCalendar1";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VCard2Contact: &str = "VCard2Contact";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VCard3Contact: &str = "VCard3Contact";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_KeyFrameDistance: &str = "KeyFrameDistance";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_ScanType: &str = "ScanType";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_Source: &str = "Source";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_VideoBitRate: &str = "VideoBitRate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_VideoFormatCode: &str = "VideoFormatCode";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_VideoFrameRate: &str = "VideoFrameRate";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WAVFile: &str = "WAVFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WBMPImage: &str = "WBMPImage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WMAFile: &str = "WMAFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WMVFile: &str = "WMVFile";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WPLPlaylist: &str = "WPLPlaylist";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WordDocument: &str = "WordDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_XMLDocument: &str = "XMLDocument";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DRM_SCHEME_PDDRM: &str = "PDDRM";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DRM_SCHEME_WMDRM10_PD: &str = "WMDRM10-PD";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_ICON: &str = "Icons";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_IS_MASS_STORAGE: &str = "PortableDeviceIsMassStorage";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_NAMESPACE_EXCLUDE_FROM_SHELL: &str = "PortableDeviceNameSpaceExcludeFromShell";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_NAMESPACE_THUMBNAIL_CONTENT_TYPES: &str = "PortableDeviceNameSpaceThumbnailContentTypes";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_NAMESPACE_TIMEOUT: &str = "PortableDeviceNameSpaceTimeout";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_TYPE: &str = "PortableDeviceType";
pub const PortableDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921655237, data2: 15774, data3: 18647, data4: [152, 16, 134, 72, 72, 240, 244, 4] };
pub const PortableDeviceDispatchFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1126375987, data2: 33592, data3: 18008, data4: [174, 1, 11, 74, 232, 48, 182, 176] };
pub const PortableDeviceFTM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4156556186, data2: 18274, data3: 18570, data4: [180, 179, 118, 14, 249, 161, 186, 155] };
pub const PortableDeviceKeyCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3727491629, data2: 9344, data3: 17342, data4: [151, 240, 209, 250, 44, 249, 143, 79] };
pub const PortableDeviceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 183569644, data2: 11981, data3: 19346, data4: [149, 129, 52, 246, 174, 6, 55, 243] };
pub const PortableDevicePropVariantCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 145333807, data2: 28013, data3: 19328, data4: [175, 90, 186, 242, 188, 190, 76, 185] };
pub const PortableDeviceService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4015895746, data2: 37650, data3: 16940, data4: [145, 82, 65, 28, 217, 196, 221, 132] };
pub const PortableDeviceServiceFTM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 373928276, data2: 51092, data3: 18810, data4: [155, 3, 243, 240, 18, 19, 2, 243] };
pub const PortableDeviceValues: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 202757379, data2: 53271, data3: 18382, data4: [144, 22, 123, 63, 151, 135, 33, 204] };
pub const PortableDeviceValuesCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 948048717, data2: 5327, data3: 16928, data4: [156, 180, 67, 95, 134, 216, 63, 96] };
pub const PortableDeviceWebControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 409849900, data2: 11756, data3: 16821, data4: [167, 212, 181, 144, 86, 250, 222, 81] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type SMS_MESSAGE_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const STR_WPDNSE_FAST_ENUM: &str = "WPDNSE Fast Enum";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const STR_WPDNSE_SIMPLE_ITEM: &str = "WPDNSE SimpleItem";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type SYSTEM_RADIO_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_CalendarSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_ContactsSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_HintsSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_MessageSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_NotesSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_RingtonesSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_StatusSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_TasksSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] }, pid: 7u32 };
pub const WPDNSE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 886510601, data2: 19271, data3: 19840, data4: [170, 172, 58, 40, 164, 163, 179, 230] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
pub const WPD_API_OPTIONS_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 283462206, data2: 1325, data3: 18295, data4: [161, 60, 222, 118, 20, 190, 43, 196] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_IOCTL_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 283462206, data2: 1325, data3: 18295, data4: [161, 60, 222, 118, 20, 190, 43, 196] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 283462206, data2: 1325, data3: 18295, data4: [161, 60, 222, 118, 20, 190, 43, 196] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 3u32 };
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4187946243, data2: 17181, data3: 16600, data4: [161, 201, 78, 34, 13, 156, 136, 211] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BIT_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BLOCK_ALIGNMENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_CHANNEL_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_FORMAT_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_BITRATE_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_CAPTURE_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = 3i32;
pub const WPD_CATEGORY_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] };
pub const WPD_CATEGORY_COMMON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] };
pub const WPD_CATEGORY_DEVICE_HINTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 224377131, data2: 52038, data3: 19535, data4: [131, 67, 11, 195, 211, 241, 124, 132] };
pub const WPD_CATEGORY_MEDIA_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1504981946, data2: 65092, data3: 19853, data4: [128, 140, 107, 203, 155, 15, 21, 232] };
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] };
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2029635324, data2: 31160, data3: 18236, data4: [144, 96, 107, 210, 61, 208, 114, 196] };
pub const WPD_CATEGORY_NULL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const WPD_CATEGORY_OBJECT_ENUMERATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] };
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] };
pub const WPD_CATEGORY_OBJECT_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] };
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] };
pub const WPD_CATEGORY_OBJECT_RESOURCES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] };
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] };
pub const WPD_CATEGORY_SERVICE_COMMON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 841942813, data2: 14063, data3: 18303, data4: [180, 181, 111, 82, 215, 52, 186, 238] };
pub const WPD_CATEGORY_SERVICE_METHODS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] };
pub const WPD_CATEGORY_SMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] };
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1338861954, data2: 8866, data3: 19205, data4: [164, 139, 98, 211, 139, 242, 123, 50] };
pub const WPD_CATEGORY_STORAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3640199078, data2: 13516, data3: 17914, data4: [151, 251, 208, 7, 250, 71, 236, 148] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1043699162, data2: 19825, data3: 18942, data4: [160, 180, 212, 64, 108, 58, 233, 63] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1661599727, data2: 43132, data3: 19623, data4: [132, 52, 121, 117, 118, 228, 10, 150] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1043699162, data2: 19825, data3: 18942, data4: [160, 180, 212, 64, 108, 58, 233, 63] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1661599727, data2: 43132, data3: 19623, data4: [132, 52, 121, 117, 118, 228, 10, 150] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1707172088, data2: 4967, data3: 19682, data4: [147, 157, 131, 16, 131, 159, 13, 48] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1661599727, data2: 43132, data3: 19623, data4: [132, 52, 121, 117, 118, 228, 10, 150] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1043699162, data2: 19825, data3: 18942, data4: [160, 180, 212, 64, 108, 58, 233, 63] }, pid: 4u32 };
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1661599727, data2: 43132, data3: 19623, data4: [132, 52, 121, 117, 118, 228, 10, 150] };
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1043699162, data2: 19825, data3: 18942, data4: [160, 180, 212, 64, 108, 58, 233, 63] };
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1707172088, data2: 4967, data3: 19682, data4: [147, 157, 131, 16, 131, 159, 13, 48] };
pub const WPD_CLASS_EXTENSION_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 872090897, data2: 25763, data3: 20396, data4: [180, 199, 61, 254, 170, 153, 176, 81] };
pub const WPD_CLASS_EXTENSION_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2131196341, data2: 64043, data3: 18278, data4: [156, 178, 247, 59, 163, 11, 103, 88] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_DESIRED_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_EVENT_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 11u32 };
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MAJOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_REVISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SHARE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 541957900, data2: 8850, data3: 16512, data4: [159, 66, 64, 102, 78, 112, 248, 89] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_COLOR_CORRECTED_STATUS_VALUES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    pub Command: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    pub AccessType: u32,
    pub AccessProperty: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_COMMAND_ACCESS_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = 8i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = 16i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2131196341, data2: 64043, data3: 18278, data4: [156, 178, 247, 59, 163, 11, 103, 88] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2131196341, data2: 64043, data3: 18278, data4: [156, 178, 247, 59, 163, 11, 103, 88] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 872090897, data2: 25763, data3: 20396, data4: [180, 199, 61, 254, 170, 153, 176, 81] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMIT_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2029635324, data2: 31160, data3: 18236, data4: [144, 96, 107, 210, 61, 208, 114, 196] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_RESET_DEVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 224377131, data2: 52038, data3: 19535, data4: [131, 67, 11, 195, 211, 241, 124, 132] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_GENERATE_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2029635324, data2: 31160, data3: 18236, data4: [144, 96, 107, 210, 61, 208, 114, 196] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1504981946, data2: 65092, data3: 19853, data4: [128, 140, 107, 203, 155, 15, 21, 232] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1504981946, data2: 65092, data3: 19853, data4: [128, 140, 107, 203, 155, 15, 21, 232] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1504981946, data2: 65092, data3: 19853, data4: [128, 140, 107, 203, 155, 15, 21, 232] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_READ_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2029635324, data2: 31160, data3: 18236, data4: [144, 96, 107, 210, 61, 208, 114, 196] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 841942813, data2: 14063, data3: 18303, data4: [180, 181, 111, 82, 215, 52, 186, 238] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SMS_SEND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1338861954, data2: 8866, data3: 19205, data4: [164, 139, 98, 211, 139, 242, 123, 50] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_EJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3640199078, data2: 13516, data3: 17914, data4: [151, 251, 208, 7, 250, 71, 236, 148] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3640199078, data2: 13516, data3: 17914, data4: [151, 251, 208, 7, 250, 71, 236, 148] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_BODY_TEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_END_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_NOTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] }, pid: 7u32 };
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_PRIORITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_START_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_SUBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2995448139, data2: 1444, data3: 20110, data4: [190, 1, 114, 204, 126, 9, 157, 143] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ANNIVERSARY_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 62u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ASSISTANT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 61u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BIRTHDATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 57u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 34u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 35u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 45u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 40u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 41u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 22u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 50u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_CHILDREN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 60u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 54u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 51u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 52u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 53u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MIDDLE_NAMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 42u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 43u32 };
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_EMAILS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 36u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_PHONES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 47u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 30u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PAGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 46u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 32u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 33u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 44u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 38u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 39u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 49u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 55u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PREFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 31u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 58u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 37u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 48u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_RINGTONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 63u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ROLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 56u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SPOUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 59u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SUFFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4225039787, data2: 39037, data3: 18295, data4: [179, 249, 114, 97, 133, 169, 49, 43] }, pid: 7u32 };
pub const WPD_CONTENT_TYPE_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2162258130, data2: 4181, data3: 19006, data4: [185, 82, 130, 204, 79, 138, 134, 137] };
pub const WPD_CONTENT_TYPE_APPOINTMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 267191822, data2: 34707, data3: 19230, data4: [144, 201, 72, 172, 56, 154, 198, 49] };
pub const WPD_CONTENT_TYPE_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1255327838, data2: 24109, data3: 17893, data4: [136, 100, 79, 34, 158, 60, 108, 240] };
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2853729150, data2: 20489, data3: 18682, data4: [174, 33, 133, 242, 67, 131, 180, 230] };
pub const WPD_CONTENT_TYPE_CALENDAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2717735271, data2: 24611, data3: 18848, data4: [157, 241, 248, 6, 11, 231, 81, 176] };
pub const WPD_CONTENT_TYPE_CERTIFICATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3694687976, data2: 43336, data3: 16480, data4: [144, 80, 203, 215, 126, 138, 61, 135] };
pub const WPD_CONTENT_TYPE_CONTACT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3938091795, data2: 17701, data3: 18183, data4: [159, 14, 135, 198, 128, 142, 148, 53] };
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 879462706, data2: 19510, data3: 16600, data4: [148, 21, 24, 40, 41, 31, 157, 233] };
pub const WPD_CONTENT_TYPE_DOCUMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1745542994, data2: 38154, data3: 16449, data4: [155, 65, 101, 227, 147, 100, 129, 85] };
pub const WPD_CONTENT_TYPE_EMAIL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2151154762, data2: 32337, data3: 20367, data4: [136, 61, 29, 6, 35, 209, 69, 51] };
pub const WPD_CONTENT_TYPE_FOLDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 669180818, data2: 41233, data3: 18656, data4: [171, 12, 225, 119, 5, 160, 95, 133] };
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2582446432, data2: 6143, data3: 19524, data4: [157, 152, 29, 122, 111, 148, 25, 33] };
pub const WPD_CONTENT_TYPE_GENERIC_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 8773798, data2: 36148, data3: 17879, data4: [188, 92, 68, 126, 89, 199, 61, 72] };
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3893275384, data2: 45787, data3: 16691, data4: [182, 126, 27, 239, 75, 74, 110, 95] };
pub const WPD_CONTENT_TYPE_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011919317, data2: 42282, data3: 16963, data4: [162, 107, 98, 212, 23, 109, 118, 3] };
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1970876744, data2: 5621, data3: 18992, data4: [168, 19, 84, 237, 138, 55, 226, 38] };
pub const WPD_CONTENT_TYPE_MEDIA_CAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1586017228, data2: 15973, data3: 20066, data4: [191, 255, 34, 148, 149, 37, 58, 176] };
pub const WPD_CONTENT_TYPE_MEMO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2631012047, data2: 15184, data3: 16719, data4: [166, 65, 228, 115, 255, 228, 87, 81] };
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15778732, data2: 42387, data3: 18860, data4: [146, 25, 36, 171, 202, 90, 37, 99] };
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 52275182, data2: 6344, data3: 16901, data4: [132, 126, 137, 161, 18, 97, 208, 243] };
pub const WPD_CONTENT_TYPE_PLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 439613412, data2: 44819, data3: 18677, data4: [153, 78, 119, 54, 157, 254, 4, 163] };
pub const WPD_CONTENT_TYPE_PROGRAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3530160490, data2: 9340, data3: 19455, data4: [152, 251, 151, 243, 196, 146, 32, 230] };
pub const WPD_CONTENT_TYPE_SECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2182121973, data2: 7569, data3: 19913, data4: [190, 60, 187, 177, 179, 91, 24, 206] };
pub const WPD_CONTENT_TYPE_TASK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1663381292, data2: 34943, data3: 19638, data4: [177, 172, 210, 152, 85, 220, 239, 108] };
pub const WPD_CONTENT_TYPE_TELEVISION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1621191119, data2: 62126, data3: 20001, data4: [147, 117, 150, 119, 241, 28, 28, 110] };
pub const WPD_CONTENT_TYPE_UNSPECIFIED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 685298462, data2: 9372, data3: 17742, data4: [170, 188, 52, 136, 49, 104, 230, 52] };
pub const WPD_CONTENT_TYPE_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2455875644, data2: 15736, data3: 17689, data4: [133, 227, 2, 197, 225, 245, 11, 185] };
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 19598775, data2: 54465, data3: 17878, data4: [176, 129, 148, 184, 119, 121, 97, 79] };
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 195823370, data2: 40799, data3: 19876, data4: [168, 246, 61, 228, 77, 104, 253, 108] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_CROPPED_STATUS_VALUES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_EDP_IDENTITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1814792076, data2: 49900, data3: 18701, data4: [180, 37, 215, 167, 94, 35, 229, 237] }, pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FIRMWARE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1178457698, data2: 32708, data3: 17041, data4: [145, 28, 127, 76, 156, 202, 151, 153] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1178457698, data2: 32708, data3: 17041, data4: [145, 28, 127, 76, 156, 202, 151, 153] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_NETWORK_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_OBJECT_ID: &str = "DEVICE";
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_LEVEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 5u32 };
pub const WPD_DEVICE_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] };
pub const WPD_DEVICE_PROPERTIES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1178457698, data2: 32708, data3: 17041, data4: [145, 28, 127, 76, 156, 202, 151, 153] };
pub const WPD_DEVICE_PROPERTIES_V3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1814792076, data2: 49900, data3: 18701, data4: [180, 37, 215, 167, 94, 35, 229, 237] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_PROTOCOL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SYNC_PARTNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TRANSPORT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1178457698, data2: 32708, data3: 17041, data4: [145, 28, 127, 76, 156, 202, 151, 153] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_DEVICE_TRANSPORTS = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 651466650, data2: 58947, data3: 17958, data4: [158, 43, 115, 109, 192, 201, 47, 220] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_DEVICE_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_USE_DEVICE_STAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1178457698, data2: 32708, data3: 17041, data4: [145, 28, 127, 76, 156, 202, 151, 153] }, pid: 5u32 };
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 185664003, data2: 60309, data3: 20226, data4: [147, 224, 151, 198, 49, 73, 58, 213] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_EFFECT_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_BCC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_CC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_ATTACHMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_BEEN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 7u32 };
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_RECEIVED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_SENDER_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_TO_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1106835034, data2: 21636, data3: 18306, data4: [177, 61, 71, 64, 221, 124, 55, 197] }, pid: 2u32 };
pub const WPD_EVENT_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 281634168, data2: 11905, data3: 16657, data4: [173, 222, 224, 140, 166, 19, 143, 109] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 281634168, data2: 11905, data3: 16657, data4: [173, 222, 224, 140, 166, 19, 143, 109] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 281634168, data2: 11905, data3: 16657, data4: [173, 222, 224, 140, 166, 19, 143, 109] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 281634168, data2: 11905, data3: 16657, data4: [173, 222, 224, 140, 166, 19, 143, 109] }, pid: 3u32 };
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 914905761, data2: 52564, data3: 19882, data4: [179, 208, 175, 179, 224, 63, 89, 153] };
pub const WPD_EVENT_DEVICE_REMOVED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838560795, data2: 26904, data3: 18617, data4: [133, 238, 2, 190, 124, 133, 10, 249] };
pub const WPD_EVENT_DEVICE_RESET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2002112339, data2: 49645, data3: 17651, data4: [181, 162, 69, 30, 44, 55, 107, 39] };
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 22328, data3: 20466, data4: [132, 69, 190, 49, 38, 105, 16, 89] };
pub const WPD_EVENT_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 732095498, data2: 27468, data3: 17045, data4: [187, 67, 38, 50, 43, 153, 174, 178] };
pub const WPD_EVENT_OBJECT_ADDED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2804341397, data2: 57863, data3: 19202, data4: [141, 68, 190, 242, 232, 108, 191, 252] };
pub const WPD_EVENT_OBJECT_REMOVED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3196234632, data2: 42284, data3: 18467, data4: [150, 229, 208, 39, 38, 113, 252, 56] };
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2367070369, data2: 62150, data3: 16858, data4: [143, 25, 94, 83, 114, 26, 219, 242] };
pub const WPD_EVENT_OBJECT_UPDATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 340109145, data2: 11777, data3: 18525, data4: [159, 39, 255, 7, 218, 230, 151, 171] };
pub const WPD_EVENT_OPTIONS_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3017333463, data2: 41825, data3: 19331, data4: [138, 72, 91, 2, 206, 16, 113, 59] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3017333463, data2: 41825, data3: 19331, data4: [138, 72, 91, 2, 206, 16, 113, 59] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3017333463, data2: 41825, data3: 19331, data4: [138, 72, 91, 2, 206, 16, 113, 59] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1384151946, data2: 18708, data3: 17187, data4: [155, 154, 116, 246, 84, 178, 184, 70] }, pid: 2u32 };
pub const WPD_EVENT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 363534675, data2: 63511, data3: 20463, data4: [169, 33, 86, 118, 232, 56, 246, 224] };
pub const WPD_EVENT_PROPERTIES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1384151946, data2: 18708, data3: 17187, data4: [155, 154, 116, 246, 84, 178, 184, 70] };
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2318661112, data2: 2764, data3: 19867, data4: [156, 196, 17, 45, 53, 59, 134, 202] };
pub const WPD_EVENT_STORAGE_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 931291499, data2: 8892, data3: 17524, data4: [162, 81, 48, 112, 248, 211, 136, 87] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_EXPOSURE_METERING_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_EXPOSURE_PROGRAM_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = 7i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_FLASH_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_FOCUS_METERING_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_FOCUS_MODES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2124053183, data2: 58728, data3: 19252, data4: [170, 47, 19, 187, 18, 171, 23, 125] }, pid: 2u32 };
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2124053183, data2: 58728, data3: 19252, data4: [170, 47, 19, 187, 18, 171, 23, 125] };
pub const WPD_FORMAT_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2694848512, data2: 48303, data3: 19432, data4: [179, 245, 35, 63, 35, 28, 245, 143] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2694848512, data2: 48303, data3: 19432, data4: [179, 245, 35, 63, 35, 28, 245, 143] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2694848512, data2: 48303, data3: 19432, data4: [179, 245, 35, 63, 35, 28, 245, 143] }, pid: 2u32 };
pub const WPD_FUNCTIONAL_CATEGORY_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 764044562, data2: 42828, data3: 17550, data4: [186, 138, 244, 172, 7, 196, 147, 153] };
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1059723545, data2: 51138, data3: 18944, data4: [133, 93, 245, 124, 240, 109, 235, 187] };
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 149571179, data2: 58276, data3: 17206, data4: [161, 243, 164, 77, 43, 92, 67, 140] };
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1224006514, data2: 31850, data3: 19120, data4: [158, 26, 71, 14, 60, 219, 242, 106] };
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 140512164, data2: 42938, data3: 18945, data4: [171, 14, 0, 101, 208, 163, 86, 211] };
pub const WPD_FUNCTIONAL_CATEGORY_SMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4497585, data2: 49641, data3: 19197, data4: [179, 88, 166, 44, 97, 23, 201, 207] };
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1631363879, data2: 43923, data3: 18688, data4: [180, 250, 137, 91, 181, 135, 75, 121] };
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 602954684, data2: 5598, data3: 19498, data4: [165, 91, 169, 175, 92, 228, 18, 239] };
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3795738475, data2: 29251, data3: 17322, data4: [141, 241, 14, 179, 217, 104, 169, 24] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2399481235, data2: 43978, data3: 20421, data4: [165, 172, 176, 29, 244, 219, 229, 152] }, pid: 2u32 };
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2399481235, data2: 43978, data3: 20421, data4: [165, 172, 176, 29, 244, 219, 229, 152] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_BITDEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_CROPPED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 9u32 };
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_VERTICAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1674987784, data2: 40865, data3: 18335, data4: [133, 186, 153, 82, 33, 100, 71, 219] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ALBUM_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 49u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BITRATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BUY_NOW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BYTE_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 36u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COMPOSER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COPYRIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 31u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESTINATION_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 30u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DURATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 32u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 38u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_ACCESSED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_BUILD_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 35u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_MANAGING_EDITOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_META_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OBJECT_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 34u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_PARENTAL_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 9u32 };
pub const WPD_MEDIA_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_RELEASE_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SAMPLE_RATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SKIP_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SOURCE_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_STAR_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 39u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 33u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_TO_LIVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 37u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TOTAL_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WEBMASTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 785955333, data2: 2771, data3: 17116, data4: [176, 208, 188, 149, 172, 57, 106, 200] }, pid: 22u32 };
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1610349691, data2: 29827, data3: 16813, data4: [175, 185, 218, 63, 78, 89, 43, 141] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_META_GENRES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = 17i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = 18i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = 19i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = 20i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = 21i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = 33i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = 34i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = 35i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = 36i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = 37i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = 38i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = 39i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = 40i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = 48i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = 64i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = 65i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = 66i32;
pub const WPD_METHOD_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4051325041, data2: 61497, data3: 17583, data4: [142, 254, 67, 44, 243, 46, 67, 42] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_ALBUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_LYRICS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_MOOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 8u32 };
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_TRACK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3005543786, data2: 56413, data3: 18149, data4: [182, 223, 210, 234, 65, 72, 136, 198] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3838393375, data2: 45571, data3: 17393, data4: [161, 0, 90, 7, 209, 27, 2, 116] }, pid: 2u32 };
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838393375, data2: 45571, data3: 17393, data4: [161, 0, 90, 7, 209, 27, 2, 116] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3838393375, data2: 45571, data3: 17393, data4: [161, 0, 90, 7, 209, 27, 2, 116] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_BACK_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_AUTHORED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_CREATED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_MODIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 6u32 };
pub const WPD_OBJECT_FORMAT_3G2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112501248, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_3G2A: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 437329965, data2: 34649, data3: 20020, data4: [186, 94, 177, 33, 16, 135, 238, 228] };
pub const WPD_OBJECT_FORMAT_3GP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112435712, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_3GPA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3843499824, data2: 63857, data3: 16879, data4: [161, 11, 34, 113, 160, 1, 157, 122] };
pub const WPD_OBJECT_FORMAT_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3103981568, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3145793536, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3120955392, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121283072, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_AIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805765120, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3254136498, data2: 19379, data3: 18332, data4: [156, 250, 5, 181, 243, 165, 123, 34] };
pub const WPD_OBJECT_FORMAT_AMR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3104309248, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ASF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 806092800, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121807360, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ATSCTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112632320, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_AUDIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3104047104, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_AVCHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112566784, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_AVI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805961728, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_BMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 939786240, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_CIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 939851776, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_DPOF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805699584, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_DVBTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112697856, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_EXECUTABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805502976, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_EXIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 939589632, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_FLAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3104178176, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_FLASHPIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 939720704, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_GIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 939982848, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_HTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805634048, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ICALENDAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187867648, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_ICON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124924653, data2: 4140, data3: 17976, data4: [156, 34, 131, 241, 66, 191, 200, 34] };
pub const WPD_OBJECT_FORMAT_JFIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940048384, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_JP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940507136, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_JPEGXR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3087269888, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_JPX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940572672, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121676288, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_M4A: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 816555948, data2: 28669, data3: 19491, data4: [163, 89, 62, 155, 82, 243, 241, 200] };
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3129212928, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3129278464, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3129344000, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2969829376, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3129147392, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MKV: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3113222144, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112370176, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805896192, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MP4: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112304640, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 806027264, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121741824, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2969698304, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_OGG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3103916032, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_PCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940113920, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_PICT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940179456, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121872896, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_PNG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940244992, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805371904, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_QCELP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3104243712, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_SCRIPT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805437440, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_TEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805568512, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_TIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940376064, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_TIFFEP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 939655168, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_TIFFIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 940441600, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805306368, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_VCALENDAR1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187802112, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_VCARD2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3145859072, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_VCARD3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3145924608, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_WAVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 805830656, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_WBMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3087204352, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3095461888, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_WMA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3103850496, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_WMV: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112239104, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121610752, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2969763840, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
pub const WPD_OBJECT_FORMAT_XML: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3129081856, data2: 44652, data3: 18436, data4: [152, 186, 197, 123, 70, 150, 95, 231] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISHIDDEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISSYSTEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_IS_DRM_PROTECTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_KEYWORDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_LANGUAGE_LOCALE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 5u32 };
pub const WPD_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] };
pub const WPD_OBJECT_PROPERTIES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 57920829, data2: 19014, data3: 16599, data4: [180, 216, 115, 232, 218, 116, 231, 117] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SUPPORTED_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 57920829, data2: 19014, data3: 16599, data4: [180, 216, 115, 232, 218, 116, 231, 117] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SYNC_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4016785677, data2: 23768, data3: 17274, data4: [175, 252, 218, 139, 96, 238, 74, 60] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_OPERATION_STATES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 5001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 5003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 5001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 5002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] }, pid: 5001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_VALID_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 5001u32 };
pub const WPD_PARAMETER_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3867561431, data2: 62245, data3: 17898, data4: [161, 213, 151, 207, 115, 182, 202, 88] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_PARAMETER_USAGE_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_POWER_SOURCES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = 1i32;
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1297371224, data2: 35072, data3: 16563, data4: [143, 29, 220, 36, 110, 30, 131, 112] };
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1297371224, data2: 20430, data3: 17784, data4: [149, 200, 134, 152, 169, 188, 15, 73] };
pub const WPD_PROPERTY_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] };
pub const WPD_PROPERTY_ATTRIBUTES_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1570611552, data2: 29870, data3: 17356, data4: [133, 169, 254, 85, 90, 128, 121, 142] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1570611552, data2: 29870, data3: 17356, data4: [133, 169, 254, 85, 90, 128, 121, 142] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2876851160, data2: 25394, data3: 17503, data4: [160, 13, 141, 94, 241, 233, 111, 55] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1570611552, data2: 29870, data3: 17356, data4: [133, 169, 254, 85, 90, 128, 121, 142] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1014u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1015u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1010u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1009u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1012u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 212593784, data2: 27508, data3: 16838, data4: [146, 22, 38, 57, 209, 252, 227, 86] }, pid: 1013u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 872090897, data2: 25763, data3: 20396, data4: [180, 199, 61, 254, 170, 153, 176, 81] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 872090897, data2: 25763, data3: 20396, data4: [180, 199, 61, 254, 170, 153, 176, 81] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2131196341, data2: 64043, data3: 18278, data4: [156, 178, 247, 59, 163, 11, 103, 88] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2131196341, data2: 64043, data3: 18278, data4: [156, 178, 247, 59, 163, 11, 103, 88] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2131196341, data2: 64043, data3: 18278, data4: [156, 178, 247, 59, 163, 11, 103, 88] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1009u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1010u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4030868124, data2: 24008, data3: 17472, data4: [181, 189, 93, 242, 136, 53, 101, 138] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 224377131, data2: 52038, data3: 19535, data4: [131, 67, 11, 195, 211, 241, 124, 132] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 224377131, data2: 52038, data3: 19535, data4: [131, 67, 11, 195, 211, 241, 124, 132] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 61320, data3: 20045, data4: [149, 195, 79, 50, 127, 114, 138, 150] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1013u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1012u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1009u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1010u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1014u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1297371224, data2: 6702, data3: 16646, data4: [163, 87, 119, 30, 8, 25, 252, 86] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_NULL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3074903697, data2: 59384, data3: 19161, data4: [180, 0, 173, 26, 75, 88, 238, 236] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1013u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1010u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1012u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1016u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1009u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1015u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4011738077, data2: 43501, data3: 17217, data4: [139, 204, 24, 97, 146, 174, 160, 137] }, pid: 1014u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 298329309, data2: 1229, data3: 20046, data4: [140, 123, 246, 239, 183, 148, 216, 78] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2656404196, data2: 2068, data3: 17638, data4: [152, 26, 178, 153, 141, 88, 56, 4] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1010u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1009u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1014u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1012u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1013u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1016u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3013784109, data2: 42389, data3: 16648, data4: [190, 10, 252, 60, 150, 95, 61, 74] }, pid: 1015u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_PUBLIC_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2029635324, data2: 31160, data3: 18236, data4: [144, 96, 107, 210, 61, 208, 114, 196] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1018u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1019u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1012u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1013u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1007u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1008u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1014u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1015u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1006u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1010u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1009u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1016u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1017u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1011u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 608534132, data2: 11935, data3: 17657, data4: [140, 87, 29, 27, 203, 23, 11, 137] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 1005u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 760356008, data2: 49584, data3: 17000, data4: [163, 66, 207, 25, 50, 21, 105, 188] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 841942813, data2: 14063, data3: 18303, data4: [180, 181, 111, 82, 215, 52, 186, 238] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] }, pid: 1004u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_RECIPIENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] }, pid: 1001u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2948750694, data2: 65037, data3: 16660, data4: [144, 151, 151, 12, 147, 233, 32, 209] }, pid: 1003u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3640199078, data2: 13516, data3: 17914, data4: [151, 251, 208, 7, 250, 71, 236, 148] }, pid: 1002u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3640199078, data2: 13516, data3: 17914, data4: [151, 251, 208, 7, 250, 71, 236, 148] }, pid: 1001u32 };
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309110175, data2: 60963, data3: 18993, data4: [133, 144, 118, 57, 135, 152, 112, 180] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3309110175, data2: 60963, data3: 18993, data4: [133, 144, 118, 57, 135, 152, 112, 180] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3309110175, data2: 60963, data3: 18993, data4: [133, 144, 118, 57, 135, 152, 112, 180] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3309110175, data2: 60963, data3: 18993, data4: [133, 144, 118, 57, 135, 152, 112, 180] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4029326164, data2: 8960, data3: 20013, data4: [161, 185, 59, 103, 48, 247, 250, 33] }, pid: 0u32 };
pub const WPD_RESOURCE_ATTRIBUTES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 515307012, data2: 37496, data3: 17055, data4: [147, 204, 91, 184, 192, 102, 86, 182] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1002518914, data2: 34225, data3: 18656, data4: [149, 166, 141, 58, 208, 107, 225, 23] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_BRANDING_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3056841134, data2: 27823, data3: 19079, data4: [149, 137, 34, 222, 214, 221, 88, 153] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 743270403, data2: 33002, data3: 17792, data4: [175, 154, 91, 225, 162, 62, 221, 203] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_DEFAULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3894311358, data2: 13552, data3: 16831, data4: [181, 63, 241, 160, 106, 232, 120, 66] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_GENERIC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3115971861, data2: 47728, data3: 17991, data4: [148, 220, 250, 73, 37, 233, 90, 7] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4053139160, data2: 43560, data3: 20195, data4: [177, 83, 225, 130, 221, 94, 220, 57] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3351513018, data2: 39162, data3: 18101, data4: [153, 96, 35, 254, 193, 36, 207, 222] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_VIDEO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3043421762, data2: 25448, data3: 17040, data4: [134, 98, 112, 24, 47, 183, 159, 32] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1365966123, data2: 50766, data3: 17648, data4: [152, 220, 190, 225, 200, 143, 125, 102] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1365966123, data2: 50766, data3: 17648, data4: [152, 220, 190, 225, 200, 143, 125, 102] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1365966123, data2: 50766, data3: 17648, data4: [152, 220, 190, 225, 200, 143, 125, 102] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1365966123, data2: 50766, data3: 17648, data4: [152, 220, 190, 225, 200, 143, 125, 102] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_SECTION_DATA_UNITS_VALUES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = 1i32;
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1365966123, data2: 50766, data3: 17648, data4: [152, 220, 190, 225, 200, 143, 125, 102] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_SERVICE_INHERITANCE_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = 0i32;
pub const WPD_SERVICE_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1964009866, data2: 52052, data3: 18460, data4: [184, 219, 13, 117, 201, 63, 28, 6] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SERVICE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1964009866, data2: 52052, data3: 18460, data4: [184, 219, 13, 117, 201, 63, 28, 6] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_ENCODING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_SMS_ENCODING_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_MAX_PAYLOAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] }, pid: 4u32 };
pub const WPD_SMS_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_PROVIDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_TIMEOUT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2115007692, data2: 20735, data3: 19921, data4: [167, 66, 83, 190, 111, 9, 58, 13] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 31u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 30u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 18u32 };
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CONTRAST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EFFECT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 22u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FLASH_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_RGB_GAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_SHARPNESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_UPLOAD_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_WHITE_BALANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1489334764, data2: 7115, data3: 17063, data4: [138, 197, 187, 41, 21, 115, 162, 96] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_ACCESS_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_STORAGE_ACCESS_CAPABILITY_VALUES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_MAX_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 9u32 };
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 27460986, data2: 29910, data3: 20096, data4: [190, 167, 220, 76, 33, 44, 229, 10] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_STORAGE_TYPE_VALUES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_STREAM_UNITS = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = 8i32;
pub const WPD_TASK_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3813992798, data2: 55456, data3: 17975, data4: [160, 58, 12, 178, 104, 56, 219, 199] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3813992798, data2: 55456, data3: 17975, data4: [160, 58, 12, 178, 104, 56, 219, 199] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_PERCENT_COMPLETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3813992798, data2: 55456, data3: 17975, data4: [160, 58, 12, 178, 104, 56, 219, 199] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_REMINDER_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3813992798, data2: 55456, data3: 17975, data4: [160, 58, 12, 178, 104, 56, 219, 199] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3813992798, data2: 55456, data3: 17975, data4: [160, 58, 12, 178, 104, 56, 219, 199] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_AUTHOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_CREDITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FOURCC_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FRAMERATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 10u32 };
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_QUALITY_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_REPEAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_SCAN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 879698275, data2: 63896, data3: 16710, data4: [139, 1, 209, 155, 76, 0, 222, 154] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_VIDEO_SCAN_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = 7i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WPD_WHITE_BALANCE_SETTINGS = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = 5i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = 6i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = 7i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WpdAttributeForm = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = 4i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub type WpdParameterAttributeForm = i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = 0i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = 1i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = 2i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = 3i32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = 4i32;
pub const WpdSerializer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 194094923, data2: 44412, data3: 19101, data4: [181, 99, 41, 238, 249, 22, 113, 114] };
