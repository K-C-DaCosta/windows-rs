pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
#[repr(transparent)]
pub struct INetworkTransportSettings(::windows_core::IUnknown);
impl INetworkTransportSettings {
    #[cfg(feature = "win32-networking")]
    pub unsafe fn ApplySetting(&self, settingid: *const ::win32_networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplySetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(settingid), valuein.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(valuein)), ::core::mem::transmute(lengthout), ::core::mem::transmute(valueout)).ok()
    }
    #[cfg(feature = "win32-networking")]
    pub unsafe fn QuerySetting(&self, settingid: *const ::win32_networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QuerySetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(settingid), valuein.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(valuein)), ::core::mem::transmute(lengthout), ::core::mem::transmute(valueout)).ok()
    }
}
impl ::core::convert::From<INetworkTransportSettings> for ::windows_core::IUnknown {
    fn from(value: INetworkTransportSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkTransportSettings> for ::windows_core::IUnknown {
    fn from(value: &INetworkTransportSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetworkTransportSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetworkTransportSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkTransportSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkTransportSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkTransportSettings {}
impl ::core::fmt::Debug for INetworkTransportSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkTransportSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetworkTransportSettings {
    type Vtable = INetworkTransportSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e7abb2c_f2c1_4a61_bd35_deb7a08ab0f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkTransportSettings_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-networking")]
    pub ApplySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingid: *const ::win32_networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-networking"))]
    ApplySetting: usize,
    #[cfg(feature = "win32-networking")]
    pub QuerySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingid: *const ::win32_networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-networking"))]
    QuerySetting: usize,
}
#[repr(transparent)]
pub struct INotificationTransportSync(::windows_core::IUnknown);
impl INotificationTransportSync {
    pub unsafe fn CompleteDelivery(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompleteDelivery)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<INotificationTransportSync> for ::windows_core::IUnknown {
    fn from(value: INotificationTransportSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotificationTransportSync> for ::windows_core::IUnknown {
    fn from(value: &INotificationTransportSync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INotificationTransportSync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INotificationTransportSync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INotificationTransportSync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotificationTransportSync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotificationTransportSync {}
impl ::core::fmt::Debug for INotificationTransportSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotificationTransportSync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INotificationTransportSync {
    type Vtable = INotificationTransportSync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79eb1402_0ab8_49c0_9e14_a1ae4ba93058);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationTransportSync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CompleteDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCBuddy(::windows_core::IUnknown);
impl IRTCBuddy {
    pub unsafe fn PresentityURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PresentityURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPresentityURI)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Persistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPersistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PRESENCE_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PRESENCE_STATUS>(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Notes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IRTCBuddy> for ::windows_core::IUnknown {
    fn from(value: IRTCBuddy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy> for ::windows_core::IUnknown {
    fn from(value: &IRTCBuddy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCBuddy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCBuddy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCBuddy> for IRTCPresenceContact {
    fn from(value: IRTCBuddy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy> for IRTCPresenceContact {
    fn from(value: &IRTCBuddy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for IRTCBuddy {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for &'a IRTCBuddy {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCBuddy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCBuddy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCBuddy {}
impl ::core::fmt::Debug for IRTCBuddy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCBuddy {
    type Vtable = IRTCBuddy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcb136c8_7b90_4e0c_befe_56edf0ba6f1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCBuddy2(::windows_core::IUnknown);
impl IRTCBuddy2 {
    pub unsafe fn PresentityURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PresentityURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPresentityURI)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Persistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPersistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PRESENCE_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PRESENCE_STATUS>(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Notes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> ::windows_core::Result<IRTCEnumGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumGroups>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Groups(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Groups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_PresenceProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn EnumeratePresenceDevices(&self) -> ::windows_core::Result<IRTCEnumPresenceDevices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumeratePresenceDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumPresenceDevices>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PresenceDevices(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PresenceDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn SubscriptionType(&self) -> ::windows_core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_BUDDY_SUBSCRIPTION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).SubscriptionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_BUDDY_SUBSCRIPTION_TYPE>(result__)
    }
}
impl ::core::convert::From<IRTCBuddy2> for ::windows_core::IUnknown {
    fn from(value: IRTCBuddy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy2> for ::windows_core::IUnknown {
    fn from(value: &IRTCBuddy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCBuddy2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCBuddy2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCBuddy2> for IRTCPresenceContact {
    fn from(value: IRTCBuddy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy2> for IRTCPresenceContact {
    fn from(value: &IRTCBuddy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for IRTCBuddy2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for &'a IRTCBuddy2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCBuddy2> for IRTCBuddy {
    fn from(value: IRTCBuddy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy2> for IRTCBuddy {
    fn from(value: &IRTCBuddy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCBuddy> for IRTCBuddy2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCBuddy> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCBuddy> for &'a IRTCBuddy2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCBuddy> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCBuddy2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCBuddy2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCBuddy2 {}
impl ::core::fmt::Debug for IRTCBuddy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddy2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCBuddy2 {
    type Vtable = IRTCBuddy2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x102f9588_23e7_40e3_954d_cd7a1d5c0361);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy2_Vtbl {
    pub base__: IRTCBuddy_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Groups: usize,
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub EnumeratePresenceDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub PresenceDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PresenceDevices: usize,
    pub SubscriptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCBuddyEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCBuddyEvent {
    pub unsafe fn Buddy(&self) -> ::windows_core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Buddy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCBuddyEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCBuddyEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCBuddyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCBuddyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyEvent> for super::Com::IDispatch {
    fn from(value: IRTCBuddyEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyEvent> for super::Com::IDispatch {
    fn from(value: &IRTCBuddyEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCBuddyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCBuddyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCBuddyEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCBuddyEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCBuddyEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCBuddyEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCBuddyEvent {
    type Vtable = IRTCBuddyEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf36d755d_17e6_404e_954f_0fc07574c78d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Buddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCBuddyEvent2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCBuddyEvent2 {
    pub unsafe fn Buddy(&self) -> ::windows_core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Buddy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_BUDDY_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_BUDDY_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_BUDDY_EVENT_TYPE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyEvent2> for ::windows_core::IUnknown {
    fn from(value: IRTCBuddyEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyEvent2> for ::windows_core::IUnknown {
    fn from(value: &IRTCBuddyEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyEvent2> for super::Com::IDispatch {
    fn from(value: IRTCBuddyEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCBuddyEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyEvent2> for IRTCBuddyEvent {
    fn from(value: IRTCBuddyEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyEvent2> for IRTCBuddyEvent {
    fn from(value: &IRTCBuddyEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCBuddyEvent> for IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCBuddyEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCBuddyEvent> for &'a IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCBuddyEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCBuddyEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCBuddyEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCBuddyEvent2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCBuddyEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCBuddyEvent2 {
    type Vtable = IRTCBuddyEvent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x484a7f1e_73f0_4990_bfc2_60bc3978a720);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent2_Vtbl {
    pub base__: IRTCBuddyEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCBuddyGroup(::windows_core::IUnknown);
impl IRTCBuddyGroup {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrgroupname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi()).ok()
    }
    pub unsafe fn AddBuddy<'a, Param0: ::windows_core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddBuddy)(::windows_core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn RemoveBuddy<'a, Param0: ::windows_core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveBuddy)(::windows_core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateBuddies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumBuddies>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Buddies(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Buddies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile2>(result__)
    }
}
impl ::core::convert::From<IRTCBuddyGroup> for ::windows_core::IUnknown {
    fn from(value: IRTCBuddyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddyGroup> for ::windows_core::IUnknown {
    fn from(value: &IRTCBuddyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCBuddyGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCBuddyGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCBuddyGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCBuddyGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCBuddyGroup {}
impl ::core::fmt::Debug for IRTCBuddyGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCBuddyGroup {
    type Vtable = IRTCBuddyGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60361e68_9164_4389_a4c6_d0b3925bda5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroup_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuddy: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuddy: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumerateBuddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Buddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Buddies: usize,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCBuddyGroupEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCBuddyGroupEvent {
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_GROUP_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_GROUP_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_GROUP_EVENT_TYPE>(result__)
    }
    pub unsafe fn Group(&self) -> ::windows_core::Result<IRTCBuddyGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Group)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddyGroup>(result__)
    }
    pub unsafe fn Buddy(&self) -> ::windows_core::Result<IRTCBuddy2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Buddy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy2>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyGroupEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCBuddyGroupEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyGroupEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCBuddyGroupEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCBuddyGroupEvent> for super::Com::IDispatch {
    fn from(value: IRTCBuddyGroupEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCBuddyGroupEvent> for super::Com::IDispatch {
    fn from(value: &IRTCBuddyGroupEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCBuddyGroupEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCBuddyGroupEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCBuddyGroupEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCBuddyGroupEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyGroupEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCBuddyGroupEvent {
    type Vtable = IRTCBuddyGroupEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a79e1d1_b736_4414_96f8_bbc7f08863e4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroupEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Buddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClient(::windows_core::IUnknown);
impl IRTCClient {
    pub unsafe fn Initialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareForShutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfilter)).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EventFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreferredMediaTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatypes), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PreferredMediaTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MediaCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CreateSession<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetListenForIncomingSessions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enlisten)).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows_core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_LISTEN_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).ListenForIncomingSessions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_LISTEN_MODE>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_NetworkAddresses(&self, ftcp: i16, fexternal: i16) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_NetworkAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ftcp), ::core::mem::transmute(fexternal), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_Volume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).get_Volume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn put_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_AudioMuted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(fmuted)).ok()
    }
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_AudioMuted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-media", feature = "win32-system"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows_core::Result<::win32_media::DirectShow::IVideoWindow> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_IVideoWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_media::DirectShow::IVideoWindow>(result__)
    }
    pub unsafe fn put_PreferredAudioDevice<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_PreferredAudioDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_PreferredAudioDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_PreferredVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).get_PreferredVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredAEC(&self, benable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreferredAEC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(benable)).ok()
    }
    pub unsafe fn PreferredAEC(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).PreferredAEC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPreferredVideoDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdevicename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreferredVideoDevice)(::windows_core::Interface::as_raw(self), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PreferredVideoDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ActiveMedia)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxbitrate)).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTemporalSpatialTradeOff)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lvalue)).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).TemporalSpatialTradeOff)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).NetworkQuality)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartT120Applet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enapplet)).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopT120Applets)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).get_IsT120AppletRunning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enapplet), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn LocalUserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalUserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalUserURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstruseruri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalUserURI)(::windows_core::Interface::as_raw(self), bstruseruri.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LocalUserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalUserName)(::windows_core::Interface::as_raw(self), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PlayRing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), ::core::mem::transmute(bplay)).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendDTMF)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endtmf)).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeTuningWizard)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwndparent)).ok()
    }
    pub unsafe fn IsTuned(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTuned)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IRTCClient> for ::windows_core::IUnknown {
    fn from(value: IRTCClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClient> for ::windows_core::IUnknown {
    fn from(value: &IRTCClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClient {}
impl ::core::fmt::Debug for IRTCClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClient {
    type Vtable = IRTCClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07829e45_9a34_408e_a011_bddf13487cd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrepareForShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetEventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows_core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows_core::HRESULT,
    pub SetPreferredMediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: i16) -> ::windows_core::HRESULT,
    pub PreferredMediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT,
    pub MediaCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT,
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pprofile: ::windows_core::RawPtr, lflags: i32, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetListenForIncomingSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows_core::HRESULT,
    pub ListenForIncomingSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_NetworkAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftcp: i16, fexternal: i16, pvaddresses: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_NetworkAddresses: usize,
    pub put_Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::HRESULT,
    pub get_Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows_core::HRESULT,
    pub put_AudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows_core::HRESULT,
    pub get_AudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-media", feature = "win32-system"))]
    pub get_IVideoWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-media", feature = "win32-system")))]
    get_IVideoWindow: usize,
    pub put_PreferredAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub get_PreferredAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub put_PreferredVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::HRESULT,
    pub get_PreferredVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows_core::HRESULT,
    pub SetPreferredAEC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: i16) -> ::windows_core::HRESULT,
    pub PreferredAEC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetPreferredVideoDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PreferredVideoDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ActiveMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows_core::HRESULT,
    pub MaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows_core::HRESULT,
    pub SetTemporalSpatialTradeOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows_core::HRESULT,
    pub TemporalSpatialTradeOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows_core::HRESULT,
    pub NetworkQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows_core::HRESULT,
    pub StartT120Applet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows_core::HRESULT,
    pub StopT120Applets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_IsT120AppletRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows_core::HRESULT,
    pub LocalUserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalUserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LocalUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLocalUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlayRing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: i16) -> ::windows_core::HRESULT,
    pub SendDTMF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows_core::HRESULT,
    pub InvokeTuningWizard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows_core::HRESULT,
    pub IsTuned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftuned: *mut i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClient2(::windows_core::IUnknown);
impl IRTCClient2 {
    pub unsafe fn Initialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Shutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PrepareForShutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEventFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfilter)).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EventFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPreferredMediaTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatypes), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PreferredMediaTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MediaCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CreateSession<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetListenForIncomingSessions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enlisten)).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows_core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_LISTEN_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ListenForIncomingSessions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_LISTEN_MODE>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_NetworkAddresses(&self, ftcp: i16, fexternal: i16) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_NetworkAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ftcp), ::core::mem::transmute(fexternal), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_Volume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_Volume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn put_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_AudioMuted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(fmuted)).ok()
    }
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_AudioMuted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-media", feature = "win32-system"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows_core::Result<::win32_media::DirectShow::IVideoWindow> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_IVideoWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_media::DirectShow::IVideoWindow>(result__)
    }
    pub unsafe fn put_PreferredAudioDevice<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_PreferredAudioDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_PreferredAudioDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_PreferredVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_PreferredVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endevice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredAEC(&self, benable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPreferredAEC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(benable)).ok()
    }
    pub unsafe fn PreferredAEC(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PreferredAEC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPreferredVideoDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdevicename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPreferredVideoDevice)(::windows_core::Interface::as_raw(self), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PreferredVideoDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActiveMedia)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaxBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxbitrate)).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MaxBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTemporalSpatialTradeOff)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lvalue)).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.TemporalSpatialTradeOff)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NetworkQuality)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartT120Applet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enapplet)).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopT120Applets)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_IsT120AppletRunning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enapplet), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn LocalUserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalUserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalUserURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstruseruri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalUserURI)(::windows_core::Interface::as_raw(self), bstruseruri.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalUserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLocalUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalUserName)(::windows_core::Interface::as_raw(self), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PlayRing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), ::core::mem::transmute(bplay)).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendDTMF)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(endtmf)).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InvokeTuningWizard)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwndparent)).ok()
    }
    pub unsafe fn IsTuned(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsTuned)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn put_AnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_AnswerMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), ::core::mem::transmute(enmode)).ok()
    }
    pub unsafe fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> ::windows_core::Result<RTC_ANSWER_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_ANSWER_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).get_AnswerMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_ANSWER_MODE>(result__)
    }
    pub unsafe fn InvokeTuningWizardEx(&self, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeTuningWizardEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(fallowaudio), ::core::mem::transmute(fallowvideo)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetClientName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrclientname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientName)(::windows_core::Interface::as_raw(self), bstrclientname.into_param().abi()).ok()
    }
    pub unsafe fn SetClientCurVer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrclientcurver: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientCurVer)(::windows_core::Interface::as_raw(self), bstrclientcurver.into_param().abi()).ok()
    }
    pub unsafe fn InitializeEx(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn CreateSessionWithDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1, pprofile: Param2, lflags: i32) -> ::windows_core::Result<IRTCSession2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSessionWithDescription)(::windows_core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn SetSessionDescriptionManager<'a, Param0: ::windows_core::IntoParam<'a, IRTCSessionDescriptionManager>>(&self, psessiondescriptionmanager: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSessionDescriptionManager)(::windows_core::Interface::as_raw(self), psessiondescriptionmanager.into_param().abi()).ok()
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_PreferredSecurityLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(ensecuritylevel)).ok()
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SECURITY_LEVEL>::zeroed();
        (::windows_core::Interface::vtable(self).get_PreferredSecurityLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_AllowedPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltransport), ::core::mem::transmute(enlistenmode)).ok()
    }
    pub unsafe fn get_AllowedPorts(&self, ltransport: i32) -> ::windows_core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_LISTEN_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).get_AllowedPorts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltransport), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_LISTEN_MODE>(result__)
    }
}
impl ::core::convert::From<IRTCClient2> for ::windows_core::IUnknown {
    fn from(value: IRTCClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClient2> for ::windows_core::IUnknown {
    fn from(value: &IRTCClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClient2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClient2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCClient2> for IRTCClient {
    fn from(value: IRTCClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClient2> for IRTCClient {
    fn from(value: &IRTCClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCClient> for IRTCClient2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCClient> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCClient> for &'a IRTCClient2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCClient> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClient2 {}
impl ::core::fmt::Debug for IRTCClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClient2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClient2 {
    type Vtable = IRTCClient2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c91d71d_1064_42da_bfa5_572beb8eea84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient2_Vtbl {
    pub base__: IRTCClient_Vtbl,
    pub put_AnswerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows_core::HRESULT,
    pub get_AnswerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows_core::HRESULT,
    pub InvokeTuningWizardEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclientname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetClientCurVer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclientcurver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub InitializeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub CreateSessionWithDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pprofile: ::windows_core::RawPtr, lflags: i32, ppsession2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSessionDescriptionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT,
    pub put_AllowedPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows_core::HRESULT,
    pub get_AllowedPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCClientEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCClientEvent {
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_CLIENT_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_CLIENT_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_CLIENT_EVENT_TYPE>(result__)
    }
    pub unsafe fn Client(&self) -> ::windows_core::Result<IRTCClient> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Client)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCClient>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCClientEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCClientEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCClientEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCClientEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClientEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClientEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCClientEvent> for super::Com::IDispatch {
    fn from(value: IRTCClientEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCClientEvent> for super::Com::IDispatch {
    fn from(value: &IRTCClientEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCClientEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCClientEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCClientEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCClientEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCClientEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCClientEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCClientEvent {
    type Vtable = IRTCClientEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b493b7a_3cba_4170_9c8b_76a9dacdd644);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub Client: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClientPortManagement(::windows_core::IUnknown);
impl IRTCClientPortManagement {
    pub unsafe fn StartListenAddressAndPort<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinternallocaladdress: Param0, linternallocalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartListenAddressAndPort)(::windows_core::Interface::as_raw(self), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport)).ok()
    }
    pub unsafe fn StopListenAddressAndPort<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinternallocaladdress: Param0, linternallocalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopListenAddressAndPort)(::windows_core::Interface::as_raw(self), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport)).ok()
    }
    pub unsafe fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPortRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enporttype), ::core::mem::transmute(plminvalue), ::core::mem::transmute(plmaxvalue)).ok()
    }
}
impl ::core::convert::From<IRTCClientPortManagement> for ::windows_core::IUnknown {
    fn from(value: IRTCClientPortManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientPortManagement> for ::windows_core::IUnknown {
    fn from(value: &IRTCClientPortManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClientPortManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClientPortManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClientPortManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClientPortManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientPortManagement {}
impl ::core::fmt::Debug for IRTCClientPortManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientPortManagement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClientPortManagement {
    type Vtable = IRTCClientPortManagement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5df3f03_4bde_4417_aefe_71177bdaea66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPortManagement_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub StartListenAddressAndPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linternallocalport: i32) -> ::windows_core::HRESULT,
    pub StopListenAddressAndPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linternallocalport: i32) -> ::windows_core::HRESULT,
    pub GetPortRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClientPresence(::windows_core::IUnknown);
impl IRTCClientPresence {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnablePresence<'a, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, fusestorage: i16, varstorage: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnablePresence)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fusestorage), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Export<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Export)(::windows_core::Interface::as_raw(self), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Import<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0, freplaceall: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Import)(::windows_core::Interface::as_raw(self), varstorage.into_param().abi(), ::core::mem::transmute(freplaceall)).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateBuddies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumBuddies>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Buddies(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Buddies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn get_Buddy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Buddy)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn AddBuddy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fpersistent: i16, pprofile: Param4, lflags: i32) -> ::windows_core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddBuddy)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fpersistent), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn RemoveBuddy<'a, Param0: ::windows_core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveBuddy)(::windows_core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows_core::Result<IRTCEnumWatchers> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateWatchers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumWatchers>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Watchers(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Watchers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn get_Watcher<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Watcher)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn AddWatcher<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fblocked: i16, fpersistent: i16) -> ::windows_core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddWatcher)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fblocked), ::core::mem::transmute(fpersistent), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn RemoveWatcher<'a, Param0: ::windows_core::IntoParam<'a, IRTCWatcher>>(&self, pwatcher: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveWatcher)(::windows_core::Interface::as_raw(self), pwatcher.into_param().abi()).ok()
    }
    pub unsafe fn SetLocalPresenceInfo<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalPresenceInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enstatus), bstrnotes.into_param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows_core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_OFFER_WATCHER_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).OfferWatcherMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_OFFER_WATCHER_MODE>(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOfferWatcherMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enmode)).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows_core::Result<RTC_PRIVACY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PRIVACY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).PrivacyMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PRIVACY_MODE>(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivacyMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enmode)).ok()
    }
}
impl ::core::convert::From<IRTCClientPresence> for ::windows_core::IUnknown {
    fn from(value: IRTCClientPresence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientPresence> for ::windows_core::IUnknown {
    fn from(value: &IRTCClientPresence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClientPresence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClientPresence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClientPresence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClientPresence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientPresence {}
impl ::core::fmt::Debug for IRTCClientPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientPresence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClientPresence {
    type Vtable = IRTCClientPresence_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11c3cbcc_0744_42d1_968a_51aa1bb274c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnablePresence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnablePresence: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Export: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Export: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Import: usize,
    pub EnumerateBuddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Buddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Buddies: usize,
    pub get_Buddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppbuddy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fpersistent: i16, pprofile: ::windows_core::RawPtr, lflags: i32, ppbuddy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuddy: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumerateWatchers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Watchers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Watchers: usize,
    pub get_Watcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppwatcher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwatcher: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLocalPresenceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub OfferWatcherMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows_core::HRESULT,
    pub SetOfferWatcherMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows_core::HRESULT,
    pub PrivacyMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows_core::HRESULT,
    pub SetPrivacyMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClientPresence2(::windows_core::IUnknown);
impl IRTCClientPresence2 {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnablePresence<'a, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, fusestorage: i16, varstorage: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnablePresence)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fusestorage), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Export<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Export)(::windows_core::Interface::as_raw(self), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Import<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0, freplaceall: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Import)(::windows_core::Interface::as_raw(self), varstorage.into_param().abi(), ::core::mem::transmute(freplaceall)).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerateBuddies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumBuddies>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Buddies(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Buddies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn get_Buddy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_Buddy)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn AddBuddy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fpersistent: i16, pprofile: Param4, lflags: i32) -> ::windows_core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddBuddy)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fpersistent), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn RemoveBuddy<'a, Param0: ::windows_core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveBuddy)(::windows_core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows_core::Result<IRTCEnumWatchers> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerateWatchers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumWatchers>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Watchers(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Watchers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn get_Watcher<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_Watcher)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn AddWatcher<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fblocked: i16, fpersistent: i16) -> ::windows_core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddWatcher)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fblocked), ::core::mem::transmute(fpersistent), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn RemoveWatcher<'a, Param0: ::windows_core::IntoParam<'a, IRTCWatcher>>(&self, pwatcher: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveWatcher)(::windows_core::Interface::as_raw(self), pwatcher.into_param().abi()).ok()
    }
    pub unsafe fn SetLocalPresenceInfo<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalPresenceInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enstatus), bstrnotes.into_param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows_core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_OFFER_WATCHER_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OfferWatcherMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_OFFER_WATCHER_MODE>(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOfferWatcherMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enmode)).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows_core::Result<RTC_PRIVACY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PRIVACY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PrivacyMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PRIVACY_MODE>(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPrivacyMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enmode)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnablePresenceEx<'a, Param0: ::windows_core::IntoParam<'a, IRTCProfile>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, pprofile: Param0, varstorage: Param1, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnablePresenceEx)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi(), varstorage.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn DisablePresence(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisablePresence)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, bstrgroupname: Param0, bstrdata: Param1, pprofile: Param2, lflags: i32) -> ::windows_core::Result<IRTCBuddyGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), bstrdata.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddyGroup>(result__)
    }
    pub unsafe fn RemoveGroup<'a, Param0: ::windows_core::IntoParam<'a, IRTCBuddyGroup>>(&self, pgroup: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveGroup)(::windows_core::Interface::as_raw(self), pgroup.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> ::windows_core::Result<IRTCEnumGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumGroups>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Groups(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Groups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn get_Group<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrgroupname: Param0) -> ::windows_core::Result<IRTCBuddyGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Group)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddyGroup>(result__)
    }
    pub unsafe fn AddWatcherEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: Param6, lflags: i32) -> ::windows_core::Result<IRTCWatcher2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddWatcherEx)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(enstate), ::core::mem::transmute(fpersistent), ::core::mem::transmute(enscope), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher2>(result__)
    }
    pub unsafe fn get_WatcherEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: Param1) -> ::windows_core::Result<IRTCWatcher2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_WatcherEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enmode), bstrpresentityuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher2>(result__)
    }
    pub unsafe fn put_PresenceProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_PresenceProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enproperty), bstrproperty.into_param().abi()).ok()
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_PresenceProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPresenceData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrnamespace: Param0, bstrdata: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPresenceData)(::windows_core::Interface::as_raw(self), bstrnamespace.into_param().abi(), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut ::win32_foundation::BSTR, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPresenceData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLocalPresenceInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penstatus), ::core::mem::transmute(pbstrnotes)).ok()
    }
    pub unsafe fn AddBuddyEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: Param5, lflags: i32) -> ::windows_core::Result<IRTCBuddy2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddBuddyEx)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fpersistent), ::core::mem::transmute(ensubscriptiontype), pprofile.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCBuddy2>(result__)
    }
}
impl ::core::convert::From<IRTCClientPresence2> for ::windows_core::IUnknown {
    fn from(value: IRTCClientPresence2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientPresence2> for ::windows_core::IUnknown {
    fn from(value: &IRTCClientPresence2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClientPresence2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClientPresence2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCClientPresence2> for IRTCClientPresence {
    fn from(value: IRTCClientPresence2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientPresence2> for IRTCClientPresence {
    fn from(value: &IRTCClientPresence2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCClientPresence> for IRTCClientPresence2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCClientPresence> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCClientPresence> for &'a IRTCClientPresence2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCClientPresence> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClientPresence2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClientPresence2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientPresence2 {}
impl ::core::fmt::Debug for IRTCClientPresence2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientPresence2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClientPresence2 {
    type Vtable = IRTCClientPresence2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad1809e8_62f7_4783_909a_29c9d2cb1d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence2_Vtbl {
    pub base__: IRTCClientPresence_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnablePresenceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows_core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnablePresenceEx: usize,
    pub DisablePresence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pprofile: ::windows_core::RawPtr, lflags: i32, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Groups: usize,
    pub get_Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddWatcherEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::windows_core::RawPtr, lflags: i32, ppwatcher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub get_WatcherEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppwatcher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub put_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnamespace: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::win32_foundation::BSTR, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub AddBuddyEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::windows_core::RawPtr, lflags: i32, ppbuddy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClientProvisioning(::windows_core::IUnknown);
impl IRTCClientProvisioning {
    pub unsafe fn CreateProfile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprofilexml: Param0) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateProfile)(::windows_core::Interface::as_raw(self), bstrprofilexml.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn EnableProfile<'a, Param0: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0, lregisterflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableProfile)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi(), ::core::mem::transmute(lregisterflags)).ok()
    }
    pub unsafe fn DisableProfile<'a, Param0: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableProfile)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows_core::Result<IRTCEnumProfiles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateProfiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumProfiles>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn GetProfile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstruseraccount: Param0, bstruserpassword: Param1, bstruseruri: Param2, bstrserver: Param3, ltransport: i32, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProfile)(::windows_core::Interface::as_raw(self), bstruseraccount.into_param().abi(), bstruserpassword.into_param().abi(), bstruseruri.into_param().abi(), bstrserver.into_param().abi(), ::core::mem::transmute(ltransport), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SessionCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IRTCClientProvisioning> for ::windows_core::IUnknown {
    fn from(value: IRTCClientProvisioning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientProvisioning> for ::windows_core::IUnknown {
    fn from(value: &IRTCClientProvisioning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClientProvisioning {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClientProvisioning {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClientProvisioning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClientProvisioning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientProvisioning {}
impl ::core::fmt::Debug for IRTCClientProvisioning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientProvisioning").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClientProvisioning {
    type Vtable = IRTCClientProvisioning_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9f5cf06_65b9_4a80_a0e6_73cae3ef3822);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprofilexml: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows_core::RawPtr, lregisterflags: i32) -> ::windows_core::HRESULT,
    pub DisableProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumerateProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Profiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Profiles: usize,
    pub GetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruseraccount: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows_core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCClientProvisioning2(::windows_core::IUnknown);
impl IRTCClientProvisioning2 {
    pub unsafe fn CreateProfile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprofilexml: Param0) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateProfile)(::windows_core::Interface::as_raw(self), bstrprofilexml.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn EnableProfile<'a, Param0: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0, lregisterflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnableProfile)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi(), ::core::mem::transmute(lregisterflags)).ok()
    }
    pub unsafe fn DisableProfile<'a, Param0: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisableProfile)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows_core::Result<IRTCEnumProfiles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerateProfiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumProfiles>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Profiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn GetProfile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstruseraccount: Param0, bstruserpassword: Param1, bstruseruri: Param2, bstrserver: Param3, ltransport: i32, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProfile)(::windows_core::Interface::as_raw(self), bstruseraccount.into_param().abi(), bstruserpassword.into_param().abi(), bstruseruri.into_param().abi(), bstrserver.into_param().abi(), ::core::mem::transmute(ltransport), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EnableProfileEx<'a, Param0: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0, lregisterflags: i32, lroamingflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableProfileEx)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi(), ::core::mem::transmute(lregisterflags), ::core::mem::transmute(lroamingflags)).ok()
    }
}
impl ::core::convert::From<IRTCClientProvisioning2> for ::windows_core::IUnknown {
    fn from(value: IRTCClientProvisioning2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientProvisioning2> for ::windows_core::IUnknown {
    fn from(value: &IRTCClientProvisioning2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCClientProvisioning2> for IRTCClientProvisioning {
    fn from(value: IRTCClientProvisioning2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientProvisioning2> for IRTCClientProvisioning {
    fn from(value: &IRTCClientProvisioning2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCClientProvisioning> for IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCClientProvisioning> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCClientProvisioning> for &'a IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCClientProvisioning> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCClientProvisioning2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCClientProvisioning2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientProvisioning2 {}
impl ::core::fmt::Debug for IRTCClientProvisioning2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientProvisioning2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCClientProvisioning2 {
    type Vtable = IRTCClientProvisioning2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa70909b5_f40e_4587_bb75_e6bc0845023e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning2_Vtbl {
    pub base__: IRTCClientProvisioning_Vtbl,
    pub EnableProfileEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows_core::RawPtr, lregisterflags: i32, lroamingflags: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCCollection> for ::windows_core::IUnknown {
    fn from(value: IRTCCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCCollection> for ::windows_core::IUnknown {
    fn from(value: &IRTCCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCCollection> for super::Com::IDispatch {
    fn from(value: IRTCCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCCollection> for super::Com::IDispatch {
    fn from(value: &IRTCCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCCollection {
    type Vtable = IRTCCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec7c8096_b918_4044_94f1_e4fba0361d5c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCDispatchEventNotification(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCDispatchEventNotification {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCDispatchEventNotification> for ::windows_core::IUnknown {
    fn from(value: IRTCDispatchEventNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCDispatchEventNotification> for ::windows_core::IUnknown {
    fn from(value: &IRTCDispatchEventNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCDispatchEventNotification> for super::Com::IDispatch {
    fn from(value: IRTCDispatchEventNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCDispatchEventNotification> for super::Com::IDispatch {
    fn from(value: &IRTCDispatchEventNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCDispatchEventNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCDispatchEventNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCDispatchEventNotification {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCDispatchEventNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCDispatchEventNotification").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCDispatchEventNotification {
    type Vtable = IRTCDispatchEventNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x176ddfbe_fec0_4d55_bc87_84cff1ef7f91);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCDispatchEventNotification_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[repr(transparent)]
pub struct IRTCEnumBuddies(::windows_core::IUnknown);
impl IRTCEnumBuddies {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCBuddy>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumBuddies>(result__)
    }
}
impl ::core::convert::From<IRTCEnumBuddies> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumBuddies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumBuddies> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumBuddies) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumBuddies {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumBuddies {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumBuddies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumBuddies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumBuddies {}
impl ::core::fmt::Debug for IRTCEnumBuddies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumBuddies").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumBuddies {
    type Vtable = IRTCEnumBuddies_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7296917_5569_4b3b_b3af_98d1144b2b87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumBuddies_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEnumGroups(::windows_core::IUnknown);
impl IRTCEnumGroups {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCBuddyGroup>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumGroups>(result__)
    }
}
impl ::core::convert::From<IRTCEnumGroups> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumGroups> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumGroups) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumGroups {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumGroups {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumGroups {}
impl ::core::fmt::Debug for IRTCEnumGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumGroups").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumGroups {
    type Vtable = IRTCEnumGroups_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x742378d6_a141_4415_8f27_35d99076cf5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumGroups_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEnumParticipants(::windows_core::IUnknown);
impl IRTCEnumParticipants {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCParticipant>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumParticipants> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumParticipants>(result__)
    }
}
impl ::core::convert::From<IRTCEnumParticipants> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumParticipants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumParticipants> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumParticipants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumParticipants {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumParticipants {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumParticipants {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumParticipants {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumParticipants {}
impl ::core::fmt::Debug for IRTCEnumParticipants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumParticipants").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumParticipants {
    type Vtable = IRTCEnumParticipants_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcd56f29_4a4f_41b2_ba5c_f5bccc060bf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumParticipants_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEnumPresenceDevices(::windows_core::IUnknown);
impl IRTCEnumPresenceDevices {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCPresenceDevice>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumPresenceDevices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumPresenceDevices>(result__)
    }
}
impl ::core::convert::From<IRTCEnumPresenceDevices> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumPresenceDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumPresenceDevices> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumPresenceDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumPresenceDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumPresenceDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumPresenceDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumPresenceDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumPresenceDevices {}
impl ::core::fmt::Debug for IRTCEnumPresenceDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumPresenceDevices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumPresenceDevices {
    type Vtable = IRTCEnumPresenceDevices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x708c2ab7_8bf8_42f8_8c7d_635197ad5539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumPresenceDevices_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEnumProfiles(::windows_core::IUnknown);
impl IRTCEnumProfiles {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCProfile>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumProfiles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumProfiles>(result__)
    }
}
impl ::core::convert::From<IRTCEnumProfiles> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumProfiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumProfiles> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumProfiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumProfiles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumProfiles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumProfiles {}
impl ::core::fmt::Debug for IRTCEnumProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumProfiles").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumProfiles {
    type Vtable = IRTCEnumProfiles_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29b7c41c_ed82_4bca_84ad_39d5101b58e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumProfiles_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEnumUserSearchResults(::windows_core::IUnknown);
impl IRTCEnumUserSearchResults {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCUserSearchResult>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumUserSearchResults> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumUserSearchResults>(result__)
    }
}
impl ::core::convert::From<IRTCEnumUserSearchResults> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumUserSearchResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumUserSearchResults> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumUserSearchResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumUserSearchResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumUserSearchResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumUserSearchResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumUserSearchResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumUserSearchResults {}
impl ::core::fmt::Debug for IRTCEnumUserSearchResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumUserSearchResults").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumUserSearchResults {
    type Vtable = IRTCEnumUserSearchResults_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83d4d877_aa5d_4a5b_8d0e_002a8067e0e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumUserSearchResults_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEnumWatchers(::windows_core::IUnknown);
impl IRTCEnumWatchers {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCWatcher>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppelements)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IRTCEnumWatchers> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumWatchers>(result__)
    }
}
impl ::core::convert::From<IRTCEnumWatchers> for ::windows_core::IUnknown {
    fn from(value: IRTCEnumWatchers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEnumWatchers> for ::windows_core::IUnknown {
    fn from(value: &IRTCEnumWatchers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEnumWatchers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEnumWatchers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEnumWatchers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEnumWatchers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumWatchers {}
impl ::core::fmt::Debug for IRTCEnumWatchers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumWatchers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEnumWatchers {
    type Vtable = IRTCEnumWatchers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa87d55d7_db74_4ed1_9ca4_77a0e41b413e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumWatchers_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCEventNotification(::windows_core::IUnknown);
impl IRTCEventNotification {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Event<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, rtcevent: RTC_EVENT, pevent: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Event)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rtcevent), pevent.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRTCEventNotification> for ::windows_core::IUnknown {
    fn from(value: IRTCEventNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCEventNotification> for ::windows_core::IUnknown {
    fn from(value: &IRTCEventNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCEventNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCEventNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCEventNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCEventNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEventNotification {}
impl ::core::fmt::Debug for IRTCEventNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEventNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCEventNotification {
    type Vtable = IRTCEventNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13fa24c7_5748_4b21_91f5_7397609ce747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEventNotification_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Event: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCInfoEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCInfoEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows_core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Participant)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn Info(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Info)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn InfoHeader(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InfoHeader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCInfoEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCInfoEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCInfoEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCInfoEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCInfoEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCInfoEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCInfoEvent> for super::Com::IDispatch {
    fn from(value: IRTCInfoEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCInfoEvent> for super::Com::IDispatch {
    fn from(value: &IRTCInfoEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCInfoEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCInfoEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCInfoEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCInfoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCInfoEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCInfoEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCInfoEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCInfoEvent {
    type Vtable = IRTCInfoEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e1d68ae_1912_4f49_b2c3_594fadfd425f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCInfoEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinfo: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub InfoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCIntensityEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCIntensityEvent {
    pub unsafe fn Level(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Level)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Min(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Min)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Max(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Max)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<RTC_AUDIO_DEVICE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_AUDIO_DEVICE>::zeroed();
        (::windows_core::Interface::vtable(self).Direction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_AUDIO_DEVICE>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCIntensityEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCIntensityEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCIntensityEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCIntensityEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCIntensityEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCIntensityEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCIntensityEvent> for super::Com::IDispatch {
    fn from(value: IRTCIntensityEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCIntensityEvent> for super::Com::IDispatch {
    fn from(value: &IRTCIntensityEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCIntensityEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCIntensityEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCIntensityEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCIntensityEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCIntensityEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCIntensityEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCIntensityEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCIntensityEvent {
    type Vtable = IRTCIntensityEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c23bf51_390c_4992_a41d_41eec05b2a4b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCIntensityEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCMediaEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCMediaEvent {
    pub unsafe fn MediaType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_MEDIA_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_MEDIA_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_MEDIA_EVENT_TYPE>(result__)
    }
    pub unsafe fn EventReason(&self) -> ::windows_core::Result<RTC_MEDIA_EVENT_REASON> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_MEDIA_EVENT_REASON>::zeroed();
        (::windows_core::Interface::vtable(self).EventReason)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_MEDIA_EVENT_REASON>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCMediaEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCMediaEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCMediaEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCMediaEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCMediaEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCMediaEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCMediaEvent> for super::Com::IDispatch {
    fn from(value: IRTCMediaEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCMediaEvent> for super::Com::IDispatch {
    fn from(value: &IRTCMediaEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCMediaEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCMediaEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCMediaEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCMediaEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCMediaEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCMediaEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCMediaEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCMediaEvent {
    type Vtable = IRTCMediaEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x099944fb_bcda_453e_8c41_e13da2adf7f3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows_core::HRESULT,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub EventReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCMediaRequestEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCMediaRequestEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn ProposedMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ProposedMedia)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentMedia(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentMedia)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Accept(&self, lmediatypes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Accept)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatypes)).ok()
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SECURITY_LEVEL>::zeroed();
        (::windows_core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn Reject(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reject)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_REINVITE_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_REINVITE_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_REINVITE_STATE>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCMediaRequestEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCMediaRequestEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCMediaRequestEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCMediaRequestEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCMediaRequestEvent> for super::Com::IDispatch {
    fn from(value: IRTCMediaRequestEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCMediaRequestEvent> for super::Com::IDispatch {
    fn from(value: &IRTCMediaRequestEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCMediaRequestEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCMediaRequestEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCMediaRequestEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCMediaRequestEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCMediaRequestEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCMediaRequestEvent {
    type Vtable = IRTCMediaRequestEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52572d15_148c_4d97_a36c_2da55c289d63);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaRequestEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProposedMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT,
    pub CurrentMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows_core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCMessagingEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCMessagingEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows_core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Participant)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_MESSAGING_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_MESSAGING_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_MESSAGING_EVENT_TYPE>(result__)
    }
    pub unsafe fn Message(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MessageHeader(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MessageHeader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserStatus(&self) -> ::windows_core::Result<RTC_MESSAGING_USER_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_MESSAGING_USER_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).UserStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_MESSAGING_USER_STATUS>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCMessagingEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCMessagingEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCMessagingEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCMessagingEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCMessagingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCMessagingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCMessagingEvent> for super::Com::IDispatch {
    fn from(value: IRTCMessagingEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCMessagingEvent> for super::Com::IDispatch {
    fn from(value: &IRTCMessagingEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCMessagingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCMessagingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCMessagingEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCMessagingEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCMessagingEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCMessagingEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCMessagingEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCMessagingEvent {
    type Vtable = IRTCMessagingEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3609541_1b29_4de5_a4ad_5aebaf319512);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMessagingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmessage: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub MessageHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCParticipant(::windows_core::IUnknown);
impl IRTCParticipant {
    pub unsafe fn UserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Removable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Removable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PARTICIPANT_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PARTICIPANT_STATE>(result__)
    }
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
}
impl ::core::convert::From<IRTCParticipant> for ::windows_core::IUnknown {
    fn from(value: IRTCParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCParticipant> for ::windows_core::IUnknown {
    fn from(value: &IRTCParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCParticipant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCParticipant {}
impl ::core::fmt::Debug for IRTCParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCParticipant").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCParticipant {
    type Vtable = IRTCParticipant_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae86add5_26b1_4414_af1d_b94cd938d739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipant_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub UserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Removable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfremovable: *mut i16) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCParticipantStateChangeEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCParticipantStateChangeEvent {
    pub unsafe fn Participant(&self) -> ::windows_core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Participant)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PARTICIPANT_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PARTICIPANT_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCParticipantStateChangeEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCParticipantStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCParticipantStateChangeEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCParticipantStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCParticipantStateChangeEvent> for super::Com::IDispatch {
    fn from(value: IRTCParticipantStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCParticipantStateChangeEvent> for super::Com::IDispatch {
    fn from(value: &IRTCParticipantStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCParticipantStateChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCParticipantStateChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCParticipantStateChangeEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCParticipantStateChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCParticipantStateChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCParticipantStateChangeEvent {
    type Vtable = IRTCParticipantStateChangeEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09bcb597_f0fa_48f9_b420_468cea7fde04);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipantStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCPortManager(::windows_core::IUnknown);
impl IRTCPortManager {
    pub unsafe fn GetMapping<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrremoteaddress: Param0, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::win32_foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::win32_foundation::BSTR, plexternallocalport: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMapping)(::windows_core::Interface::as_raw(self), bstrremoteaddress.into_param().abi(), ::core::mem::transmute(enporttype), ::core::mem::transmute(pbstrinternallocaladdress), ::core::mem::transmute(plinternallocalport), ::core::mem::transmute(pbstrexternallocaladdress), ::core::mem::transmute(plexternallocalport)).ok()
    }
    pub unsafe fn UpdateRemoteAddress<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrremoteaddress: Param0, bstrinternallocaladdress: Param1, linternallocalport: i32, bstrexternallocaladdress: Param3, lexternallocalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateRemoteAddress)(::windows_core::Interface::as_raw(self), bstrremoteaddress.into_param().abi(), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport), bstrexternallocaladdress.into_param().abi(), ::core::mem::transmute(lexternallocalport)).ok()
    }
    pub unsafe fn ReleaseMapping<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinternallocaladdress: Param0, linternallocalport: i32, bstrexternallocaladdress: Param2, lexternallocaladdress: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMapping)(::windows_core::Interface::as_raw(self), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport), bstrexternallocaladdress.into_param().abi(), ::core::mem::transmute(lexternallocaladdress)).ok()
    }
}
impl ::core::convert::From<IRTCPortManager> for ::windows_core::IUnknown {
    fn from(value: IRTCPortManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCPortManager> for ::windows_core::IUnknown {
    fn from(value: &IRTCPortManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCPortManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCPortManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCPortManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCPortManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCPortManager {}
impl ::core::fmt::Debug for IRTCPortManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPortManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCPortManager {
    type Vtable = IRTCPortManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda77c14b_6208_43ca_8ddf_5b60a0a69fac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPortManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::win32_foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::win32_foundation::BSTR, plexternallocalport: *mut i32) -> ::windows_core::HRESULT,
    pub UpdateRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lexternallocalport: i32) -> ::windows_core::HRESULT,
    pub ReleaseMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lexternallocaladdress: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCPresenceContact(::windows_core::IUnknown);
impl IRTCPresenceContact {
    pub unsafe fn PresentityURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PresentityURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPresentityURI)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Persistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPersistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fpersistent)).ok()
    }
}
impl ::core::convert::From<IRTCPresenceContact> for ::windows_core::IUnknown {
    fn from(value: IRTCPresenceContact) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCPresenceContact> for ::windows_core::IUnknown {
    fn from(value: &IRTCPresenceContact) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCPresenceContact {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCPresenceContact {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCPresenceContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCPresenceContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCPresenceContact {}
impl ::core::fmt::Debug for IRTCPresenceContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceContact").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCPresenceContact {
    type Vtable = IRTCPresenceContact_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b22f92c_cd90_42db_a733_212205c3e3df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceContact_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PresentityURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPresentityURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Persistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfpersistent: *mut i16) -> ::windows_core::HRESULT,
    pub SetPersistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fpersistent: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCPresenceDataEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCPresenceDataEvent {
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut ::win32_foundation::BSTR, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPresenceData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCPresenceDataEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCPresenceDataEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCPresenceDataEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCPresenceDataEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCPresenceDataEvent> for super::Com::IDispatch {
    fn from(value: IRTCPresenceDataEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCPresenceDataEvent> for super::Com::IDispatch {
    fn from(value: &IRTCPresenceDataEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCPresenceDataEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCPresenceDataEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCPresenceDataEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCPresenceDataEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceDataEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCPresenceDataEvent {
    type Vtable = IRTCPresenceDataEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38f0e78c_8b87_4c04_a82d_aedd83c909bb);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDataEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::win32_foundation::BSTR, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCPresenceDevice(::windows_core::IUnknown);
impl IRTCPresenceDevice {
    pub unsafe fn Status(&self) -> ::windows_core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PRESENCE_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PRESENCE_STATUS>(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Notes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_PresenceProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut ::win32_foundation::BSTR, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPresenceData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
}
impl ::core::convert::From<IRTCPresenceDevice> for ::windows_core::IUnknown {
    fn from(value: IRTCPresenceDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCPresenceDevice> for ::windows_core::IUnknown {
    fn from(value: &IRTCPresenceDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCPresenceDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCPresenceDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCPresenceDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCPresenceDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCPresenceDevice {}
impl ::core::fmt::Debug for IRTCPresenceDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCPresenceDevice {
    type Vtable = IRTCPresenceDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc6a90dd_ad9a_48da_9b0c_2515e38521ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDevice_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::win32_foundation::BSTR, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCPresencePropertyEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCPresencePropertyEvent {
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn PresenceProperty(&self) -> ::windows_core::Result<RTC_PRESENCE_PROPERTY> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PRESENCE_PROPERTY>::zeroed();
        (::windows_core::Interface::vtable(self).PresenceProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PRESENCE_PROPERTY>(result__)
    }
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCPresencePropertyEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCPresencePropertyEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCPresencePropertyEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCPresencePropertyEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCPresencePropertyEvent> for super::Com::IDispatch {
    fn from(value: IRTCPresencePropertyEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCPresencePropertyEvent> for super::Com::IDispatch {
    fn from(value: &IRTCPresencePropertyEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCPresencePropertyEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCPresencePropertyEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCPresencePropertyEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCPresencePropertyEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresencePropertyEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCPresencePropertyEvent {
    type Vtable = IRTCPresencePropertyEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf777f570_a820_49d5_86bd_e099493f1518);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresencePropertyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCPresenceStatusEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCPresenceStatusEvent {
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLocalPresenceInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(penstatus), ::core::mem::transmute(pbstrnotes)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCPresenceStatusEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCPresenceStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCPresenceStatusEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCPresenceStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCPresenceStatusEvent> for super::Com::IDispatch {
    fn from(value: IRTCPresenceStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCPresenceStatusEvent> for super::Com::IDispatch {
    fn from(value: &IRTCPresenceStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCPresenceStatusEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCPresenceStatusEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCPresenceStatusEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCPresenceStatusEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceStatusEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCPresenceStatusEvent {
    type Vtable = IRTCPresenceStatusEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78673f32_4a0f_462c_89aa_ee7706707678);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCProfile(::windows_core::IUnknown);
impl IRTCProfile {
    pub unsafe fn Key(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Key)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn XML(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).XML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProviderName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ProviderName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_ProviderURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enuri), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProviderData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ProviderData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientBanner(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ClientBanner)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ClientMinVer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientMinVer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientCurVer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientCurVer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientUpdateURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientUpdateURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstruseruri: Param0, bstruseraccount: Param1, bstrpassword: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCredentials)(::windows_core::Interface::as_raw(self), bstruseruri.into_param().abi(), bstruseraccount.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SessionCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_REGISTRATION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_REGISTRATION_STATE>(result__)
    }
}
impl ::core::convert::From<IRTCProfile> for ::windows_core::IUnknown {
    fn from(value: IRTCProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCProfile> for ::windows_core::IUnknown {
    fn from(value: &IRTCProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCProfile {}
impl ::core::fmt::Debug for IRTCProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCProfile {
    type Vtable = IRTCProfile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd07eca9e_4062_4dd4_9e7d_722a49ba7303);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrkey: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub XML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub get_ProviderURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ProviderData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClientBanner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbanner: *mut i16) -> ::windows_core::HRESULT,
    pub ClientMinVer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrminver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClientCurVer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcurver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClientUpdateURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClientData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCProfile2(::windows_core::IUnknown);
impl IRTCProfile2 {
    pub unsafe fn Key(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Key)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn XML(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.XML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProviderName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProviderName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_ProviderURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enuri), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProviderData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProviderData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientBanner(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientBanner)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ClientMinVer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientMinVer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientCurVer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientCurVer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientUpdateURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientUpdateURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClientData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstruseruri: Param0, bstruseraccount: Param1, bstrpassword: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCredentials)(::windows_core::Interface::as_raw(self), bstruseruri.into_param().abi(), bstruseraccount.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SessionCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_REGISTRATION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_REGISTRATION_STATE>(result__)
    }
    pub unsafe fn Realm(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Realm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRealm<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrealm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRealm)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi()).ok()
    }
    pub unsafe fn AllowedAuth(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AllowedAuth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAllowedAuth(&self, lallowedauth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowedAuth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lallowedauth)).ok()
    }
}
impl ::core::convert::From<IRTCProfile2> for ::windows_core::IUnknown {
    fn from(value: IRTCProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCProfile2> for ::windows_core::IUnknown {
    fn from(value: &IRTCProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCProfile2> for IRTCProfile {
    fn from(value: IRTCProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCProfile2> for IRTCProfile {
    fn from(value: &IRTCProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCProfile> for IRTCProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCProfile> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCProfile> for &'a IRTCProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCProfile> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCProfile2 {}
impl ::core::fmt::Debug for IRTCProfile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfile2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCProfile2 {
    type Vtable = IRTCProfile2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b81f84e_bdc7_4184_9154_3cb2dd7917fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile2_Vtbl {
    pub base__: IRTCProfile_Vtbl,
    pub Realm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrealm: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRealm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AllowedAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows_core::HRESULT,
    pub SetAllowedAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCProfileEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCProfileEvent {
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).Cookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCProfileEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCProfileEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCProfileEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCProfileEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCProfileEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCProfileEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCProfileEvent> for super::Com::IDispatch {
    fn from(value: IRTCProfileEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCProfileEvent> for super::Com::IDispatch {
    fn from(value: &IRTCProfileEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCProfileEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCProfileEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCProfileEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCProfileEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCProfileEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCProfileEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfileEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCProfileEvent {
    type Vtable = IRTCProfileEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6d5ab3b_770e_43e8_800a_79b062395fca);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCProfileEvent2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCProfileEvent2 {
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Cookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_PROFILE_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_PROFILE_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_PROFILE_EVENT_TYPE>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCProfileEvent2> for ::windows_core::IUnknown {
    fn from(value: IRTCProfileEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCProfileEvent2> for ::windows_core::IUnknown {
    fn from(value: &IRTCProfileEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCProfileEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCProfileEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCProfileEvent2> for super::Com::IDispatch {
    fn from(value: IRTCProfileEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCProfileEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCProfileEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCProfileEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCProfileEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCProfileEvent2> for IRTCProfileEvent {
    fn from(value: IRTCProfileEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCProfileEvent2> for IRTCProfileEvent {
    fn from(value: &IRTCProfileEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCProfileEvent> for IRTCProfileEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCProfileEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCProfileEvent> for &'a IRTCProfileEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCProfileEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCProfileEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCProfileEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCProfileEvent2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCProfileEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfileEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCProfileEvent2 {
    type Vtable = IRTCProfileEvent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62e56edc_03fa_4121_94fb_23493fd0ae64);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent2_Vtbl {
    pub base__: IRTCProfileEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCReInviteEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCReInviteEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn Accept<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Accept)(::windows_core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi()).ok()
    }
    pub unsafe fn Reject(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reject)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_REINVITE_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_REINVITE_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_REINVITE_STATE>(result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::win32_foundation::BSTR, pbstrsessiondescription: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRemoteSessionDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCReInviteEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCReInviteEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCReInviteEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCReInviteEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCReInviteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCReInviteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCReInviteEvent> for super::Com::IDispatch {
    fn from(value: IRTCReInviteEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCReInviteEvent> for super::Com::IDispatch {
    fn from(value: &IRTCReInviteEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCReInviteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCReInviteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCReInviteEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCReInviteEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCReInviteEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCReInviteEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCReInviteEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCReInviteEvent {
    type Vtable = IRTCReInviteEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11558d84_204c_43e7_99b0_2034e9417f7d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCReInviteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::win32_foundation::BSTR, pbstrsessiondescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCRegistrationStateChangeEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCRegistrationStateChangeEvent {
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_REGISTRATION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_REGISTRATION_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCRegistrationStateChangeEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCRegistrationStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCRegistrationStateChangeEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCRegistrationStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCRegistrationStateChangeEvent> for super::Com::IDispatch {
    fn from(value: IRTCRegistrationStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCRegistrationStateChangeEvent> for super::Com::IDispatch {
    fn from(value: &IRTCRegistrationStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCRegistrationStateChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCRegistrationStateChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCRegistrationStateChangeEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCRegistrationStateChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCRegistrationStateChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCRegistrationStateChangeEvent {
    type Vtable = IRTCRegistrationStateChangeEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62d0991b_50ab_4f02_b948_ca94f26f8f95);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRegistrationStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCRoamingEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCRoamingEvent {
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_ROAMING_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_ROAMING_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_ROAMING_EVENT_TYPE>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCRoamingEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCRoamingEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCRoamingEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCRoamingEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCRoamingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCRoamingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCRoamingEvent> for super::Com::IDispatch {
    fn from(value: IRTCRoamingEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCRoamingEvent> for super::Com::IDispatch {
    fn from(value: &IRTCRoamingEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCRoamingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCRoamingEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCRoamingEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCRoamingEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCRoamingEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCRoamingEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCRoamingEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCRoamingEvent {
    type Vtable = IRTCRoamingEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79960a6b_0cb1_4dc8_a805_7318e99902e8);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRoamingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCSession(::windows_core::IUnknown);
impl IRTCSession {
    pub unsafe fn Client(&self) -> ::windows_core::Result<IRTCClient> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Client)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCClient>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<RTC_SESSION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_TYPE>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Participants(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Participants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Answer)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enreason)).ok()
    }
    pub unsafe fn Redirect<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Redirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn AddParticipant<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstraddress: Param0, bstrname: Param1) -> ::windows_core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddParticipant)(::windows_core::Interface::as_raw(self), bstraddress.into_param().abi(), bstrname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn RemoveParticipant<'a, Param0: ::windows_core::IntoParam<'a, IRTCParticipant>>(&self, pparticipant: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveParticipant)(::windows_core::Interface::as_raw(self), pparticipant.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows_core::Result<IRTCEnumParticipants> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateParticipants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumParticipants>(result__)
    }
    pub unsafe fn CanAddParticipants(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CanAddParticipants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn RedirectedUserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RedirectedUserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RedirectedUserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RedirectedUserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextRedirectedUser)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmessageheader: Param0, bstrmessage: Param1, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendMessage)(::windows_core::Interface::as_raw(self), bstrmessageheader.into_param().abi(), bstrmessage.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendMessageStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enuserstatus), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn put_EncryptionKey<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lmediatype: i32, encryptionkey: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_EncryptionKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatype), encryptionkey.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRTCSession> for ::windows_core::IUnknown {
    fn from(value: IRTCSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSession> for ::windows_core::IUnknown {
    fn from(value: &IRTCSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSession {}
impl ::core::fmt::Debug for IRTCSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCSession {
    type Vtable = IRTCSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x387c8086_99be_42fb_9973_7c0fc0ca9fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Client: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Participants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Participants: usize,
    pub Answer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows_core::HRESULT,
    pub Redirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pprofile: ::windows_core::RawPtr, lflags: i32) -> ::windows_core::HRESULT,
    pub AddParticipant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppparticipant: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveParticipant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparticipant: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumerateParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanAddParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcanadd: *mut i16) -> ::windows_core::HRESULT,
    pub RedirectedUserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RedirectedUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub NextRedirectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageheader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lcookie: isize) -> ::windows_core::HRESULT,
    pub SendMessageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows_core::HRESULT,
    pub put_EncryptionKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCSession2(::windows_core::IUnknown);
impl IRTCSession2 {
    pub unsafe fn Client(&self) -> ::windows_core::Result<IRTCClient> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Client)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCClient>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<RTC_SESSION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_TYPE>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Participants(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Participants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Answer)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enreason)).ok()
    }
    pub unsafe fn Redirect<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Redirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn AddParticipant<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstraddress: Param0, bstrname: Param1) -> ::windows_core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddParticipant)(::windows_core::Interface::as_raw(self), bstraddress.into_param().abi(), bstrname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn RemoveParticipant<'a, Param0: ::windows_core::IntoParam<'a, IRTCParticipant>>(&self, pparticipant: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveParticipant)(::windows_core::Interface::as_raw(self), pparticipant.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows_core::Result<IRTCEnumParticipants> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumerateParticipants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumParticipants>(result__)
    }
    pub unsafe fn CanAddParticipants(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanAddParticipants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn RedirectedUserURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RedirectedUserURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RedirectedUserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RedirectedUserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NextRedirectedUser)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmessageheader: Param0, bstrmessage: Param1, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendMessage)(::windows_core::Interface::as_raw(self), bstrmessageheader.into_param().abi(), bstrmessage.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendMessageStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enuserstatus), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn put_EncryptionKey<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lmediatype: i32, encryptionkey: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_EncryptionKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmediatype), encryptionkey.into_param().abi()).ok()
    }
    pub unsafe fn SendInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinfoheader: Param0, bstrinfo: Param1, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendInfo)(::windows_core::Interface::as_raw(self), bstrinfoheader.into_param().abi(), bstrinfo.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_PreferredSecurityLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(ensecuritylevel)).ok()
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SECURITY_LEVEL>::zeroed();
        (::windows_core::Interface::vtable(self).get_PreferredSecurityLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsSecurityEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn AnswerWithSessionDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AnswerWithSessionDescription)(::windows_core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi()).ok()
    }
    pub unsafe fn ReInviteWithSessionDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReInviteWithSessionDescription)(::windows_core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
}
impl ::core::convert::From<IRTCSession2> for ::windows_core::IUnknown {
    fn from(value: IRTCSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSession2> for ::windows_core::IUnknown {
    fn from(value: &IRTCSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCSession2> for IRTCSession {
    fn from(value: IRTCSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSession2> for IRTCSession {
    fn from(value: &IRTCSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCSession> for IRTCSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCSession> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCSession> for &'a IRTCSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCSession> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCSession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSession2 {}
impl ::core::fmt::Debug for IRTCSession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSession2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCSession2 {
    type Vtable = IRTCSession2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17d7cdfc_b007_484c_99d2_86a8a820991d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession2_Vtbl {
    pub base__: IRTCSession_Vtbl,
    pub SendInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfoheader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lcookie: isize) -> ::windows_core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows_core::HRESULT,
    pub AnswerWithSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ReInviteWithSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lcookie: isize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCSessionCallControl(::windows_core::IUnknown);
impl IRTCSessionCallControl {
    pub unsafe fn Hold(&self, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Hold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn UnHold(&self, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnHold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn Forward<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrforwardtouri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Forward)(::windows_core::Interface::as_raw(self), bstrforwardtouri.into_param().abi()).ok()
    }
    pub unsafe fn Refer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrefertouri: Param0, bstrrefercookie: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refer)(::windows_core::Interface::as_raw(self), bstrrefertouri.into_param().abi(), bstrrefercookie.into_param().abi()).ok()
    }
    pub unsafe fn SetReferredByURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrreferredbyuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReferredByURI)(::windows_core::Interface::as_raw(self), bstrreferredbyuri.into_param().abi()).ok()
    }
    pub unsafe fn ReferredByURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ReferredByURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetReferCookie<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrefercookie: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReferCookie)(::windows_core::Interface::as_raw(self), bstrrefercookie.into_param().abi()).ok()
    }
    pub unsafe fn ReferCookie(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ReferCookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsReferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsReferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IRTCSessionCallControl> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionCallControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSessionCallControl> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionCallControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionCallControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionCallControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCSessionCallControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCSessionCallControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSessionCallControl {}
impl ::core::fmt::Debug for IRTCSessionCallControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionCallControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCSessionCallControl {
    type Vtable = IRTCSessionCallControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9a50d94_190b_4f82_9530_3b8ebf60758a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionCallControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Hold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows_core::HRESULT,
    pub UnHold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows_core::HRESULT,
    pub Forward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Refer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrefertouri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetReferredByURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetReferCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrefercookie: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsReferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisreferred: *mut i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCSessionDescriptionManager(::windows_core::IUnknown);
impl IRTCSessionDescriptionManager {
    pub unsafe fn EvaluateSessionDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1, pfapplicationsession: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EvaluateSessionDescription)(::windows_core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), ::core::mem::transmute(pfapplicationsession)).ok()
    }
}
impl ::core::convert::From<IRTCSessionDescriptionManager> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionDescriptionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSessionDescriptionManager> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionDescriptionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionDescriptionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionDescriptionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCSessionDescriptionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCSessionDescriptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSessionDescriptionManager {}
impl ::core::fmt::Debug for IRTCSessionDescriptionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionDescriptionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCSessionDescriptionManager {
    type Vtable = IRTCSessionDescriptionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba7f518e_d336_4070_93a6_865395c843f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionDescriptionManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EvaluateSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCSessionOperationCompleteEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCSessionOperationCompleteEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).Cookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionOperationCompleteEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionOperationCompleteEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionOperationCompleteEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionOperationCompleteEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCSessionOperationCompleteEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCSessionOperationCompleteEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCSessionOperationCompleteEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCSessionOperationCompleteEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionOperationCompleteEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCSessionOperationCompleteEvent {
    type Vtable = IRTCSessionOperationCompleteEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6bff4c0_f7c8_4d3c_9a41_3550f78a95b0);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCSessionOperationCompleteEvent2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCSessionOperationCompleteEvent2 {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Cookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows_core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Participant)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::win32_foundation::BSTR, pbstrsessiondescription: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRemoteSessionDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent2> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionOperationCompleteEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent2> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionOperationCompleteEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent2> for super::Com::IDispatch {
    fn from(value: IRTCSessionOperationCompleteEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCSessionOperationCompleteEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent2> for IRTCSessionOperationCompleteEvent {
    fn from(value: IRTCSessionOperationCompleteEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent2> for IRTCSessionOperationCompleteEvent {
    fn from(value: &IRTCSessionOperationCompleteEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCSessionOperationCompleteEvent> for IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCSessionOperationCompleteEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCSessionOperationCompleteEvent> for &'a IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCSessionOperationCompleteEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCSessionOperationCompleteEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCSessionOperationCompleteEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCSessionOperationCompleteEvent2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCSessionOperationCompleteEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionOperationCompleteEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCSessionOperationCompleteEvent2 {
    type Vtable = IRTCSessionOperationCompleteEvent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6fc2a9b_d5bc_4241_b436_1b8460c13832);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent2_Vtbl {
    pub base__: IRTCSessionOperationCompleteEvent_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::win32_foundation::BSTR, pbstrsessiondescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCSessionPortManagement(::windows_core::IUnknown);
impl IRTCSessionPortManagement {
    pub unsafe fn SetPortManager<'a, Param0: ::windows_core::IntoParam<'a, IRTCPortManager>>(&self, pportmanager: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPortManager)(::windows_core::Interface::as_raw(self), pportmanager.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRTCSessionPortManagement> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionPortManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSessionPortManagement> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionPortManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionPortManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionPortManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCSessionPortManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCSessionPortManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSessionPortManagement {}
impl ::core::fmt::Debug for IRTCSessionPortManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionPortManagement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCSessionPortManagement {
    type Vtable = IRTCSessionPortManagement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa072f1d6_0286_4e1f_85f2_17a2948456ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionPortManagement_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetPortManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportmanager: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCSessionReferStatusEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCSessionReferStatusEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn ReferStatus(&self) -> ::windows_core::Result<RTC_SESSION_REFER_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_REFER_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).ReferStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_REFER_STATUS>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionReferStatusEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionReferStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionReferStatusEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionReferStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionReferStatusEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionReferStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionReferStatusEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionReferStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCSessionReferStatusEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCSessionReferStatusEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCSessionReferStatusEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCSessionReferStatusEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionReferStatusEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCSessionReferStatusEvent {
    type Vtable = IRTCSessionReferStatusEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d8fc2cd_5d76_44ab_bb68_2a80353b34a2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReferStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCSessionReferredEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCSessionReferredEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn ReferredByURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ReferredByURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ReferToURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ReferToURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ReferCookie(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ReferCookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Accept(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Accept)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reject(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reject)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReferredSessionState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enstate)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionReferredEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionReferredEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionReferredEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionReferredEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionReferredEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionReferredEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionReferredEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionReferredEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCSessionReferredEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCSessionReferredEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCSessionReferredEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCSessionReferredEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionReferredEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCSessionReferredEvent {
    type Vtable = IRTCSessionReferredEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x176a6828_4fcc_4f28_a862_04597a6cf1c4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferredEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ReferToURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetReferredSessionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCSessionStateChangeEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCSessionStateChangeEvent {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionStateChangeEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionStateChangeEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCSessionStateChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCSessionStateChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCSessionStateChangeEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCSessionStateChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionStateChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCSessionStateChangeEvent {
    type Vtable = IRTCSessionStateChangeEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5bad703_5952_48b3_9321_7f4500521506);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCSessionStateChangeEvent2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCSessionStateChangeEvent2 {
    pub unsafe fn Session(&self) -> ::windows_core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Session)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SESSION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.StatusText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MediaTypes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MediaTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_SECURITY_LEVEL>::zeroed();
        (::windows_core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn IsForked(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsForked)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::win32_foundation::BSTR, pbstrsessiondescription: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRemoteSessionDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionStateChangeEvent2> for ::windows_core::IUnknown {
    fn from(value: IRTCSessionStateChangeEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent2> for ::windows_core::IUnknown {
    fn from(value: &IRTCSessionStateChangeEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionStateChangeEvent2> for super::Com::IDispatch {
    fn from(value: IRTCSessionStateChangeEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCSessionStateChangeEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCSessionStateChangeEvent2> for IRTCSessionStateChangeEvent {
    fn from(value: IRTCSessionStateChangeEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent2> for IRTCSessionStateChangeEvent {
    fn from(value: &IRTCSessionStateChangeEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCSessionStateChangeEvent> for IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCSessionStateChangeEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCSessionStateChangeEvent> for &'a IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCSessionStateChangeEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCSessionStateChangeEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCSessionStateChangeEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCSessionStateChangeEvent2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCSessionStateChangeEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionStateChangeEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCSessionStateChangeEvent2 {
    type Vtable = IRTCSessionStateChangeEvent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f933171_6f95_4880_80d9_2ec8d495d261);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent2_Vtbl {
    pub base__: IRTCSessionStateChangeEvent_Vtbl,
    pub MediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows_core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT,
    pub IsForked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisforked: *mut i16) -> ::windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::win32_foundation::BSTR, pbstrsessiondescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCUserSearch(::windows_core::IUnknown);
impl IRTCUserSearch {
    pub unsafe fn CreateQuery(&self) -> ::windows_core::Result<IRTCUserSearchQuery> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCUserSearchQuery>(result__)
    }
    pub unsafe fn ExecuteSearch<'a, Param0: ::windows_core::IntoParam<'a, IRTCUserSearchQuery>, Param1: ::windows_core::IntoParam<'a, IRTCProfile>>(&self, pquery: Param0, pprofile: Param1, lcookie: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecuteSearch)(::windows_core::Interface::as_raw(self), pquery.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
}
impl ::core::convert::From<IRTCUserSearch> for ::windows_core::IUnknown {
    fn from(value: IRTCUserSearch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCUserSearch> for ::windows_core::IUnknown {
    fn from(value: &IRTCUserSearch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCUserSearch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCUserSearch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCUserSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCUserSearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCUserSearch {}
impl ::core::fmt::Debug for IRTCUserSearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCUserSearch {
    type Vtable = IRTCUserSearch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb619882b_860c_4db4_be1b_693b6505bbe5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearch_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecuteSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquery: ::windows_core::RawPtr, pprofile: ::windows_core::RawPtr, lcookie: isize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCUserSearchQuery(::windows_core::IUnknown);
impl IRTCUserSearchQuery {
    pub unsafe fn put_SearchTerm<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0, bstrvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_SearchTerm)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn get_SearchTerm<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_SearchTerm)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SearchTerms(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SearchTerms)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_SearchPreference)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enpreference), ::core::mem::transmute(lvalue)).ok()
    }
    pub unsafe fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).get_SearchPreference)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enpreference), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSearchDomain<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdomain: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSearchDomain)(::windows_core::Interface::as_raw(self), bstrdomain.into_param().abi()).ok()
    }
    pub unsafe fn SearchDomain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SearchDomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IRTCUserSearchQuery> for ::windows_core::IUnknown {
    fn from(value: IRTCUserSearchQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCUserSearchQuery> for ::windows_core::IUnknown {
    fn from(value: &IRTCUserSearchQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCUserSearchQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCUserSearchQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCUserSearchQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCUserSearchQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCUserSearchQuery {}
impl ::core::fmt::Debug for IRTCUserSearchQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearchQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCUserSearchQuery {
    type Vtable = IRTCUserSearchQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x288300f5_d23a_4365_9a73_9985c98c2881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchQuery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub put_SearchTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub get_SearchTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SearchTerms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnames: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub put_SearchPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows_core::HRESULT,
    pub get_SearchPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetSearchDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SearchDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomain: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCUserSearchResult(::windows_core::IUnknown);
impl IRTCUserSearchResult {
    pub unsafe fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(encolumn), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IRTCUserSearchResult> for ::windows_core::IUnknown {
    fn from(value: IRTCUserSearchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCUserSearchResult> for ::windows_core::IUnknown {
    fn from(value: &IRTCUserSearchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCUserSearchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCUserSearchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCUserSearchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCUserSearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCUserSearchResult {}
impl ::core::fmt::Debug for IRTCUserSearchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCUserSearchResult {
    type Vtable = IRTCUserSearchResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x851278b2_9592_480f_8db5_2de86b26b54d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub get_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCUserSearchResultsEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCUserSearchResultsEvent {
    pub unsafe fn EnumerateResults(&self) -> ::windows_core::Result<IRTCEnumUserSearchResults> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateResults)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCEnumUserSearchResults>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Results(&self) -> ::windows_core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Results)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn Query(&self) -> ::windows_core::Result<IRTCUserSearchQuery> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCUserSearchQuery>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).Cookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MoreAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MoreAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCUserSearchResultsEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCUserSearchResultsEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCUserSearchResultsEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCUserSearchResultsEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCUserSearchResultsEvent> for super::Com::IDispatch {
    fn from(value: IRTCUserSearchResultsEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCUserSearchResultsEvent> for super::Com::IDispatch {
    fn from(value: &IRTCUserSearchResultsEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCUserSearchResultsEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCUserSearchResultsEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCUserSearchResultsEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCUserSearchResultsEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearchResultsEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCUserSearchResultsEvent {
    type Vtable = IRTCUserSearchResultsEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8c8c3cd_7fac_4088_81c5_c24cbc0938e3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResultsEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EnumerateResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Results: usize,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
    pub MoreAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCWatcher(::windows_core::IUnknown);
impl IRTCWatcher {
    pub unsafe fn PresentityURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PresentityURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPresentityURI)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Persistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPersistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_WATCHER_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_WATCHER_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_WATCHER_STATE>(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enstate)).ok()
    }
}
impl ::core::convert::From<IRTCWatcher> for ::windows_core::IUnknown {
    fn from(value: IRTCWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher> for ::windows_core::IUnknown {
    fn from(value: &IRTCWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCWatcher> for IRTCPresenceContact {
    fn from(value: IRTCWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher> for IRTCPresenceContact {
    fn from(value: &IRTCWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for IRTCWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for &'a IRTCWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCWatcher {}
impl ::core::fmt::Debug for IRTCWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCWatcher {
    type Vtable = IRTCWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7cedad8_346b_4d1b_ac02_a2088df9be4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRTCWatcher2(::windows_core::IUnknown);
impl IRTCWatcher2 {
    pub unsafe fn PresentityURI(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PresentityURI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPresentityURI)(::windows_core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Persistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPersistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<RTC_WATCHER_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_WATCHER_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_WATCHER_STATE>(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enstate)).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<IRTCProfile2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<RTC_ACE_SCOPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_ACE_SCOPE>::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_ACE_SCOPE>(result__)
    }
}
impl ::core::convert::From<IRTCWatcher2> for ::windows_core::IUnknown {
    fn from(value: IRTCWatcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher2> for ::windows_core::IUnknown {
    fn from(value: &IRTCWatcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCWatcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCWatcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCWatcher2> for IRTCPresenceContact {
    fn from(value: IRTCWatcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher2> for IRTCPresenceContact {
    fn from(value: &IRTCWatcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for IRTCWatcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCPresenceContact> for &'a IRTCWatcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCPresenceContact> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCWatcher2> for IRTCWatcher {
    fn from(value: IRTCWatcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher2> for IRTCWatcher {
    fn from(value: &IRTCWatcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCWatcher> for IRTCWatcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCWatcher> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRTCWatcher> for &'a IRTCWatcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCWatcher> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRTCWatcher2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRTCWatcher2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCWatcher2 {}
impl ::core::fmt::Debug for IRTCWatcher2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcher2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRTCWatcher2 {
    type Vtable = IRTCWatcher2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4d9967f_d011_4b1d_91e3_aba78f96393d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher2_Vtbl {
    pub base__: IRTCWatcher_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCWatcherEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCWatcherEvent {
    pub unsafe fn Watcher(&self) -> ::windows_core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Watcher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCWatcherEvent> for ::windows_core::IUnknown {
    fn from(value: IRTCWatcherEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCWatcherEvent> for ::windows_core::IUnknown {
    fn from(value: &IRTCWatcherEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCWatcherEvent> for super::Com::IDispatch {
    fn from(value: IRTCWatcherEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCWatcherEvent> for super::Com::IDispatch {
    fn from(value: &IRTCWatcherEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCWatcherEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCWatcherEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCWatcherEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCWatcherEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcherEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCWatcherEvent {
    type Vtable = IRTCWatcherEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf30d7261_587a_424f_822c_312788f43548);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Watcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwatcher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRTCWatcherEvent2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRTCWatcherEvent2 {
    pub unsafe fn Watcher(&self) -> ::windows_core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Watcher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows_core::Result<RTC_WATCHER_EVENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<RTC_WATCHER_EVENT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RTC_WATCHER_EVENT_TYPE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).StatusCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCWatcherEvent2> for ::windows_core::IUnknown {
    fn from(value: IRTCWatcherEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCWatcherEvent2> for ::windows_core::IUnknown {
    fn from(value: &IRTCWatcherEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCWatcherEvent2> for super::Com::IDispatch {
    fn from(value: IRTCWatcherEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCWatcherEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCWatcherEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRTCWatcherEvent2> for IRTCWatcherEvent {
    fn from(value: IRTCWatcherEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRTCWatcherEvent2> for IRTCWatcherEvent {
    fn from(value: &IRTCWatcherEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCWatcherEvent> for IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCWatcherEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IRTCWatcherEvent> for &'a IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRTCWatcherEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRTCWatcherEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRTCWatcherEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRTCWatcherEvent2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRTCWatcherEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcherEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRTCWatcherEvent2 {
    type Vtable = IRTCWatcherEvent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe52891e8_188c_49af_b005_98ed13f83f9c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent2_Vtbl {
    pub base__: IRTCWatcherEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransportSettingsInternal(::windows_core::IUnknown);
impl ITransportSettingsInternal {
    #[cfg(feature = "win32-networking")]
    pub unsafe fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplySetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(setting)).ok()
    }
    #[cfg(feature = "win32-networking")]
    pub unsafe fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QuerySetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(setting)).ok()
    }
}
impl ::core::convert::From<ITransportSettingsInternal> for ::windows_core::IUnknown {
    fn from(value: ITransportSettingsInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransportSettingsInternal> for ::windows_core::IUnknown {
    fn from(value: &ITransportSettingsInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransportSettingsInternal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransportSettingsInternal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransportSettingsInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransportSettingsInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransportSettingsInternal {}
impl ::core::fmt::Debug for ITransportSettingsInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransportSettingsInternal").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransportSettingsInternal {
    type Vtable = ITransportSettingsInternal_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5123e076_29e3_4bfd_84fe_0192d411e3e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransportSettingsInternal_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-networking")]
    pub ApplySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-networking"))]
    ApplySetting: usize,
    #[cfg(feature = "win32-networking")]
    pub QuerySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-networking"))]
    QuerySetting: usize,
}
pub const RTCAU_BASIC: u32 = 1u32;
pub const RTCAU_DIGEST: u32 = 2u32;
pub const RTCAU_KERBEROS: u32 = 8u32;
pub const RTCAU_NTLM: u32 = 4u32;
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
pub const RTCClient: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a42ea29_a2b7_40c4_b091_f6f024aa89be);
pub const RTCEF_ALL: u32 = 33554431u32;
pub const RTCEF_BUDDY: u32 = 256u32;
pub const RTCEF_BUDDY2: u32 = 262144u32;
pub const RTCEF_CLIENT: u32 = 1u32;
pub const RTCEF_GROUP: u32 = 8192u32;
pub const RTCEF_INFO: u32 = 4096u32;
pub const RTCEF_INTENSITY: u32 = 64u32;
pub const RTCEF_MEDIA: u32 = 32u32;
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
pub const RTCEF_MESSAGING: u32 = 128u32;
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
pub const RTCEF_PROFILE: u32 = 1024u32;
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
pub const RTCEF_REINVITE: u32 = 4194304u32;
pub const RTCEF_ROAMING: u32 = 65536u32;
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
pub const RTCEF_USERSEARCH: u32 = 2048u32;
pub const RTCEF_WATCHER: u32 = 512u32;
pub const RTCEF_WATCHER2: u32 = 524288u32;
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
pub const RTCSI_APPLICATION: u32 = 32u32;
pub const RTCSI_IM: u32 = 8u32;
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
pub const RTCSI_PC_TO_PC: u32 = 1u32;
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
pub const RTCTR_TCP: u32 = 2u32;
pub const RTCTR_TLS: u32 = 4u32;
pub const RTCTR_UDP: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_ACE_SCOPE(pub i32);
pub const RTCAS_SCOPE_USER: RTC_ACE_SCOPE = RTC_ACE_SCOPE(0i32);
pub const RTCAS_SCOPE_DOMAIN: RTC_ACE_SCOPE = RTC_ACE_SCOPE(1i32);
pub const RTCAS_SCOPE_ALL: RTC_ACE_SCOPE = RTC_ACE_SCOPE(2i32);
impl ::core::marker::Copy for RTC_ACE_SCOPE {}
impl ::core::clone::Clone for RTC_ACE_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_ACE_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_ACE_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_ACE_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ACE_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_ANSWER_MODE(pub i32);
pub const RTCAM_OFFER_SESSION_EVENT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(0i32);
pub const RTCAM_AUTOMATICALLY_ACCEPT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(1i32);
pub const RTCAM_AUTOMATICALLY_REJECT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(2i32);
pub const RTCAM_NOT_SUPPORTED: RTC_ANSWER_MODE = RTC_ANSWER_MODE(3i32);
impl ::core::marker::Copy for RTC_ANSWER_MODE {}
impl ::core::clone::Clone for RTC_ANSWER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_ANSWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_ANSWER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_ANSWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ANSWER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_AUDIO_DEVICE(pub i32);
pub const RTCAD_SPEAKER: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(0i32);
pub const RTCAD_MICROPHONE: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(1i32);
impl ::core::marker::Copy for RTC_AUDIO_DEVICE {}
impl ::core::clone::Clone for RTC_AUDIO_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_AUDIO_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_AUDIO_DEVICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_AUDIO_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_AUDIO_DEVICE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_BUDDY_EVENT_TYPE(pub i32);
pub const RTCBET_BUDDY_ADD: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(0i32);
pub const RTCBET_BUDDY_REMOVE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(1i32);
pub const RTCBET_BUDDY_UPDATE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(2i32);
pub const RTCBET_BUDDY_STATE_CHANGE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(3i32);
pub const RTCBET_BUDDY_ROAMED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(4i32);
pub const RTCBET_BUDDY_SUBSCRIBED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(5i32);
impl ::core::marker::Copy for RTC_BUDDY_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_BUDDY_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_BUDDY_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_BUDDY_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_BUDDY_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_BUDDY_SUBSCRIPTION_TYPE(pub i32);
pub const RTCBT_SUBSCRIBED: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(0i32);
pub const RTCBT_ALWAYS_OFFLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(1i32);
pub const RTCBT_ALWAYS_ONLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(2i32);
pub const RTCBT_POLL: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(3i32);
impl ::core::marker::Copy for RTC_BUDDY_SUBSCRIPTION_TYPE {}
impl ::core::clone::Clone for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_BUDDY_SUBSCRIPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_SUBSCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_CLIENT_EVENT_TYPE(pub i32);
pub const RTCCET_VOLUME_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(0i32);
pub const RTCCET_DEVICE_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(1i32);
pub const RTCCET_NETWORK_QUALITY_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(2i32);
pub const RTCCET_ASYNC_CLEANUP_DONE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(3i32);
impl ::core::marker::Copy for RTC_CLIENT_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_CLIENT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_CLIENT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_CLIENT_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_CLIENT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_CLIENT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_DTMF(pub i32);
pub const RTC_DTMF_0: RTC_DTMF = RTC_DTMF(0i32);
pub const RTC_DTMF_1: RTC_DTMF = RTC_DTMF(1i32);
pub const RTC_DTMF_2: RTC_DTMF = RTC_DTMF(2i32);
pub const RTC_DTMF_3: RTC_DTMF = RTC_DTMF(3i32);
pub const RTC_DTMF_4: RTC_DTMF = RTC_DTMF(4i32);
pub const RTC_DTMF_5: RTC_DTMF = RTC_DTMF(5i32);
pub const RTC_DTMF_6: RTC_DTMF = RTC_DTMF(6i32);
pub const RTC_DTMF_7: RTC_DTMF = RTC_DTMF(7i32);
pub const RTC_DTMF_8: RTC_DTMF = RTC_DTMF(8i32);
pub const RTC_DTMF_9: RTC_DTMF = RTC_DTMF(9i32);
pub const RTC_DTMF_STAR: RTC_DTMF = RTC_DTMF(10i32);
pub const RTC_DTMF_POUND: RTC_DTMF = RTC_DTMF(11i32);
pub const RTC_DTMF_A: RTC_DTMF = RTC_DTMF(12i32);
pub const RTC_DTMF_B: RTC_DTMF = RTC_DTMF(13i32);
pub const RTC_DTMF_C: RTC_DTMF = RTC_DTMF(14i32);
pub const RTC_DTMF_D: RTC_DTMF = RTC_DTMF(15i32);
pub const RTC_DTMF_FLASH: RTC_DTMF = RTC_DTMF(16i32);
impl ::core::marker::Copy for RTC_DTMF {}
impl ::core::clone::Clone for RTC_DTMF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_DTMF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_DTMF {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_DTMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_DTMF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_EVENT(pub i32);
pub const RTCE_CLIENT: RTC_EVENT = RTC_EVENT(0i32);
pub const RTCE_REGISTRATION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(1i32);
pub const RTCE_SESSION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(2i32);
pub const RTCE_SESSION_OPERATION_COMPLETE: RTC_EVENT = RTC_EVENT(3i32);
pub const RTCE_PARTICIPANT_STATE_CHANGE: RTC_EVENT = RTC_EVENT(4i32);
pub const RTCE_MEDIA: RTC_EVENT = RTC_EVENT(5i32);
pub const RTCE_INTENSITY: RTC_EVENT = RTC_EVENT(6i32);
pub const RTCE_MESSAGING: RTC_EVENT = RTC_EVENT(7i32);
pub const RTCE_BUDDY: RTC_EVENT = RTC_EVENT(8i32);
pub const RTCE_WATCHER: RTC_EVENT = RTC_EVENT(9i32);
pub const RTCE_PROFILE: RTC_EVENT = RTC_EVENT(10i32);
pub const RTCE_USERSEARCH: RTC_EVENT = RTC_EVENT(11i32);
pub const RTCE_INFO: RTC_EVENT = RTC_EVENT(12i32);
pub const RTCE_GROUP: RTC_EVENT = RTC_EVENT(13i32);
pub const RTCE_MEDIA_REQUEST: RTC_EVENT = RTC_EVENT(14i32);
pub const RTCE_ROAMING: RTC_EVENT = RTC_EVENT(15i32);
pub const RTCE_PRESENCE_PROPERTY: RTC_EVENT = RTC_EVENT(16i32);
pub const RTCE_PRESENCE_DATA: RTC_EVENT = RTC_EVENT(17i32);
pub const RTCE_PRESENCE_STATUS: RTC_EVENT = RTC_EVENT(18i32);
pub const RTCE_SESSION_REFER_STATUS: RTC_EVENT = RTC_EVENT(19i32);
pub const RTCE_SESSION_REFERRED: RTC_EVENT = RTC_EVENT(20i32);
pub const RTCE_REINVITE: RTC_EVENT = RTC_EVENT(21i32);
impl ::core::marker::Copy for RTC_EVENT {}
impl ::core::clone::Clone for RTC_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_EVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_EVENT").field(&self.0).finish()
    }
}
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885961i32);
pub const RTC_E_BASIC_AUTH_SET_TLS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886017i32);
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886042i32);
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886041i32);
pub const RTC_E_CLIENT_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886043i32);
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886061i32);
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886059i32);
pub const RTC_E_DUPLICATE_BUDDY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886006i32);
pub const RTC_E_DUPLICATE_GROUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885998i32);
pub const RTC_E_DUPLICATE_REALM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886013i32);
pub const RTC_E_DUPLICATE_WATCHER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886005i32);
pub const RTC_E_INVALID_ACL_LIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886000i32);
pub const RTC_E_INVALID_ADDRESS_LOCAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886060i32);
pub const RTC_E_INVALID_BUDDY_LIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886001i32);
pub const RTC_E_INVALID_LISTEN_SOCKET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885957i32);
pub const RTC_E_INVALID_OBJECT_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885983i32);
pub const RTC_E_INVALID_PORTRANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885988i32);
pub const RTC_E_INVALID_PREFERENCE_LIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885991i32);
pub const RTC_E_INVALID_PROFILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886034i32);
pub const RTC_E_INVALID_PROXY_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886058i32);
pub const RTC_E_INVALID_REGISTRATION_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885971i32);
pub const RTC_E_INVALID_SESSION_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886038i32);
pub const RTC_E_INVALID_SESSION_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886039i32);
pub const RTC_E_INVALID_SIP_URL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886062i32);
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885958i32);
pub const RTC_E_LOCAL_PHONE_NEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886036i32);
pub const RTC_E_MALFORMED_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886004i32);
pub const RTC_E_MAX_PENDING_OPERATIONS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885990i32);
pub const RTC_E_MAX_REDIRECTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885960i32);
pub const RTC_E_MEDIA_AEC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886044i32);
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886047i32);
pub const RTC_E_MEDIA_CONTROLLER_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886049i32);
pub const RTC_E_MEDIA_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885970i32);
pub const RTC_E_MEDIA_ENABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885969i32);
pub const RTC_E_MEDIA_NEED_TERMINAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886048i32);
pub const RTC_E_MEDIA_SESSION_IN_HOLD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885962i32);
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885963i32);
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886046i32);
pub const RTC_E_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885950i32);
pub const RTC_E_NOT_EXIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885992i32);
pub const RTC_E_NOT_PRESENCE_PROFILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885974i32);
pub const RTC_E_NO_BUDDY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885996i32);
pub const RTC_E_NO_DEVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886035i32);
pub const RTC_E_NO_GROUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885999i32);
pub const RTC_E_NO_PROFILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886037i32);
pub const RTC_E_NO_REALM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885994i32);
pub const RTC_E_NO_TRANSPORT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885993i32);
pub const RTC_E_NO_WATCHER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885995i32);
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886018i32);
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131755001i32);
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131754997i32);
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131755003i32);
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131754998i32);
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131755002i32);
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131755000i32);
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131754999i32);
pub const RTC_E_PLATFORM_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885952i32);
pub const RTC_E_POLICY_NOT_ALLOW: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886012i32);
pub const RTC_E_PORT_MANAGER_ALREADY_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885956i32);
pub const RTC_E_PORT_MAPPING_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886010i32);
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886011i32);
pub const RTC_E_PRESENCE_ENABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885982i32);
pub const RTC_E_PRESENCE_NOT_ENABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886040i32);
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886024i32);
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886025i32);
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886023i32);
pub const RTC_E_PROFILE_INVALID_SESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886021i32);
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886020i32);
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886019i32);
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886022i32);
pub const RTC_E_PROFILE_NO_KEY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886032i32);
pub const RTC_E_PROFILE_NO_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886031i32);
pub const RTC_E_PROFILE_NO_PROVISION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886033i32);
pub const RTC_E_PROFILE_NO_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886028i32);
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886027i32);
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886026i32);
pub const RTC_E_PROFILE_NO_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886030i32);
pub const RTC_E_PROFILE_NO_USER_URI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886029i32);
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886014i32);
pub const RTC_E_REDIRECT_PROCESSING_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885959i32);
pub const RTC_E_REFER_NOT_ACCEPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885968i32);
pub const RTC_E_REFER_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885967i32);
pub const RTC_E_REFER_NOT_EXIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885966i32);
pub const RTC_E_REGISTRATION_DEACTIVATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885949i32);
pub const RTC_E_REGISTRATION_REJECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885948i32);
pub const RTC_E_REGISTRATION_UNREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885947i32);
pub const RTC_E_ROAMING_ENABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885981i32);
pub const RTC_E_ROAMING_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886002i32);
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886003i32);
pub const RTC_E_SDP_CONNECTION_ADDR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886070i32);
pub const RTC_E_SDP_FAILED_TO_BUILD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886067i32);
pub const RTC_E_SDP_MULTICAST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886071i32);
pub const RTC_E_SDP_NOT_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886074i32);
pub const RTC_E_SDP_NO_MEDIA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886069i32);
pub const RTC_E_SDP_PARSE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886073i32);
pub const RTC_E_SDP_UPDATE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886072i32);
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885955i32);
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886009i32);
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886008i32);
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886007i32);
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885986i32);
pub const RTC_E_SIP_AUTH_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886063i32);
pub const RTC_E_SIP_AUTH_HEADER_SENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886065i32);
pub const RTC_E_SIP_AUTH_TIME_SKEW: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885972i32);
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886064i32);
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885987i32);
pub const RTC_E_SIP_CALL_DISCONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886055i32);
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886080i32);
pub const RTC_E_SIP_DNS_FAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885978i32);
pub const RTC_E_SIP_HEADER_NOT_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886075i32);
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886016i32);
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885965i32);
pub const RTC_E_SIP_INVALID_CERTIFICATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885979i32);
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885973i32);
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886066i32);
pub const RTC_E_SIP_NEED_MORE_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886056i32);
pub const RTC_E_SIP_NO_STREAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886077i32);
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885984i32);
pub const RTC_E_SIP_PARSE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886076i32);
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885985i32);
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885951i32);
pub const RTC_E_SIP_REFER_OPERATION_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885953i32);
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886054i32);
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886051i32);
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886052i32);
pub const RTC_E_SIP_STACK_SHUTDOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886050i32);
pub const RTC_E_SIP_STREAM_NOT_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886078i32);
pub const RTC_E_SIP_STREAM_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886079i32);
pub const RTC_E_SIP_TCP_FAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885977i32);
pub const RTC_E_SIP_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886068i32);
pub const RTC_E_SIP_TLS_FAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885975i32);
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885980i32);
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886057i32);
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886053i32);
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885964i32);
pub const RTC_E_START_STREAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131886045i32);
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820060i32);
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820059i32);
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820124i32);
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820144i32);
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820058i32);
pub const RTC_E_STATUS_CLIENT_CONFLICT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820135i32);
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820141i32);
pub const RTC_E_STATUS_CLIENT_GONE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820134i32);
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820133i32);
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820062i32);
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820139i32);
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820138i32);
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820140i32);
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820142i32);
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820137i32);
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820131i32);
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820136i32);
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820130i32);
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820064i32);
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820061i32);
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820063i32);
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820143i32);
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820129i32);
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131819944i32);
pub const RTC_E_STATUS_GLOBAL_DECLINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131819941i32);
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131819940i32);
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131819938i32);
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: ::windows_core::HRESULT = ::windows_core::HRESULT(15663285i32);
pub const RTC_E_STATUS_INFO_QUEUED: ::windows_core::HRESULT = ::windows_core::HRESULT(15663286i32);
pub const RTC_E_STATUS_INFO_RINGING: ::windows_core::HRESULT = ::windows_core::HRESULT(15663284i32);
pub const RTC_E_STATUS_INFO_TRYING: ::windows_core::HRESULT = ::windows_core::HRESULT(15663204i32);
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820056i32);
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820164i32);
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820243i32);
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820242i32);
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820244i32);
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820241i32);
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820239i32);
pub const RTC_E_STATUS_REQUEST_TERMINATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820057i32);
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820042i32);
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820044i32);
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820043i32);
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820040i32);
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820041i32);
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131820039i32);
pub const RTC_E_STATUS_SESSION_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(15663287i32);
pub const RTC_E_STATUS_SUCCESS: ::windows_core::HRESULT = ::windows_core::HRESULT(15663304i32);
pub const RTC_E_TOO_MANY_GROUPS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885997i32);
pub const RTC_E_TOO_MANY_RETRIES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885989i32);
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885976i32);
pub const RTC_E_UDP_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2131885954i32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_GROUP_EVENT_TYPE(pub i32);
pub const RTCGET_GROUP_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(0i32);
pub const RTCGET_GROUP_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(1i32);
pub const RTCGET_GROUP_UPDATE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(2i32);
pub const RTCGET_GROUP_BUDDY_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(3i32);
pub const RTCGET_GROUP_BUDDY_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(4i32);
pub const RTCGET_GROUP_ROAMED: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(5i32);
impl ::core::marker::Copy for RTC_GROUP_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_GROUP_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_GROUP_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_GROUP_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_LISTEN_MODE(pub i32);
pub const RTCLM_NONE: RTC_LISTEN_MODE = RTC_LISTEN_MODE(0i32);
pub const RTCLM_DYNAMIC: RTC_LISTEN_MODE = RTC_LISTEN_MODE(1i32);
pub const RTCLM_BOTH: RTC_LISTEN_MODE = RTC_LISTEN_MODE(2i32);
impl ::core::marker::Copy for RTC_LISTEN_MODE {}
impl ::core::clone::Clone for RTC_LISTEN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_LISTEN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_LISTEN_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_LISTEN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_LISTEN_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_MEDIA_EVENT_REASON(pub i32);
pub const RTCMER_NORMAL: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(0i32);
pub const RTCMER_HOLD: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(1i32);
pub const RTCMER_TIMEOUT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(2i32);
pub const RTCMER_BAD_DEVICE: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(3i32);
pub const RTCMER_NO_PORT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(4i32);
pub const RTCMER_PORT_MAPPING_FAILED: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(5i32);
pub const RTCMER_REMOTE_REQUEST: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(6i32);
impl ::core::marker::Copy for RTC_MEDIA_EVENT_REASON {}
impl ::core::clone::Clone for RTC_MEDIA_EVENT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MEDIA_EVENT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_MEDIA_EVENT_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_MEDIA_EVENT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_MEDIA_EVENT_TYPE(pub i32);
pub const RTCMET_STOPPED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(0i32);
pub const RTCMET_STARTED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(1i32);
pub const RTCMET_FAILED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(2i32);
impl ::core::marker::Copy for RTC_MEDIA_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_MEDIA_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MEDIA_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_MEDIA_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_MEDIA_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_MESSAGING_EVENT_TYPE(pub i32);
pub const RTCMSET_MESSAGE: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(0i32);
pub const RTCMSET_STATUS: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(1i32);
impl ::core::marker::Copy for RTC_MESSAGING_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_MESSAGING_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MESSAGING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_MESSAGING_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_MESSAGING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_MESSAGING_USER_STATUS(pub i32);
pub const RTCMUS_IDLE: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(0i32);
pub const RTCMUS_TYPING: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(1i32);
impl ::core::marker::Copy for RTC_MESSAGING_USER_STATUS {}
impl ::core::clone::Clone for RTC_MESSAGING_USER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MESSAGING_USER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_MESSAGING_USER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_MESSAGING_USER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_USER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_OFFER_WATCHER_MODE(pub i32);
pub const RTCOWM_OFFER_WATCHER_EVENT: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(0i32);
pub const RTCOWM_AUTOMATICALLY_ADD_WATCHER: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(1i32);
impl ::core::marker::Copy for RTC_OFFER_WATCHER_MODE {}
impl ::core::clone::Clone for RTC_OFFER_WATCHER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_OFFER_WATCHER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_OFFER_WATCHER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_OFFER_WATCHER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_OFFER_WATCHER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PARTICIPANT_STATE(pub i32);
pub const RTCPS_IDLE: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(0i32);
pub const RTCPS_PENDING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(1i32);
pub const RTCPS_INCOMING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(2i32);
pub const RTCPS_ANSWERING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(3i32);
pub const RTCPS_INPROGRESS: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(4i32);
pub const RTCPS_ALERTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(5i32);
pub const RTCPS_CONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(6i32);
pub const RTCPS_DISCONNECTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(7i32);
pub const RTCPS_DISCONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(8i32);
impl ::core::marker::Copy for RTC_PARTICIPANT_STATE {}
impl ::core::clone::Clone for RTC_PARTICIPANT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PARTICIPANT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PARTICIPANT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PARTICIPANT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PARTICIPANT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PORT_TYPE(pub i32);
pub const RTCPT_AUDIO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(0i32);
pub const RTCPT_AUDIO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(1i32);
pub const RTCPT_VIDEO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(2i32);
pub const RTCPT_VIDEO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(3i32);
pub const RTCPT_SIP: RTC_PORT_TYPE = RTC_PORT_TYPE(4i32);
impl ::core::marker::Copy for RTC_PORT_TYPE {}
impl ::core::clone::Clone for RTC_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PORT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PRESENCE_PROPERTY(pub i32);
pub const RTCPP_PHONENUMBER: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(0i32);
pub const RTCPP_DISPLAYNAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(1i32);
pub const RTCPP_EMAIL: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(2i32);
pub const RTCPP_DEVICE_NAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(3i32);
pub const RTCPP_MULTIPLE: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(4i32);
impl ::core::marker::Copy for RTC_PRESENCE_PROPERTY {}
impl ::core::clone::Clone for RTC_PRESENCE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PRESENCE_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PRESENCE_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PRESENCE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PRESENCE_STATUS(pub i32);
pub const RTCXS_PRESENCE_OFFLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(0i32);
pub const RTCXS_PRESENCE_ONLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(1i32);
pub const RTCXS_PRESENCE_AWAY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(2i32);
pub const RTCXS_PRESENCE_IDLE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(3i32);
pub const RTCXS_PRESENCE_BUSY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(4i32);
pub const RTCXS_PRESENCE_BE_RIGHT_BACK: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(5i32);
pub const RTCXS_PRESENCE_ON_THE_PHONE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(6i32);
pub const RTCXS_PRESENCE_OUT_TO_LUNCH: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(7i32);
impl ::core::marker::Copy for RTC_PRESENCE_STATUS {}
impl ::core::clone::Clone for RTC_PRESENCE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PRESENCE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PRESENCE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PRESENCE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PRIVACY_MODE(pub i32);
pub const RTCPM_BLOCK_LIST_EXCLUDED: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(0i32);
pub const RTCPM_ALLOW_LIST_ONLY: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(1i32);
impl ::core::marker::Copy for RTC_PRIVACY_MODE {}
impl ::core::clone::Clone for RTC_PRIVACY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PRIVACY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PRIVACY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PRIVACY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRIVACY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PROFILE_EVENT_TYPE(pub i32);
pub const RTCPFET_PROFILE_GET: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(0i32);
pub const RTCPFET_PROFILE_UPDATE: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(1i32);
impl ::core::marker::Copy for RTC_PROFILE_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_PROFILE_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PROFILE_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PROFILE_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PROFILE_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PROFILE_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_PROVIDER_URI(pub i32);
pub const RTCPU_URIHOMEPAGE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(0i32);
pub const RTCPU_URIHELPDESK: RTC_PROVIDER_URI = RTC_PROVIDER_URI(1i32);
pub const RTCPU_URIPERSONALACCOUNT: RTC_PROVIDER_URI = RTC_PROVIDER_URI(2i32);
pub const RTCPU_URIDISPLAYDURINGCALL: RTC_PROVIDER_URI = RTC_PROVIDER_URI(3i32);
pub const RTCPU_URIDISPLAYDURINGIDLE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(4i32);
impl ::core::marker::Copy for RTC_PROVIDER_URI {}
impl ::core::clone::Clone for RTC_PROVIDER_URI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PROVIDER_URI {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_PROVIDER_URI {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_PROVIDER_URI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PROVIDER_URI").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_REGISTRATION_STATE(pub i32);
pub const RTCRS_NOT_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(0i32);
pub const RTCRS_REGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(1i32);
pub const RTCRS_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(2i32);
pub const RTCRS_REJECTED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(3i32);
pub const RTCRS_UNREGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(4i32);
pub const RTCRS_ERROR: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(5i32);
pub const RTCRS_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(6i32);
pub const RTCRS_LOCAL_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(7i32);
pub const RTCRS_REMOTE_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(8i32);
impl ::core::marker::Copy for RTC_REGISTRATION_STATE {}
impl ::core::clone::Clone for RTC_REGISTRATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_REGISTRATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_REGISTRATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_REGISTRATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_REGISTRATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_REINVITE_STATE(pub i32);
pub const RTCRIN_INCOMING: RTC_REINVITE_STATE = RTC_REINVITE_STATE(0i32);
pub const RTCRIN_SUCCEEDED: RTC_REINVITE_STATE = RTC_REINVITE_STATE(1i32);
pub const RTCRIN_FAIL: RTC_REINVITE_STATE = RTC_REINVITE_STATE(2i32);
impl ::core::marker::Copy for RTC_REINVITE_STATE {}
impl ::core::clone::Clone for RTC_REINVITE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_REINVITE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_REINVITE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_REINVITE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_REINVITE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_RING_TYPE(pub i32);
pub const RTCRT_PHONE: RTC_RING_TYPE = RTC_RING_TYPE(0i32);
pub const RTCRT_MESSAGE: RTC_RING_TYPE = RTC_RING_TYPE(1i32);
pub const RTCRT_RINGBACK: RTC_RING_TYPE = RTC_RING_TYPE(2i32);
impl ::core::marker::Copy for RTC_RING_TYPE {}
impl ::core::clone::Clone for RTC_RING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_RING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_RING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_RING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_RING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_ROAMING_EVENT_TYPE(pub i32);
pub const RTCRET_BUDDY_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(0i32);
pub const RTCRET_WATCHER_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(1i32);
pub const RTCRET_PRESENCE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(2i32);
pub const RTCRET_PROFILE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(3i32);
pub const RTCRET_WPENDING_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(4i32);
impl ::core::marker::Copy for RTC_ROAMING_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_ROAMING_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_ROAMING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_ROAMING_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_ROAMING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ROAMING_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_SECURITY_LEVEL(pub i32);
pub const RTCSECL_UNSUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(1i32);
pub const RTCSECL_SUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(2i32);
pub const RTCSECL_REQUIRED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(3i32);
impl ::core::marker::Copy for RTC_SECURITY_LEVEL {}
impl ::core::clone::Clone for RTC_SECURITY_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SECURITY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_SECURITY_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_SECURITY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_SECURITY_TYPE(pub i32);
pub const RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(0i32);
pub const RTCSECT_T120_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(1i32);
impl ::core::marker::Copy for RTC_SECURITY_TYPE {}
impl ::core::clone::Clone for RTC_SECURITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SECURITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_SECURITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_SECURITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_SESSION_REFER_STATUS(pub i32);
pub const RTCSRS_REFERRING: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(0i32);
pub const RTCSRS_ACCEPTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(1i32);
pub const RTCSRS_ERROR: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(2i32);
pub const RTCSRS_REJECTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(3i32);
pub const RTCSRS_DROPPED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(4i32);
pub const RTCSRS_DONE: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(5i32);
impl ::core::marker::Copy for RTC_SESSION_REFER_STATUS {}
impl ::core::clone::Clone for RTC_SESSION_REFER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SESSION_REFER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_SESSION_REFER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_SESSION_REFER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_REFER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_SESSION_STATE(pub i32);
pub const RTCSS_IDLE: RTC_SESSION_STATE = RTC_SESSION_STATE(0i32);
pub const RTCSS_INCOMING: RTC_SESSION_STATE = RTC_SESSION_STATE(1i32);
pub const RTCSS_ANSWERING: RTC_SESSION_STATE = RTC_SESSION_STATE(2i32);
pub const RTCSS_INPROGRESS: RTC_SESSION_STATE = RTC_SESSION_STATE(3i32);
pub const RTCSS_CONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(4i32);
pub const RTCSS_DISCONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(5i32);
pub const RTCSS_HOLD: RTC_SESSION_STATE = RTC_SESSION_STATE(6i32);
pub const RTCSS_REFER: RTC_SESSION_STATE = RTC_SESSION_STATE(7i32);
impl ::core::marker::Copy for RTC_SESSION_STATE {}
impl ::core::clone::Clone for RTC_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_SESSION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_SESSION_TYPE(pub i32);
pub const RTCST_PC_TO_PC: RTC_SESSION_TYPE = RTC_SESSION_TYPE(0i32);
pub const RTCST_PC_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(1i32);
pub const RTCST_PHONE_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(2i32);
pub const RTCST_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(3i32);
pub const RTCST_MULTIPARTY_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(4i32);
pub const RTCST_APPLICATION: RTC_SESSION_TYPE = RTC_SESSION_TYPE(5i32);
impl ::core::marker::Copy for RTC_SESSION_TYPE {}
impl ::core::clone::Clone for RTC_SESSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SESSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_SESSION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_TYPE").field(&self.0).finish()
    }
}
pub const RTC_S_ROAMING_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(15597633i32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_T120_APPLET(pub i32);
pub const RTCTA_WHITEBOARD: RTC_T120_APPLET = RTC_T120_APPLET(0i32);
pub const RTCTA_APPSHARING: RTC_T120_APPLET = RTC_T120_APPLET(1i32);
impl ::core::marker::Copy for RTC_T120_APPLET {}
impl ::core::clone::Clone for RTC_T120_APPLET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_T120_APPLET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_T120_APPLET {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_T120_APPLET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_T120_APPLET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_TERMINATE_REASON(pub i32);
pub const RTCTR_NORMAL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(0i32);
pub const RTCTR_DND: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(1i32);
pub const RTCTR_BUSY: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(2i32);
pub const RTCTR_REJECT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(3i32);
pub const RTCTR_TIMEOUT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(4i32);
pub const RTCTR_SHUTDOWN: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(5i32);
pub const RTCTR_INSUFFICIENT_SECURITY_LEVEL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(6i32);
pub const RTCTR_NOT_SUPPORTED: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(7i32);
impl ::core::marker::Copy for RTC_TERMINATE_REASON {}
impl ::core::clone::Clone for RTC_TERMINATE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_TERMINATE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_TERMINATE_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_TERMINATE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_TERMINATE_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_USER_SEARCH_COLUMN(pub i32);
pub const RTCUSC_URI: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(0i32);
pub const RTCUSC_DISPLAYNAME: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(1i32);
pub const RTCUSC_TITLE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(2i32);
pub const RTCUSC_OFFICE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(3i32);
pub const RTCUSC_PHONE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(4i32);
pub const RTCUSC_COMPANY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(5i32);
pub const RTCUSC_CITY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(6i32);
pub const RTCUSC_STATE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(7i32);
pub const RTCUSC_COUNTRY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(8i32);
pub const RTCUSC_EMAIL: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(9i32);
impl ::core::marker::Copy for RTC_USER_SEARCH_COLUMN {}
impl ::core::clone::Clone for RTC_USER_SEARCH_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_USER_SEARCH_COLUMN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_USER_SEARCH_COLUMN {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_USER_SEARCH_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_COLUMN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_USER_SEARCH_PREFERENCE(pub i32);
pub const RTCUSP_MAX_MATCHES: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(0i32);
pub const RTCUSP_TIME_LIMIT: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(1i32);
impl ::core::marker::Copy for RTC_USER_SEARCH_PREFERENCE {}
impl ::core::clone::Clone for RTC_USER_SEARCH_PREFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_USER_SEARCH_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_USER_SEARCH_PREFERENCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_USER_SEARCH_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_PREFERENCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_VIDEO_DEVICE(pub i32);
pub const RTCVD_RECEIVE: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(0i32);
pub const RTCVD_PREVIEW: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(1i32);
impl ::core::marker::Copy for RTC_VIDEO_DEVICE {}
impl ::core::clone::Clone for RTC_VIDEO_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_VIDEO_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_VIDEO_DEVICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_VIDEO_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_VIDEO_DEVICE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_WATCHER_EVENT_TYPE(pub i32);
pub const RTCWET_WATCHER_ADD: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(0i32);
pub const RTCWET_WATCHER_REMOVE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(1i32);
pub const RTCWET_WATCHER_UPDATE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(2i32);
pub const RTCWET_WATCHER_OFFERING: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(3i32);
pub const RTCWET_WATCHER_ROAMED: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(4i32);
impl ::core::marker::Copy for RTC_WATCHER_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_WATCHER_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_WATCHER_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_WATCHER_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_WATCHER_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_WATCHER_MATCH_MODE(pub i32);
pub const RTCWMM_EXACT_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(0i32);
pub const RTCWMM_BEST_ACE_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(1i32);
impl ::core::marker::Copy for RTC_WATCHER_MATCH_MODE {}
impl ::core::clone::Clone for RTC_WATCHER_MATCH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_WATCHER_MATCH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_WATCHER_MATCH_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_WATCHER_MATCH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_MATCH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTC_WATCHER_STATE(pub i32);
pub const RTCWS_UNKNOWN: RTC_WATCHER_STATE = RTC_WATCHER_STATE(0i32);
pub const RTCWS_OFFERING: RTC_WATCHER_STATE = RTC_WATCHER_STATE(1i32);
pub const RTCWS_ALLOWED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(2i32);
pub const RTCWS_BLOCKED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(3i32);
pub const RTCWS_DENIED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(4i32);
pub const RTCWS_PROMPT: RTC_WATCHER_STATE = RTC_WATCHER_STATE(5i32);
impl ::core::marker::Copy for RTC_WATCHER_STATE {}
impl ::core::clone::Clone for RTC_WATCHER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_WATCHER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RTC_WATCHER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTC_WATCHER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_STATE").field(&self.0).finish()
    }
}
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "win32-networking")]
pub struct TRANSPORT_SETTING {
    pub SettingId: ::win32_networking::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "win32-networking")]
impl ::core::marker::Copy for TRANSPORT_SETTING {}
#[cfg(feature = "win32-networking")]
impl ::core::clone::Clone for TRANSPORT_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-networking")]
impl ::core::fmt::Debug for TRANSPORT_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_SETTING").field("SettingId", &self.SettingId).field("Length", &self.Length).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "win32-networking")]
unsafe impl ::windows_core::Abi for TRANSPORT_SETTING {
    type Abi = Self;
}
#[cfg(feature = "win32-networking")]
impl ::core::cmp::PartialEq for TRANSPORT_SETTING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORT_SETTING>()) == 0 }
    }
}
#[cfg(feature = "win32-networking")]
impl ::core::cmp::Eq for TRANSPORT_SETTING {}
#[cfg(feature = "win32-networking")]
impl ::core::default::Default for TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
