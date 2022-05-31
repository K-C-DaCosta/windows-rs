#[doc(hidden)]
#[repr(transparent)]
pub struct INamedResource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INamedResource {
    type Vtable = INamedResource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c98c219_0b13_4240_89a5_d495dc189a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedResource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Candidates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Candidates: usize,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResolveForContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ResolveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResolveAll: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ResolveAllForContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResolveAllForContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Qualifiers: usize,
    pub IsMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMatchAsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub GetValueAsFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetValueAsFileAsync: usize,
    pub GetQualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate2 {
    type Vtable = IResourceCandidate2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69e5b468_f6fc_4013_aaa2_d53f1757d3b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub GetValueAsStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetValueAsStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate3 {
    type Vtable = IResourceCandidate3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08ae97f8_517a_4674_958c_4a3c7cd2cc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResourceCandidateKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContext {
    type Vtable = IResourceContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub QualifierValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    QualifierValues: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ResetQualifierValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiernames: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResetQualifierValues: usize,
    #[cfg(feature = "winrt-foundation")]
    pub OverrideToMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OverrideToMatch: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Languages: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics {
    type Vtable = IResourceContextStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98be9d6c_6338_4b31_99df_b2b442f17149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateMatchingContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateMatchingContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics2 {
    type Vtable = IResourceContextStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41f752ef_12af_41b9_ab36_b1eb4b512460);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGlobalQualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResetGlobalQualifierValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ResetGlobalQualifierValuesForSpecifiedQualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiernames: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResetGlobalQualifierValuesForSpecifiedQualifiers: usize,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics3 {
    type Vtable = IResourceContextStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20cf492c_af0f_450b_9da6_106dd0c29a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetGlobalQualifierValueWithPersistence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics4 {
    type Vtable = IResourceContextStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22eb9ccd_fb31_4bfa_b86b_df9d9d7bdc39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetForUIContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf744d97b_9988_44fb_abd6_5378844cfa8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MainResourceMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AllResourceMaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AllResourceMaps: usize,
    pub DefaultContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub LoadPriFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    LoadPriFiles: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub UnloadPriFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    UnloadPriFiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d66fe6c_a4d7_4c23_9e85_675f304c252d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetAllNamedResourcesForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAllNamedResourcesForPackage: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetAllSubtreesForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAllSubtreesForPackage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManagerStatics {
    type Vtable = IResourceManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cc0fdfc_69ee_4e43_9901_47f12687baf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsResourceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcereference: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceMap {
    type Vtable = IResourceMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72284824_db8c_42f8_b08c_53ff357dad82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetValueForContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceQualifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceQualifier {
    type Vtable = IResourceQualifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceQualifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub QualifierName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Score: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct NamedResource(::windows_core::IUnknown);
impl NamedResource {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Candidates(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Candidates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
    pub fn Resolve(&self) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Resolve)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn ResolveForContext<'a, Param0: ::windows_core::IntoParam<'a, ResourceContext>>(&self, resourcecontext: Param0) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveForContext)(::windows_core::Interface::as_raw(this), resourcecontext.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResolveAll(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveAll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResolveAllForContext<'a, Param0: ::windows_core::IntoParam<'a, ResourceContext>>(&self, resourcecontext: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveAllForContext)(::windows_core::Interface::as_raw(this), resourcecontext.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
}
impl ::core::clone::Clone for NamedResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NamedResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NamedResource {}
impl ::core::fmt::Debug for NamedResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamedResource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NamedResource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NamedResource {
    type Vtable = INamedResource_Vtbl;
    const IID: ::windows_core::GUID = <INamedResource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.NamedResource";
}
impl ::core::convert::From<NamedResource> for ::windows_core::IUnknown {
    fn from(value: NamedResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NamedResource> for ::windows_core::IUnknown {
    fn from(value: &NamedResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NamedResource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NamedResource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NamedResource> for ::windows_core::IInspectable {
    fn from(value: NamedResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NamedResource> for ::windows_core::IInspectable {
    fn from(value: &NamedResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NamedResource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NamedResource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NamedResource {}
unsafe impl ::core::marker::Sync for NamedResource {}
#[repr(transparent)]
pub struct ResourceCandidate(::windows_core::IUnknown);
impl ResourceCandidate {
    #[cfg(feature = "winrt-foundation")]
    pub fn Qualifiers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Qualifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ResourceQualifier>>(result__)
        }
    }
    pub fn IsMatch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMatch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMatchAsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMatchAsDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ValueAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ValueAsString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetValueAsFileAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetValueAsFileAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>>(result__)
        }
    }
    pub fn GetQualifierValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, qualifiername: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetQualifierValue)(::windows_core::Interface::as_raw(this), qualifiername.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetValueAsStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>> {
        let this = &::windows_core::Interface::cast::<IResourceCandidate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetValueAsStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ResourceCandidateKind> {
        let this = &::windows_core::Interface::cast::<IResourceCandidate3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ResourceCandidateKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceCandidateKind>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCandidate {}
impl ::core::fmt::Debug for ResourceCandidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceCandidate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
    const IID: ::windows_core::GUID = <IResourceCandidate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidate";
}
impl ::core::convert::From<ResourceCandidate> for ::windows_core::IUnknown {
    fn from(value: ResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows_core::IUnknown {
    fn from(value: &ResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceCandidate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceCandidate> for ::windows_core::IInspectable {
    fn from(value: ResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows_core::IInspectable {
    fn from(value: &ResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceCandidate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceCandidateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ResourceCandidateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceCandidateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceCandidateVectorView(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceCandidateVectorView {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<ResourceCandidate>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<ResourceCandidate>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<ResourceCandidate>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ResourceCandidate>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ResourceCandidate>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceCandidateVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceCandidateVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceCandidateVectorView {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceCandidateVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateVectorView").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceCandidateVectorView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceCandidateVectorView {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<ResourceCandidate>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<ResourceCandidate> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceCandidateVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceCandidateVectorView> for ::windows_core::IUnknown {
    fn from(value: ResourceCandidateVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceCandidateVectorView> for ::windows_core::IUnknown {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceCandidateVectorView> for ::windows_core::IInspectable {
    fn from(value: ResourceCandidateVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceCandidateVectorView> for ::windows_core::IInspectable {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceCandidateVectorView> for ::winrt_foundation::Collections::IIterable<ResourceCandidate> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceCandidateVectorView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceCandidateVectorView> for ::winrt_foundation::Collections::IIterable<ResourceCandidate> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceCandidateVectorView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ResourceCandidate>> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ResourceCandidate>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ResourceCandidate>> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ResourceCandidate>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<ResourceCandidate>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceCandidateVectorView> for ::winrt_foundation::Collections::IVectorView<ResourceCandidate> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceCandidateVectorView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceCandidateVectorView> for ::winrt_foundation::Collections::IVectorView<ResourceCandidate> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceCandidateVectorView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<ResourceCandidate>> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<ResourceCandidate>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<ResourceCandidate>> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<ResourceCandidate>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<ResourceCandidate>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceCandidateVectorView {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceCandidateVectorView {}
#[repr(transparent)]
pub struct ResourceContext(::windows_core::IUnknown);
impl ResourceContext {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceContext, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn QualifierValues(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResetQualifierValues<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, qualifiernames: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResetQualifierValues)(::windows_core::Interface::as_raw(this), qualifiernames.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OverrideToMatch<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ResourceQualifier>>>(&self, result: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OverrideToMatch)(::windows_core::Interface::as_raw(this), result.into_param().abi()).ok() }
    }
    pub fn Clone(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Clone)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceContext>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetLanguages<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>>(&self, languages: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguages)(::windows_core::Interface::as_raw(this), languages.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateMatchingContext<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ResourceQualifier>>>(result: Param0) -> ::windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMatchingContext)(::windows_core::Interface::as_raw(this), result.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn SetGlobalQualifierValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(key: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetGlobalQualifierValue)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn ResetGlobalQualifierValues() -> ::windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).ResetGlobalQualifierValues)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResetGlobalQualifierValuesForSpecifiedQualifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(qualifiernames: Param0) -> ::windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).ResetGlobalQualifierValuesForSpecifiedQualifiers)(::windows_core::Interface::as_raw(this), qualifiernames.into_param().abi()).ok() })
    }
    pub fn GetForViewIndependentUse() -> ::windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForViewIndependentUse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn SetGlobalQualifierValueWithPersistence<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(key: Param0, value: Param1, persistence: ResourceQualifierPersistence) -> ::windows_core::Result<()> {
        Self::IResourceContextStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetGlobalQualifierValueWithPersistence)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), persistence).ok() })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetForUIContext<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::UIContext>>(context: Param0) -> ::windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUIContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn IResourceContextStatics<R, F: FnOnce(&IResourceContextStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceContext, IResourceContextStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics2<R, F: FnOnce(&IResourceContextStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceContext, IResourceContextStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics3<R, F: FnOnce(&IResourceContextStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceContext, IResourceContextStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics4<R, F: FnOnce(&IResourceContextStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceContext, IResourceContextStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ResourceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceContext {}
impl ::core::fmt::Debug for ResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContext;{2fa22f4b-707e-4b27-ad0d-d0d8cd468fd2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceContext {
    type Vtable = IResourceContext_Vtbl;
    const IID: ::windows_core::GUID = <IResourceContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContext";
}
impl ::core::convert::From<ResourceContext> for ::windows_core::IUnknown {
    fn from(value: ResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows_core::IUnknown {
    fn from(value: &ResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceContext> for ::windows_core::IInspectable {
    fn from(value: ResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows_core::IInspectable {
    fn from(value: &ResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceContextLanguagesVectorView(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceContextLanguagesVectorView {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::windows_core::HSTRING]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceContextLanguagesVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceContextLanguagesVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceContextLanguagesVectorView {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceContextLanguagesVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceContextLanguagesVectorView").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceContextLanguagesVectorView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};string))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceContextLanguagesVectorView {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<::windows_core::HSTRING>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceContextLanguagesVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceContextLanguagesVectorView {
    type Item = ::windows_core::HSTRING;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceContextLanguagesVectorView {
    type Item = ::windows_core::HSTRING;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for ::windows_core::IUnknown {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for ::windows_core::IUnknown {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for ::windows_core::IInspectable {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for ::windows_core::IInspectable {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceContextLanguagesVectorView> for ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceContextLanguagesVectorView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceContextLanguagesVectorView> for ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceContextLanguagesVectorView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceContextLanguagesVectorView> for ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceContextLanguagesVectorView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceContextLanguagesVectorView> for ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceContextLanguagesVectorView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceContextLanguagesVectorView {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceContextLanguagesVectorView {}
#[repr(C)]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl ::core::marker::Copy for ResourceLayoutInfo {}
impl ::core::clone::Clone for ResourceLayoutInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ResourceLayoutInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResourceLayoutInfo").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("ResourceSubtreeCount", &self.ResourceSubtreeCount).field("NamedResourceCount", &self.NamedResourceCount).field("Checksum", &self.Checksum).finish()
    }
}
unsafe impl ::windows_core::Abi for ResourceLayoutInfo {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for ResourceLayoutInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo;u4;u4;u4;u4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ResourceLayoutInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ResourceLayoutInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for ResourceLayoutInfo {}
impl ::core::default::Default for ResourceLayoutInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct ResourceManager(::windows_core::IUnknown);
impl ResourceManager {
    pub fn MainResourceMap(&self) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MainResourceMap)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AllResourceMaps(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllResourceMaps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>>(result__)
        }
    }
    pub fn DefaultContext(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceContext>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn LoadPriFiles<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_storage::IStorageFile>>>(&self, files: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadPriFiles)(::windows_core::Interface::as_raw(this), files.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn UnloadPriFiles<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_storage::IStorageFile>>>(&self, files: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnloadPriFiles)(::windows_core::Interface::as_raw(this), files.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAllNamedResourcesForPackage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ResourceLayoutInfo>>(&self, packagename: Param0, resourcelayoutinfo: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<NamedResource>> {
        let this = &::windows_core::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllNamedResourcesForPackage)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), resourcelayoutinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<NamedResource>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAllSubtreesForPackage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ResourceLayoutInfo>>(&self, packagename: Param0, resourcelayoutinfo: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ResourceMap>> {
        let this = &::windows_core::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllSubtreesForPackage)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), resourcelayoutinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ResourceMap>>(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<ResourceManager> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceManager>(result__)
        })
    }
    pub fn IsResourceReference<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(resourcereference: Param0) -> ::windows_core::Result<bool> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsResourceReference)(::windows_core::Interface::as_raw(this), resourcereference.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IResourceManagerStatics<R, F: FnOnce(&IResourceManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceManager, IResourceManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceManager {}
impl ::core::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceManager;{f744d97b-9988-44fb-abd6-5378844cfa8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: ::windows_core::GUID = <IResourceManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceManager";
}
impl ::core::convert::From<ResourceManager> for ::windows_core::IUnknown {
    fn from(value: ResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows_core::IUnknown {
    fn from(value: &ResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceManager> for ::windows_core::IInspectable {
    fn from(value: ResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows_core::IInspectable {
    fn from(value: &ResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[repr(transparent)]
pub struct ResourceMap(::windows_core::IUnknown);
impl ResourceMap {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<NamedResource> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<NamedResource>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Split(&self, first: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>, second: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn GetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, resource: Param0) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(::windows_core::Interface::as_raw(this), resource.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetValueForContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ResourceContext>>(&self, resource: Param0, context: Param1) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetValueForContext)(::windows_core::Interface::as_raw(this), resource.into_param().abi(), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetSubtree<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reference: Param0) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSubtree)(::windows_core::Interface::as_raw(this), reference.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceMap>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceMap {}
impl ::core::fmt::Debug for ResourceMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceMap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceMap {
    type Vtable = IResourceMap_Vtbl;
    const IID: ::windows_core::GUID = <IResourceMap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMap";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceMap {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceMap {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<ResourceMap> for ::windows_core::IUnknown {
    fn from(value: ResourceMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows_core::IUnknown {
    fn from(value: &ResourceMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceMap> for ::windows_core::IInspectable {
    fn from(value: ResourceMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows_core::IInspectable {
    fn from(value: &ResourceMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceMap> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceMap> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> for ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> for &ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceMap> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceMap> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>> for ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>> for &ResourceMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceMapIterator(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceMapIterator {
    #[cfg(feature = "winrt-foundation")]
    pub fn Current(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceMapIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceMapIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceMapIterator {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceMapIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMapIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceMapIterator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00}))))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceMapIterator {
    type Vtable = ::winrt_foundation::Collections::IIterator_Vtbl<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceMapIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceMapIterator> for ::windows_core::IUnknown {
    fn from(value: ResourceMapIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceMapIterator> for ::windows_core::IUnknown {
    fn from(value: &ResourceMapIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceMapIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceMapIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceMapIterator> for ::windows_core::IInspectable {
    fn from(value: ResourceMapIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceMapIterator> for ::windows_core::IInspectable {
    fn from(value: &ResourceMapIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceMapIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceMapIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceMapIterator> for ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceMapIterator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceMapIterator> for ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceMapIterator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> for ResourceMapIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> for &ResourceMapIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceMapIterator {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceMapIterator {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceMapMapView(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceMapMapView {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Split(&self, first: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>>, second: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceMapMapView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceMapMapView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceMapMapView {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceMapMapView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMapMapView").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceMapMapView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceMapMapView {
    type Vtable = ::winrt_foundation::Collections::IMapView_Vtbl<::windows_core::HSTRING, ResourceMap>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceMapMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceMapMapView {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceMapMapView {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceMapMapView> for ::windows_core::IUnknown {
    fn from(value: ResourceMapMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceMapMapView> for ::windows_core::IUnknown {
    fn from(value: &ResourceMapMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceMapMapView> for ::windows_core::IInspectable {
    fn from(value: ResourceMapMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceMapMapView> for ::windows_core::IInspectable {
    fn from(value: &ResourceMapMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceMapMapView> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceMapMapView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceMapMapView> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceMapMapView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> for ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> for &ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceMapMapView> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceMapMapView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceMapMapView> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceMapMapView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> for ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> for &ResourceMapMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceMapMapView {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceMapMapView {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceMapMapViewIterator(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceMapMapViewIterator {
    #[cfg(feature = "winrt-foundation")]
    pub fn Current(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceMapMapViewIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceMapMapViewIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceMapMapViewIterator {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceMapMapViewIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMapMapViewIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceMapMapViewIterator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82}))))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceMapMapViewIterator {
    type Vtable = ::winrt_foundation::Collections::IIterator_Vtbl<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceMapMapViewIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceMapMapViewIterator> for ::windows_core::IUnknown {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for ::windows_core::IUnknown {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceMapMapViewIterator> for ::windows_core::IInspectable {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for ::windows_core::IInspectable {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceMapMapViewIterator> for ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceMapMapViewIterator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceMapMapViewIterator> for ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceMapMapViewIterator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> for &ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceMapMapViewIterator {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceMapMapViewIterator {}
#[repr(transparent)]
pub struct ResourceQualifier(::windows_core::IUnknown);
impl ResourceQualifier {
    pub fn QualifierName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QualifierName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn QualifierValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMatch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMatch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Score(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Score)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceQualifier {}
impl ::core::fmt::Debug for ResourceQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceQualifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceQualifier {
    type Vtable = IResourceQualifier_Vtbl;
    const IID: ::windows_core::GUID = <IResourceQualifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifier";
}
impl ::core::convert::From<ResourceQualifier> for ::windows_core::IUnknown {
    fn from(value: ResourceQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceQualifier> for ::windows_core::IUnknown {
    fn from(value: &ResourceQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceQualifier> for ::windows_core::IInspectable {
    fn from(value: ResourceQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceQualifier> for ::windows_core::IInspectable {
    fn from(value: &ResourceQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceQualifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceQualifier {}
unsafe impl ::core::marker::Sync for ResourceQualifier {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceQualifierMapView(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceQualifierMapView {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Split(&self, first: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>, second: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceQualifierMapView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceQualifierMapView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceQualifierMapView {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceQualifierMapView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierMapView").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceQualifierMapView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;string))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceQualifierMapView {
    type Vtable = ::winrt_foundation::Collections::IMapView_Vtbl<::windows_core::HSTRING, ::windows_core::HSTRING>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceQualifierMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceQualifierMapView {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceQualifierMapView {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceQualifierMapView> for ::windows_core::IUnknown {
    fn from(value: ResourceQualifierMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceQualifierMapView> for ::windows_core::IUnknown {
    fn from(value: &ResourceQualifierMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceQualifierMapView> for ::windows_core::IInspectable {
    fn from(value: ResourceQualifierMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceQualifierMapView> for ::windows_core::IInspectable {
    fn from(value: &ResourceQualifierMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierMapView> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierMapView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierMapView> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierMapView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierMapView> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierMapView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierMapView> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierMapView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceQualifierMapView {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceQualifierMapView {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceQualifierObservableMap(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceQualifierObservableMap {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MapChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::HSTRING>>>(&self, vhnd: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveMapChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceQualifierObservableMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceQualifierObservableMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceQualifierObservableMap {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceQualifierObservableMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierObservableMap").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceQualifierObservableMap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap;pinterface({65df2bf5-bf39-41b5-aebc-5a9d865e472b};string;string))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceQualifierObservableMap {
    type Vtable = ::winrt_foundation::Collections::IObservableMap_Vtbl<::windows_core::HSTRING, ::windows_core::HSTRING>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceQualifierObservableMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceQualifierObservableMap {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceQualifierObservableMap {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceQualifierObservableMap> for ::windows_core::IUnknown {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for ::windows_core::IUnknown {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceQualifierObservableMap> for ::windows_core::IInspectable {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for ::windows_core::IInspectable {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceQualifierObservableMap {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceQualifierObservableMap {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
impl ::core::marker::Copy for ResourceQualifierPersistence {}
impl ::core::clone::Clone for ResourceQualifierPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceQualifierPersistence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ResourceQualifierPersistence {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResourceQualifierPersistence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierPersistence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceQualifierPersistence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ResourceQualifierVectorView(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ResourceQualifierVectorView {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<ResourceQualifier>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<ResourceQualifier>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<ResourceQualifier>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ResourceQualifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<ResourceQualifier>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ResourceQualifier>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ResourceQualifier>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ResourceQualifierVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ResourceQualifierVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ResourceQualifierVectorView {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ResourceQualifierVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierVectorView").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ResourceQualifierVectorView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ResourceQualifierVectorView {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<ResourceQualifier>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<ResourceQualifier> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ResourceQualifierVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceQualifierVectorView> for ::windows_core::IUnknown {
    fn from(value: ResourceQualifierVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceQualifierVectorView> for ::windows_core::IUnknown {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ResourceQualifierVectorView> for ::windows_core::IInspectable {
    fn from(value: ResourceQualifierVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ResourceQualifierVectorView> for ::windows_core::IInspectable {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierVectorView> for ::winrt_foundation::Collections::IIterable<ResourceQualifier> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierVectorView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierVectorView> for ::winrt_foundation::Collections::IIterable<ResourceQualifier> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierVectorView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ResourceQualifier>> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ResourceQualifier>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ResourceQualifier>> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ResourceQualifier>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<ResourceQualifier>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ResourceQualifierVectorView> for ::winrt_foundation::Collections::IVectorView<ResourceQualifier> {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceQualifierVectorView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ResourceQualifierVectorView> for ::winrt_foundation::Collections::IVectorView<ResourceQualifier> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceQualifierVectorView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<ResourceQualifier>> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<ResourceQualifier>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<ResourceQualifier>> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<ResourceQualifier>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<ResourceQualifier>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ResourceQualifierVectorView {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ResourceQualifierVectorView {}
