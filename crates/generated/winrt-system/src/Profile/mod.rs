#[cfg(feature = "SystemManufacturers")]
pub mod SystemManufacturers;
pub struct AnalyticsInfo;
impl AnalyticsInfo {
    pub fn VersionInfo() -> ::windows_core::Result<AnalyticsVersionInfo> {
        Self::IAnalyticsInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AnalyticsVersionInfo>(result__)
        })
    }
    pub fn DeviceForm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAnalyticsInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceForm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSystemPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(attributenames: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        Self::IAnalyticsInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemPropertiesAsync)(::windows_core::Interface::as_raw(this), attributenames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        })
    }
    pub fn IAnalyticsInfoStatics<R, F: FnOnce(&IAnalyticsInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AnalyticsInfo, IAnalyticsInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAnalyticsInfoStatics2<R, F: FnOnce(&IAnalyticsInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AnalyticsInfo, IAnalyticsInfoStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AnalyticsInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsInfo";
}
#[repr(transparent)]
pub struct AnalyticsVersionInfo(::windows_core::IUnknown);
impl AnalyticsVersionInfo {
    pub fn DeviceFamily(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceFamily)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceFamilyVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceFamilyVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ProductName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAnalyticsVersionInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AnalyticsVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnalyticsVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnalyticsVersionInfo {}
impl ::core::fmt::Debug for AnalyticsVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnalyticsVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnalyticsVersionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.AnalyticsVersionInfo;{926130b8-9955-4c74-bdc1-7cd0decf9b03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AnalyticsVersionInfo {
    type Vtable = IAnalyticsVersionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAnalyticsVersionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AnalyticsVersionInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsVersionInfo";
}
impl ::core::convert::From<AnalyticsVersionInfo> for ::windows_core::IUnknown {
    fn from(value: AnalyticsVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnalyticsVersionInfo> for ::windows_core::IUnknown {
    fn from(value: &AnalyticsVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AnalyticsVersionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AnalyticsVersionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnalyticsVersionInfo> for ::windows_core::IInspectable {
    fn from(value: AnalyticsVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnalyticsVersionInfo> for ::windows_core::IInspectable {
    fn from(value: &AnalyticsVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AnalyticsVersionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AnalyticsVersionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AnalyticsVersionInfo {}
unsafe impl ::core::marker::Sync for AnalyticsVersionInfo {}
pub struct AppApplicability;
impl AppApplicability {
    #[cfg(feature = "winrt-foundation")]
    pub fn GetUnsupportedAppRequirements<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(capabilities: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<UnsupportedAppRequirement>> {
        Self::IAppApplicabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUnsupportedAppRequirements)(::windows_core::Interface::as_raw(this), capabilities.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<UnsupportedAppRequirement>>(result__)
        })
    }
    pub fn IAppApplicabilityStatics<R, F: FnOnce(&IAppApplicabilityStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppApplicability, IAppApplicabilityStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AppApplicability {
    const NAME: &'static str = "Windows.System.Profile.AppApplicability";
}
pub struct EducationSettings;
impl EducationSettings {
    pub fn IsEducationEnvironment() -> ::windows_core::Result<bool> {
        Self::IEducationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEducationEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IEducationSettingsStatics<R, F: FnOnce(&IEducationSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EducationSettings, IEducationSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for EducationSettings {
    const NAME: &'static str = "Windows.System.Profile.EducationSettings";
}
pub struct HardwareIdentification;
impl HardwareIdentification {
    #[cfg(feature = "winrt-storage")]
    pub fn GetPackageSpecificToken<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(nonce: Param0) -> ::windows_core::Result<HardwareToken> {
        Self::IHardwareIdentificationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPackageSpecificToken)(::windows_core::Interface::as_raw(this), nonce.into_param().abi(), result__.as_mut_ptr()).from_abi::<HardwareToken>(result__)
        })
    }
    pub fn IHardwareIdentificationStatics<R, F: FnOnce(&IHardwareIdentificationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HardwareIdentification, IHardwareIdentificationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for HardwareIdentification {
    const NAME: &'static str = "Windows.System.Profile.HardwareIdentification";
}
#[repr(transparent)]
pub struct HardwareToken(::windows_core::IUnknown);
impl HardwareToken {
    #[cfg(feature = "winrt-storage")]
    pub fn Id(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Signature(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Signature)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Certificate(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Certificate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for HardwareToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HardwareToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HardwareToken {}
impl ::core::fmt::Debug for HardwareToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HardwareToken").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HardwareToken {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.HardwareToken;{28f6d4c0-fb12-40a4-8167-7f4e03d2724c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HardwareToken {
    type Vtable = IHardwareToken_Vtbl;
    const IID: ::windows_core::GUID = <IHardwareToken as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HardwareToken {
    const NAME: &'static str = "Windows.System.Profile.HardwareToken";
}
impl ::core::convert::From<HardwareToken> for ::windows_core::IUnknown {
    fn from(value: HardwareToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HardwareToken> for ::windows_core::IUnknown {
    fn from(value: &HardwareToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HardwareToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HardwareToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HardwareToken> for ::windows_core::IInspectable {
    fn from(value: HardwareToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HardwareToken> for ::windows_core::IInspectable {
    fn from(value: &HardwareToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HardwareToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HardwareToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HardwareToken {}
unsafe impl ::core::marker::Sync for HardwareToken {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnalyticsInfoStatics {
    type Vtable = IAnalyticsInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d5ee066_188d_5ba9_4387_acaeb0e7e305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnalyticsInfoStatics2 {
    type Vtable = IAnalyticsInfoStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x101704ea_a7f9_46d2_ab94_016865afdb25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetSystemPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSystemPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsVersionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnalyticsVersionInfo {
    type Vtable = IAnalyticsVersionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x926130b8_9955_4c74_bdc1_7cd0decf9b03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceFamilyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsVersionInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnalyticsVersionInfo2 {
    type Vtable = IAnalyticsVersionInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76e915b1_ff36_407c_9f57_160d3e540747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppApplicabilityStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppApplicabilityStatics {
    type Vtable = IAppApplicabilityStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1664a082_0f38_5c99_83e4_48995970861c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppApplicabilityStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetUnsupportedAppRequirements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetUnsupportedAppRequirements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEducationSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEducationSettingsStatics {
    type Vtable = IEducationSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc53f0ef_4d3e_4e13_9b23_505f4d091e92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEducationSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEducationEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareIdentificationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHardwareIdentificationStatics {
    type Vtable = IHardwareIdentificationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x971260e0_f170_4a42_bd55_a900b212dae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareIdentificationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub GetPackageSpecificToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonce: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetPackageSpecificToken: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareToken(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHardwareToken {
    type Vtable = IHardwareToken_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28f6d4c0_fb12_40a4_8167_7f4e03d2724c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareToken_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Id: usize,
    #[cfg(feature = "winrt-storage")]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Signature: usize,
    #[cfg(feature = "winrt-storage")]
    pub Certificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Certificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownRetailInfoPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownRetailInfoPropertiesStatics {
    type Vtable = IKnownRetailInfoPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99571178_500f_487e_8e75_29e551728712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRetailInfoPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RetailAccessCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsFeatured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ScreenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BatteryLifeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProcessorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Memory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StorageDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GraphicsDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FrontCameraDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RearCameraDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasNfc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasSdSlot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasOpticalDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsOfficeInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WindowsEdition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    type Vtable = IPlatformDiagnosticsAndUsageDataSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6e24c1b_7b1c_4b32_8c62_a66597ce723a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CollectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlatformDataCollectionLevel) -> ::windows_core::HRESULT,
    pub CollectionLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCollectionLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CanCollectDiagnostics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: PlatformDataCollectionLevel, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRetailInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRetailInfoStatics {
    type Vtable = IRetailInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0712c6b8_8b92_4f2a_8499_031f1798d6ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRetailInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDemoModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedModeSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedModeSettingsStatics {
    type Vtable = ISharedModeSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x893df40e_cad6_4d50_8c49_6fcfc03edb29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedModeSettingsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedModeSettingsStatics2 {
    type Vtable = ISharedModeSettingsStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x608988a4_ccf1_4ee8_a5e2_fd6a1d0cfac8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldAvoidLocalStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemIdentificationInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemIdentificationInfo {
    type Vtable = ISystemIdentificationInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c659e7d_c3c2_4d33_a2df_21bc41916eb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemIdentificationInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Id: usize,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemIdentificationSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemIdentificationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemIdentificationStatics {
    type Vtable = ISystemIdentificationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5581f42a_d3df_4d93_a37d_c41a616c6d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemIdentificationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetSystemIdForPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSystemIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSetupInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSetupInfoStatics {
    type Vtable = ISystemSetupInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c9620a8_1d88_5e2d_a324_a543af4247ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSetupInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OutOfBoxExperienceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemOutOfBoxExperienceState) -> ::windows_core::HRESULT,
    pub OutOfBoxExperienceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOutOfBoxExperienceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnsupportedAppRequirement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnsupportedAppRequirement {
    type Vtable = IUnsupportedAppRequirement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6182445c_894b_5cbc_8976_a98e0a9b998d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsupportedAppRequirement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Requirement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Reasons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnsupportedAppRequirementReasons) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsIntegrityPolicyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsIntegrityPolicyStatics {
    type Vtable = IWindowsIntegrityPolicyStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d1d81db_8d63_4789_9ea5_ddcf65a94f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsIntegrityPolicyStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsEnabledForTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisableSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
pub struct KnownRetailInfoProperties;
impl KnownRetailInfoProperties {
    pub fn RetailAccessCode() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RetailAccessCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ManufacturerName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ManufacturerName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ModelName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DisplayModelName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayModelName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Price() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Price)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IsFeatured() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IsFeatured)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FormFactor() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ScreenSize() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Weight() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Weight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DisplayDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BatteryLifeDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BatteryLifeDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ProcessorDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessorDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Memory() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Memory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn StorageDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StorageDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GraphicsDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GraphicsDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FrontCameraDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FrontCameraDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RearCameraDescription() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RearCameraDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HasNfc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HasNfc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HasSdSlot() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HasSdSlot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HasOpticalDrive() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HasOpticalDrive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IsOfficeInstalled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IsOfficeInstalled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn WindowsEdition() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WindowsEdition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKnownRetailInfoPropertiesStatics<R, F: FnOnce(&IKnownRetailInfoPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownRetailInfoProperties, IKnownRetailInfoPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownRetailInfoProperties {
    const NAME: &'static str = "Windows.System.Profile.KnownRetailInfoProperties";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enhanced: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for PlatformDataCollectionLevel {}
impl ::core::clone::Clone for PlatformDataCollectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDataCollectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlatformDataCollectionLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDataCollectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDataCollectionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlatformDataCollectionLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.PlatformDataCollectionLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct PlatformDiagnosticsAndUsageDataSettings;
impl PlatformDiagnosticsAndUsageDataSettings {
    pub fn CollectionLevel() -> ::windows_core::Result<PlatformDataCollectionLevel> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlatformDataCollectionLevel>::zeroed();
            (::windows_core::Interface::vtable(this).CollectionLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlatformDataCollectionLevel>(result__)
        })
    }
    pub fn CollectionLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CollectionLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveCollectionLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCollectionLevelChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn CanCollectDiagnostics(level: PlatformDataCollectionLevel) -> ::windows_core::Result<bool> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCollectDiagnostics)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IPlatformDiagnosticsAndUsageDataSettingsStatics<R, F: FnOnce(&IPlatformDiagnosticsAndUsageDataSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlatformDiagnosticsAndUsageDataSettings, IPlatformDiagnosticsAndUsageDataSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PlatformDiagnosticsAndUsageDataSettings {
    const NAME: &'static str = "Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings";
}
pub struct RetailInfo;
impl RetailInfo {
    pub fn IsDemoModeEnabled() -> ::windows_core::Result<bool> {
        Self::IRetailInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDemoModeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties() -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        Self::IRetailInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        })
    }
    pub fn IRetailInfoStatics<R, F: FnOnce(&IRetailInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RetailInfo, IRetailInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for RetailInfo {
    const NAME: &'static str = "Windows.System.Profile.RetailInfo";
}
pub struct SharedModeSettings;
impl SharedModeSettings {
    pub fn IsEnabled() -> ::windows_core::Result<bool> {
        Self::ISharedModeSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ShouldAvoidLocalStorage() -> ::windows_core::Result<bool> {
        Self::ISharedModeSettingsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldAvoidLocalStorage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ISharedModeSettingsStatics<R, F: FnOnce(&ISharedModeSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SharedModeSettings, ISharedModeSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISharedModeSettingsStatics2<R, F: FnOnce(&ISharedModeSettingsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SharedModeSettings, ISharedModeSettingsStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SharedModeSettings {
    const NAME: &'static str = "Windows.System.Profile.SharedModeSettings";
}
pub struct SystemIdentification;
impl SystemIdentification {
    pub fn GetSystemIdForPublisher() -> ::windows_core::Result<SystemIdentificationInfo> {
        Self::ISystemIdentificationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemIdForPublisher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemIdentificationInfo>(result__)
        })
    }
    pub fn GetSystemIdForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<SystemIdentificationInfo> {
        Self::ISystemIdentificationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<SystemIdentificationInfo>(result__)
        })
    }
    pub fn ISystemIdentificationStatics<R, F: FnOnce(&ISystemIdentificationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemIdentification, ISystemIdentificationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SystemIdentification {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentification";
}
#[repr(transparent)]
pub struct SystemIdentificationInfo(::windows_core::IUnknown);
impl SystemIdentificationInfo {
    #[cfg(feature = "winrt-storage")]
    pub fn Id(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<SystemIdentificationSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemIdentificationSource>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemIdentificationSource>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemIdentificationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemIdentificationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemIdentificationInfo {}
impl ::core::fmt::Debug for SystemIdentificationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemIdentificationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemIdentificationInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemIdentificationInfo;{0c659e7d-c3c2-4d33-a2df-21bc41916eb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemIdentificationInfo {
    type Vtable = ISystemIdentificationInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISystemIdentificationInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemIdentificationInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentificationInfo";
}
impl ::core::convert::From<SystemIdentificationInfo> for ::windows_core::IUnknown {
    fn from(value: SystemIdentificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemIdentificationInfo> for ::windows_core::IUnknown {
    fn from(value: &SystemIdentificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemIdentificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemIdentificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemIdentificationInfo> for ::windows_core::IInspectable {
    fn from(value: SystemIdentificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemIdentificationInfo> for ::windows_core::IInspectable {
    fn from(value: &SystemIdentificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemIdentificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemIdentificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemIdentificationInfo {}
unsafe impl ::core::marker::Sync for SystemIdentificationInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: Self = Self(0i32);
    pub const Tpm: Self = Self(1i32);
    pub const Uefi: Self = Self(2i32);
    pub const Registry: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemIdentificationSource {}
impl ::core::clone::Clone for SystemIdentificationSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemIdentificationSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemIdentificationSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemIdentificationSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemIdentificationSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemIdentificationSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemIdentificationSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemOutOfBoxExperienceState {}
impl ::core::clone::Clone for SystemOutOfBoxExperienceState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemOutOfBoxExperienceState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemOutOfBoxExperienceState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemOutOfBoxExperienceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemOutOfBoxExperienceState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemOutOfBoxExperienceState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemOutOfBoxExperienceState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct SystemSetupInfo;
impl SystemSetupInfo {
    pub fn OutOfBoxExperienceState() -> ::windows_core::Result<SystemOutOfBoxExperienceState> {
        Self::ISystemSetupInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemOutOfBoxExperienceState>::zeroed();
            (::windows_core::Interface::vtable(this).OutOfBoxExperienceState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemOutOfBoxExperienceState>(result__)
        })
    }
    pub fn OutOfBoxExperienceStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ISystemSetupInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OutOfBoxExperienceStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveOutOfBoxExperienceStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ISystemSetupInfoStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveOutOfBoxExperienceStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ISystemSetupInfoStatics<R, F: FnOnce(&ISystemSetupInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemSetupInfo, ISystemSetupInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SystemSetupInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemSetupInfo";
}
#[repr(transparent)]
pub struct UnsupportedAppRequirement(::windows_core::IUnknown);
impl UnsupportedAppRequirement {
    pub fn Requirement(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Requirement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Reasons(&self) -> ::windows_core::Result<UnsupportedAppRequirementReasons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnsupportedAppRequirementReasons>::zeroed();
            (::windows_core::Interface::vtable(this).Reasons)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnsupportedAppRequirementReasons>(result__)
        }
    }
}
impl ::core::clone::Clone for UnsupportedAppRequirement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnsupportedAppRequirement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnsupportedAppRequirement {}
impl ::core::fmt::Debug for UnsupportedAppRequirement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnsupportedAppRequirement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnsupportedAppRequirement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.UnsupportedAppRequirement;{6182445c-894b-5cbc-8976-a98e0a9b998d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UnsupportedAppRequirement {
    type Vtable = IUnsupportedAppRequirement_Vtbl;
    const IID: ::windows_core::GUID = <IUnsupportedAppRequirement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UnsupportedAppRequirement {
    const NAME: &'static str = "Windows.System.Profile.UnsupportedAppRequirement";
}
impl ::core::convert::From<UnsupportedAppRequirement> for ::windows_core::IUnknown {
    fn from(value: UnsupportedAppRequirement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnsupportedAppRequirement> for ::windows_core::IUnknown {
    fn from(value: &UnsupportedAppRequirement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UnsupportedAppRequirement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UnsupportedAppRequirement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnsupportedAppRequirement> for ::windows_core::IInspectable {
    fn from(value: UnsupportedAppRequirement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnsupportedAppRequirement> for ::windows_core::IInspectable {
    fn from(value: &UnsupportedAppRequirement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UnsupportedAppRequirement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UnsupportedAppRequirement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnsupportedAppRequirement {}
unsafe impl ::core::marker::Sync for UnsupportedAppRequirement {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: Self = Self(0u32);
    pub const DeniedBySystem: Self = Self(1u32);
}
impl ::core::marker::Copy for UnsupportedAppRequirementReasons {}
impl ::core::clone::Clone for UnsupportedAppRequirementReasons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnsupportedAppRequirementReasons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UnsupportedAppRequirementReasons {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnsupportedAppRequirementReasons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnsupportedAppRequirementReasons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UnsupportedAppRequirementReasons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UnsupportedAppRequirementReasons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for UnsupportedAppRequirementReasons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.UnsupportedAppRequirementReasons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct WindowsIntegrityPolicy;
impl WindowsIntegrityPolicy {
    pub fn IsEnabled() -> ::windows_core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsEnabledForTrial() -> ::windows_core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledForTrial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CanDisable() -> ::windows_core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanDisable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsDisableSupported() -> ::windows_core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisableSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn PolicyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PolicyChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemovePolicyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePolicyChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IWindowsIntegrityPolicyStatics<R, F: FnOnce(&IWindowsIntegrityPolicyStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WindowsIntegrityPolicy, IWindowsIntegrityPolicyStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for WindowsIntegrityPolicy {
    const NAME: &'static str = "Windows.System.Profile.WindowsIntegrityPolicy";
}
