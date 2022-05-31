pub const CLSID_WPD_NAMESPACE_EXTENSION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35786d3c_b075_49b9_88dd_029876e11c01);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DELETE_OBJECT_OPTIONS(pub i32);
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(0i32);
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(1i32);
impl ::core::marker::Copy for DELETE_OBJECT_OPTIONS {}
impl ::core::clone::Clone for DELETE_OBJECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DELETE_OBJECT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DELETE_OBJECT_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DELETE_OBJECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DELETE_OBJECT_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVICE_RADIO_STATE(pub i32);
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(0i32);
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(1i32);
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(2i32);
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(3i32);
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(4i32);
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(5i32);
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
impl ::core::marker::Copy for DEVICE_RADIO_STATE {}
impl ::core::clone::Clone for DEVICE_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEVICE_RADIO_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_RADIO_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-devices")]
pub const DEVPKEY_MTPBTH_IsConnected: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xea1237fa_589d_4472_84e4_0abe36fd62ef), pid: 2u32 };
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
#[inline]
pub unsafe fn DMProcessConfigXMLFiltered<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszxmlin: Param0, rgszallowedcspnodes: &[::windows_core::PWSTR]) -> ::windows_core::Result<::win32_foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMProcessConfigXMLFiltered(pszxmlin: ::windows_core::PCWSTR, rgszallowedcspnodes: *const ::windows_core::PWSTR, dwnumallowedcspnodes: u32, pbstrxmlout: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        DMProcessConfigXMLFiltered(pszxmlin.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(rgszallowedcspnodes)), rgszallowedcspnodes.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
pub const E_WPD_DEVICE_ALREADY_OPENED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731135i32);
pub const E_WPD_DEVICE_IS_HUNG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731130i32);
pub const E_WPD_DEVICE_NOT_OPEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731134i32);
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731133i32);
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144730934i32);
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731132i32);
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144730933i32);
pub const E_WPD_OBJECT_NOT_COMMITED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731131i32);
pub const E_WPD_SERVICE_ALREADY_OPENED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144730936i32);
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144730932i32);
pub const E_WPD_SERVICE_NOT_OPEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144730935i32);
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731035i32);
pub const E_WPD_SMS_INVALID_RECIPIENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731036i32);
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2144731034i32);
pub const EnumBthMtpConnectors: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1570149_e645_4f43_8b0d_409b061db2fc);
pub const FACILITY_WPD: u32 = 42u32;
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
pub const GUID_DEVINTERFACE_WPD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ac27878_a6fa_4155_ba85_f98f491d4f33);
pub const GUID_DEVINTERFACE_WPD_PRIVATE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba0c718f_4ded_49b7_bdd3_fabe28661211);
pub const GUID_DEVINTERFACE_WPD_SERVICE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ef44f80_3d64_4246_a6aa_206f328d1edc);
#[repr(transparent)]
pub struct IConnectionRequestCallback(::windows_core::IUnknown);
impl IConnectionRequestCallback {
    pub unsafe fn OnComplete(&self, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrstatus)).ok()
    }
}
impl ::core::convert::From<IConnectionRequestCallback> for ::windows_core::IUnknown {
    fn from(value: IConnectionRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionRequestCallback> for ::windows_core::IUnknown {
    fn from(value: &IConnectionRequestCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IConnectionRequestCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IConnectionRequestCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionRequestCallback {}
impl ::core::fmt::Debug for IConnectionRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionRequestCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IConnectionRequestCallback {
    type Vtable = IConnectionRequestCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x272c9ae0_7161_4ae0_91bd_9f448ee9c427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumPortableDeviceConnectors(::windows_core::IUnknown);
impl IEnumPortableDeviceConnectors {
    pub unsafe fn Next(&self, pconnectors: &mut [::core::option::Option<IPortableDeviceConnector>], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), pconnectors.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pconnectors)), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cconnectors: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cconnectors)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumPortableDeviceConnectors> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumPortableDeviceConnectors>(result__)
    }
}
impl ::core::convert::From<IEnumPortableDeviceConnectors> for ::windows_core::IUnknown {
    fn from(value: IEnumPortableDeviceConnectors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumPortableDeviceConnectors> for ::windows_core::IUnknown {
    fn from(value: &IEnumPortableDeviceConnectors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumPortableDeviceConnectors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumPortableDeviceConnectors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumPortableDeviceConnectors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumPortableDeviceConnectors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPortableDeviceConnectors {}
impl ::core::fmt::Debug for IEnumPortableDeviceConnectors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPortableDeviceConnectors").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumPortableDeviceConnectors {
    type Vtable = IEnumPortableDeviceConnectors_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfdef549_9247_454f_bd82_06fe80853faa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceConnectors_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumPortableDeviceObjectIDs(::windows_core::IUnknown);
impl IEnumPortableDeviceObjectIDs {
    pub unsafe fn Next(&self, pobjids: &mut [::windows_core::PWSTR], pcfetched: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), pobjids.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pobjids)), ::core::mem::transmute(pcfetched)))
    }
    pub unsafe fn Skip(&self, cobjects: u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cobjects)))
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IEnumPortableDeviceObjectIDs> for ::windows_core::IUnknown {
    fn from(value: IEnumPortableDeviceObjectIDs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumPortableDeviceObjectIDs> for ::windows_core::IUnknown {
    fn from(value: &IEnumPortableDeviceObjectIDs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumPortableDeviceObjectIDs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumPortableDeviceObjectIDs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumPortableDeviceObjectIDs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumPortableDeviceObjectIDs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPortableDeviceObjectIDs {}
impl ::core::fmt::Debug for IEnumPortableDeviceObjectIDs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPortableDeviceObjectIDs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumPortableDeviceObjectIDs {
    type Vtable = IEnumPortableDeviceObjectIDs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10ece955_cf41_4728_bfa0_41eedf1bbf19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceObjectIDs_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut ::windows_core::PWSTR, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaRadioManager(::windows_core::IUnknown);
impl IMediaRadioManager {
    pub unsafe fn GetRadioInstances(&self) -> ::windows_core::Result<IRadioInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRadioInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRadioInstanceCollection>(result__)
    }
    pub unsafe fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnSystemRadioStateChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sysradiostate), ::core::mem::transmute(utimeoutsec)).ok()
    }
}
impl ::core::convert::From<IMediaRadioManager> for ::windows_core::IUnknown {
    fn from(value: IMediaRadioManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaRadioManager> for ::windows_core::IUnknown {
    fn from(value: &IMediaRadioManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaRadioManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaRadioManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaRadioManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaRadioManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaRadioManager {}
impl ::core::fmt::Debug for IMediaRadioManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaRadioManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMediaRadioManager {
    type Vtable = IMediaRadioManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cfdcab5_fc47_42a5_9241_074b58830e73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetRadioInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnSystemRadioStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaRadioManagerNotifySink(::windows_core::IUnknown);
impl IMediaRadioManagerNotifySink {
    pub unsafe fn OnInstanceAdd<'a, Param0: ::windows_core::IntoParam<'a, IRadioInstance>>(&self, pradioinstance: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnInstanceAdd)(::windows_core::Interface::as_raw(self), pradioinstance.into_param().abi()).ok()
    }
    pub unsafe fn OnInstanceRemove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrradioinstanceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnInstanceRemove)(::windows_core::Interface::as_raw(self), bstrradioinstanceid.into_param().abi()).ok()
    }
    pub unsafe fn OnInstanceRadioChange<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrradioinstanceid: Param0, radiostate: DEVICE_RADIO_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnInstanceRadioChange)(::windows_core::Interface::as_raw(self), bstrradioinstanceid.into_param().abi(), ::core::mem::transmute(radiostate)).ok()
    }
}
impl ::core::convert::From<IMediaRadioManagerNotifySink> for ::windows_core::IUnknown {
    fn from(value: IMediaRadioManagerNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaRadioManagerNotifySink> for ::windows_core::IUnknown {
    fn from(value: &IMediaRadioManagerNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaRadioManagerNotifySink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaRadioManagerNotifySink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaRadioManagerNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaRadioManagerNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaRadioManagerNotifySink {}
impl ::core::fmt::Debug for IMediaRadioManagerNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaRadioManagerNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMediaRadioManagerNotifySink {
    type Vtable = IMediaRadioManagerNotifySink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89d81f5f_c147_49ed_a11c_77b20c31e7c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManagerNotifySink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnInstanceAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pradioinstance: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnInstanceRemove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub OnInstanceRadioChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows_core::HRESULT,
}
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
#[repr(transparent)]
pub struct IPortableDevice(::windows_core::IUnknown);
impl IPortableDevice {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pszpnpdeviceid: Param0, pclientinfo: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    pub unsafe fn SendCommand<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pparameters: Param1) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SendCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pparameters.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn Content(&self) -> ::windows_core::Result<IPortableDeviceContent> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Content)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceContent>(result__)
    }
    pub unsafe fn Capabilities(&self) -> ::windows_core::Result<IPortableDeviceCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Capabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceCapabilities>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceEventCallback>, Param2: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pcallback: Param1, pparameters: Param2) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi(), pparameters.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszcookie: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), pszcookie.into_param().abi()).ok()
    }
    pub unsafe fn GetPnPDeviceID(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetPnPDeviceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPortableDevice> for ::windows_core::IUnknown {
    fn from(value: IPortableDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevice> for ::windows_core::IUnknown {
    fn from(value: &IPortableDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevice {}
impl ::core::fmt::Debug for IPortableDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDevice {
    type Vtable = IPortableDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevice_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pclientinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows_core::RawPtr, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows_core::RawPtr, pparameters: ::windows_core::RawPtr, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcookie: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPnPDeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceCapabilities(::windows_core::IUnknown);
impl IPortableDeviceCapabilities {
    pub unsafe fn GetSupportedCommands(&self) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedCommands)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetCommandOptions(&self, command: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCommandOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(command), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn GetFunctionalCategories(&self) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFunctionalCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetFunctionalObjects(&self, category: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFunctionalObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(category), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetSupportedContentTypes(&self, category: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedContentTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(category), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetSupportedFormats(&self, contenttype: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(contenttype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedFormatProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetFixedPropertyAttributes(&self, format: *const ::windows_core::GUID, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFixedPropertyAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSupportedEvents(&self) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedEvents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetEventOptions(&self, event: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(event), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceCapabilities> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceCapabilities> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceCapabilities {}
impl ::core::fmt::Debug for IPortableDeviceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceCapabilities {
    type Vtable = IPortableDeviceCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c8c6dbf_e3dc_4061_becc_8542e810d126);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceCapabilities_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetCommandOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetCommandOptions: usize,
    pub GetFunctionalCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcategories: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFunctionalObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *const ::windows_core::GUID, ppobjectids: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSupportedContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *const ::windows_core::GUID, ppcontenttypes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSupportedFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *const ::windows_core::GUID, ppformats: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppkeys: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetFixedPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetFixedPropertyAttributes: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSupportedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetEventOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows_core::GUID, ppoptions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceConnector(::windows_core::IUnknown);
impl IPortableDeviceConnector {
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect<'a, Param0: ::windows_core::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn Cancel<'a, Param0: ::windows_core::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-devices")]
    pub unsafe fn GetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropertykey), ::core::mem::transmute(ppropertytype), ::core::mem::transmute(ppdata), ::core::mem::transmute(pcbdata)).ok()
    }
    #[cfg(feature = "win32-devices")]
    pub unsafe fn SetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pdata)), pdata.len() as _).ok()
    }
    pub unsafe fn GetPnPID(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetPnPID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceConnector> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceConnector> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceConnector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceConnector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceConnector {}
impl ::core::fmt::Debug for IPortableDeviceConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceConnector {
    type Vtable = IPortableDeviceConnector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceConnector_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-devices")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-devices"))]
    GetProperty: usize,
    #[cfg(feature = "win32-devices")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-devices"))]
    SetProperty: usize,
    pub GetPnPID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceContent(::windows_core::IUnknown);
impl IPortableDeviceContent {
    pub unsafe fn EnumObjects<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pszparentobjectid: Param1, pfilter: Param2) -> ::windows_core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows_core::Result<IPortableDeviceProperties> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceProperties>(result__)
    }
    pub unsafe fn Transfer(&self) -> ::windows_core::Result<IPortableDeviceResources> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Transfer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceResources>(result__)
    }
    pub unsafe fn CreateObjectWithPropertiesOnly<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateObjectWithPropertiesOnly)(::windows_core::Interface::as_raw(self), pvalues.into_param().abi(), ::core::mem::transmute(ppszobjectid)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateObjectWithPropertiesAndData<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppdata: *mut ::core::option::Option<::win32_system::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateObjectWithPropertiesAndData)(::windows_core::Interface::as_raw(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
    pub unsafe fn Delete<'a, Param1: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, dwoptions: u32, pobjectids: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoptions), pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, ppersistentuniqueids: Param0) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectIDsFromPersistentUniqueIDs)(::windows_core::Interface::as_raw(self), ppersistentuniqueids.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Move<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn Copy<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Copy)(::windows_core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceContent> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceContent {}
impl ::core::fmt::Debug for IPortableDeviceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceContent {
    type Vtable = IPortableDeviceContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a96ed84_7c73_4480_9938_bf5af477d426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: ::windows_core::PCWSTR, pfilter: ::windows_core::RawPtr, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Transfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateObjectWithPropertiesOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows_core::RawPtr, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateObjectWithPropertiesAndData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows_core::RawPtr, ppdata: *mut ::windows_core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateObjectWithPropertiesAndData: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows_core::RawPtr, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetObjectIDsFromPersistentUniqueIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows_core::RawPtr, ppobjectids: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows_core::RawPtr, pszdestinationfolderobjectid: ::windows_core::PCWSTR, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows_core::RawPtr, pszdestinationfolderobjectid: ::windows_core::PCWSTR, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceContent2(::windows_core::IUnknown);
impl IPortableDeviceContent2 {
    pub unsafe fn EnumObjects<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pszparentobjectid: Param1, pfilter: Param2) -> ::windows_core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows_core::Result<IPortableDeviceProperties> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceProperties>(result__)
    }
    pub unsafe fn Transfer(&self) -> ::windows_core::Result<IPortableDeviceResources> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Transfer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceResources>(result__)
    }
    pub unsafe fn CreateObjectWithPropertiesOnly<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CreateObjectWithPropertiesOnly)(::windows_core::Interface::as_raw(self), pvalues.into_param().abi(), ::core::mem::transmute(ppszobjectid)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateObjectWithPropertiesAndData<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppdata: *mut ::core::option::Option<::win32_system::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CreateObjectWithPropertiesAndData)(::windows_core::Interface::as_raw(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
    pub unsafe fn Delete<'a, Param1: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, dwoptions: u32, pobjectids: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoptions), pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, ppersistentuniqueids: Param0) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetObjectIDsFromPersistentUniqueIDs)(::windows_core::Interface::as_raw(self), ppersistentuniqueids.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Move<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Move)(::windows_core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn Copy<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Copy)(::windows_core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn UpdateObjectWithPropertiesAndData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pszobjectid: Param0, pproperties: Param1, ppdata: *mut ::core::option::Option<::win32_system::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateObjectWithPropertiesAndData)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), pproperties.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceContent2> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent2> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPortableDeviceContent2> for IPortableDeviceContent {
    fn from(value: IPortableDeviceContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent2> for IPortableDeviceContent {
    fn from(value: &IPortableDeviceContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPortableDeviceContent> for IPortableDeviceContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPortableDeviceContent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPortableDeviceContent> for &'a IPortableDeviceContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPortableDeviceContent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceContent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceContent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceContent2 {}
impl ::core::fmt::Debug for IPortableDeviceContent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceContent2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceContent2 {
    type Vtable = IPortableDeviceContent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b4add96_f6bf_4034_8708_eca72bf10554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent2_Vtbl {
    pub base__: IPortableDeviceContent_Vtbl,
    #[cfg(feature = "win32-system")]
    pub UpdateObjectWithPropertiesAndData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pproperties: ::windows_core::RawPtr, ppdata: *mut ::windows_core::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    UpdateObjectWithPropertiesAndData: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPortableDeviceDataStream(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPortableDeviceDataStream {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)))
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)))
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: ::win32_system::Com::STREAM_SEEK) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CopyTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Commit(&self, grfcommitflags: ::win32_system::Com::STGC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnlockRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Stat(&self, pstatstg: *mut ::win32_system::Com::STATSTG, grfstatflag: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<::win32_system::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IStream>(result__)
    }
    pub unsafe fn GetObjectID(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPortableDeviceDataStream> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPortableDeviceDataStream> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceDataStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPortableDeviceDataStream> for ::win32_system::Com::ISequentialStream {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPortableDeviceDataStream> for ::win32_system::Com::ISequentialStream {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::ISequentialStream> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::ISequentialStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::ISequentialStream> for &'a IPortableDeviceDataStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::ISequentialStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPortableDeviceDataStream> for ::win32_system::Com::IStream {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPortableDeviceDataStream> for ::win32_system::Com::IStream {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IStream> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IStream> for &'a IPortableDeviceDataStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPortableDeviceDataStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPortableDeviceDataStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPortableDeviceDataStream {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPortableDeviceDataStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDataStream").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPortableDeviceDataStream {
    type Vtable = IPortableDeviceDataStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88e04db3_1012_4d64_9996_f703a950d3f4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDataStream_Vtbl {
    pub base__: ::win32_system::Com::IStream_Vtbl,
    pub GetObjectID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceDispatchFactory(::windows_core::IUnknown);
impl IPortableDeviceDispatchFactory {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetDeviceDispatch<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpdeviceid: Param0) -> ::windows_core::Result<::win32_system::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceDispatch)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IDispatch>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceDispatchFactory> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceDispatchFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceDispatchFactory> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceDispatchFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceDispatchFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceDispatchFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceDispatchFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceDispatchFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceDispatchFactory {}
impl ::core::fmt::Debug for IPortableDeviceDispatchFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDispatchFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceDispatchFactory {
    type Vtable = IPortableDeviceDispatchFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e1eafc3_e3d7_4132_96fa_759c0f9d1e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDispatchFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub GetDeviceDispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, ppdevicedispatch: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetDeviceDispatch: usize,
}
#[repr(transparent)]
pub struct IPortableDeviceEventCallback(::windows_core::IUnknown);
impl IPortableDeviceEventCallback {
    pub unsafe fn OnEvent<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, peventparameters: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnEvent)(::windows_core::Interface::as_raw(self), peventparameters.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPortableDeviceEventCallback> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceEventCallback> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceEventCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceEventCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceEventCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceEventCallback {}
impl ::core::fmt::Debug for IPortableDeviceEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceEventCallback {
    type Vtable = IPortableDeviceEventCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8792a31_f385_493c_a893_40f64eb45f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceEventCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventparameters: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceKeyCollection(::windows_core::IUnknown);
impl IPortableDeviceKeyCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pkey)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn Add(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceKeyCollection> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceKeyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceKeyCollection> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceKeyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceKeyCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceKeyCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceKeyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceKeyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceKeyCollection {}
impl ::core::fmt::Debug for IPortableDeviceKeyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceKeyCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceKeyCollection {
    type Vtable = IPortableDeviceKeyCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdada2357_e0ad_492e_98db_dd61c53ba353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceKeyCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetAt: usize,
    #[cfg(feature = "win32-ui")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceManager(::windows_core::IUnknown);
impl IPortableDeviceManager {
    pub unsafe fn GetDevices(&self, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppnpdeviceids), ::core::mem::transmute(pcpnpdeviceids)).ok()
    }
    pub unsafe fn RefreshDeviceList(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshDeviceList)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceFriendlyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpdeviceid: Param0, pdevicefriendlyname: ::windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceFriendlyName)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(pdevicefriendlyname), ::core::mem::transmute(pcchdevicefriendlyname)).ok()
    }
    pub unsafe fn GetDeviceDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpdeviceid: Param0, pdevicedescription: ::windows_core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceDescription)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(pdevicedescription), ::core::mem::transmute(pcchdevicedescription)).ok()
    }
    pub unsafe fn GetDeviceManufacturer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpdeviceid: Param0, pdevicemanufacturer: ::windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceManufacturer)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(pdevicemanufacturer), ::core::mem::transmute(pcchdevicemanufacturer)).ok()
    }
    pub unsafe fn GetDeviceProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpdeviceid: Param0, pszdevicepropertyname: Param1, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceProperty)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), pszdevicepropertyname.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(pcbdata), ::core::mem::transmute(pdwtype)).ok()
    }
    pub unsafe fn GetPrivateDevices(&self, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrivateDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppnpdeviceids), ::core::mem::transmute(pcpnpdeviceids)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceManager> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceManager> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceManager {}
impl ::core::fmt::Debug for IPortableDeviceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceManager {
    type Vtable = IPortableDeviceManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1567595_4c2f_4574_a6fa_ecef917b9a40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::HRESULT,
    pub RefreshDeviceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pdevicefriendlyname: ::windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pdevicedescription: ::windows_core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pdevicemanufacturer: ::windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pszdevicepropertyname: ::windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows_core::HRESULT,
    pub GetPrivateDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDevicePropVariantCollection(::windows_core::IUnknown);
impl IPortableDevicePropVariantCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add(&self, pvalue: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn ChangeType(&self, vt: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangeType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vt)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<IPortableDevicePropVariantCollection> for ::windows_core::IUnknown {
    fn from(value: IPortableDevicePropVariantCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevicePropVariantCollection> for ::windows_core::IUnknown {
    fn from(value: &IPortableDevicePropVariantCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDevicePropVariantCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDevicePropVariantCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevicePropVariantCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDevicePropVariantCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropVariantCollection {}
impl ::core::fmt::Debug for IPortableDevicePropVariantCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropVariantCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDevicePropVariantCollection {
    type Vtable = IPortableDevicePropVariantCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89b2e422_4f1b_4316_bcef_a44afea83eb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropVariantCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetAt: usize,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceProperties(::windows_core::IUnknown);
impl IPortableDeviceProperties {
    pub unsafe fn GetSupportedProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszobjectid: Param0) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedProperties)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetPropertyAttributes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszobjectid: Param0, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyAttributes)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn GetValues<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetValues)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), pkeys.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn SetValues<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pszobjectid: Param0, pvalues: Param1) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SetValues)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), pvalues.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceProperties> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceProperties> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceProperties {}
impl ::core::fmt::Debug for IPortableDeviceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceProperties {
    type Vtable = IPortableDeviceProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f6d695c_03df_4439_a809_59266beee3a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceProperties_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSupportedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, ppkeys: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetPropertyAttributes: usize,
    pub GetValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pkeys: ::windows_core::RawPtr, ppvalues: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pvalues: ::windows_core::RawPtr, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pkeys: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDevicePropertiesBulk(::windows_core::IUnknown);
impl IPortableDevicePropertiesBulk {
    pub unsafe fn QueueGetValuesByObjectList<'a, Param0: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceKeyCollection>, Param2: ::windows_core::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pobjectids: Param0, pkeys: Param1, pcallback: Param2) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).QueueGetValuesByObjectList)(::windows_core::Interface::as_raw(self), pobjectids.into_param().abi(), pkeys.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn QueueGetValuesByObjectFormat<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IPortableDeviceKeyCollection>, Param4: ::windows_core::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pguidobjectformat: *const ::windows_core::GUID, pszparentobjectid: Param1, dwdepth: u32, pkeys: Param3, pcallback: Param4) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).QueueGetValuesByObjectFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidobjectformat), pszparentobjectid.into_param().abi(), ::core::mem::transmute(dwdepth), pkeys.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn QueueSetValuesByObjectList<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValuesCollection>, Param1: ::windows_core::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pobjectvalues: Param0, pcallback: Param1) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).QueueSetValuesByObjectList)(::windows_core::Interface::as_raw(self), pobjectvalues.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Start(&self, pcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn Cancel(&self, pcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontext)).ok()
    }
}
impl ::core::convert::From<IPortableDevicePropertiesBulk> for ::windows_core::IUnknown {
    fn from(value: IPortableDevicePropertiesBulk) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevicePropertiesBulk> for ::windows_core::IUnknown {
    fn from(value: &IPortableDevicePropertiesBulk) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDevicePropertiesBulk {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDevicePropertiesBulk {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevicePropertiesBulk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDevicePropertiesBulk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropertiesBulk {}
impl ::core::fmt::Debug for IPortableDevicePropertiesBulk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropertiesBulk").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDevicePropertiesBulk {
    type Vtable = IPortableDevicePropertiesBulk_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x482b05c0_4056_44ed_9e0f_5e23b009da93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulk_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueueGetValuesByObjectList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows_core::RawPtr, pkeys: ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr, pcontext: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub QueueGetValuesByObjectFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows_core::GUID, pszparentobjectid: ::windows_core::PCWSTR, dwdepth: u32, pkeys: ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr, pcontext: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub QueueSetValuesByObjectList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectvalues: ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr, pcontext: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDevicePropertiesBulkCallback(::windows_core::IUnknown);
impl IPortableDevicePropertiesBulkCallback {
    pub unsafe fn OnStart(&self, pcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn OnProgress<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValuesCollection>>(&self, pcontext: *const ::windows_core::GUID, presults: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontext), presults.into_param().abi()).ok()
    }
    pub unsafe fn OnEnd(&self, pcontext: *const ::windows_core::GUID, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnEnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcontext), ::core::mem::transmute(hrstatus)).ok()
    }
}
impl ::core::convert::From<IPortableDevicePropertiesBulkCallback> for ::windows_core::IUnknown {
    fn from(value: IPortableDevicePropertiesBulkCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevicePropertiesBulkCallback> for ::windows_core::IUnknown {
    fn from(value: &IPortableDevicePropertiesBulkCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDevicePropertiesBulkCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDevicePropertiesBulkCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevicePropertiesBulkCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDevicePropertiesBulkCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropertiesBulkCallback {}
impl ::core::fmt::Debug for IPortableDevicePropertiesBulkCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropertiesBulkCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDevicePropertiesBulkCallback {
    type Vtable = IPortableDevicePropertiesBulkCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9deacb80_11e8_40e3_a9f3_f557986a7845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID, presults: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceResources(::windows_core::IUnknown);
impl IPortableDeviceResources {
    pub unsafe fn GetSupportedResources<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszobjectid: Param0) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedResources)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetResourceAttributes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszobjectid: Param0, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetResourceAttributes)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn GetStream<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszobjectid: Param0, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<::win32_system::Com::IStream>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdwoptimalbuffersize), ::core::mem::transmute(ppstream)).ok()
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateResource<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, presourceattributes: Param0, ppdata: *mut ::core::option::Option<::win32_system::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateResource)(::windows_core::Interface::as_raw(self), presourceattributes.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceResources> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceResources) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceResources> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceResources) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceResources {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceResources {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceResources {}
impl ::core::fmt::Debug for IPortableDeviceResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceResources").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceResources {
    type Vtable = IPortableDeviceResources_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd8878ac_d841_4d17_891c_e6829cdb6934);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceResources_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSupportedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, ppkeys: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetResourceAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetResourceAttributes: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-ui")))]
    GetStream: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pkeys: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourceattributes: ::windows_core::RawPtr, ppdata: *mut ::windows_core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateResource: usize,
}
#[repr(transparent)]
pub struct IPortableDeviceService(::windows_core::IUnknown);
impl IPortableDeviceService {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pszpnpserviceid: Param0, pclientinfo: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    pub unsafe fn Capabilities(&self) -> ::windows_core::Result<IPortableDeviceServiceCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Capabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceServiceCapabilities>(result__)
    }
    pub unsafe fn Content(&self) -> ::windows_core::Result<IPortableDeviceContent2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Content)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceContent2>(result__)
    }
    pub unsafe fn Methods(&self) -> ::windows_core::Result<IPortableDeviceServiceMethods> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Methods)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceServiceMethods>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetServiceObjectID(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceObjectID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPnPServiceID(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetPnPServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn Advise<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceEventCallback>, Param2: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pcallback: Param1, pparameters: Param2) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi(), pparameters.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszcookie: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), pszcookie.into_param().abi()).ok()
    }
    pub unsafe fn SendCommand<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pparameters: Param1) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SendCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pparameters.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceService> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceService> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceService {}
impl ::core::fmt::Debug for IPortableDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceService {
    type Vtable = IPortableDeviceService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3bd3a44_d7b5_40a9_98b7_2fa4d01dec08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceService_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows_core::PCWSTR, pclientinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Methods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetServiceObjectID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPnPServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows_core::RawPtr, pparameters: ::windows_core::RawPtr, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcookie: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows_core::RawPtr, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceServiceActivation(::windows_core::IUnknown);
impl IPortableDeviceServiceActivation {
    pub unsafe fn OpenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>, Param2: ::windows_core::IntoParam<'a, IPortableDeviceServiceOpenCallback>>(&self, pszpnpserviceid: Param0, pclientinfo: Param1, pcallback: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenAsync)(::windows_core::Interface::as_raw(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelOpenAsync(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelOpenAsync)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceActivation> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceServiceActivation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceActivation> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceServiceActivation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceServiceActivation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceServiceActivation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceActivation {}
impl ::core::fmt::Debug for IPortableDeviceServiceActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceActivation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceServiceActivation {
    type Vtable = IPortableDeviceServiceActivation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe56b0534_d9b9_425c_9b99_75f97cb3d7c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceActivation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows_core::PCWSTR, pclientinfo: ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CancelOpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceServiceCapabilities(::windows_core::IUnknown);
impl IPortableDeviceServiceCapabilities {
    pub unsafe fn GetSupportedMethods(&self) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedMethods)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetSupportedMethodsByFormat(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedMethodsByFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetMethodAttributes(&self, method: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMethodAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(method), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetMethodParameterAttributes(&self, method: *const ::windows_core::GUID, parameter: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMethodParameterAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(method), ::core::mem::transmute(parameter), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn GetSupportedFormats(&self) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetFormatAttributes(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedFormatProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetFormatPropertyAttributes(&self, format: *const ::windows_core::GUID, property: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatPropertyAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(property), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn GetSupportedEvents(&self) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedEvents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetEventAttributes(&self, event: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(event), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetEventParameterAttributes(&self, event: *const ::windows_core::GUID, parameter: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventParameterAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(event), ::core::mem::transmute(parameter), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn GetInheritedServices(&self, dwinheritancetype: u32) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInheritedServices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinheritancetype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    pub unsafe fn GetFormatRenderingProfiles(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValuesCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatRenderingProfiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(format), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValuesCollection>(result__)
    }
    pub unsafe fn GetSupportedCommands(&self) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSupportedCommands)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetCommandOptions(&self, command: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCommandOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(command), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceCapabilities> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceServiceCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceCapabilities> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceServiceCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceServiceCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceServiceCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceCapabilities {}
impl ::core::fmt::Debug for IPortableDeviceServiceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceServiceCapabilities {
    type Vtable = IPortableDeviceServiceCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24dbd89d_413e_43e0_bd5b_197f3c56c886);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceCapabilities_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSupportedMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSupportedMethodsByFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppmethods: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMethodAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetMethodParameterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, parameter: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetMethodParameterAttributes: usize,
    pub GetSupportedFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformats: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFormatAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppkeys: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetFormatPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, property: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetFormatPropertyAttributes: usize,
    pub GetSupportedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetEventAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows_core::GUID, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetEventParameterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows_core::GUID, parameter: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetEventParameterAttributes: usize,
    pub GetInheritedServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFormatRenderingProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, pprenderingprofiles: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetCommandOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetCommandOptions: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceServiceManager(::windows_core::IUnknown);
impl IPortableDeviceServiceManager {
    pub unsafe fn GetDeviceServices<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpdeviceid: Param0, guidservicecategory: *const ::windows_core::GUID, pservices: *mut ::windows_core::PWSTR, pcservices: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceServices)(::windows_core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(guidservicecategory), ::core::mem::transmute(pservices), ::core::mem::transmute(pcservices)).ok()
    }
    pub unsafe fn GetDeviceForService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpnpserviceid: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceForService)(::windows_core::Interface::as_raw(self), pszpnpserviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceServiceManager> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceServiceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceManager> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceServiceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceServiceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceServiceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceManager {}
impl ::core::fmt::Debug for IPortableDeviceServiceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceServiceManager {
    type Vtable = IPortableDeviceServiceManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8abc4e9_a84a_47a9_80b3_c5d9b172a961);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDeviceServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, guidservicecategory: *const ::windows_core::GUID, pservices: *mut ::windows_core::PWSTR, pcservices: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceForService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows_core::PCWSTR, ppszpnpdeviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceServiceMethodCallback(::windows_core::IUnknown);
impl IPortableDeviceServiceMethodCallback {
    pub unsafe fn OnComplete<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, hrstatus: ::windows_core::HRESULT, presults: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrstatus), presults.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceMethodCallback> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceServiceMethodCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceMethodCallback> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceServiceMethodCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceServiceMethodCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceServiceMethodCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceMethodCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceMethodCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceMethodCallback {}
impl ::core::fmt::Debug for IPortableDeviceServiceMethodCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceMethodCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceServiceMethodCallback {
    type Vtable = IPortableDeviceServiceMethodCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc424233c_afce_4828_a756_7ed7a2350083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, presults: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceServiceMethods(::windows_core::IUnknown);
impl IPortableDeviceServiceMethods {
    pub unsafe fn Invoke<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, method: *const ::windows_core::GUID, pparameters: Param1, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(method), pparameters.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn InvokeAsync<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>, Param2: ::windows_core::IntoParam<'a, IPortableDeviceServiceMethodCallback>>(&self, method: *const ::windows_core::GUID, pparameters: Param1, pcallback: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeAsync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(method), pparameters.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn Cancel<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceServiceMethodCallback>>(&self, pcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceMethods> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceServiceMethods) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceMethods> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceServiceMethods) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceServiceMethods {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceServiceMethods {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceMethods {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceMethods {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceMethods {}
impl ::core::fmt::Debug for IPortableDeviceServiceMethods {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceMethods").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceServiceMethods {
    type Vtable = IPortableDeviceServiceMethods_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe20333c9_fd34_412d_a381_cc6f2d820df7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethods_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, pparameters: ::windows_core::RawPtr, ppresults: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InvokeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, pparameters: ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceServiceOpenCallback(::windows_core::IUnknown);
impl IPortableDeviceServiceOpenCallback {
    pub unsafe fn OnComplete(&self, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrstatus)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceOpenCallback> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceServiceOpenCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceOpenCallback> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceServiceOpenCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceServiceOpenCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceServiceOpenCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceOpenCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceOpenCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceOpenCallback {}
impl ::core::fmt::Debug for IPortableDeviceServiceOpenCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceOpenCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceServiceOpenCallback {
    type Vtable = IPortableDeviceServiceOpenCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbced49c8_8efe_41ed_960b_61313abd47a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceOpenCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceUnitsStream(::windows_core::IUnknown);
impl IPortableDeviceUnitsStream {
    pub unsafe fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).SeekInUnits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(units), ::core::mem::transmute(dworigin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceUnitsStream> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceUnitsStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceUnitsStream> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceUnitsStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceUnitsStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceUnitsStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceUnitsStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceUnitsStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceUnitsStream {}
impl ::core::fmt::Debug for IPortableDeviceUnitsStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceUnitsStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceUnitsStream {
    type Vtable = IPortableDeviceUnitsStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e98025f_bfc4_47a2_9a5f_bc900a507c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceUnitsStream_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SeekInUnits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceValues(::windows_core::IUnknown);
impl IPortableDeviceValues {
    pub unsafe fn GetCount(&self, pcelt: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn GetAt(&self, index: u32, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(pkey), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn SetValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn GetValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::win32_system::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetStringValue<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStringValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), value.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetStringValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetStringValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetUnsignedIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUnsignedIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetUnsignedIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetUnsignedIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetSignedIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSignedIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetSignedIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSignedIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetUnsignedLargeIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUnsignedLargeIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetUnsignedLargeIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetUnsignedLargeIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetSignedLargeIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSignedLargeIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetSignedLargeIntegerValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        (::windows_core::Interface::vtable(self).GetSignedLargeIntegerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetFloatValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFloatValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetFloatValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFloatValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetErrorValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetErrorValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetErrorValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetKeyValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKeyValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetKeyValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).GetKeyValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetBoolValue<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBoolValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), value.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetBoolValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetBoolValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetIUnknownValue<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIUnknownValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetIUnknownValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetIUnknownValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetGuidValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGuidValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetGuidValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetGuidValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetBufferValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBufferValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetBufferValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBufferValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(ppvalue), ::core::mem::transmute(pcbvalue)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetIPortableDeviceValuesValue<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIPortableDeviceValuesValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetIPortableDeviceValuesValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIPortableDeviceValuesValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetIPortableDevicePropVariantCollectionValue<'a, Param1: ::windows_core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIPortableDevicePropVariantCollectionValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIPortableDevicePropVariantCollectionValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetIPortableDeviceKeyCollectionValue<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIPortableDeviceKeyCollectionValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetIPortableDeviceKeyCollectionValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIPortableDeviceKeyCollectionValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn SetIPortableDeviceValuesCollectionValue<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValuesCollection>>(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIPortableDeviceValuesCollectionValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetIPortableDeviceValuesCollectionValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValuesCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIPortableDeviceValuesCollectionValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValuesCollection>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn RemoveValue(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key)).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn CopyValuesFromPropertyStore<'a, Param0: ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore>>(&self, pstore: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyValuesFromPropertyStore)(::windows_core::Interface::as_raw(self), pstore.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn CopyValuesToPropertyStore<'a, Param0: ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore>>(&self, pstore: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyValuesToPropertyStore)(::windows_core::Interface::as_raw(self), pstore.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceValues> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceValues) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceValues> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceValues) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceValues {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceValues {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceValues {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceValues {}
impl ::core::fmt::Debug for IPortableDeviceValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceValues").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceValues {
    type Vtable = IPortableDeviceValues_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6848f6f2_3155_4f86_b6f5_263eeeab3143);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValues_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-ui")))]
    GetAt: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-ui")))]
    SetValue: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-ui")))]
    GetValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetStringValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetStringValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetUnsignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetUnsignedIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetUnsignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetUnsignedIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetSignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetSignedIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetSignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetSignedIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetUnsignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetUnsignedLargeIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetUnsignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetUnsignedLargeIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetSignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetSignedLargeIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetSignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetSignedLargeIntegerValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetFloatValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetFloatValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetFloatValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetFloatValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetErrorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetErrorValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetErrorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetErrorValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetKeyValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetKeyValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetBoolValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetBoolValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetBoolValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetBoolValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetIUnknownValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetIUnknownValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetIUnknownValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetIUnknownValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetGuidValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetGuidValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetBufferValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetBufferValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetBufferValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetBufferValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetIPortableDeviceValuesValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetIPortableDeviceValuesValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetIPortableDeviceValuesValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetIPortableDeviceValuesValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(feature = "win32-ui")]
    pub SetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    SetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(feature = "win32-ui")]
    pub GetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(feature = "win32-ui")]
    pub RemoveValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    RemoveValue: usize,
    #[cfg(feature = "win32-ui")]
    pub CopyValuesFromPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstore: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    CopyValuesFromPropertyStore: usize,
    #[cfg(feature = "win32-ui")]
    pub CopyValuesToPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstore: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    CopyValuesToPropertyStore: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPortableDeviceValuesCollection(::windows_core::IUnknown);
impl IPortableDeviceValuesCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelems)).ok()
    }
    pub unsafe fn GetAt(&self, dwindex: u32) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pvalues.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceValuesCollection> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceValuesCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceValuesCollection> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceValuesCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceValuesCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceValuesCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceValuesCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceValuesCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceValuesCollection {}
impl ::core::fmt::Debug for IPortableDeviceValuesCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceValuesCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPortableDeviceValuesCollection {
    type Vtable = IPortableDeviceValuesCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e3f2d79_4e07_48c4_8208_d8c2e5af4a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPortableDeviceWebControl(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPortableDeviceWebControl {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetDeviceFromId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, deviceid: Param0) -> ::windows_core::Result<::win32_system::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceFromId)(::windows_core::Interface::as_raw(self), deviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetDeviceFromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch>>(&self, deviceid: Param0, pcompletionhandler: Param1, perrorhandler: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceFromIdAsync)(::windows_core::Interface::as_raw(self), deviceid.into_param().abi(), pcompletionhandler.into_param().abi(), perrorhandler.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPortableDeviceWebControl> for ::windows_core::IUnknown {
    fn from(value: IPortableDeviceWebControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPortableDeviceWebControl> for ::windows_core::IUnknown {
    fn from(value: &IPortableDeviceWebControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPortableDeviceWebControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPortableDeviceWebControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPortableDeviceWebControl> for ::win32_system::Com::IDispatch {
    fn from(value: IPortableDeviceWebControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPortableDeviceWebControl> for ::win32_system::Com::IDispatch {
    fn from(value: &IPortableDeviceWebControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IPortableDeviceWebControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IPortableDeviceWebControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPortableDeviceWebControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPortableDeviceWebControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPortableDeviceWebControl {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPortableDeviceWebControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceWebControl").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPortableDeviceWebControl {
    type Vtable = IPortableDeviceWebControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94fc7953_5ca1_483a_8aee_df52e7747d00);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceWebControl_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetDeviceFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetDeviceFromId: usize,
    #[cfg(feature = "win32-system")]
    pub GetDeviceFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pcompletionhandler: ::windows_core::RawPtr, perrorhandler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetDeviceFromIdAsync: usize,
}
#[repr(transparent)]
pub struct IRadioInstance(::windows_core::IUnknown);
impl IRadioInstance {
    pub unsafe fn GetRadioManagerSignature(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetRadioManagerSignature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetInstanceSignature(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceSignature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetFriendlyName(&self, lcid: u32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFriendlyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetRadioState(&self) -> ::windows_core::Result<DEVICE_RADIO_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<DEVICE_RADIO_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).GetRadioState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DEVICE_RADIO_STATE>(result__)
    }
    pub unsafe fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRadioState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(radiostate), ::core::mem::transmute(utimeoutsec)).ok()
    }
    pub unsafe fn IsMultiComm(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsMultiComm)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn IsAssociatingDevice(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsAssociatingDevice)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IRadioInstance> for ::windows_core::IUnknown {
    fn from(value: IRadioInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadioInstance> for ::windows_core::IUnknown {
    fn from(value: &IRadioInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRadioInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRadioInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRadioInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRadioInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadioInstance {}
impl ::core::fmt::Debug for IRadioInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadioInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRadioInstance {
    type Vtable = IRadioInstance_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70aa1c9e_f2b4_4c61_86d3_6b9fb75fd1a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstance_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetRadioManagerSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetInstanceSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows_core::HRESULT,
    pub SetRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::HRESULT,
    pub IsMultiComm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub IsAssociatingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
}
#[repr(transparent)]
pub struct IRadioInstanceCollection(::windows_core::IUnknown);
impl IRadioInstanceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, uindex: u32) -> ::windows_core::Result<IRadioInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRadioInstance>(result__)
    }
}
impl ::core::convert::From<IRadioInstanceCollection> for ::windows_core::IUnknown {
    fn from(value: IRadioInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadioInstanceCollection> for ::windows_core::IUnknown {
    fn from(value: &IRadioInstanceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRadioInstanceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRadioInstanceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRadioInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRadioInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadioInstanceCollection {}
impl ::core::fmt::Debug for IRadioInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadioInstanceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRadioInstanceCollection {
    type Vtable = IRadioInstanceCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5791fae_5665_4e0c_95be_5fde31644185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstanceCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWpdSerializer(::windows_core::IUnknown);
impl IWpdSerializer {
    pub unsafe fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: &[u8]) -> ::windows_core::Result<IPortableDeviceValues> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIPortableDeviceValuesFromBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPortableDeviceValues>(result__)
    }
    pub unsafe fn WriteIPortableDeviceValuesToBuffer<'a, Param1: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, presults: Param1, pbuffer: &mut [u8], pdwbyteswritten: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteIPortableDeviceValuesToBuffer)(::windows_core::Interface::as_raw(self), pbuffer.len() as _, presults.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pbuffer)), ::core::mem::transmute(pdwbyteswritten)).ok()
    }
    pub unsafe fn GetBufferFromIPortableDeviceValues<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, psource: Param0, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBufferFromIPortableDeviceValues)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pdwbuffersize)).ok()
    }
    pub unsafe fn GetSerializedSize<'a, Param0: ::windows_core::IntoParam<'a, IPortableDeviceValues>>(&self, psource: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSerializedSize)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWpdSerializer> for ::windows_core::IUnknown {
    fn from(value: IWpdSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWpdSerializer> for ::windows_core::IUnknown {
    fn from(value: &IWpdSerializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWpdSerializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWpdSerializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWpdSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWpdSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWpdSerializer {}
impl ::core::fmt::Debug for IWpdSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWpdSerializer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWpdSerializer {
    type Vtable = IWpdSerializer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb32f4002_bb27_45ff_af4f_06631c1e8dad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWpdSerializer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIPortableDeviceValuesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WriteIPortableDeviceValuesToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: ::windows_core::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows_core::HRESULT,
    pub GetBufferFromIPortableDeviceValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows_core::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows_core::HRESULT,
    pub GetSerializedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows_core::RawPtr, pdwsize: *mut u32) -> ::windows_core::HRESULT,
}
pub const NAME_3GPP2File: &str = "3GPP2File";
pub const NAME_3GPPFile: &str = "3GPPFile";
pub const NAME_AACFile: &str = "AACFile";
pub const NAME_AIFFFile: &str = "AIFFFile";
pub const NAME_AMRFile: &str = "AMRFile";
pub const NAME_ASFFile: &str = "ASFFile";
pub const NAME_ASXPlaylist: &str = "ASXPlaylist";
pub const NAME_ATSCTSFile: &str = "ATSCTSFile";
pub const NAME_AVCHDFile: &str = "AVCHDFile";
pub const NAME_AVIFile: &str = "AVIFile";
pub const NAME_AbstractActivity: &str = "AbstractActivity";
pub const NAME_AbstractActivityOccurrence: &str = "AbstractActivityOccurrence";
pub const NAME_AbstractAudioAlbum: &str = "AbstractAudioAlbum";
pub const NAME_AbstractAudioPlaylist: &str = "AbstractAudioPlaylist";
pub const NAME_AbstractAudioVideoAlbum: &str = "AbstractAudioVideoAlbum";
pub const NAME_AbstractChapteredProduction: &str = "AbstractChapteredProduction";
pub const NAME_AbstractContact: &str = "AbstractContact";
pub const NAME_AbstractContactGroup: &str = "AbstractContactGroup";
pub const NAME_AbstractDocument: &str = "AbstractDocument";
pub const NAME_AbstractImageAlbum: &str = "AbstractImageAlbum";
pub const NAME_AbstractMediacast: &str = "AbstractMediacast";
pub const NAME_AbstractMessage: &str = "AbstractMessage";
pub const NAME_AbstractMessageFolder: &str = "AbstractMessageFolder";
pub const NAME_AbstractMultimediaAlbum: &str = "AbstractMultimediaAlbum";
pub const NAME_AbstractNote: &str = "AbstractNote";
pub const NAME_AbstractTask: &str = "AbstractTask";
pub const NAME_AbstractVideoAlbum: &str = "AbstractVideoAlbum";
pub const NAME_AbstractVideoPlaylist: &str = "AbstractVideoPlaylist";
pub const NAME_AnchorResults: &str = "AnchorResults";
pub const NAME_AnchorResults_Anchor: &str = "Anchor";
pub const NAME_AnchorResults_AnchorState: &str = "AnchorState";
pub const NAME_AnchorResults_ResultObjectID: &str = "ResultObjectID";
pub const NAME_AnchorSyncKnowledge: &str = "AnchorSyncKnowledge";
pub const NAME_AnchorSyncSvc: &str = "AnchorSync";
pub const NAME_AnchorSyncSvc_BeginSync: &str = "BeginSync";
pub const NAME_AnchorSyncSvc_CurrentAnchor: &str = "AnchorCurrentAnchor";
pub const NAME_AnchorSyncSvc_EndSync: &str = "EndSync";
pub const NAME_AnchorSyncSvc_FilterType: &str = "FilterType";
pub const NAME_AnchorSyncSvc_GetChangesSinceAnchor: &str = "GetChangesSinceAnchor";
pub const NAME_AnchorSyncSvc_KnowledgeObjectID: &str = "AnchorKnowledgeObjectID";
pub const NAME_AnchorSyncSvc_LastSyncProxyID: &str = "AnchorLastSyncProxyID";
pub const NAME_AnchorSyncSvc_LocalOnlyDelete: &str = "LocalOnlyDelete";
pub const NAME_AnchorSyncSvc_ProviderVersion: &str = "AnchorProviderVersion";
pub const NAME_AnchorSyncSvc_ReplicaID: &str = "AnchorReplicaID";
pub const NAME_AnchorSyncSvc_SyncFormat: &str = "SyncFormat";
pub const NAME_AnchorSyncSvc_VersionProps: &str = "AnchorVersionProps";
pub const NAME_Association: &str = "Association";
pub const NAME_AudibleFile: &str = "AudibleFile";
pub const NAME_AudioObj_AudioBitDepth: &str = "AudioBitDepth";
pub const NAME_AudioObj_AudioBitRate: &str = "AudioBitRate";
pub const NAME_AudioObj_AudioBlockAlignment: &str = "AudioBlockAlignment";
pub const NAME_AudioObj_AudioFormatCode: &str = "AudioFormatCode";
pub const NAME_AudioObj_Channels: &str = "Channels";
pub const NAME_AudioObj_Lyrics: &str = "Lyrics";
pub const NAME_BMPImage: &str = "BMPImage";
pub const NAME_CIFFImage: &str = "CIFFImage";
pub const NAME_CalendarObj_Accepted: &str = "Accepted";
pub const NAME_CalendarObj_BeginDateTime: &str = "BeginDateTime";
pub const NAME_CalendarObj_BusyStatus: &str = "BusyStatus";
pub const NAME_CalendarObj_Declined: &str = "Declined";
pub const NAME_CalendarObj_EndDateTime: &str = "EndDateTime";
pub const NAME_CalendarObj_Location: &str = "Location";
pub const NAME_CalendarObj_PatternDuration: &str = "PatternDuration";
pub const NAME_CalendarObj_PatternStartTime: &str = "PatternStartTime";
pub const NAME_CalendarObj_ReminderOffset: &str = "ReminderOffset";
pub const NAME_CalendarObj_Tentative: &str = "Tentative";
pub const NAME_CalendarObj_TimeZone: &str = "TimeZone";
pub const NAME_CalendarSvc: &str = "Calendar";
pub const NAME_CalendarSvc_SyncWindowEnd: &str = "SyncWindowEnd";
pub const NAME_CalendarSvc_SyncWindowStart: &str = "SyncWindowStart";
pub const NAME_ContactObj_AnniversaryDate: &str = "AnniversaryDate";
pub const NAME_ContactObj_Assistant: &str = "Assistant";
pub const NAME_ContactObj_Birthdate: &str = "Birthdate";
pub const NAME_ContactObj_BusinessAddressCity: &str = "BusinessAddressCity";
pub const NAME_ContactObj_BusinessAddressCountry: &str = "BusinessAddressCountry";
pub const NAME_ContactObj_BusinessAddressFull: &str = "BusinessAddressFull";
pub const NAME_ContactObj_BusinessAddressLine2: &str = "BusinessAddressLine2";
pub const NAME_ContactObj_BusinessAddressPostalCode: &str = "BusinessAddressPostalCode";
pub const NAME_ContactObj_BusinessAddressRegion: &str = "BusinessAddressRegion";
pub const NAME_ContactObj_BusinessAddressStreet: &str = "BusinessAddressStreet";
pub const NAME_ContactObj_BusinessEmail: &str = "BusinessEmail";
pub const NAME_ContactObj_BusinessEmail2: &str = "BusinessEmail2";
pub const NAME_ContactObj_BusinessFax: &str = "BusinessFax";
pub const NAME_ContactObj_BusinessPhone: &str = "BusinessPhone";
pub const NAME_ContactObj_BusinessPhone2: &str = "BusinessPhone2";
pub const NAME_ContactObj_BusinessWebAddress: &str = "BusinessWebAddress";
pub const NAME_ContactObj_Children: &str = "Children";
pub const NAME_ContactObj_Email: &str = "Email";
pub const NAME_ContactObj_FamilyName: &str = "FamilyName";
pub const NAME_ContactObj_Fax: &str = "Fax";
pub const NAME_ContactObj_GivenName: &str = "GivenName";
pub const NAME_ContactObj_IMAddress: &str = "IMAddress";
pub const NAME_ContactObj_IMAddress2: &str = "IMAddress2";
pub const NAME_ContactObj_IMAddress3: &str = "IMAddress3";
pub const NAME_ContactObj_MiddleNames: &str = "MiddleNames";
pub const NAME_ContactObj_MobilePhone: &str = "MobilePhone";
pub const NAME_ContactObj_MobilePhone2: &str = "MobilePhone2";
pub const NAME_ContactObj_Organization: &str = "Organization";
pub const NAME_ContactObj_OtherAddressCity: &str = "OtherAddressCity";
pub const NAME_ContactObj_OtherAddressCountry: &str = "OtherAddressCountry";
pub const NAME_ContactObj_OtherAddressFull: &str = "OtherAddressFull";
pub const NAME_ContactObj_OtherAddressLine2: &str = "OtherAddressLine2";
pub const NAME_ContactObj_OtherAddressPostalCode: &str = "OtherAddressPostalCode";
pub const NAME_ContactObj_OtherAddressRegion: &str = "OtherAddressRegion";
pub const NAME_ContactObj_OtherAddressStreet: &str = "OtherAddressStreet";
pub const NAME_ContactObj_OtherEmail: &str = "OtherEmail";
pub const NAME_ContactObj_OtherPhone: &str = "OtherPhone";
pub const NAME_ContactObj_Pager: &str = "Pager";
pub const NAME_ContactObj_PersonalAddressCity: &str = "PersonalAddressCity";
pub const NAME_ContactObj_PersonalAddressCountry: &str = "PersonalAddressCountry";
pub const NAME_ContactObj_PersonalAddressFull: &str = "PersonalAddressFull";
pub const NAME_ContactObj_PersonalAddressLine2: &str = "PersonalAddressLine2";
pub const NAME_ContactObj_PersonalAddressPostalCode: &str = "PersonalAddressPostalCode";
pub const NAME_ContactObj_PersonalAddressRegion: &str = "PersonalAddressRegion";
pub const NAME_ContactObj_PersonalAddressStreet: &str = "PersonalAddressStreet";
pub const NAME_ContactObj_PersonalEmail: &str = "PersonalEmail";
pub const NAME_ContactObj_PersonalEmail2: &str = "PersonalEmail2";
pub const NAME_ContactObj_PersonalFax: &str = "PersonalFax";
pub const NAME_ContactObj_PersonalPhone: &str = "PersonalPhone";
pub const NAME_ContactObj_PersonalPhone2: &str = "PersonalPhone2";
pub const NAME_ContactObj_PersonalWebAddress: &str = "PersonalWebAddress";
pub const NAME_ContactObj_Phone: &str = "Phone";
pub const NAME_ContactObj_PhoneticFamilyName: &str = "PhoneticFamilyName";
pub const NAME_ContactObj_PhoneticGivenName: &str = "PhoneticGivenName";
pub const NAME_ContactObj_PhoneticOrganization: &str = "PhoneticOrganization";
pub const NAME_ContactObj_Ringtone: &str = "Ringtone";
pub const NAME_ContactObj_Role: &str = "Role";
pub const NAME_ContactObj_Spouse: &str = "Spouse";
pub const NAME_ContactObj_Suffix: &str = "Suffix";
pub const NAME_ContactObj_Title: &str = "Title";
pub const NAME_ContactObj_WebAddress: &str = "WebAddress";
pub const NAME_ContactSvc_SyncWithPhoneOnly: &str = "FilterType";
pub const NAME_ContactsSvc: &str = "Contacts";
pub const NAME_DPOFDocument: &str = "DPOFDocument";
pub const NAME_DVBTSFile: &str = "DVBTSFile";
pub const NAME_DeviceExecutable: &str = "DeviceExecutable";
pub const NAME_DeviceMetadataCAB: &str = "DeviceMetadataCAB";
pub const NAME_DeviceMetadataObj_ContentID: &str = "ContentID";
pub const NAME_DeviceMetadataObj_DefaultCAB: &str = "DefaultCAB";
pub const NAME_DeviceMetadataSvc: &str = "Metadata";
pub const NAME_DeviceScript: &str = "DeviceScript";
pub const NAME_EXIFImage: &str = "EXIFImage";
pub const NAME_ExcelDocument: &str = "ExcelDocument";
pub const NAME_FLACFile: &str = "FLACFile";
pub const NAME_FirmwareFile: &str = "FirmwareFile";
pub const NAME_FlashPixImage: &str = "FlashPixImage";
pub const NAME_FullEnumSyncKnowledge: &str = "FullEnumSyncKnowledge";
pub const NAME_FullEnumSyncSvc: &str = "FullEnumSync";
pub const NAME_FullEnumSyncSvc_BeginSync: &str = "BeginSync";
pub const NAME_FullEnumSyncSvc_EndSync: &str = "EndSync";
pub const NAME_FullEnumSyncSvc_FilterType: &str = "FilterType";
pub const NAME_FullEnumSyncSvc_KnowledgeObjectID: &str = "FullEnumKnowledgeObjectID";
pub const NAME_FullEnumSyncSvc_LastSyncProxyID: &str = "FullEnumLastSyncProxyID";
pub const NAME_FullEnumSyncSvc_LocalOnlyDelete: &str = "LocalOnlyDelete";
pub const NAME_FullEnumSyncSvc_ProviderVersion: &str = "FullEnumProviderVersion";
pub const NAME_FullEnumSyncSvc_ReplicaID: &str = "FullEnumReplicaID";
pub const NAME_FullEnumSyncSvc_SyncFormat: &str = "SyncFormat";
pub const NAME_FullEnumSyncSvc_VersionProps: &str = "FullEnumVersionProps";
pub const NAME_GIFImage: &str = "GIFImage";
pub const NAME_GenericObj_AllowedFolderContents: &str = "AllowedFolderContents";
pub const NAME_GenericObj_AssociationDesc: &str = "AssociationDesc";
pub const NAME_GenericObj_AssociationType: &str = "AssociationType";
pub const NAME_GenericObj_Copyright: &str = "Copyright";
pub const NAME_GenericObj_Corrupt: &str = "Corrupt";
pub const NAME_GenericObj_DRMStatus: &str = "DRMStatus";
pub const NAME_GenericObj_DateAccessed: &str = "DateAccessed";
pub const NAME_GenericObj_DateAdded: &str = "DateAdded";
pub const NAME_GenericObj_DateAuthored: &str = "DateAuthored";
pub const NAME_GenericObj_DateCreated: &str = "DateCreated";
pub const NAME_GenericObj_DateModified: &str = "DateModified";
pub const NAME_GenericObj_DateRevised: &str = "DateRevised";
pub const NAME_GenericObj_Description: &str = "Description";
pub const NAME_GenericObj_Hidden: &str = "Hidden";
pub const NAME_GenericObj_Keywords: &str = "Keywords";
pub const NAME_GenericObj_LanguageLocale: &str = "LanguageLocale";
pub const NAME_GenericObj_Name: &str = "Name";
pub const NAME_GenericObj_NonConsumable: &str = "NonConsumable";
pub const NAME_GenericObj_ObjectFileName: &str = "ObjectFileName";
pub const NAME_GenericObj_ObjectFormat: &str = "ObjectFormat";
pub const NAME_GenericObj_ObjectID: &str = "ObjectID";
pub const NAME_GenericObj_ObjectSize: &str = "ObjectSize";
pub const NAME_GenericObj_ParentID: &str = "ParentID";
pub const NAME_GenericObj_PersistentUID: &str = "PersistentUID";
pub const NAME_GenericObj_PropertyBag: &str = "PropertyBag";
pub const NAME_GenericObj_ProtectionStatus: &str = "ProtectionStatus";
pub const NAME_GenericObj_ReferenceParentID: &str = "ReferenceParentID";
pub const NAME_GenericObj_StorageID: &str = "StorageID";
pub const NAME_GenericObj_SubDescription: &str = "SubDescription";
pub const NAME_GenericObj_SyncID: &str = "SyncID";
pub const NAME_GenericObj_SystemObject: &str = "SystemObject";
pub const NAME_GenericObj_TimeToLive: &str = "TimeToLive";
pub const NAME_HDPhotoImage: &str = "HDPhotoImage";
pub const NAME_HTMLDocument: &str = "HTMLDocument";
pub const NAME_HintsSvc: &str = "Hints";
pub const NAME_ICalendarActivity: &str = "ICalendar";
pub const NAME_ImageObj_Aperature: &str = "Aperature";
pub const NAME_ImageObj_Exposure: &str = "Exposure";
pub const NAME_ImageObj_ISOSpeed: &str = "ISOSpeed";
pub const NAME_ImageObj_ImageBitDepth: &str = "ImageBitDepth";
pub const NAME_ImageObj_IsColorCorrected: &str = "IsColorCorrected";
pub const NAME_ImageObj_IsCropped: &str = "IsCropped";
pub const NAME_JFIFImage: &str = "JFIFImage";
pub const NAME_JP2Image: &str = "JP2Image";
pub const NAME_JPEGXRImage: &str = "JPEGXRImage";
pub const NAME_JPXImage: &str = "JPXImage";
pub const NAME_M3UPlaylist: &str = "M3UPlaylist";
pub const NAME_MHTDocument: &str = "MHTDocument";
pub const NAME_MP3File: &str = "MP3File";
pub const NAME_MPEG2File: &str = "MPEG2File";
pub const NAME_MPEG4File: &str = "MPEG4File";
pub const NAME_MPEGFile: &str = "MPEGFile";
pub const NAME_MPLPlaylist: &str = "MPLPlaylist";
pub const NAME_MediaObj_AlbumArtist: &str = "AlbumArtist";
pub const NAME_MediaObj_AlbumName: &str = "AlbumName";
pub const NAME_MediaObj_Artist: &str = "Artist";
pub const NAME_MediaObj_AudioEncodingProfile: &str = "AudioEncodingProfile";
pub const NAME_MediaObj_BitRateType: &str = "BitRateType";
pub const NAME_MediaObj_BookmarkByte: &str = "BookmarkByte";
pub const NAME_MediaObj_BookmarkObject: &str = "BookmarkObject";
pub const NAME_MediaObj_BookmarkTime: &str = "BookmarkTime";
pub const NAME_MediaObj_BufferSize: &str = "BufferSize";
pub const NAME_MediaObj_Composer: &str = "Composer";
pub const NAME_MediaObj_Credits: &str = "Credits";
pub const NAME_MediaObj_DateOriginalRelease: &str = "DateOriginalRelease";
pub const NAME_MediaObj_Duration: &str = "Duration";
pub const NAME_MediaObj_Editor: &str = "Editor";
pub const NAME_MediaObj_EffectiveRating: &str = "EffectiveRating";
pub const NAME_MediaObj_EncodingProfile: &str = "EncodingProfile";
pub const NAME_MediaObj_EncodingQuality: &str = "EncodingQuality";
pub const NAME_MediaObj_Genre: &str = "Genre";
pub const NAME_MediaObj_GeographicOrigin: &str = "GeographicOrigin";
pub const NAME_MediaObj_Height: &str = "Height";
pub const NAME_MediaObj_MediaType: &str = "MediaType";
pub const NAME_MediaObj_MediaUID: &str = "MediaUID";
pub const NAME_MediaObj_Mood: &str = "Mood";
pub const NAME_MediaObj_Owner: &str = "Owner";
pub const NAME_MediaObj_ParentalRating: &str = "ParentalRating";
pub const NAME_MediaObj_Producer: &str = "Producer";
pub const NAME_MediaObj_SampleRate: &str = "SampleRate";
pub const NAME_MediaObj_SkipCount: &str = "SkipCount";
pub const NAME_MediaObj_SubscriptionContentID: &str = "SubscriptionContentID";
pub const NAME_MediaObj_Subtitle: &str = "Subtitle";
pub const NAME_MediaObj_TotalBitRate: &str = "TotalBitRate";
pub const NAME_MediaObj_Track: &str = "Track";
pub const NAME_MediaObj_URLLink: &str = "URLLink";
pub const NAME_MediaObj_URLSource: &str = "URLSource";
pub const NAME_MediaObj_UseCount: &str = "UseCount";
pub const NAME_MediaObj_UserRating: &str = "UserRating";
pub const NAME_MediaObj_WebMaster: &str = "WebMaster";
pub const NAME_MediaObj_Width: &str = "Width";
pub const NAME_MessageObj_BCC: &str = "BCC";
pub const NAME_MessageObj_Body: &str = "Body";
pub const NAME_MessageObj_CC: &str = "CC";
pub const NAME_MessageObj_Category: &str = "Category";
pub const NAME_MessageObj_PatternDayOfMonth: &str = "PatternDayOfMonth";
pub const NAME_MessageObj_PatternDayOfWeek: &str = "PatternDayOfWeek";
pub const NAME_MessageObj_PatternDeleteDates: &str = "PatternDeleteDates";
pub const NAME_MessageObj_PatternInstance: &str = "PatternInstance";
pub const NAME_MessageObj_PatternMonthOfYear: &str = "PatternMonthOfYear";
pub const NAME_MessageObj_PatternOriginalDateTime: &str = "PatternOriginalDateTime";
pub const NAME_MessageObj_PatternPeriod: &str = "PatternPeriod";
pub const NAME_MessageObj_PatternType: &str = "PatternType";
pub const NAME_MessageObj_PatternValidEndDate: &str = "PatternValidEndDate";
pub const NAME_MessageObj_PatternValidStartDate: &str = "PatternValidStartDate";
pub const NAME_MessageObj_Priority: &str = "Priority";
pub const NAME_MessageObj_Read: &str = "Read";
pub const NAME_MessageObj_ReceivedTime: &str = "ReceivedTime";
pub const NAME_MessageObj_Sender: &str = "Sender";
pub const NAME_MessageObj_Subject: &str = "Subject";
pub const NAME_MessageObj_To: &str = "To";
pub const NAME_MessageSvc: &str = "Message";
pub const NAME_NotesSvc: &str = "Notes";
pub const NAME_OGGFile: &str = "OGGFile";
pub const NAME_PCDImage: &str = "PCDImage";
pub const NAME_PICTImage: &str = "PICTImage";
pub const NAME_PNGImage: &str = "PNGImage";
pub const NAME_PSLPlaylist: &str = "PSLPlaylist";
pub const NAME_PowerPointDocument: &str = "PowerPointDocument";
pub const NAME_QCELPFile: &str = "QCELPFile";
pub const NAME_RingtonesSvc: &str = "Ringtones";
pub const NAME_RingtonesSvc_DefaultRingtone: &str = "DefaultRingtone";
pub const NAME_Services_ServiceDisplayName: &str = "ServiceDisplayName";
pub const NAME_Services_ServiceIcon: &str = "ServiceIcon";
pub const NAME_Services_ServiceLocale: &str = "ServiceLocale";
pub const NAME_StatusSvc: &str = "Status";
pub const NAME_StatusSvc_BatteryLife: &str = "BatteryLife";
pub const NAME_StatusSvc_ChargingState: &str = "ChargingState";
pub const NAME_StatusSvc_MissedCalls: &str = "MissedCalls";
pub const NAME_StatusSvc_NetworkName: &str = "NetworkName";
pub const NAME_StatusSvc_NetworkType: &str = "NetworkType";
pub const NAME_StatusSvc_NewPictures: &str = "NewPictures";
pub const NAME_StatusSvc_Roaming: &str = "Roaming";
pub const NAME_StatusSvc_SignalStrength: &str = "SignalStrength";
pub const NAME_StatusSvc_StorageCapacity: &str = "StorageCapacity";
pub const NAME_StatusSvc_StorageFreeSpace: &str = "StorageFreeSpace";
pub const NAME_StatusSvc_TextMessages: &str = "TextMessages";
pub const NAME_StatusSvc_VoiceMail: &str = "VoiceMail";
pub const NAME_SyncObj_LastAuthorProxyID: &str = "LastAuthorProxyID";
pub const NAME_SyncSvc_BeginSync: &str = "BeginSync";
pub const NAME_SyncSvc_EndSync: &str = "EndSync";
pub const NAME_SyncSvc_FilterType: &str = "FilterType";
pub const NAME_SyncSvc_LocalOnlyDelete: &str = "LocalOnlyDelete";
pub const NAME_SyncSvc_SyncFormat: &str = "SyncFormat";
pub const NAME_SyncSvc_SyncObjectReferences: &str = "SyncObjectReferences";
pub const NAME_TIFFEPImage: &str = "TIFFEPImage";
pub const NAME_TIFFITImage: &str = "TIFFITImage";
pub const NAME_TIFFImage: &str = "TIFFImage";
pub const NAME_TaskObj_BeginDate: &str = "BeginDate";
pub const NAME_TaskObj_Complete: &str = "Complete";
pub const NAME_TaskObj_EndDate: &str = "EndDate";
pub const NAME_TaskObj_ReminderDateTime: &str = "ReminderDateTime";
pub const NAME_TasksSvc: &str = "Tasks";
pub const NAME_TasksSvc_SyncActiveOnly: &str = "FilterType";
pub const NAME_TextDocument: &str = "TextDocument";
pub const NAME_Undefined: &str = "Undefined";
pub const NAME_UndefinedAudio: &str = "UndefinedAudio";
pub const NAME_UndefinedCollection: &str = "UndefinedCollection";
pub const NAME_UndefinedDocument: &str = "UndefinedDocument";
pub const NAME_UndefinedVideo: &str = "UndefinedVideo";
pub const NAME_UnknownImage: &str = "UnknownImage";
pub const NAME_VCalendar1Activity: &str = "VCalendar1";
pub const NAME_VCard2Contact: &str = "VCard2Contact";
pub const NAME_VCard3Contact: &str = "VCard3Contact";
pub const NAME_VideoObj_KeyFrameDistance: &str = "KeyFrameDistance";
pub const NAME_VideoObj_ScanType: &str = "ScanType";
pub const NAME_VideoObj_Source: &str = "Source";
pub const NAME_VideoObj_VideoBitRate: &str = "VideoBitRate";
pub const NAME_VideoObj_VideoFormatCode: &str = "VideoFormatCode";
pub const NAME_VideoObj_VideoFrameRate: &str = "VideoFrameRate";
pub const NAME_WAVFile: &str = "WAVFile";
pub const NAME_WBMPImage: &str = "WBMPImage";
pub const NAME_WMAFile: &str = "WMAFile";
pub const NAME_WMVFile: &str = "WMVFile";
pub const NAME_WPLPlaylist: &str = "WPLPlaylist";
pub const NAME_WordDocument: &str = "WordDocument";
pub const NAME_XMLDocument: &str = "XMLDocument";
pub const PORTABLE_DEVICE_DRM_SCHEME_PDDRM: &str = "PDDRM";
pub const PORTABLE_DEVICE_DRM_SCHEME_WMDRM10_PD: &str = "WMDRM10-PD";
pub const PORTABLE_DEVICE_ICON: &str = "Icons";
pub const PORTABLE_DEVICE_IS_MASS_STORAGE: &str = "PortableDeviceIsMassStorage";
pub const PORTABLE_DEVICE_NAMESPACE_EXCLUDE_FROM_SHELL: &str = "PortableDeviceNameSpaceExcludeFromShell";
pub const PORTABLE_DEVICE_NAMESPACE_THUMBNAIL_CONTENT_TYPES: &str = "PortableDeviceNameSpaceThumbnailContentTypes";
pub const PORTABLE_DEVICE_NAMESPACE_TIMEOUT: &str = "PortableDeviceNameSpaceTimeout";
pub const PORTABLE_DEVICE_TYPE: &str = "PortableDeviceType";
pub const PortableDevice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x728a21c5_3d9e_48d7_9810_864848f0f404);
pub const PortableDeviceDispatchFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43232233_8338_4658_ae01_0b4ae830b6b0);
pub const PortableDeviceFTM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7c0039a_4762_488a_b4b3_760ef9a1ba9b);
pub const PortableDeviceKeyCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde2d022d_2480_43be_97f0_d1fa2cf98f4f);
pub const PortableDeviceManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0af10cec_2ecd_4b92_9581_34f6ae0637f3);
pub const PortableDevicePropVariantCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08a99e2f_6d6d_4b80_af5a_baf2bcbe4cb9);
pub const PortableDeviceService: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef5db4c2_9312_422c_9152_411cd9c4dd84);
pub const PortableDeviceServiceFTM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1649b154_c794_497a_9b03_f3f0121302f3);
pub const PortableDeviceValues: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c15d503_d017_47ce_9016_7b3f978721cc);
pub const PortableDeviceValuesCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3882134d_14cf_4220_9cb4_435f86d83f60);
pub const PortableDeviceWebControl: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x186dd02c_2dec_41b5_a7d4_b59056fade51);
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SMS_MESSAGE_TYPES(pub i32);
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(0i32);
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(1i32);
impl ::core::marker::Copy for SMS_MESSAGE_TYPES {}
impl ::core::clone::Clone for SMS_MESSAGE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SMS_MESSAGE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SMS_MESSAGE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SMS_MESSAGE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMS_MESSAGE_TYPES").field(&self.0).finish()
    }
}
pub const STR_WPDNSE_FAST_ENUM: &str = "WPDNSE Fast Enum";
pub const STR_WPDNSE_SIMPLE_ITEM: &str = "WPDNSE SimpleItem";
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_RADIO_STATE(pub i32);
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(0i32);
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(1i32);
impl ::core::marker::Copy for SYSTEM_RADIO_STATE {}
impl ::core::clone::Clone for SYSTEM_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYSTEM_RADIO_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_RADIO_STATE").field(&self.0).finish()
    }
}
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
pub const TYPE_CalendarSvc: u32 = 0u32;
pub const TYPE_ContactsSvc: u32 = 0u32;
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
pub const TYPE_HintsSvc: u32 = 0u32;
pub const TYPE_MessageSvc: u32 = 0u32;
pub const TYPE_NotesSvc: u32 = 0u32;
pub const TYPE_RingtonesSvc: u32 = 0u32;
pub const TYPE_StatusSvc: u32 = 0u32;
pub const TYPE_TasksSvc: u32 = 0u32;
#[cfg(feature = "win32-ui")]
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPDNSE_OBJECT_HAS_ICON: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 7u32 };
pub const WPDNSE_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6);
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
pub const WPD_API_OPTIONS_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4);
#[cfg(feature = "win32-ui")]
pub const WPD_API_OPTION_IOCTL_ACCESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_LOCATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 3u32 };
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3);
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_RESOURCES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_APPOINTMENT_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_AUDIO_BITRATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_AUDIO_BIT_DEPTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_AUDIO_BLOCK_ALIGNMENT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_AUDIO_CHANNEL_COUNT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_AUDIO_FORMAT_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 11u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_BITRATE_TYPES(pub i32);
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(0i32);
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(1i32);
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(2i32);
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(3i32);
impl ::core::marker::Copy for WPD_BITRATE_TYPES {}
impl ::core::clone::Clone for WPD_BITRATE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_BITRATE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_BITRATE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_BITRATE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_BITRATE_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_CAPTURE_MODES(pub i32);
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(0i32);
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(1i32);
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(2i32);
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(3i32);
impl ::core::marker::Copy for WPD_CAPTURE_MODES {}
impl ::core::clone::Clone for WPD_CAPTURE_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_CAPTURE_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_CAPTURE_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_CAPTURE_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_CAPTURE_MODES").field(&self.0).finish()
    }
}
pub const WPD_CATEGORY_CAPABILITIES: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356);
pub const WPD_CATEGORY_COMMON: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a);
pub const WPD_CATEGORY_DEVICE_HINTS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84);
pub const WPD_CATEGORY_MEDIA_CAPTURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8);
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56);
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4);
pub const WPD_CATEGORY_NULL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const WPD_CATEGORY_OBJECT_ENUMERATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec);
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089);
pub const WPD_CATEGORY_OBJECT_PROPERTIES: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804);
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e);
pub const WPD_CATEGORY_OBJECT_RESOURCES: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a);
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89);
pub const WPD_CATEGORY_SERVICE_COMMON: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee);
pub const WPD_CATEGORY_SERVICE_METHODS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc);
pub const WPD_CATEGORY_SMS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1);
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32);
pub const WPD_CATEGORY_STORAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94);
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 4u32 };
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96);
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f);
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30);
pub const WPD_CLASS_EXTENSION_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051);
pub const WPD_CLASS_EXTENSION_V2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758);
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_DESIRED_ACCESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_EVENT_COOKIE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 11u32 };
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859);
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_MAJOR_VERSION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_MINOR_VERSION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_REVISION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_SHARE_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 6u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_COLOR_CORRECTED_STATUS_VALUES(pub i32);
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(0i32);
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(1i32);
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(2i32);
impl ::core::marker::Copy for WPD_COLOR_CORRECTED_STATUS_VALUES {}
impl ::core::clone::Clone for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_COLOR_CORRECTED_STATUS_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_COLOR_CORRECTED_STATUS_VALUES").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "win32-ui")]
pub struct WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    pub Command: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY,
    pub AccessType: u32,
    pub AccessProperty: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "win32-ui")]
impl ::core::marker::Copy for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "win32-ui")]
impl ::core::clone::Clone for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-ui")]
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WPD_COMMAND_ACCESS_LOOKUP_ENTRY").field("Command", &self.Command).field("AccessType", &self.AccessType).field("AccessProperty", &self.AccessProperty).finish()
    }
}
#[cfg(feature = "win32-ui")]
unsafe impl ::windows_core::Abi for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "win32-ui")]
impl ::core::cmp::PartialEq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WPD_COMMAND_ACCESS_LOOKUP_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "win32-ui")]
impl ::core::cmp::Eq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "win32-ui")]
impl ::core::default::Default for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_COMMAND_ACCESS_TYPES(pub i32);
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(1i32);
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(3i32);
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(4i32);
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(8i32);
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(16i32);
impl ::core::marker::Copy for WPD_COMMAND_ACCESS_TYPES {}
impl ::core::clone::Clone for WPD_COMMAND_ACCESS_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_COMMAND_ACCESS_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_COMMAND_ACCESS_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_COMMAND_ACCESS_TYPES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_COMMIT_KEYPAIR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_COMMON_RESET_DEVICE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_GENERATE_KEYPAIR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MEDIA_CAPTURE_START: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 17u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 18u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_READ_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 16u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 16u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_SMS_SEND: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_STORAGE_EJECT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMAND_STORAGE_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMON_INFORMATION_BODY_TEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMON_INFORMATION_END_DATETIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMON_INFORMATION_NOTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 7u32 };
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f);
#[cfg(feature = "win32-ui")]
pub const WPD_COMMON_INFORMATION_PRIORITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMON_INFORMATION_START_DATETIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_COMMON_INFORMATION_SUBJECT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_ANNIVERSARY_DATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 62u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_ASSISTANT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 61u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BIRTHDATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 57u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_EMAIL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 34u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_EMAIL2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 35u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_FAX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 45u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 17u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_PHONE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 40u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_PHONE2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 41u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 20u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 23u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 18u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 19u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 22u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 21u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 50u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_CHILDREN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 60u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_COMPANY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 54u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_DISPLAY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_FIRST_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_INSTANT_MESSENGER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 51u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_INSTANT_MESSENGER2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 52u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_INSTANT_MESSENGER3: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 53u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_LAST_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_MIDDLE_NAMES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_MOBILE_PHONE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 42u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_MOBILE_PHONE2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 43u32 };
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b);
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_EMAILS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 36u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 24u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_PHONES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 47u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 27u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 25u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 26u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 29u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 30u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 28u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PAGER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 46u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_EMAIL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 32u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_EMAIL2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 33u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_FAX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 44u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_PHONE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 38u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_PHONE2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 39u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 16u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 49u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 55u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PHONETIC_LAST_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PREFIX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 31u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PRIMARY_FAX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 58u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PRIMARY_PHONE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 37u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 48u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_RINGTONE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 63u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_ROLE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 56u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_SPOUSE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 59u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_CONTACT_SUFFIX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 7u32 };
pub const WPD_CONTENT_TYPE_ALL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80e170d2_1055_4a3e_b952_82cc4f8a8689);
pub const WPD_CONTENT_TYPE_APPOINTMENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fed060e_8793_4b1e_90c9_48ac389ac631);
pub const WPD_CONTENT_TYPE_AUDIO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ad2c85e_5e2d_45e5_8864_4f229e3c6cf0);
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa18737e_5009_48fa_ae21_85f24383b4e6);
pub const WPD_CONTENT_TYPE_CALENDAR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1fd5967_6023_49a0_9df1_f8060be751b0);
pub const WPD_CONTENT_TYPE_CERTIFICATE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc3876e8_a948_4060_9050_cbd77e8a3d87);
pub const WPD_CONTENT_TYPE_CONTACT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeaba8313_4525_4707_9f0e_87c6808e9435);
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x346b8932_4c36_40d8_9415_1828291f9de9);
pub const WPD_CONTENT_TYPE_DOCUMENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x680adf52_950a_4041_9b41_65e393648155);
pub const WPD_CONTENT_TYPE_EMAIL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8038044a_7e51_4f8f_883d_1d0623d14533);
pub const WPD_CONTENT_TYPE_FOLDER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27e2e392_a111_48e0_ab0c_e17705a05f85);
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99ed0160_17ff_4c44_9d98_1d7a6f941921);
pub const WPD_CONTENT_TYPE_GENERIC_FILE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0085e0a6_8d34_45d7_bc5c_447e59c73d48);
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe80eaaf8_b2db_4133_b67e_1bef4b4a6e5f);
pub const WPD_CONTENT_TYPE_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef2107d5_a52a_4243_a26b_62d4176d7603);
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75793148_15f5_4a30_a813_54ed8a37e226);
pub const WPD_CONTENT_TYPE_MEDIA_CAST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e88b3cc_3e65_4e62_bfff_229495253ab0);
pub const WPD_CONTENT_TYPE_MEMO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cd20ecf_3b50_414f_a641_e473ffe45751);
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f0c3ac_a593_49ac_9219_24abca5a2563);
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x031da7ee_18c8_4205_847e_89a11261d0f3);
pub const WPD_CONTENT_TYPE_PLAYLIST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a33f7e4_af13_48f5_994e_77369dfe04a3);
pub const WPD_CONTENT_TYPE_PROGRAM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd269f96a_247c_4bff_98fb_97f3c49220e6);
pub const WPD_CONTENT_TYPE_SECTION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x821089f5_1d91_4dc9_be3c_bbb1b35b18ce);
pub const WPD_CONTENT_TYPE_TASK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63252f2c_887f_4cb6_b1ac_d29855dcef6c);
pub const WPD_CONTENT_TYPE_TELEVISION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60a169cf_f2ae_4e21_9375_9677f11c1c6e);
pub const WPD_CONTENT_TYPE_UNSPECIFIED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28d8d31e_249c_454e_aabc_34883168e634);
pub const WPD_CONTENT_TYPE_VIDEO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9261b03c_3d78_4519_85e3_02c5e1f50bb9);
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x012b0db7_d4c1_45d6_b081_94b87779614f);
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bac070a_9f5f_4da4_a8f6_3de44d68fd6c);
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_CROPPED_STATUS_VALUES(pub i32);
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(0i32);
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(1i32);
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(2i32);
impl ::core::marker::Copy for WPD_CROPPED_STATUS_VALUES {}
impl ::core::clone::Clone for WPD_CROPPED_STATUS_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_CROPPED_STATUS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_CROPPED_STATUS_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_CROPPED_STATUS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_CROPPED_STATUS_VALUES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_DATETIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_EDP_IDENTITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed), pid: 1u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_FIRMWARE_VERSION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_FRIENDLY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_MANUFACTURER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_MODEL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_MODEL_UNIQUE_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_NETWORK_IDENTIFIER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 16u32 };
pub const WPD_DEVICE_OBJECT_ID: &str = "DEVICE";
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_POWER_LEVEL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_POWER_SOURCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 5u32 };
pub const WPD_DEVICE_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc);
pub const WPD_DEVICE_PROPERTIES_V2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799);
pub const WPD_DEVICE_PROPERTIES_V3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed);
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_PROTOCOL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_SERIAL_NUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_SYNC_PARTNER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_TRANSPORT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 4u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_DEVICE_TRANSPORTS(pub i32);
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(0i32);
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(1i32);
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(2i32);
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(3i32);
impl ::core::marker::Copy for WPD_DEVICE_TRANSPORTS {}
impl ::core::clone::Clone for WPD_DEVICE_TRANSPORTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_DEVICE_TRANSPORTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_DEVICE_TRANSPORTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_DEVICE_TRANSPORTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_DEVICE_TRANSPORTS").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 15u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_DEVICE_TYPES(pub i32);
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(0i32);
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(1i32);
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(2i32);
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(3i32);
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(4i32);
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(5i32);
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(6i32);
impl ::core::marker::Copy for WPD_DEVICE_TYPES {}
impl ::core::clone::Clone for WPD_DEVICE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_DEVICE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_DEVICE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_DEVICE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_DEVICE_TYPES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_DEVICE_USE_DEVICE_STAGE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 5u32 };
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b110203_eb95_4f02_93e0_97c631493ad5);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_EFFECT_MODES(pub i32);
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = WPD_EFFECT_MODES(0i32);
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = WPD_EFFECT_MODES(1i32);
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = WPD_EFFECT_MODES(2i32);
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = WPD_EFFECT_MODES(3i32);
impl ::core::marker::Copy for WPD_EFFECT_MODES {}
impl ::core::clone::Clone for WPD_EFFECT_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_EFFECT_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_EFFECT_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_EFFECT_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EFFECT_MODES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_BCC_LINE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_CC_LINE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_HAS_ATTACHMENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_HAS_BEEN_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 7u32 };
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5);
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_RECEIVED_TIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_SENDER_ADDRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EMAIL_TO_LINE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 2u32 };
pub const WPD_EVENT_ATTRIBUTES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d);
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_ATTRIBUTE_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 3u32 };
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36885aa1_cd54_4daa_b3d0_afb3e03f5999);
pub const WPD_EVENT_DEVICE_REMOVED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4cbca1b_6918_48b9_85ee_02be7c850af9);
pub const WPD_EVENT_DEVICE_RESET: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7755cf53_c1ed_44f3_b5a2_451e2c376b27);
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_5738_4ff2_8445_be3126691059);
pub const WPD_EVENT_NOTIFICATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ba2e40a_6b4c_4295_bb43_26322b99aeb2);
pub const WPD_EVENT_OBJECT_ADDED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa726da95_e207_4b02_8d44_bef2e86cbffc);
pub const WPD_EVENT_OBJECT_REMOVED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe82ab88_a52c_4823_96e5_d0272671fc38);
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d16a0a1_f2c6_41da_8f19_5e53721adbf2);
pub const WPD_EVENT_OBJECT_UPDATED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1445a759_2e01_485d_9f27_ff07dae697ab);
pub const WPD_EVENT_OPTIONS_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b);
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_EVENT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846), pid: 2u32 };
pub const WPD_EVENT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0);
pub const WPD_EVENT_PROPERTIES_V2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846);
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a33f5f8_0acc_4d9b_9cc4_112d353b86ca);
pub const WPD_EVENT_STORAGE_FORMAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3782616b_22bc_4474_a251_3070f8d38857);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_EXPOSURE_METERING_MODES(pub i32);
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(0i32);
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(1i32);
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(2i32);
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(3i32);
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(4i32);
impl ::core::marker::Copy for WPD_EXPOSURE_METERING_MODES {}
impl ::core::clone::Clone for WPD_EXPOSURE_METERING_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_EXPOSURE_METERING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_EXPOSURE_METERING_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_EXPOSURE_METERING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EXPOSURE_METERING_MODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_EXPOSURE_PROGRAM_MODES(pub i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(0i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(1i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(2i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(3i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(4i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(5i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(6i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(7i32);
impl ::core::marker::Copy for WPD_EXPOSURE_PROGRAM_MODES {}
impl ::core::clone::Clone for WPD_EXPOSURE_PROGRAM_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_EXPOSURE_PROGRAM_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_EXPOSURE_PROGRAM_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_EXPOSURE_PROGRAM_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EXPOSURE_PROGRAM_MODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_FLASH_MODES(pub i32);
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = WPD_FLASH_MODES(0i32);
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(1i32);
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = WPD_FLASH_MODES(2i32);
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(3i32);
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(4i32);
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(5i32);
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = WPD_FLASH_MODES(6i32);
impl ::core::marker::Copy for WPD_FLASH_MODES {}
impl ::core::clone::Clone for WPD_FLASH_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_FLASH_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_FLASH_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_FLASH_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FLASH_MODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_FOCUS_METERING_MODES(pub i32);
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(0i32);
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(1i32);
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(2i32);
impl ::core::marker::Copy for WPD_FOCUS_METERING_MODES {}
impl ::core::clone::Clone for WPD_FOCUS_METERING_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_FOCUS_METERING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_FOCUS_METERING_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_FOCUS_METERING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FOCUS_METERING_MODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_FOCUS_MODES(pub i32);
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = WPD_FOCUS_MODES(0i32);
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = WPD_FOCUS_MODES(1i32);
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = WPD_FOCUS_MODES(2i32);
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = WPD_FOCUS_MODES(3i32);
impl ::core::marker::Copy for WPD_FOCUS_MODES {}
impl ::core::clone::Clone for WPD_FOCUS_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_FOCUS_MODES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_FOCUS_MODES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_FOCUS_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FOCUS_MODES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d), pid: 2u32 };
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d);
pub const WPD_FORMAT_ATTRIBUTES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f);
#[cfg(feature = "win32-ui")]
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_FORMAT_ATTRIBUTE_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 2u32 };
pub const WPD_FUNCTIONAL_CATEGORY_ALL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d8a6512_a74c_448e_ba8a_f4ac07c49399);
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f2a1919_c7c2_4a00_855d_f57cf06debbb);
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08ea466b_e3a4_4336_a1f3_a44d2b5c438c);
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48f4db72_7c6a_4ab0_9e1a_470e3cdbf26a);
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08600ba4_a7ba_4a01_ab0e_0065d0a356d3);
pub const WPD_FUNCTIONAL_CATEGORY_SMS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0044a0b1_c1e9_4afd_b358_a62c6117c9cf);
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x613ca327_ab93_4900_b4fa_895bb5874b79);
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23f05bbc_15de_4c2a_a55b_a9af5ce412ef);
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe23e5f6b_7243_43aa_8df1_0eb3d968a918);
#[cfg(feature = "win32-ui")]
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598), pid: 2u32 };
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598);
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_BITDEPTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_CROPPED_STATUS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_EXPOSURE_INDEX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_EXPOSURE_TIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_FNUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 9u32 };
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db);
#[cfg(feature = "win32-ui")]
pub const WPD_IMAGE_VERTICAL_RESOLUTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_ALBUM_ARTIST: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 25u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_ARTIST: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 24u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 49u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_BITRATE_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_BUY_NOW: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 20u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_BYTE_BOOKMARK: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 36u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_COMPOSER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_COPYRIGHT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 31u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_DESTINATION_URL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 30u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_DURATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 19u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_EFFECTIVE_RATING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_ENCODING_PROFILE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 21u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_GENRE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 32u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_GUID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 38u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_HEIGHT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 23u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_LAST_ACCESSED_TIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_LAST_BUILD_DATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 35u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_MANAGING_EDITOR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 27u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_META_GENRE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_OBJECT_BOOKMARK: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 34u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_OWNER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 26u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_PARENTAL_RATING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 9u32 };
pub const WPD_MEDIA_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8);
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_RELEASE_DATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_SAMPLE_RATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_SKIP_COUNT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_SOURCE_URL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 29u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_STAR_RATING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 16u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_SUB_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 39u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_SUB_TITLE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_TIME_BOOKMARK: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 33u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_TIME_TO_LIVE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 37u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_TITLE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 18u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_TOTAL_BITRATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 17u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_USE_COUNT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_WEBMASTER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 28u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MEDIA_WIDTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 22u32 };
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ffbfc7b_7483_41ad_afb9_da3f4e592b8d);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_META_GENRES(pub i32);
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = WPD_META_GENRES(0i32);
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(1i32);
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(17i32);
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = WPD_META_GENRES(18i32);
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = WPD_META_GENRES(19i32);
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = WPD_META_GENRES(20i32);
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = WPD_META_GENRES(21i32);
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(33i32);
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(34i32);
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(35i32);
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(36i32);
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(37i32);
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(38i32);
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(39i32);
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(40i32);
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = WPD_META_GENRES(48i32);
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(64i32);
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(65i32);
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = WPD_META_GENRES(66i32);
impl ::core::marker::Copy for WPD_META_GENRES {}
impl ::core::clone::Clone for WPD_META_GENRES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_META_GENRES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_META_GENRES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_META_GENRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_META_GENRES").field(&self.0).finish()
    }
}
pub const WPD_METHOD_ATTRIBUTES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a);
#[cfg(feature = "win32-ui")]
pub const WPD_METHOD_ATTRIBUTE_ACCESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_METHOD_ATTRIBUTE_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MUSIC_ALBUM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MUSIC_LYRICS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_MUSIC_MOOD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 8u32 };
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6);
#[cfg(feature = "win32-ui")]
pub const WPD_MUSIC_TRACK: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 2u32 };
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274);
#[cfg(feature = "win32-ui")]
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_BACK_REFERENCES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 21u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_CAN_DELETE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 26u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 23u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_CONTENT_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_DATE_AUTHORED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 20u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_DATE_CREATED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 18u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_DATE_MODIFIED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 19u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 6u32 };
pub const WPD_OBJECT_FORMAT_3G2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9850000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_3G2A: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a11202d_8759_4e34_ba5e_b1211087eee4);
pub const WPD_OBJECT_FORMAT_3GP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9840000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_3GPA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5172730_f971_41ef_a10b_2271a0019d7a);
pub const WPD_OBJECT_FORMAT_AAC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba0b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AIFF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ALL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1f62eb2_4bb3_479c_9cfa_05b5f3a57b22);
pub const WPD_OBJECT_FORMAT_AMR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ASF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x300c0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba130000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ATSCTS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9870000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AUDIBLE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AVCHD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9860000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AVI: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x300a0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_BMP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_CIFF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38050000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_DPOF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_DVBTS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9880000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_EXECUTABLE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_EXIF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_FLAC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_FLASHPIX: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_GIF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_HTML: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30050000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ICALENDAR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ICON: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x077232ed_102c_4638_9c22_83f142bfc822);
pub const WPD_OBJECT_FORMAT_JFIF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JP2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380f0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JPEGXR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JPX: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38100000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba110000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_M4A: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30aba7ac_6ffd_4c23_a359_3e9b52f3f1c8);
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba840000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba850000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba860000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MKV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9900000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30090000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP4: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MPEG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x300b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba120000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_OGG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PCD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38090000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PICT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380a0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba140000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PNG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_QCELP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_SCRIPT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TEXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380d0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFFEP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFFIT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380e0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30000000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCALENDAR1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCARD2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCARD3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WAVE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WBMP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WMA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WMV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba100000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_XML: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba820000_ae6c_4804_98ba_c57b46965fe7);
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 24u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 25u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_ISHIDDEN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_ISSYSTEM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_IS_DRM_PROTECTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 17u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_KEYWORDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_LANGUAGE_LOCALE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 27u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_NON_CONSUMABLE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_PARENT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 5u32 };
pub const WPD_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c);
pub const WPD_OBJECT_PROPERTIES_V2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775);
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_REFERENCES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_SUPPORTED_UNITS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OBJECT_SYNC_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 16u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_OPERATION_STATES(pub i32);
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(0i32);
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(1i32);
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = WPD_OPERATION_STATES(2i32);
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(3i32);
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(4i32);
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(5i32);
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(6i32);
impl ::core::marker::Copy for WPD_OPERATION_STATES {}
impl ::core::clone::Clone for WPD_OPERATION_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_OPERATION_STATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_OPERATION_STATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_OPERATION_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_OPERATION_STATES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 5001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_OPTION_VALID_OBJECT_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 5001u32 };
pub const WPD_PARAMETER_ATTRIBUTES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58);
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_FORM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 12u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_PARAMETER_USAGE_TYPES(pub i32);
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(0i32);
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(1i32);
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(2i32);
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(3i32);
impl ::core::marker::Copy for WPD_PARAMETER_USAGE_TYPES {}
impl ::core::clone::Clone for WPD_PARAMETER_USAGE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_PARAMETER_USAGE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_PARAMETER_USAGE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_PARAMETER_USAGE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_PARAMETER_USAGE_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_POWER_SOURCES(pub i32);
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = WPD_POWER_SOURCES(0i32);
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = WPD_POWER_SOURCES(1i32);
impl ::core::marker::Copy for WPD_POWER_SOURCES {}
impl ::core::clone::Clone for WPD_POWER_SOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_POWER_SOURCES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_POWER_SOURCES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_POWER_SOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_POWER_SOURCES").field(&self.0).finish()
    }
}
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d545058_8900_40b3_8f1d_dc246e1e8370);
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d545058_4fce_4578_95c8_8698a9bc0f49);
pub const WPD_PROPERTY_ATTRIBUTES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37);
pub const WPD_PROPERTY_ATTRIBUTES_V2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e);
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_FORM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1014u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1015u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1010u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1009u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1012u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1013u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1009u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1010u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_COMMAND_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_HRESULT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_ef88_4e4d_95c3_4f327f728a96), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1013u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1012u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1009u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1010u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1014u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_NULL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1013u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1010u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1012u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1016u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1009u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1015u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1014u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1010u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1009u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1014u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1012u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1013u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1016u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1015u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_PUBLIC_KEY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1018u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1019u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1012u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1013u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1007u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1008u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1014u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1015u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1006u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1010u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1009u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1016u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1017u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1011u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_METHOD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1005u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1004u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SMS_RECIPIENT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1001u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1003u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1002u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1001u32 };
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4);
#[cfg(feature = "win32-ui")]
pub const WPD_RENDERING_INFORMATION_PROFILES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 3u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(pub i32);
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(0i32);
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(1i32);
impl ::core::marker::Copy for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {}
impl ::core::clone::Clone for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ALBUM_ART: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf02aa354_2300_4e2d_a1b9_3b6730f7fa21), pid: 0u32 };
pub const WPD_RESOURCE_ATTRIBUTES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6);
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_AUDIO_CLIP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3bc13982_85b1_48e0_95a6_8d3ad06be117), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_BRANDING_ART: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb633b1ae_6caf_4a87_9589_22ded6dd5899), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_CONTACT_PHOTO: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2c4d6803_80ea_4580_af9a_5be1a23eddcb), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_DEFAULT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe81e79be_34f0_41bf_b53f_f1a06ae87842), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_GENERIC: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb9b9f515_ba70_4647_94dc_fa4925e95a07), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_ICON: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf195fed8_aa28_4ee3_b153_e182dd5edc39), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_THUMBNAIL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc7c407ba_98fa_46b5_9960_23fec124cfde), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_RESOURCE_VIDEO_CLIP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb566ee42_6368_4290_8662_70182fb79f20), pid: 0u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_SECTION_DATA_LENGTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_SECTION_DATA_OFFSET: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_SECTION_DATA_UNITS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 4u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_SECTION_DATA_UNITS_VALUES(pub i32);
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(0i32);
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(1i32);
impl ::core::marker::Copy for WPD_SECTION_DATA_UNITS_VALUES {}
impl ::core::clone::Clone for WPD_SECTION_DATA_UNITS_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_SECTION_DATA_UNITS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_SECTION_DATA_UNITS_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_SECTION_DATA_UNITS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SECTION_DATA_UNITS_VALUES").field(&self.0).finish()
    }
}
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_SERVICE_INHERITANCE_TYPES(pub i32);
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = WPD_SERVICE_INHERITANCE_TYPES(0i32);
impl ::core::marker::Copy for WPD_SERVICE_INHERITANCE_TYPES {}
impl ::core::clone::Clone for WPD_SERVICE_INHERITANCE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_SERVICE_INHERITANCE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_SERVICE_INHERITANCE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_SERVICE_INHERITANCE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SERVICE_INHERITANCE_TYPES").field(&self.0).finish()
    }
}
pub const WPD_SERVICE_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06);
#[cfg(feature = "win32-ui")]
pub const WPD_SERVICE_VERSION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_SMS_ENCODING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 5u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_SMS_ENCODING_TYPES(pub i32);
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(0i32);
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(1i32);
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(2i32);
impl ::core::marker::Copy for WPD_SMS_ENCODING_TYPES {}
impl ::core::clone::Clone for WPD_SMS_ENCODING_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_SMS_ENCODING_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_SMS_ENCODING_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_SMS_ENCODING_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SMS_ENCODING_TYPES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_SMS_MAX_PAYLOAD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 4u32 };
pub const WPD_SMS_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d);
#[cfg(feature = "win32-ui")]
pub const WPD_SMS_PROVIDER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_SMS_TIMEOUT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_ARTIST: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 29u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_BURST_INTERVAL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 24u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_BURST_NUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 23u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 31u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CAMERA_MODEL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 30u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 17u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CAPTURE_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 18u32 };
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260);
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_CONTRAST: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 19u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 21u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_EFFECT_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 22u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 16u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_FLASH_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 12u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_FNUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 27u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_FOCUS_MODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_RGB_GAIN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_SHARPNESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 20u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 26u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 25u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_UPLOAD_URL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 28u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STILL_IMAGE_WHITE_BALANCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_ACCESS_CAPABILITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 11u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_STORAGE_ACCESS_CAPABILITY_VALUES(pub i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(0i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(1i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(2i32);
impl ::core::marker::Copy for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {}
impl ::core::clone::Clone for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STORAGE_ACCESS_CAPABILITY_VALUES").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_CAPACITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_DESCRIPTION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 3u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_MAX_OBJECT_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 9u32 };
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a);
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_SERIAL_NUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_STORAGE_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 2u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_STORAGE_TYPE_VALUES(pub i32);
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(0i32);
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(1i32);
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(2i32);
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(3i32);
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(4i32);
impl ::core::marker::Copy for WPD_STORAGE_TYPE_VALUES {}
impl ::core::clone::Clone for WPD_STORAGE_TYPE_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_STORAGE_TYPE_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_STORAGE_TYPE_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_STORAGE_TYPE_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STORAGE_TYPE_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_STREAM_UNITS(pub i32);
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(0i32);
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(1i32);
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(2i32);
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(4i32);
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(8i32);
impl ::core::marker::Copy for WPD_STREAM_UNITS {}
impl ::core::clone::Clone for WPD_STREAM_UNITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_STREAM_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_STREAM_UNITS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_STREAM_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STREAM_UNITS").field(&self.0).finish()
    }
}
pub const WPD_TASK_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7);
#[cfg(feature = "win32-ui")]
pub const WPD_TASK_OWNER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_TASK_PERCENT_COMPLETE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_TASK_REMINDER_DATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 10u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_TASK_STATUS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 6u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_AUTHOR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 2u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_BITRATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 13u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_BUFFER_SIZE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 8u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_CREDITS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 9u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_FOURCC_CODE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 14u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_FRAMERATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 15u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 10u32 };
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a);
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_QUALITY_SETTING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 11u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 5u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_RECORDEDTV_REPEAT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 7u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 4u32 };
#[cfg(feature = "win32-ui")]
pub const WPD_VIDEO_SCAN_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 12u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_VIDEO_SCAN_TYPES(pub i32);
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(0i32);
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(1i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(2i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(3i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(4i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(5i32);
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(6i32);
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(7i32);
impl ::core::marker::Copy for WPD_VIDEO_SCAN_TYPES {}
impl ::core::clone::Clone for WPD_VIDEO_SCAN_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_VIDEO_SCAN_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_VIDEO_SCAN_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_VIDEO_SCAN_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_VIDEO_SCAN_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPD_WHITE_BALANCE_SETTINGS(pub i32);
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(0i32);
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(1i32);
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(2i32);
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(3i32);
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(4i32);
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(5i32);
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(6i32);
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(7i32);
impl ::core::marker::Copy for WPD_WHITE_BALANCE_SETTINGS {}
impl ::core::clone::Clone for WPD_WHITE_BALANCE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_WHITE_BALANCE_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WPD_WHITE_BALANCE_SETTINGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPD_WHITE_BALANCE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_WHITE_BALANCE_SETTINGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WpdAttributeForm(pub i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = WpdAttributeForm(0i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = WpdAttributeForm(1i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = WpdAttributeForm(2i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = WpdAttributeForm(3i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = WpdAttributeForm(4i32);
impl ::core::marker::Copy for WpdAttributeForm {}
impl ::core::clone::Clone for WpdAttributeForm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WpdAttributeForm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WpdAttributeForm {
    type Abi = Self;
}
impl ::core::fmt::Debug for WpdAttributeForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WpdAttributeForm").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WpdParameterAttributeForm(pub i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = WpdParameterAttributeForm(0i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = WpdParameterAttributeForm(1i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = WpdParameterAttributeForm(2i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = WpdParameterAttributeForm(3i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = WpdParameterAttributeForm(4i32);
impl ::core::marker::Copy for WpdParameterAttributeForm {}
impl ::core::clone::Clone for WpdParameterAttributeForm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WpdParameterAttributeForm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WpdParameterAttributeForm {
    type Abi = Self;
}
impl ::core::fmt::Debug for WpdParameterAttributeForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WpdParameterAttributeForm").field(&self.0).finish()
    }
}
pub const WpdSerializer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b91a74b_ad7c_4a9d_b563_29eef9167172);
