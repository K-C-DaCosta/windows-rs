pub const ADDRESS_FAMILY_VALUE_NAME: &str = "AddressFamily";
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
pub const FAULT_INVALID_ACTION: u32 = 401u32;
pub const FAULT_INVALID_ARG: u32 = 402u32;
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HSWDEVICE(pub isize);
impl HSWDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSWDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSWDEVICE {}
impl ::core::fmt::Debug for HSWDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSWDEVICE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HSWDEVICE {
    type Abi = Self;
}
#[repr(transparent)]
pub struct IUPnPAddressFamilyControl(::windows_core::IUnknown);
impl IUPnPAddressFamilyControl {
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressFamily)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetAddressFamily(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAddressFamily)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IUPnPAddressFamilyControl> for ::windows_core::IUnknown {
    fn from(value: IUPnPAddressFamilyControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPAddressFamilyControl> for ::windows_core::IUnknown {
    fn from(value: &IUPnPAddressFamilyControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPAddressFamilyControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPAddressFamilyControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPAddressFamilyControl {}
impl ::core::fmt::Debug for IUPnPAddressFamilyControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPAddressFamilyControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPAddressFamilyControl {
    type Vtable = IUPnPAddressFamilyControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3bf6178_694e_459f_a5a6_191ea0ffa1c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAddressFamilyControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows_core::HRESULT,
    pub GetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPAsyncResult(::windows_core::IUnknown);
impl IUPnPAsyncResult {
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncOperationComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullrequestid)).ok()
    }
}
impl ::core::convert::From<IUPnPAsyncResult> for ::windows_core::IUnknown {
    fn from(value: IUPnPAsyncResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPAsyncResult> for ::windows_core::IUnknown {
    fn from(value: &IUPnPAsyncResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPAsyncResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPAsyncResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPAsyncResult {}
impl ::core::fmt::Debug for IUPnPAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPAsyncResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPAsyncResult {
    type Vtable = IUPnPAsyncResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d65fd08_d13e_4274_9c8b_dd8d028c8644);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAsyncResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPDescriptionDocument(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPDescriptionDocument {
    pub unsafe fn ReadyState(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ReadyState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn LoadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, bstrurl: Param0, punkcallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadAsync)(::windows_core::Interface::as_raw(self), bstrurl.into_param().abi(), punkcallback.into_param().abi()).ok()
    }
    pub unsafe fn LoadResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LoadResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RootDevice(&self) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RootDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevice>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeviceByUDN<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceByUDN)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevice>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDescriptionDocument> for ::windows_core::IUnknown {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDescriptionDocument> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDescriptionDocument> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDescriptionDocument> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPDescriptionDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPDescriptionDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPDescriptionDocument {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPDescriptionDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDescriptionDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPDescriptionDocument {
    type Vtable = IUPnPDescriptionDocument_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11d1c1b2_7daa_4c9e_9595_7f82ed206d1e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocument_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub ReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub RootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RootDevice: usize,
    #[cfg(feature = "win32-system")]
    pub DeviceByUDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppuddevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeviceByUDN: usize,
}
#[repr(transparent)]
pub struct IUPnPDescriptionDocumentCallback(::windows_core::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    pub unsafe fn LoadComplete(&self, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrloadresult)).ok()
    }
}
impl ::core::convert::From<IUPnPDescriptionDocumentCallback> for ::windows_core::IUnknown {
    fn from(value: IUPnPDescriptionDocumentCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDescriptionDocumentCallback> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDescriptionDocumentCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDescriptionDocumentCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDescriptionDocumentCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDescriptionDocumentCallback {}
impl ::core::fmt::Debug for IUPnPDescriptionDocumentCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDescriptionDocumentCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDescriptionDocumentCallback {
    type Vtable = IUPnPDescriptionDocumentCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77394c69_5486_40d6_9bc3_4991983e02da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LoadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrloadresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPDevice(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPDevice {
    pub unsafe fn IsRootDevice(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsRootDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RootDevice(&self) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RootDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevice>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ParentDevice(&self) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ParentDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevice>(result__)
    }
    pub unsafe fn HasChildren(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).HasChildren)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Children(&self) -> ::windows_core::Result<IUPnPDevices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Children)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevices>(result__)
    }
    pub unsafe fn UniqueDeviceName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UniqueDeviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FriendlyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn PresentationURL(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PresentationURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ManufacturerName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ManufacturerName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ManufacturerURL(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ManufacturerURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ModelName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ModelName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ModelNumber(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ModelNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ModelURL(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ModelURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UPC(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UPC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SerialNumber(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SerialNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IconURL<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrencodingformat: Param0, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).IconURL)(::windows_core::Interface::as_raw(self), bstrencodingformat.into_param().abi(), ::core::mem::transmute(lsizex), ::core::mem::transmute(lsizey), ::core::mem::transmute(lbitdepth), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Services(&self) -> ::windows_core::Result<IUPnPServices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Services)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPServices>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDevice> for ::windows_core::IUnknown {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDevice> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDevice> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDevice> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPDevice {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPDevice {
    type Vtable = IUPnPDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d44d0d1_98c9_4889_acd1_f9d674bf2221);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevice_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub IsRootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub RootDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RootDevice: usize,
    #[cfg(feature = "win32-system")]
    pub ParentDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ParentDevice: usize,
    pub HasChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudchildren: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Children: usize,
    pub UniqueDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub PresentationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ManufacturerURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ModelURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UPC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IconURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrencodingformat: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppusservices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Services: usize,
}
#[repr(transparent)]
pub struct IUPnPDeviceControl(::windows_core::IUnknown);
impl IUPnPDeviceControl {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrxmldesc: Param0, bstrdeviceidentifier: Param1, bstrinitstring: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), bstrxmldesc.into_param().abi(), bstrdeviceidentifier.into_param().abi(), bstrinitstring.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetServiceObject<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrudn: Param0, bstrserviceid: Param1) -> ::windows_core::Result<::win32_system::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceObject)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), bstrserviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IDispatch>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceControl> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceControl> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceControl {}
impl ::core::fmt::Debug for IUPnPDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceControl {
    type Vtable = IUPnPDeviceControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810ba_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetServiceObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdispservice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetServiceObject: usize,
}
#[repr(transparent)]
pub struct IUPnPDeviceControlHttpHeaders(::windows_core::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAdditionalResponseHeaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceControlHttpHeaders> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceControlHttpHeaders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceControlHttpHeaders> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceControlHttpHeaders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceControlHttpHeaders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceControlHttpHeaders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceControlHttpHeaders {}
impl ::core::fmt::Debug for IUPnPDeviceControlHttpHeaders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceControlHttpHeaders").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceControlHttpHeaders {
    type Vtable = IUPnPDeviceControlHttpHeaders_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810bb_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeaders_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAdditionalResponseHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccess(::windows_core::IUnknown);
impl IUPnPDeviceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceDocumentAccess> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceDocumentAccess> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceDocumentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceDocumentAccess {}
impl ::core::fmt::Debug for IUPnPDeviceDocumentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceDocumentAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceDocumentAccess {
    type Vtable = IUPnPDeviceDocumentAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7772804_3287_418e_9072_cf2b47238981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccessEx(::windows_core::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    pub unsafe fn GetDocument(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDocument)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceDocumentAccessEx> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccessEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceDocumentAccessEx> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccessEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceDocumentAccessEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceDocumentAccessEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceDocumentAccessEx {}
impl ::core::fmt::Debug for IUPnPDeviceDocumentAccessEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceDocumentAccessEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceDocumentAccessEx {
    type Vtable = IUPnPDeviceDocumentAccessEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4bc4050_6178_4bd1_a4b8_6398321f3247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPDeviceFinder(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPDeviceFinder {
    #[cfg(feature = "win32-system")]
    pub unsafe fn FindByType<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtypeuri: Param0, dwflags: u32) -> ::windows_core::Result<IUPnPDevices> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindByType)(::windows_core::Interface::as_raw(self), bstrtypeuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevices>(result__)
    }
    pub unsafe fn CreateAsyncFind<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, bstrtypeuri: Param0, dwflags: u32, punkdevicefindercallback: Param2) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAsyncFind)(::windows_core::Interface::as_raw(self), bstrtypeuri.into_param().abi(), ::core::mem::transmute(dwflags), punkdevicefindercallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartAsyncFind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfinddata)).ok()
    }
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncFind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfinddata)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn FindByUDN<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindByUDN)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevice>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDeviceFinder> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDeviceFinder> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceFinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDeviceFinder> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDeviceFinder> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPDeviceFinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPDeviceFinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPDeviceFinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPDeviceFinder {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPDeviceFinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinder").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPDeviceFinder {
    type Vtable = IUPnPDeviceFinder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadda3d55_6f72_4319_bff9_18600a539b10);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinder_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub FindByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    FindByType: usize,
    pub CreateAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows_core::HRESULT,
    pub StartAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT,
    pub CancelAsyncFind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub FindByUDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    FindByUDN: usize,
}
#[repr(transparent)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(::windows_core::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeviceAddedWithInterface<'a, Param1: ::windows_core::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceAddedWithInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfinddata), pdevice.into_param().abi(), ::core::mem::transmute(pguidinterface)).ok()
    }
}
impl ::core::convert::From<IUPnPDeviceFinderAddCallbackWithInterface> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceFinderAddCallbackWithInterface> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceFinderAddCallbackWithInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinderAddCallbackWithInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinderAddCallbackWithInterface {}
impl ::core::fmt::Debug for IUPnPDeviceFinderAddCallbackWithInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinderAddCallbackWithInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    type Vtable = IUPnPDeviceFinderAddCallbackWithInterface_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x983dfc0b_1796_44df_8975_ca545b620ee5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub DeviceAddedWithInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows_core::RawPtr, pguidinterface: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeviceAddedWithInterface: usize,
}
#[repr(transparent)]
pub struct IUPnPDeviceFinderCallback(::windows_core::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeviceAdded<'a, Param1: ::windows_core::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceAdded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfinddata), pdevice.into_param().abi()).ok()
    }
    pub unsafe fn DeviceRemoved<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lfinddata: i32, bstrudn: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceRemoved)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfinddata), bstrudn.into_param().abi()).ok()
    }
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfinddata)).ok()
    }
}
impl ::core::convert::From<IUPnPDeviceFinderCallback> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceFinderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceFinderCallback> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceFinderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceFinderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinderCallback {}
impl ::core::fmt::Debug for IUPnPDeviceFinderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinderCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceFinderCallback {
    type Vtable = IUPnPDeviceFinderCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x415a984a_88b3_49f3_92af_0508bedf0d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub DeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeviceAdded: usize,
    pub DeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPDeviceProvider(::windows_core::IUnknown);
impl IUPnPDeviceProvider {
    pub unsafe fn Start<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinitstring: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), bstrinitstring.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IUPnPDeviceProvider> for ::windows_core::IUnknown {
    fn from(value: IUPnPDeviceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceProvider> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDeviceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceProvider {}
impl ::core::fmt::Debug for IUPnPDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPDeviceProvider {
    type Vtable = IUPnPDeviceProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b8_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPDevices(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPDevices {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows_core::Result<IUPnPDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), bstrudn.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPDevice>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDevices> for ::windows_core::IUnknown {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDevices> for ::windows_core::IUnknown {
    fn from(value: &IUPnPDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPDevices> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPDevices> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPDevices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPDevices {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDevices").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPDevices {
    type Vtable = IUPnPDevices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdbc0c73_bda3_4c66_ac4f_f2d96fdad68c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevices_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
}
#[repr(transparent)]
pub struct IUPnPEventSink(::windows_core::IUnknown);
impl IUPnPEventSink {
    pub unsafe fn OnStateChanged(&self, rgdispidchanges: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStateChanged)(::windows_core::Interface::as_raw(self), rgdispidchanges.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgdispidchanges))).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn OnStateChangedSafe<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varsadispidchanges: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStateChangedSafe)(::windows_core::Interface::as_raw(self), varsadispidchanges.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPEventSink> for ::windows_core::IUnknown {
    fn from(value: IUPnPEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPEventSink> for ::windows_core::IUnknown {
    fn from(value: &IUPnPEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPEventSink {}
impl ::core::fmt::Debug for IUPnPEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPEventSink {
    type Vtable = IUPnPEventSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b4_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub OnStateChangedSafe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsadispidchanges: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    OnStateChangedSafe: usize,
}
#[repr(transparent)]
pub struct IUPnPEventSource(::windows_core::IUnknown);
impl IUPnPEventSource {
    pub unsafe fn Advise<'a, Param0: ::windows_core::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), pessubscriber.into_param().abi()).ok()
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows_core::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), pessubscriber.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPEventSource> for ::windows_core::IUnknown {
    fn from(value: IUPnPEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPEventSource> for ::windows_core::IUnknown {
    fn from(value: &IUPnPEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPEventSource {}
impl ::core::fmt::Debug for IUPnPEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPEventSource {
    type Vtable = IUPnPEventSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b5_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSource_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPHttpHeaderControl(::windows_core::IUnknown);
impl IUPnPHttpHeaderControl {
    pub unsafe fn AddRequestHeaders<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrhttpheaders: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRequestHeaders)(::windows_core::Interface::as_raw(self), bstrhttpheaders.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPHttpHeaderControl> for ::windows_core::IUnknown {
    fn from(value: IUPnPHttpHeaderControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPHttpHeaderControl> for ::windows_core::IUnknown {
    fn from(value: &IUPnPHttpHeaderControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPHttpHeaderControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPHttpHeaderControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPHttpHeaderControl {}
impl ::core::fmt::Debug for IUPnPHttpHeaderControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPHttpHeaderControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPHttpHeaderControl {
    type Vtable = IUPnPHttpHeaderControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0405af4f_8b5c_447c_80f2_b75984a31f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddRequestHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPRegistrar(::windows_core::IUnknown);
impl IUPnPRegistrar {
    pub unsafe fn RegisterDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrxmldesc: Param0, bstrprogiddevicecontrolclass: Param1, bstrinitstring: Param2, bstrcontainerid: Param3, bstrresourcepath: Param4, nlifetime: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterDevice)(::windows_core::Interface::as_raw(self), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RegisterRunningDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrxmldesc: Param0, punkdevicecontrol: Param1, bstrinitstring: Param2, bstrresourcepath: Param3, nlifetime: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterRunningDevice)(::windows_core::Interface::as_raw(self), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprovidername: Param0, bstrprogidproviderclass: Param1, bstrinitstring: Param2, bstrcontainerid: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstrprovidername.into_param().abi(), bstrprogidproviderclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi()).ok()
    }
    pub unsafe fn GetUniqueDeviceName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrtemplateudn: Param1) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetUniqueDeviceName)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), bstrtemplateudn.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UnregisterDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrdeviceidentifier: Param0, fpermanent: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterDevice)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), fpermanent.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprovidername: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterDeviceProvider)(::windows_core::Interface::as_raw(self), bstrprovidername.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPRegistrar> for ::windows_core::IUnknown {
    fn from(value: IUPnPRegistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPRegistrar> for ::windows_core::IUnknown {
    fn from(value: &IUPnPRegistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPRegistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPRegistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPRegistrar {}
impl ::core::fmt::Debug for IUPnPRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPRegistrar {
    type Vtable = IUPnPRegistrar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b6_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRegistrar_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RegisterRunningDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetUniqueDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrudn: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fpermanent: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPRemoteEndpointInfo(::windows_core::IUnknown);
impl IUPnPRemoteEndpointInfo {
    pub unsafe fn GetDwordValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDwordValue)(::windows_core::Interface::as_raw(self), bstrvaluename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStringValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetStringValue)(::windows_core::Interface::as_raw(self), bstrvaluename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetGuidValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetGuidValue)(::windows_core::Interface::as_raw(self), bstrvaluename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IUPnPRemoteEndpointInfo> for ::windows_core::IUnknown {
    fn from(value: IUPnPRemoteEndpointInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPRemoteEndpointInfo> for ::windows_core::IUnknown {
    fn from(value: &IUPnPRemoteEndpointInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPRemoteEndpointInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPRemoteEndpointInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPRemoteEndpointInfo {}
impl ::core::fmt::Debug for IUPnPRemoteEndpointInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPRemoteEndpointInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPRemoteEndpointInfo {
    type Vtable = IUPnPRemoteEndpointInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc92eb863_0269_4aff_9c72_75321bba2952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDwordValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pdwvalue: *mut u32) -> ::windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pguidvalue: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPReregistrar(::windows_core::IUnknown);
impl IUPnPReregistrar {
    pub unsafe fn ReregisterDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrxmldesc: Param1, bstrprogiddevicecontrolclass: Param2, bstrinitstring: Param3, bstrcontainerid: Param4, bstrresourcepath: Param5, nlifetime: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReregisterDevice)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime)).ok()
    }
    pub unsafe fn ReregisterRunningDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrxmldesc: Param1, punkdevicecontrol: Param2, bstrinitstring: Param3, bstrresourcepath: Param4, nlifetime: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReregisterRunningDevice)(::windows_core::Interface::as_raw(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime)).ok()
    }
}
impl ::core::convert::From<IUPnPReregistrar> for ::windows_core::IUnknown {
    fn from(value: IUPnPReregistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPReregistrar> for ::windows_core::IUnknown {
    fn from(value: &IUPnPReregistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPReregistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPReregistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPReregistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPReregistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPReregistrar {}
impl ::core::fmt::Debug for IUPnPReregistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPReregistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPReregistrar {
    type Vtable = IUPnPReregistrar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b7_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPReregistrar_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ReregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT,
    pub ReregisterRunningDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, nlifetime: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPService(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPService {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QueryStateVariable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrvariablename: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).QueryStateVariable)(::windows_core::Interface::as_raw(self), bstrvariablename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn InvokeAction<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstractionname: Param0, vinactionargs: Param1, pvoutactionargs: *mut ::win32_system::Com::VARIANT, pvretval: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeAction)(::windows_core::Interface::as_raw(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), ::core::mem::transmute(pvoutactionargs), ::core::mem::transmute(pvretval)).ok()
    }
    pub unsafe fn ServiceTypeIdentifier(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceTypeIdentifier)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn AddCallback<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkcallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddCallback)(::windows_core::Interface::as_raw(self), punkcallback.into_param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn LastTransportStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LastTransportStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPService> for ::windows_core::IUnknown {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPService> for ::windows_core::IUnknown {
    fn from(value: &IUPnPService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPService> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPService> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPService {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPService {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPService {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPService").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPService {
    type Vtable = IUPnPService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa295019c_dc65_47dd_90dc_7fe918a1ab44);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPService_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub QueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    QueryStateVariable: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub InvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvoutactionargs: *mut ::win32_system::Com::VARIANT, pvretval: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    InvokeAction: usize,
    pub ServiceTypeIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub AddCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub LastTransportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPServiceAsync(::windows_core::IUnknown);
impl IUPnPServiceAsync {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BeginInvokeAction<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, IUPnPAsyncResult>>(&self, bstractionname: Param0, vinactionargs: Param1, pasyncresult: Param2) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).BeginInvokeAction)(::windows_core::Interface::as_raw(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), pasyncresult.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut ::win32_system::Com::VARIANT, pvretval: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndInvokeAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(pvoutactionargs), ::core::mem::transmute(pvretval)).ok()
    }
    pub unsafe fn BeginQueryStateVariable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, IUPnPAsyncResult>>(&self, bstrvariablename: Param0, pasyncresult: Param1) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).BeginQueryStateVariable)(::windows_core::Interface::as_raw(self), bstrvariablename.into_param().abi(), pasyncresult.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndQueryStateVariable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn BeginSubscribeToEvents<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, IUPnPAsyncResult>>(&self, punkcallback: Param0, pasyncresult: Param1) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).BeginSubscribeToEvents)(::windows_core::Interface::as_raw(self), punkcallback.into_param().abi(), pasyncresult.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSubscribeToEvents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullrequestid)).ok()
    }
    pub unsafe fn BeginSCPDDownload<'a, Param0: ::windows_core::IntoParam<'a, IUPnPAsyncResult>>(&self, pasyncresult: Param0) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).BeginSCPDDownload)(::windows_core::Interface::as_raw(self), pasyncresult.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EndSCPDDownload)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAsyncOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullrequestid)).ok()
    }
}
impl ::core::convert::From<IUPnPServiceAsync> for ::windows_core::IUnknown {
    fn from(value: IUPnPServiceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceAsync> for ::windows_core::IUnknown {
    fn from(value: &IUPnPServiceAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPServiceAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPServiceAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceAsync {}
impl ::core::fmt::Debug for IUPnPServiceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPServiceAsync {
    type Vtable = IUPnPServiceAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x098bdaf5_5ec1_49e7_a260_b3a11dd8680c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BeginInvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pasyncresult: ::windows_core::RawPtr, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BeginInvokeAction: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EndInvokeAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut ::win32_system::Com::VARIANT, pvretval: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EndInvokeAction: usize,
    pub BeginQueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pasyncresult: ::windows_core::RawPtr, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EndQueryStateVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EndQueryStateVariable: usize,
    pub BeginSubscribeToEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: ::windows_core::RawPtr, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    pub EndSubscribeToEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT,
    pub BeginSCPDDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: ::windows_core::RawPtr, pullrequestid: *mut u64) -> ::windows_core::HRESULT,
    pub EndSCPDDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub CancelAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPServiceCallback(::windows_core::IUnknown);
impl IUPnPServiceCallback {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn StateVariableChanged<'a, Param0: ::windows_core::IntoParam<'a, IUPnPService>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pus: Param0, pcwszstatevarname: Param1, vavalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StateVariableChanged)(::windows_core::Interface::as_raw(self), pus.into_param().abi(), pcwszstatevarname.into_param().abi(), vavalue.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ServiceInstanceDied<'a, Param0: ::windows_core::IntoParam<'a, IUPnPService>>(&self, pus: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ServiceInstanceDied)(::windows_core::Interface::as_raw(self), pus.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPServiceCallback> for ::windows_core::IUnknown {
    fn from(value: IUPnPServiceCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceCallback> for ::windows_core::IUnknown {
    fn from(value: &IUPnPServiceCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPServiceCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPServiceCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceCallback {}
impl ::core::fmt::Debug for IUPnPServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPServiceCallback {
    type Vtable = IUPnPServiceCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31fadca9_ab73_464b_b67d_5c1d0f83c8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub StateVariableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: ::windows_core::RawPtr, pcwszstatevarname: ::windows_core::PCWSTR, vavalue: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    StateVariableChanged: usize,
    #[cfg(feature = "win32-system")]
    pub ServiceInstanceDied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ServiceInstanceDied: usize,
}
#[repr(transparent)]
pub struct IUPnPServiceDocumentAccess(::windows_core::IUnknown);
impl IUPnPServiceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDocument)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPServiceDocumentAccess> for ::windows_core::IUnknown {
    fn from(value: IUPnPServiceDocumentAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceDocumentAccess> for ::windows_core::IUnknown {
    fn from(value: &IUPnPServiceDocumentAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceDocumentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceDocumentAccess {}
impl ::core::fmt::Debug for IUPnPServiceDocumentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceDocumentAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPServiceDocumentAccess {
    type Vtable = IUPnPServiceDocumentAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21905529_0a5e_4589_825d_7e6d87ea6998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdoc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUPnPServiceEnumProperty(::windows_core::IUnknown);
impl IUPnPServiceEnumProperty {
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceEnumProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmask)).ok()
    }
}
impl ::core::convert::From<IUPnPServiceEnumProperty> for ::windows_core::IUnknown {
    fn from(value: IUPnPServiceEnumProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceEnumProperty> for ::windows_core::IUnknown {
    fn from(value: &IUPnPServiceEnumProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceEnumProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceEnumProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceEnumProperty {}
impl ::core::fmt::Debug for IUPnPServiceEnumProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceEnumProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUPnPServiceEnumProperty {
    type Vtable = IUPnPServiceEnumProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38873b37_91bb_49f4_b249_2e8efbb8a816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceEnumProperty_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetServiceEnumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IUPnPServices(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IUPnPServices {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrserviceid: Param0) -> ::windows_core::Result<IUPnPService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), bstrserviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUPnPService>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPServices> for ::windows_core::IUnknown {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPServices> for ::windows_core::IUnknown {
    fn from(value: &IUPnPServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUPnPServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUPnPServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IUPnPServices> for ::win32_system::Com::IDispatch {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IUPnPServices> for ::win32_system::Com::IDispatch {
    fn from(value: &IUPnPServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IUPnPServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IUPnPServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IUPnPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IUPnPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IUPnPServices {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IUPnPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServices").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IUPnPServices {
    type Vtable = IUPnPServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f8c8e9e_9a7a_4dc8_bc41_ff31fa374956);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServices_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppservice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    get_Item: usize,
}
pub const REMOTE_ADDRESS_VALUE_NAME: &str = "RemoteAddress";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
impl ::core::marker::Copy for SW_DEVICE_CAPABILITIES {}
impl ::core::clone::Clone for SW_DEVICE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SW_DEVICE_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SW_DEVICE_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SW_DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SW_DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
pub type SW_DEVICE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows_core::HRESULT, pcontext: *const ::core::ffi::c_void, pszdeviceinstanceid: ::windows_core::PCWSTR)>;
#[repr(C)]
#[cfg(feature = "win32-security")]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: ::windows_core::PCWSTR,
    pub pszzHardwareIds: ::windows_core::PCWSTR,
    pub pszzCompatibleIds: ::windows_core::PCWSTR,
    pub pContainerId: *const ::windows_core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: ::windows_core::PCWSTR,
    pub pszDeviceLocation: ::windows_core::PCWSTR,
    pub pSecurityDescriptor: *const ::win32_security::SECURITY_DESCRIPTOR,
}
#[cfg(feature = "win32-security")]
impl ::core::marker::Copy for SW_DEVICE_CREATE_INFO {}
#[cfg(feature = "win32-security")]
impl ::core::clone::Clone for SW_DEVICE_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-security")]
impl ::core::fmt::Debug for SW_DEVICE_CREATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SW_DEVICE_CREATE_INFO").field("cbSize", &self.cbSize).field("pszInstanceId", &self.pszInstanceId).field("pszzHardwareIds", &self.pszzHardwareIds).field("pszzCompatibleIds", &self.pszzCompatibleIds).field("pContainerId", &self.pContainerId).field("CapabilityFlags", &self.CapabilityFlags).field("pszDeviceDescription", &self.pszDeviceDescription).field("pszDeviceLocation", &self.pszDeviceLocation).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(feature = "win32-security")]
unsafe impl ::windows_core::Abi for SW_DEVICE_CREATE_INFO {
    type Abi = Self;
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::PartialEq for SW_DEVICE_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SW_DEVICE_CREATE_INFO>()) == 0 }
    }
}
#[cfg(feature = "win32-security")]
impl ::core::cmp::Eq for SW_DEVICE_CREATE_INFO {}
#[cfg(feature = "win32-security")]
impl ::core::default::Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SW_DEVICE_LIFETIME(pub i32);
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
impl ::core::marker::Copy for SW_DEVICE_LIFETIME {}
impl ::core::clone::Clone for SW_DEVICE_LIFETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SW_DEVICE_LIFETIME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SW_DEVICE_LIFETIME {
    type Abi = Self;
}
impl ::core::fmt::Debug for SW_DEVICE_LIFETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SW_DEVICE_LIFETIME").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn SwDeviceClose<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceClose(hswdevice: HSWDEVICE);
        }
        SwDeviceClose(hswdevice.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-devices", feature = "win32-security"))]
#[inline]
pub unsafe fn SwDeviceCreate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszenumeratorname: Param0, pszparentdeviceinstance: Param1, pcreateinfo: *const SW_DEVICE_CREATE_INFO, pproperties: &[super::super::Properties::DEVPROPERTY], pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceCreate(pszenumeratorname: ::windows_core::PCWSTR, pszparentdeviceinstance: ::windows_core::PCWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phswdevice: *mut isize) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        SwDeviceCreate(pszenumeratorname.into_param().abi(), pszparentdeviceinstance.into_param().abi(), ::core::mem::transmute(pcreateinfo), pproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pproperties)), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwDeviceGetLifetime<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) -> ::windows_core::Result<SW_DEVICE_LIFETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<SW_DEVICE_LIFETIME>::zeroed();
        SwDeviceGetLifetime(hswdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SW_DEVICE_LIFETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, pproperties: &[super::super::Properties::DEVPROPERTY]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: ::windows_core::PCWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows_core::HRESULT;
        }
        SwDeviceInterfacePropertySet(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), pproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pproperties))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hswdevice: Param0, pinterfaceclassguid: *const ::windows_core::GUID, pszreferencestring: Param2, pproperties: &[super::super::Properties::DEVPROPERTY], fenabled: Param5) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows_core::GUID, pszreferencestring: ::windows_core::PCWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: ::win32_foundation::BOOL, ppszdeviceinterfaceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        SwDeviceInterfaceRegister(hswdevice.into_param().abi(), ::core::mem::transmute(pinterfaceclassguid), pszreferencestring.into_param().abi(), pproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pproperties)), fenabled.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, fenabled: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceSetState(hswdevice: HSWDEVICE, pszdeviceinterfaceid: ::windows_core::PCWSTR, fenabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        SwDeviceInterfaceSetState(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), fenabled.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn SwDevicePropertySet<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, pproperties: &[super::super::Properties::DEVPROPERTY]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows_core::HRESULT;
        }
        SwDevicePropertySet(hswdevice.into_param().abi(), pproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pproperties))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwDeviceSetLifetime<'a, Param0: ::windows_core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, lifetime: SW_DEVICE_LIFETIME) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows_core::HRESULT;
        }
        SwDeviceSetLifetime(hswdevice.into_param().abi(), ::core::mem::transmute(lifetime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SwMemFree(pmem: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwMemFree(pmem: *const ::core::ffi::c_void);
        }
        SwMemFree(::core::mem::transmute(pmem))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220976i32);
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220736i32);
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220991i32);
pub const UPNP_E_DEVICE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220972i32);
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220988i32);
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180494i32);
pub const UPNP_E_DEVICE_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180495i32);
pub const UPNP_E_DEVICE_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220969i32);
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180511i32);
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180510i32);
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220970i32);
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220223i32);
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220987i32);
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220986i32);
pub const UPNP_E_INVALID_ACTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220985i32);
pub const UPNP_E_INVALID_ARGUMENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220984i32);
pub const UPNP_E_INVALID_DESCRIPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180509i32);
pub const UPNP_E_INVALID_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220224i32);
pub const UPNP_E_INVALID_ICON: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180507i32);
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180505i32);
pub const UPNP_E_INVALID_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180508i32);
pub const UPNP_E_INVALID_VARIABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220973i32);
pub const UPNP_E_INVALID_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180506i32);
pub const UPNP_E_OUT_OF_SYNC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220983i32);
pub const UPNP_E_PROTOCOL_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220971i32);
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180512i32);
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220992i32);
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220990i32);
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220989i32);
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180504i32);
pub const UPNP_E_TRANSPORT_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220975i32);
pub const UPNP_E_URLBASE_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180503i32);
pub const UPNP_E_VALUE_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180496i32);
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220974i32);
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
pub const UPnPDescriptionDocument: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d8a9b47_3a28_4ce2_8a4b_bd34e45bceeb);
pub const UPnPDescriptionDocumentEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33fd0563_d81a_4393_83cc_0195b1da2f91);
pub const UPnPDevice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa32552c5_ba61_457a_b59a_a2561e125e33);
pub const UPnPDeviceFinder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2085f28_feb7_404a_b8e7_e659bdeaaa02);
pub const UPnPDeviceFinderEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x181b54fc_380b_4a75_b3f1_4ac45e9605b0);
pub const UPnPDevices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9e84ffd_ad3c_40a4_b835_0882ebcbaaa8);
pub const UPnPRegistrar: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204810b9_73b2_11d4_bf42_00b0d0118b56);
pub const UPnPRemoteEndpointInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e5e84e9_4049_4244_b728_2d24227157c7);
pub const UPnPService: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc624ba95_fbcb_4409_8c03_8cceec533ef1);
pub const UPnPServices: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0bc4b4a_a406_4efc_932f_b8546b8100cc);
