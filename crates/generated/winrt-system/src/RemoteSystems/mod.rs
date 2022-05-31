#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownRemoteSystemCapabilitiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownRemoteSystemCapabilitiesStatics {
    type Vtable = IKnownRemoteSystemCapabilitiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8108e380_7f8a_44e4_92cd_03b6469b94a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRemoteSystemCapabilitiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LaunchUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SpatialEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystem {
    type Vtable = IRemoteSystem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed5838cd_1e10_4a8c_b4a6_4e5fd6f97721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemStatus) -> ::windows_core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystem2 {
    type Vtable = IRemoteSystem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09dfe4ec_fb8b_4a08_a758_6876435d769e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetCapabilitySupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystem3 {
    type Vtable = IRemoteSystem3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72b4b495_b7c6_40be_831b_73562f12ffa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ManufacturerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ModelDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystem4 {
    type Vtable = IRemoteSystem4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf164ffe5_b987_4ca5_9926_fa0438be6273);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Platform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemPlatform) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystem5 {
    type Vtable = IRemoteSystem5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb2ad723_e5e2_4ae2_a7a7_a1097a098e90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Apps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Apps: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystem6 {
    type Vtable = IRemoteSystem6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4cda942_c027_533e_9384_3a19b4f7eef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f39560f_e534_4697_8836_7abea151516e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemApp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemApp {
    type Vtable = IRemoteSystemApp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80e5bcbd_d54d_41b1_9b16_6810a871ed4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Attributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemApp2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemApp2 {
    type Vtable = IRemoteSystemApp2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6369bf15_0a96_577a_8ff6_c35904dfa8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAppRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb47947b5_7035_4a5a_b8df_962d8f8431f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Attributes: usize,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAppRegistrationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemAppRegistrationStatics {
    type Vtable = IRemoteSystemAppRegistrationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01b99840_cfd2_453f_ae25_c2539f086afd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistrationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAuthorizationKindFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b0dde8e_04d0_40f4_a27f_c2acbbd6b734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystemAuthorizationKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemAuthorizationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAuthorizationKindFilterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemAuthorizationKindFilterFactory {
    type Vtable = IRemoteSystemAuthorizationKindFilterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad65df4d_b66a_45a4_8177_8caed75d9e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemauthorizationkind: RemoteSystemAuthorizationKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23278bc3_0d09_52cb_9c6a_eed2940bee43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsProximal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionInfoStatics {
    type Vtable = IRemoteSystemConnectionInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac831e2d_66c5_56d7_a4ce_705d94925ad6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub TryCreateFromAppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connection: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    TryCreateFromAppServiceConnection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84ed4104_8d5e_4d72_8238_7621576c7a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionRequest2 {
    type Vtable = IRemoteSystemConnectionRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12df6d6f_bffc_483a_8abe_d34a6c19f92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystemApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequest3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionRequest3 {
    type Vtable = IRemoteSystemConnectionRequest3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde86c3e7_c9cc_5a50_b8d9_ba7b34bb8d0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionRequestFactory {
    type Vtable = IRemoteSystemConnectionRequestFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa0a0a20_baeb_4575_b530_810bb9786334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequestStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionRequestStatics {
    type Vtable = IRemoteSystemConnectionRequestStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86ca143d_8214_425c_8932_db49032d1306);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemapp: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequestStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemConnectionRequestStatics2 {
    type Vtable = IRemoteSystemConnectionRequestStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x460f1027_64ec_598e_a800_4f2ee58def19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontoken: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromConnectionTokenForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, connectiontoken: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemDiscoveryTypeFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42d9041f_ee5a_43da_ac6a_6fee25460741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystemDiscoveryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemDiscoveryType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemDiscoveryTypeFilterFactory {
    type Vtable = IRemoteSystemDiscoveryTypeFilterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f9eb993_c260_4161_92f2_9c021f23fe5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discoverytype: RemoteSystemDiscoveryType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemEnumerationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6e83d5f_4030_4354_a060_14f1b22c545d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemEnumerationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[repr(transparent)]
pub struct IRemoteSystemFilter(::windows_core::IUnknown);
impl IRemoteSystemFilter {}
impl ::core::convert::From<IRemoteSystemFilter> for ::windows_core::IUnknown {
    fn from(value: IRemoteSystemFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRemoteSystemFilter> for ::windows_core::IUnknown {
    fn from(value: &IRemoteSystemFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRemoteSystemFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRemoteSystemFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRemoteSystemFilter> for ::windows_core::IInspectable {
    fn from(value: IRemoteSystemFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRemoteSystemFilter> for ::windows_core::IInspectable {
    fn from(value: &IRemoteSystemFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IRemoteSystemFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IRemoteSystemFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRemoteSystemFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRemoteSystemFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteSystemFilter {}
impl ::core::fmt::Debug for IRemoteSystemFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteSystemFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IRemoteSystemFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4a3ba9e4-99eb-45eb-ba16-0367728ff374}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IRemoteSystemFilter {
    type Vtable = IRemoteSystemFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a3ba9e4_99eb_45eb_ba16_0367728ff374);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38e1c9ec_22c3_4ef6_901a_bbb1c7aad4ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub RemoteSystemKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RemoteSystemKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindFilterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemKindFilterFactory {
    type Vtable = IRemoteSystemKindFilterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1fb18ee_99ea_40bc_9a39_c670aa804a28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemkinds: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemKindStatics {
    type Vtable = IRemoteSystemKindStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6317633_ab14_41d0_9553_796aadb882db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Phone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Hub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Holographic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Xbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemKindStatics2 {
    type Vtable = IRemoteSystemKindStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9e3a3d0_0466_4749_91e8_65f9d19a96a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Iot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Laptop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b3d16bb_7306_49ea_b7df_67d5714cb013);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSession {
    type Vtable = IRemoteSystemSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69476a01_9ada_490f_9549_d31cb14c9e95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CreateParticipantWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SendInvitationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invitee: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd585d754_bc97_4c39_99b4_beca76e04c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe48b2dd2_6820_4867_b425_d89c0a3ef7ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub JoinRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveJoinRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveParticipantAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparticipant: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionControllerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionControllerFactory {
    type Vtable = IRemoteSystemSessionControllerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfcc2f6b_ac3d_4199_82cd_6670a773ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionControllerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateControllerWithSessionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionCreationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa79812c2_37de_448c_8b83_a30aa3c4ead6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionCreationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionCreationStatus) -> ::windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionDisconnectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde0bc69b_77c5_461c_8209_7c6c5d3111ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionDisconnectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionDisconnectedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff4df648_8b0a_4e9a_9905_69e4b841c588);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JoinAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInvitation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e32cc91_51d7_4766_a121_25516c3b8294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInvitationListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08f4003f_bc71_49e1_874a_31ddff9a27b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InvitationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInvitationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e964a2d_a10d_4edb_8dea_54d20ac19543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Invitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionJoinRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20600068_7994_4331_86d1_d89d882585ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbca4fc3_82b9_4816_9c24_e40e61774bd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub JoinRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionJoinResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce7b1f04_a03e_41a4_900b_1e79328c1267);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionJoinStatus) -> ::windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionMessageChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9524d12a_73d9_4c10_b751_c26784437127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub BroadcastValueSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagedata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    BroadcastValueSetAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SendValueSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagedata: ::windows_core::RawPtr, participant: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SendValueSetAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SendValueSetToParticipantsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagedata: ::windows_core::RawPtr, participants: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SendValueSetToParticipantsAsync: usize,
    pub ValueSetReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveValueSetReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionMessageChannelFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionMessageChannelFactory {
    type Vtable = IRemoteSystemSessionMessageChannelFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x295e1c4a_bd16_4298_b7ce_415482b0e11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannelFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: ::windows_core::RawPtr, channelname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithReliability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: ::windows_core::RawPtr, channelname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, reliability: RemoteSystemSessionMessageChannelReliability, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x740ed755_8418_4f01_9353_e21c9ecc6cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsInviteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsInviteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipant(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e90058c_acf9_4729_8a17_44e7baed5dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipant_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-networking"))]
    pub GetHostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-networking")))]
    GetHostNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd35a57d8_c9a1_4bb7_b6b0_79bb91adf93d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x866ef088_de68_4abf_88a1_f90d16274192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipantWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcdd02cc_aa87_4d79_b6cc_4459b3e92075);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionParticipantWatcherStatus) -> ::windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf82914e_39a1_4dea_9d63_43798d5bbbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionStatics {
    type Vtable = IRemoteSystemSessionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8524899f_fd20_44e3_9565_e75a3b14c66e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16875069_231e_4c91_8ec8_b3a39d9e55a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06f31785_2da5_4e58_a78f_9e8d0784ee25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Message: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8003e340_0c41_4a62_b6d7_bdbe2b19be2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionWatcherStatus) -> ::windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemStatics {
    type Vtable = IRemoteSystemStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa485b392_ff2b_4b47_be62_743f2f140f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-networking")]
    pub FindByHostNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    FindByHostNameAsync: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub CreateWatcherWithFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateWatcherWithFilters: usize,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemStatics2 {
    type Vtable = IRemoteSystemStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c98edca_6f99_4c52_a272_ea4f36471744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAuthorizationKindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: RemoteSystemAuthorizationKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemStatics3 {
    type Vtable = IRemoteSystemStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9995f16f_0b3c_5ac5_b325_cc73f437dfcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWatcherForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub CreateWatcherWithFiltersForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, filters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateWatcherWithFiltersForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatusTypeFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c39514e_cbb6_4777_8534_2e0c521affa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystemStatusType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemStatusType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatusTypeFilterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemStatusTypeFilterFactory {
    type Vtable = IRemoteSystemStatusTypeFilterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33cf78fa_d724_4125_ac7a_8d281e44c949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemstatustype: RemoteSystemStatusType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7502ff0e_dbcb_4155_b4ca_b30a04f27627);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d600c7e_2c07_48c5_889c_455d2b099771);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoteSystemAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoteSystemAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoteSystemUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoteSystemUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoteSystemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoteSystemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcher2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemWatcher2 {
    type Vtable = IRemoteSystemWatcher2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73436700_19ca_48f9_a4cd_780f7ad58c71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcher3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemWatcher3 {
    type Vtable = IRemoteSystemWatcher3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf79c0fcf_a913_55d3_8413_418fcf15ba54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74c5c6af_5114_4426_9216_20d81f8519ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemWatcherError) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWebAccountFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fb75873_87c8_5d8f_977e_f69f96d67238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-security")]
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    Account: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWebAccountFilterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemWebAccountFilterFactory {
    type Vtable = IRemoteSystemWebAccountFilterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x348a2709_5f4d_5127_b4a7_bf99d5252b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-security")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    Create: usize,
}
pub struct KnownRemoteSystemCapabilities;
impl KnownRemoteSystemCapabilities {
    pub fn AppService() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppService)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn LaunchUri() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RemoteSession() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SpatialEntity() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialEntity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKnownRemoteSystemCapabilitiesStatics<R, F: FnOnce(&IKnownRemoteSystemCapabilitiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownRemoteSystemCapabilities, IKnownRemoteSystemCapabilitiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownRemoteSystemCapabilities {
    const NAME: &'static str = "Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
}
#[repr(transparent)]
pub struct RemoteSystem(::windows_core::IUnknown);
impl RemoteSystem {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<RemoteSystemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemStatus>(result__)
        }
    }
    pub fn IsAvailableByProximity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableByProximity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableBySpatialProximity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetCapabilitySupportedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, capabilityname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCapabilitySupportedAsync)(::windows_core::Interface::as_raw(this), capabilityname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ManufacturerDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ManufacturerDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ModelDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Platform(&self) -> ::windows_core::Result<RemoteSystemPlatform> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemPlatform>::zeroed();
            (::windows_core::Interface::vtable(this).Platform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemPlatform>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Apps(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<RemoteSystemApp>> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Apps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<RemoteSystemApp>>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = &::windows_core::Interface::cast::<IRemoteSystem6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
    #[cfg(feature = "winrt-networking")]
    pub fn FindByHostNameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_networking::HostName>>(hostname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RemoteSystem>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindByHostNameAsync)(::windows_core::Interface::as_raw(this), hostname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RemoteSystem>>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateWatcherWithFilters<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IRemoteSystemFilter>>>(filters: Param0) -> ::windows_core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherWithFilters)(::windows_core::Interface::as_raw(this), filters.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RemoteSystemAccessStatus>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RemoteSystemAccessStatus>>(result__)
        })
    }
    pub fn IsAuthorizationKindEnabled(kind: RemoteSystemAuthorizationKind) -> ::windows_core::Result<bool> {
        Self::IRemoteSystemStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAuthorizationKindEnabled)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CreateWatcherForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateWatcherWithFiltersForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IRemoteSystemFilter>>>(user: Param0, filters: Param1) -> ::windows_core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherWithFiltersForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), filters.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    pub fn IRemoteSystemStatics<R, F: FnOnce(&IRemoteSystemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystem, IRemoteSystemStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemStatics2<R, F: FnOnce(&IRemoteSystemStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystem, IRemoteSystemStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemStatics3<R, F: FnOnce(&IRemoteSystemStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystem, IRemoteSystemStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystem {}
impl ::core::fmt::Debug for RemoteSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystem;{ed5838cd-1e10-4a8c-b4a6-4e5fd6f97721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystem {
    type Vtable = IRemoteSystem_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystem";
}
impl ::core::convert::From<RemoteSystem> for ::windows_core::IUnknown {
    fn from(value: RemoteSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystem> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystem> for ::windows_core::IInspectable {
    fn from(value: RemoteSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystem> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystem {}
unsafe impl ::core::marker::Sync for RemoteSystem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemAccessStatus(pub i32);
impl RemoteSystemAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemAccessStatus {}
impl ::core::clone::Clone for RemoteSystemAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemAddedEventArgs(::windows_core::IUnknown);
impl RemoteSystemAddedEventArgs {
    pub fn RemoteSystem(&self) -> ::windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystem>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemAddedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAddedEventArgs;{8f39560f-e534-4697-8836-7abea151516e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
}
impl ::core::convert::From<RemoteSystemAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemAddedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemApp(::windows_core::IUnknown);
impl RemoteSystemApp {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsAvailableByProximity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableByProximity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableBySpatialProximity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Attributes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
    pub fn ConnectionToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemApp {}
impl ::core::fmt::Debug for RemoteSystemApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemApp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemApp {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemApp;{80e5bcbd-d54d-41b1-9b16-6810a871ed4f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemApp {
    type Vtable = IRemoteSystemApp_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemApp as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemApp";
}
impl ::core::convert::From<RemoteSystemApp> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemApp> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemApp> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemApp> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemApp {}
unsafe impl ::core::marker::Sync for RemoteSystemApp {}
#[repr(transparent)]
pub struct RemoteSystemAppRegistration(::windows_core::IUnknown);
impl RemoteSystemAppRegistration {
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Attributes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    pub fn SaveAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemAppRegistration>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemAppRegistration>(result__)
        })
    }
    pub fn IRemoteSystemAppRegistrationStatics<R, F: FnOnce(&IRemoteSystemAppRegistrationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemAppRegistration, IRemoteSystemAppRegistrationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemAppRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemAppRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemAppRegistration {}
impl ::core::fmt::Debug for RemoteSystemAppRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAppRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemAppRegistration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAppRegistration;{b47947b5-7035-4a5a-b8df-962d8f8431f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemAppRegistration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAppRegistration";
}
impl ::core::convert::From<RemoteSystemAppRegistration> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemAppRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemAppRegistration> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemAppRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemAppRegistration> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemAppRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemAppRegistration> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemAppRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemAppRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAppRegistration {}
unsafe impl ::core::marker::Sync for RemoteSystemAppRegistration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemAuthorizationKind(pub i32);
impl RemoteSystemAuthorizationKind {
    pub const SameUser: Self = Self(0i32);
    pub const Anonymous: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemAuthorizationKind {}
impl ::core::clone::Clone for RemoteSystemAuthorizationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemAuthorizationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemAuthorizationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemAuthorizationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAuthorizationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemAuthorizationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAuthorizationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemAuthorizationKindFilter(::windows_core::IUnknown);
impl RemoteSystemAuthorizationKindFilter {
    pub fn RemoteSystemAuthorizationKind(&self) -> ::windows_core::Result<RemoteSystemAuthorizationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemAuthorizationKind>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemAuthorizationKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemAuthorizationKind>(result__)
        }
    }
    pub fn Create(remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> ::windows_core::Result<RemoteSystemAuthorizationKindFilter> {
        Self::IRemoteSystemAuthorizationKindFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), remotesystemauthorizationkind, result__.as_mut_ptr()).from_abi::<RemoteSystemAuthorizationKindFilter>(result__)
        })
    }
    pub fn IRemoteSystemAuthorizationKindFilterFactory<R, F: FnOnce(&IRemoteSystemAuthorizationKindFilterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemAuthorizationKindFilter, IRemoteSystemAuthorizationKindFilterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemAuthorizationKindFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemAuthorizationKindFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemAuthorizationKindFilter {}
impl ::core::fmt::Debug for RemoteSystemAuthorizationKindFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAuthorizationKindFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemAuthorizationKindFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter;{6b0dde8e-04d0-40f4-a27f-c2acbbd6b734})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemAuthorizationKindFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
}
impl ::core::convert::From<RemoteSystemAuthorizationKindFilter> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemAuthorizationKindFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemAuthorizationKindFilter> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemAuthorizationKindFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemAuthorizationKindFilter> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemAuthorizationKindFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemAuthorizationKindFilter> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemAuthorizationKindFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: RemoteSystemAuthorizationKindFilter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: &RemoteSystemAuthorizationKindFilter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemAuthorizationKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAuthorizationKindFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemAuthorizationKindFilter {}
#[repr(transparent)]
pub struct RemoteSystemConnectionInfo(::windows_core::IUnknown);
impl RemoteSystemConnectionInfo {
    pub fn IsProximal(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProximal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn TryCreateFromAppServiceConnection<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::AppService::AppServiceConnection>>(connection: Param0) -> ::windows_core::Result<RemoteSystemConnectionInfo> {
        Self::IRemoteSystemConnectionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromAppServiceConnection)(::windows_core::Interface::as_raw(this), connection.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemConnectionInfo>(result__)
        })
    }
    pub fn IRemoteSystemConnectionInfoStatics<R, F: FnOnce(&IRemoteSystemConnectionInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemConnectionInfo, IRemoteSystemConnectionInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemConnectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemConnectionInfo {}
impl ::core::fmt::Debug for RemoteSystemConnectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemConnectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemConnectionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionInfo;{23278bc3-0d09-52cb-9c6a-eed2940bee43})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemConnectionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
}
impl ::core::convert::From<RemoteSystemConnectionInfo> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemConnectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemConnectionInfo> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemConnectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemConnectionInfo> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemConnectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemConnectionInfo> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemConnectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemConnectionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemConnectionInfo {}
unsafe impl ::core::marker::Sync for RemoteSystemConnectionInfo {}
#[repr(transparent)]
pub struct RemoteSystemConnectionRequest(::windows_core::IUnknown);
impl RemoteSystemConnectionRequest {
    pub fn RemoteSystem(&self) -> ::windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystem>(result__)
        }
    }
    pub fn RemoteSystemApp(&self) -> ::windows_core::Result<RemoteSystemApp> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemConnectionRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemApp>(result__)
        }
    }
    pub fn ConnectionToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemConnectionRequest3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystem>>(remotesystem: Param0) -> ::windows_core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), remotesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn CreateForApp<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystemApp>>(remotesystemapp: Param0) -> ::windows_core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForApp)(::windows_core::Interface::as_raw(this), remotesystemapp.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn CreateFromConnectionToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(connectiontoken: Param0) -> ::windows_core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromConnectionToken)(::windows_core::Interface::as_raw(this), connectiontoken.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn CreateFromConnectionTokenForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, connectiontoken: Param1) -> ::windows_core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromConnectionTokenForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), connectiontoken.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn IRemoteSystemConnectionRequestFactory<R, F: FnOnce(&IRemoteSystemConnectionRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemConnectionRequestStatics<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemConnectionRequestStatics2<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemConnectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemConnectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemConnectionRequest {}
impl ::core::fmt::Debug for RemoteSystemConnectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemConnectionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemConnectionRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionRequest;{84ed4104-8d5e-4d72-8238-7621576c7a67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemConnectionRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
}
impl ::core::convert::From<RemoteSystemConnectionRequest> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemConnectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemConnectionRequest> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemConnectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemConnectionRequest> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemConnectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemConnectionRequest> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemConnectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemConnectionRequest {}
unsafe impl ::core::marker::Sync for RemoteSystemConnectionRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemDiscoveryType(pub i32);
impl RemoteSystemDiscoveryType {
    pub const Any: Self = Self(0i32);
    pub const Proximal: Self = Self(1i32);
    pub const Cloud: Self = Self(2i32);
    pub const SpatiallyProximal: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemDiscoveryType {}
impl ::core::clone::Clone for RemoteSystemDiscoveryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemDiscoveryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemDiscoveryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemDiscoveryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemDiscoveryType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemDiscoveryType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemDiscoveryType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemDiscoveryTypeFilter(::windows_core::IUnknown);
impl RemoteSystemDiscoveryTypeFilter {
    pub fn RemoteSystemDiscoveryType(&self) -> ::windows_core::Result<RemoteSystemDiscoveryType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemDiscoveryType>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemDiscoveryType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemDiscoveryType>(result__)
        }
    }
    pub fn Create(discoverytype: RemoteSystemDiscoveryType) -> ::windows_core::Result<RemoteSystemDiscoveryTypeFilter> {
        Self::IRemoteSystemDiscoveryTypeFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), discoverytype, result__.as_mut_ptr()).from_abi::<RemoteSystemDiscoveryTypeFilter>(result__)
        })
    }
    pub fn IRemoteSystemDiscoveryTypeFilterFactory<R, F: FnOnce(&IRemoteSystemDiscoveryTypeFilterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemDiscoveryTypeFilter, IRemoteSystemDiscoveryTypeFilterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemDiscoveryTypeFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemDiscoveryTypeFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemDiscoveryTypeFilter {}
impl ::core::fmt::Debug for RemoteSystemDiscoveryTypeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemDiscoveryTypeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemDiscoveryTypeFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter;{42d9041f-ee5a-43da-ac6a-6fee25460741})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemDiscoveryTypeFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
}
impl ::core::convert::From<RemoteSystemDiscoveryTypeFilter> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemDiscoveryTypeFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemDiscoveryTypeFilter> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemDiscoveryTypeFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemDiscoveryTypeFilter> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemDiscoveryTypeFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemDiscoveryTypeFilter> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemDiscoveryTypeFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: RemoteSystemDiscoveryTypeFilter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: &RemoteSystemDiscoveryTypeFilter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemDiscoveryTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemDiscoveryTypeFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemDiscoveryTypeFilter {}
#[repr(transparent)]
pub struct RemoteSystemEnumerationCompletedEventArgs(::windows_core::IUnknown);
impl RemoteSystemEnumerationCompletedEventArgs {}
impl ::core::clone::Clone for RemoteSystemEnumerationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemEnumerationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemEnumerationCompletedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemEnumerationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemEnumerationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemEnumerationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs;{c6e83d5f-4030-4354-a060-14f1b22c545d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemEnumerationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemEnumerationCompletedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
}
impl ::core::convert::From<RemoteSystemEnumerationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemEnumerationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemEnumerationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemEnumerationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemEnumerationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemEnumerationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemEnumerationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemEnumerationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemEnumerationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemEnumerationCompletedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemKindFilter(::windows_core::IUnknown);
impl RemoteSystemKindFilter {
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoteSystemKinds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemKinds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(remotesystemkinds: Param0) -> ::windows_core::Result<RemoteSystemKindFilter> {
        Self::IRemoteSystemKindFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), remotesystemkinds.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemKindFilter>(result__)
        })
    }
    pub fn IRemoteSystemKindFilterFactory<R, F: FnOnce(&IRemoteSystemKindFilterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemKindFilter, IRemoteSystemKindFilterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemKindFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemKindFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemKindFilter {}
impl ::core::fmt::Debug for RemoteSystemKindFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemKindFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemKindFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemKindFilter;{38e1c9ec-22c3-4ef6-901a-bbb1c7aad4ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemKindFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKindFilter";
}
impl ::core::convert::From<RemoteSystemKindFilter> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemKindFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemKindFilter> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemKindFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemKindFilter> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemKindFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemKindFilter> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemKindFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: RemoteSystemKindFilter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: &RemoteSystemKindFilter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemKindFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemKindFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemKindFilter {}
pub struct RemoteSystemKinds;
impl RemoteSystemKinds {
    pub fn Phone() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Phone)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Hub() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Hub)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Holographic() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Holographic)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Desktop() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Desktop)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Xbox() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Xbox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Iot() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Iot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Tablet() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tablet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Laptop() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Laptop)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IRemoteSystemKindStatics<R, F: FnOnce(&IRemoteSystemKindStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemKindStatics2<R, F: FnOnce(&IRemoteSystemKindStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for RemoteSystemKinds {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKinds";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemPlatform(pub i32);
impl RemoteSystemPlatform {
    pub const Unknown: Self = Self(0i32);
    pub const Windows: Self = Self(1i32);
    pub const Android: Self = Self(2i32);
    pub const Ios: Self = Self(3i32);
    pub const Linux: Self = Self(4i32);
}
impl ::core::marker::Copy for RemoteSystemPlatform {}
impl ::core::clone::Clone for RemoteSystemPlatform {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemPlatform {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemPlatform {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemPlatform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemPlatform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemPlatform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemPlatform;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemRemovedEventArgs(::windows_core::IUnknown);
impl RemoteSystemRemovedEventArgs {
    pub fn RemoteSystemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemRemovedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs;{8b3d16bb-7306-49ea-b7df-67d5714cb013})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
}
impl ::core::convert::From<RemoteSystemRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemRemovedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSession(::windows_core::IUnknown);
impl RemoteSystemSession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ControllerDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ControllerDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Disconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Disconnected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateParticipantWatcher(&self) -> ::windows_core::Result<RemoteSystemSessionParticipantWatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateParticipantWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionParticipantWatcher>(result__)
        }
    }
    pub fn SendInvitationAsync<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystem>>(&self, invitee: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendInvitationAsync)(::windows_core::Interface::as_raw(this), invitee.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows_core::Result<RemoteSystemSessionWatcher> {
        Self::IRemoteSystemSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionWatcher>(result__)
        })
    }
    pub fn IRemoteSystemSessionStatics<R, F: FnOnce(&IRemoteSystemSessionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemSession, IRemoteSystemSessionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSession {}
impl ::core::fmt::Debug for RemoteSystemSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSession;{69476a01-9ada-490f-9549-d31cb14c9e95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSession {
    type Vtable = IRemoteSystemSession_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSession";
}
impl ::core::convert::From<RemoteSystemSession> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSession> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSession> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSession> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RemoteSystemSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: RemoteSystemSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &RemoteSystemSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for RemoteSystemSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &RemoteSystemSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSession {}
unsafe impl ::core::marker::Sync for RemoteSystemSession {}
#[repr(transparent)]
pub struct RemoteSystemSessionAddedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionAddedEventArgs {
    pub fn SessionInfo(&self) -> ::windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SessionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionAddedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs;{d585d754-bc97-4c39-99b4-beca76e04c3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionAddedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionController(::windows_core::IUnknown);
impl RemoteSystemSessionController {
    pub fn JoinRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).JoinRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveJoinRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveJoinRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RemoveParticipantAsync<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystemSessionParticipant>>(&self, pparticipant: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveParticipantAsync)(::windows_core::Interface::as_raw(this), pparticipant.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CreateSessionAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RemoteSystemSessionCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSessionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>(result__)
        }
    }
    pub fn CreateController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(displayname: Param0) -> ::windows_core::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateController)(::windows_core::Interface::as_raw(this), displayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionController>(result__)
        })
    }
    pub fn CreateControllerWithSessionOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, RemoteSystemSessionOptions>>(displayname: Param0, options: Param1) -> ::windows_core::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateControllerWithSessionOptions)(::windows_core::Interface::as_raw(this), displayname.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionController>(result__)
        })
    }
    pub fn IRemoteSystemSessionControllerFactory<R, F: FnOnce(&IRemoteSystemSessionControllerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemSessionController, IRemoteSystemSessionControllerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionController {}
impl ::core::fmt::Debug for RemoteSystemSessionController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionController;{e48b2dd2-6820-4867-b425-d89c0a3ef7ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionController";
}
impl ::core::convert::From<RemoteSystemSessionController> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionController> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionController> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionController> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionController {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionController {}
#[repr(transparent)]
pub struct RemoteSystemSessionCreationResult(::windows_core::IUnknown);
impl RemoteSystemSessionCreationResult {
    pub fn Status(&self) -> ::windows_core::Result<RemoteSystemSessionCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemSessionCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionCreationStatus>(result__)
        }
    }
    pub fn Session(&self) -> ::windows_core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSession>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionCreationResult {}
impl ::core::fmt::Debug for RemoteSystemSessionCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionCreationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionCreationResult;{a79812c2-37de-448c-8b83-a30aa3c4ead6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionCreationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
}
impl ::core::convert::From<RemoteSystemSessionCreationResult> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionCreationResult> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionCreationResult> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionCreationResult> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionCreationResult {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionCreationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemSessionCreationStatus(pub i32);
impl RemoteSystemSessionCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemSessionCreationStatus {}
impl ::core::clone::Clone for RemoteSystemSessionCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemSessionCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemSessionDisconnectedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionDisconnectedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<RemoteSystemSessionDisconnectedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemSessionDisconnectedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionDisconnectedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionDisconnectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionDisconnectedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionDisconnectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionDisconnectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionDisconnectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs;{de0bc69b-77c5-461c-8209-7c6c5d3111ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionDisconnectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionDisconnectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionDisconnectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionDisconnectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionDisconnectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionDisconnectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionDisconnectedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemSessionDisconnectedReason(pub i32);
impl RemoteSystemSessionDisconnectedReason {
    pub const SessionUnavailable: Self = Self(0i32);
    pub const RemovedByController: Self = Self(1i32);
    pub const SessionClosed: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemSessionDisconnectedReason {}
impl ::core::clone::Clone for RemoteSystemSessionDisconnectedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionDisconnectedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemSessionDisconnectedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionDisconnectedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionDisconnectedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionDisconnectedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemSessionInfo(::windows_core::IUnknown);
impl RemoteSystemSessionInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ControllerDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ControllerDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn JoinAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RemoteSystemSessionJoinResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).JoinAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInfo {}
impl ::core::fmt::Debug for RemoteSystemSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInfo;{ff4df648-8b0a-4e9a-9905-69e4b841c588})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInfo";
}
impl ::core::convert::From<RemoteSystemSessionInfo> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInfo> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionInfo> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInfo> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInfo {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInfo {}
#[repr(transparent)]
pub struct RemoteSystemSessionInvitation(::windows_core::IUnknown);
impl RemoteSystemSessionInvitation {
    pub fn Sender(&self) -> ::windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystem>(result__)
        }
    }
    pub fn SessionInfo(&self) -> ::windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SessionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInvitation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInvitation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInvitation {}
impl ::core::fmt::Debug for RemoteSystemSessionInvitation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInvitation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionInvitation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitation;{3e32cc91-51d7-4766-a121-25516c3b8294})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionInvitation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
}
impl ::core::convert::From<RemoteSystemSessionInvitation> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionInvitation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitation> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionInvitation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionInvitation> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionInvitation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitation> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionInvitation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionInvitation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitation {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitation {}
#[repr(transparent)]
pub struct RemoteSystemSessionInvitationListener(::windows_core::IUnknown);
impl RemoteSystemSessionInvitationListener {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemSessionInvitationListener, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn InvitationReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).InvitationReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInvitationReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInvitationReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInvitationListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInvitationListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInvitationListener {}
impl ::core::fmt::Debug for RemoteSystemSessionInvitationListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInvitationListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionInvitationListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener;{08f4003f-bc71-49e1-874a-31ddff9a27b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionInvitationListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
}
impl ::core::convert::From<RemoteSystemSessionInvitationListener> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionInvitationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationListener> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionInvitationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionInvitationListener> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionInvitationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationListener> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionInvitationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionInvitationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitationListener {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitationListener {}
#[repr(transparent)]
pub struct RemoteSystemSessionInvitationReceivedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionInvitationReceivedEventArgs {
    pub fn Invitation(&self) -> ::windows_core::Result<RemoteSystemSessionInvitation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Invitation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionInvitation>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInvitationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInvitationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInvitationReceivedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionInvitationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInvitationReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionInvitationReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs;{5e964a2d-a10d-4edb-8dea-54d20ac19543})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionInvitationReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionInvitationReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionInvitationReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionInvitationReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionInvitationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitationReceivedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionJoinRequest(::windows_core::IUnknown);
impl RemoteSystemSessionJoinRequest {
    pub fn Participant(&self) -> ::windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Participant)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionJoinRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionJoinRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionJoinRequest {}
impl ::core::fmt::Debug for RemoteSystemSessionJoinRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionJoinRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest;{20600068-7994-4331-86d1-d89d882585ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionJoinRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
}
impl ::core::convert::From<RemoteSystemSessionJoinRequest> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionJoinRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequest> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionJoinRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionJoinRequest> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionJoinRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequest> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionJoinRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionJoinRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinRequest {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinRequest {}
#[repr(transparent)]
pub struct RemoteSystemSessionJoinRequestedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionJoinRequestedEventArgs {
    pub fn JoinRequest(&self) -> ::windows_core::Result<RemoteSystemSessionJoinRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).JoinRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionJoinRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionJoinRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionJoinRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionJoinRequestedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionJoinRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionJoinRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs;{dbca4fc3-82b9-4816-9c24-e40e61774bd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionJoinRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionJoinRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionJoinRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionJoinRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinRequestedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionJoinResult(::windows_core::IUnknown);
impl RemoteSystemSessionJoinResult {
    pub fn Status(&self) -> ::windows_core::Result<RemoteSystemSessionJoinStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemSessionJoinStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionJoinStatus>(result__)
        }
    }
    pub fn Session(&self) -> ::windows_core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSession>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionJoinResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionJoinResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionJoinResult {}
impl ::core::fmt::Debug for RemoteSystemSessionJoinResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionJoinResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinResult;{ce7b1f04-a03e-41a4-900b-1e79328c1267})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionJoinResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
}
impl ::core::convert::From<RemoteSystemSessionJoinResult> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionJoinResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinResult> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionJoinResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionJoinResult> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionJoinResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionJoinResult> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionJoinResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionJoinResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinResult {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemSessionJoinStatus(pub i32);
impl RemoteSystemSessionJoinStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
    pub const SessionUnavailable: Self = Self(3i32);
    pub const RejectedByController: Self = Self(4i32);
}
impl ::core::marker::Copy for RemoteSystemSessionJoinStatus {}
impl ::core::clone::Clone for RemoteSystemSessionJoinStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionJoinStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemSessionJoinStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionJoinStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionJoinStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemSessionMessageChannel(::windows_core::IUnknown);
impl RemoteSystemSessionMessageChannel {
    pub fn Session(&self) -> ::windows_core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSession>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn BroadcastValueSetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>>(&self, messagedata: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastValueSetAsync)(::windows_core::Interface::as_raw(this), messagedata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SendValueSetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>, Param1: ::windows_core::IntoParam<'a, RemoteSystemSessionParticipant>>(&self, messagedata: Param0, participant: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendValueSetAsync)(::windows_core::Interface::as_raw(this), messagedata.into_param().abi(), participant.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SendValueSetToParticipantsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<RemoteSystemSessionParticipant>>>(&self, messagedata: Param0, participants: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendValueSetToParticipantsAsync)(::windows_core::Interface::as_raw(this), messagedata.into_param().abi(), participants.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ValueSetReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ValueSetReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveValueSetReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveValueSetReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystemSession>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(session: Param0, channelname: Param1) -> ::windows_core::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), session.into_param().abi(), channelname.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionMessageChannel>(result__)
        })
    }
    pub fn CreateWithReliability<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystemSession>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(session: Param0, channelname: Param1, reliability: RemoteSystemSessionMessageChannelReliability) -> ::windows_core::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithReliability)(::windows_core::Interface::as_raw(this), session.into_param().abi(), channelname.into_param().abi(), reliability, result__.as_mut_ptr()).from_abi::<RemoteSystemSessionMessageChannel>(result__)
        })
    }
    pub fn IRemoteSystemSessionMessageChannelFactory<R, F: FnOnce(&IRemoteSystemSessionMessageChannelFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemSessionMessageChannel, IRemoteSystemSessionMessageChannelFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionMessageChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionMessageChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionMessageChannel {}
impl ::core::fmt::Debug for RemoteSystemSessionMessageChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionMessageChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionMessageChannel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel;{9524d12a-73d9-4c10-b751-c26784437127})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionMessageChannel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
}
impl ::core::convert::From<RemoteSystemSessionMessageChannel> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionMessageChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionMessageChannel> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionMessageChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionMessageChannel> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionMessageChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionMessageChannel> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionMessageChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionMessageChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionMessageChannel {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionMessageChannel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemSessionMessageChannelReliability(pub i32);
impl RemoteSystemSessionMessageChannelReliability {
    pub const Reliable: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemSessionMessageChannelReliability {}
impl ::core::clone::Clone for RemoteSystemSessionMessageChannelReliability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionMessageChannelReliability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemSessionMessageChannelReliability {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionMessageChannelReliability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionMessageChannelReliability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionMessageChannelReliability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemSessionOptions(::windows_core::IUnknown);
impl RemoteSystemSessionOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemSessionOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsInviteOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInviteOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInviteOnly(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsInviteOnly)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionOptions {}
impl ::core::fmt::Debug for RemoteSystemSessionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionOptions;{740ed755-8418-4f01-9353-e21c9ecc6cfc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionOptions";
}
impl ::core::convert::From<RemoteSystemSessionOptions> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionOptions> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionOptions> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionOptions> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionOptions {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionOptions {}
#[repr(transparent)]
pub struct RemoteSystemSessionParticipant(::windows_core::IUnknown);
impl RemoteSystemSessionParticipant {
    pub fn RemoteSystem(&self) -> ::windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystem>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-networking"))]
    pub fn GetHostNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_networking::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHostNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_networking::HostName>>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipant {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipant").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionParticipant {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipant;{7e90058c-acf9-4729-8a17-44e7baed5dcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionParticipant as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
}
impl ::core::convert::From<RemoteSystemSessionParticipant> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipant> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipant> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionParticipant) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipant> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionParticipant) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionParticipant {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipant {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipant {}
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantAddedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionParticipantAddedEventArgs {
    pub fn Participant(&self) -> ::windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Participant)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipantAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipantAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipantAddedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionParticipantAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs;{d35a57d8-c9a1-4bb7-b6b0-79bb91adf93d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionParticipantAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionParticipantAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipantAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionParticipantAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantAddedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantRemovedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionParticipantRemovedEventArgs {
    pub fn Participant(&self) -> ::windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Participant)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipantRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipantRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipantRemovedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionParticipantRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs;{866ef088-de68-4abf-88a1-f90d16274192})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionParticipantRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionParticipantRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipantRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionParticipantRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantRemovedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantWatcher(::windows_core::IUnknown);
impl RemoteSystemSessionParticipantWatcher {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<RemoteSystemSessionParticipantWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemSessionParticipantWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionParticipantWatcherStatus>(result__)
        }
    }
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipantWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipantWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipantWatcher {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionParticipantWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher;{dcdd02cc-aa87-4d79-b6cc-4459b3e92075})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionParticipantWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
}
impl ::core::convert::From<RemoteSystemSessionParticipantWatcher> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionParticipantWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantWatcher> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionParticipantWatcher> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionParticipantWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionParticipantWatcher> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionParticipantWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionParticipantWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemSessionParticipantWatcherStatus(pub i32);
impl RemoteSystemSessionParticipantWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for RemoteSystemSessionParticipantWatcherStatus {}
impl ::core::clone::Clone for RemoteSystemSessionParticipantWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionParticipantWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemSessionParticipantWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionParticipantWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemSessionRemovedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionRemovedEventArgs {
    pub fn SessionInfo(&self) -> ::windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SessionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionRemovedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs;{af82914e-39a1-4dea-9d63-43798d5bbbd0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionRemovedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionUpdatedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionUpdatedEventArgs {
    pub fn SessionInfo(&self) -> ::windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SessionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionUpdatedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs;{16875069-231e-4c91-8ec8-b3a39d9e55a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionUpdatedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionValueSetReceivedEventArgs(::windows_core::IUnknown);
impl RemoteSystemSessionValueSetReceivedEventArgs {
    pub fn Sender(&self) -> ::windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Message(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionValueSetReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionValueSetReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionValueSetReceivedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionValueSetReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionValueSetReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionValueSetReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs;{06f31785-2da5-4e58-a78f-9e8d0784ee25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionValueSetReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
}
impl ::core::convert::From<RemoteSystemSessionValueSetReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionValueSetReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionValueSetReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionValueSetReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionValueSetReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionValueSetReceivedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionValueSetReceivedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemSessionWatcher(::windows_core::IUnknown);
impl RemoteSystemSessionWatcher {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<RemoteSystemSessionWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemSessionWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemSessionWatcherStatus>(result__)
        }
    }
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Updated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionWatcher {}
impl ::core::fmt::Debug for RemoteSystemSessionWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionWatcher;{8003e340-0c41-4a62-b6d7-bdbe2b19be2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemSessionWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
}
impl ::core::convert::From<RemoteSystemSessionWatcher> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemSessionWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionWatcher> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemSessionWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemSessionWatcher> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemSessionWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemSessionWatcher> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemSessionWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemSessionWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSessionWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemSessionWatcherStatus(pub i32);
impl RemoteSystemSessionWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for RemoteSystemSessionWatcherStatus {}
impl ::core::clone::Clone for RemoteSystemSessionWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemSessionWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemSessionWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemStatus(pub i32);
impl RemoteSystemStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const DiscoveringAvailability: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemStatus {}
impl ::core::clone::Clone for RemoteSystemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemStatusType(pub i32);
impl RemoteSystemStatusType {
    pub const Any: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemStatusType {}
impl ::core::clone::Clone for RemoteSystemStatusType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemStatusType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemStatusType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemStatusType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemStatusType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemStatusType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatusType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemStatusTypeFilter(::windows_core::IUnknown);
impl RemoteSystemStatusTypeFilter {
    pub fn RemoteSystemStatusType(&self) -> ::windows_core::Result<RemoteSystemStatusType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemStatusType>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemStatusType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemStatusType>(result__)
        }
    }
    pub fn Create(remotesystemstatustype: RemoteSystemStatusType) -> ::windows_core::Result<RemoteSystemStatusTypeFilter> {
        Self::IRemoteSystemStatusTypeFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), remotesystemstatustype, result__.as_mut_ptr()).from_abi::<RemoteSystemStatusTypeFilter>(result__)
        })
    }
    pub fn IRemoteSystemStatusTypeFilterFactory<R, F: FnOnce(&IRemoteSystemStatusTypeFilterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemStatusTypeFilter, IRemoteSystemStatusTypeFilterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemStatusTypeFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemStatusTypeFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemStatusTypeFilter {}
impl ::core::fmt::Debug for RemoteSystemStatusTypeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemStatusTypeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemStatusTypeFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter;{0c39514e-cbb6-4777-8534-2e0c521affa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemStatusTypeFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
}
impl ::core::convert::From<RemoteSystemStatusTypeFilter> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemStatusTypeFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemStatusTypeFilter> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemStatusTypeFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemStatusTypeFilter> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemStatusTypeFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemStatusTypeFilter> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemStatusTypeFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: RemoteSystemStatusTypeFilter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: &RemoteSystemStatusTypeFilter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemStatusTypeFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemStatusTypeFilter {}
#[repr(transparent)]
pub struct RemoteSystemUpdatedEventArgs(::windows_core::IUnknown);
impl RemoteSystemUpdatedEventArgs {
    pub fn RemoteSystem(&self) -> ::windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystem>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemUpdatedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs;{7502ff0e-dbcb-4155-b4ca-b30a04f27627})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
}
impl ::core::convert::From<RemoteSystemUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemUpdatedEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemWatcher(::windows_core::IUnknown);
impl RemoteSystemWatcher {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoteSystemAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoteSystemAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoteSystemAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RemoteSystemUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoteSystemUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoteSystemUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RemoteSystemRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteSystemRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoteSystemRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoteSystemRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorOccurred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveErrorOccurred)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = &::windows_core::Interface::cast::<IRemoteSystemWatcher3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemWatcher {}
impl ::core::fmt::Debug for RemoteSystemWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWatcher;{5d600c7e-2c07-48c5-889c-455d2b099771})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcher";
}
impl ::core::convert::From<RemoteSystemWatcher> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemWatcher> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemWatcher> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemWatcher> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteSystemWatcherError(pub i32);
impl RemoteSystemWatcherError {
    pub const Unknown: Self = Self(0i32);
    pub const InternetNotAvailable: Self = Self(1i32);
    pub const AuthenticationError: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemWatcherError {}
impl ::core::clone::Clone for RemoteSystemWatcherError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemWatcherError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteSystemWatcherError {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemWatcherError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcherError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemWatcherError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemWatcherError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RemoteSystemWatcherErrorOccurredEventArgs(::windows_core::IUnknown);
impl RemoteSystemWatcherErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<RemoteSystemWatcherError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RemoteSystemWatcherError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RemoteSystemWatcherError>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemWatcherErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemWatcherErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemWatcherErrorOccurredEventArgs {}
impl ::core::fmt::Debug for RemoteSystemWatcherErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcherErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemWatcherErrorOccurredEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs;{74c5c6af-5114-4426-9216-20d81f8519ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemWatcherErrorOccurredEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
}
impl ::core::convert::From<RemoteSystemWatcherErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemWatcherErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemWatcherErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemWatcherErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemWatcherErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWatcherErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemWatcherErrorOccurredEventArgs {}
#[repr(transparent)]
pub struct RemoteSystemWebAccountFilter(::windows_core::IUnknown);
impl RemoteSystemWebAccountFilter {
    #[cfg(feature = "winrt-security")]
    pub fn Account(&self) -> ::windows_core::Result<::winrt_security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Account)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::WebAccount>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::WebAccount>>(account: Param0) -> ::windows_core::Result<RemoteSystemWebAccountFilter> {
        Self::IRemoteSystemWebAccountFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), account.into_param().abi(), result__.as_mut_ptr()).from_abi::<RemoteSystemWebAccountFilter>(result__)
        })
    }
    pub fn IRemoteSystemWebAccountFilterFactory<R, F: FnOnce(&IRemoteSystemWebAccountFilterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteSystemWebAccountFilter, IRemoteSystemWebAccountFilterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RemoteSystemWebAccountFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemWebAccountFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemWebAccountFilter {}
impl ::core::fmt::Debug for RemoteSystemWebAccountFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWebAccountFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteSystemWebAccountFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWebAccountFilter;{3fb75873-87c8-5d8f-977e-f69f96d67238})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteSystemWebAccountFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
}
impl ::core::convert::From<RemoteSystemWebAccountFilter> for ::windows_core::IUnknown {
    fn from(value: RemoteSystemWebAccountFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemWebAccountFilter> for ::windows_core::IUnknown {
    fn from(value: &RemoteSystemWebAccountFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteSystemWebAccountFilter> for ::windows_core::IInspectable {
    fn from(value: RemoteSystemWebAccountFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteSystemWebAccountFilter> for ::windows_core::IInspectable {
    fn from(value: &RemoteSystemWebAccountFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: RemoteSystemWebAccountFilter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows_core::Error;
    fn try_from(value: &RemoteSystemWebAccountFilter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows_core::Param<'a, IRemoteSystemFilter> {
        ::core::convert::TryInto::<IRemoteSystemFilter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWebAccountFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemWebAccountFilter {}
