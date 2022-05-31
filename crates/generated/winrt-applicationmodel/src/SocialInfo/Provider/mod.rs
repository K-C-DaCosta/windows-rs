#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialDashboardItemUpdater(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cde9dc9_4800_46cd_869b_1973ec685bde);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialDashboardItemUpdater_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub OwnerRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OwnerRemoteId: usize,
    #[cfg(feature = "winrt-")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Content: usize,
    #[cfg(feature = "winrt-")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Timestamp: usize,
    #[cfg(feature = "winrt-")]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTimestamp: usize,
    #[cfg(feature = "winrt-")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetThumbnail: usize,
    #[cfg(feature = "winrt-")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Thumbnail: usize,
    #[cfg(feature = "winrt-")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CommitAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialFeedUpdater(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a0c0aa7_ed89_4bd5_a8d9_15f4d9861c10);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedUpdater_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub OwnerRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OwnerRemoteId: usize,
    #[cfg(feature = "winrt-")]
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SocialFeedKind) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Kind: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Items: usize,
    #[cfg(feature = "winrt-")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CommitAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialInfoProviderManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialInfoProviderManagerStatics {
    type Vtable = ISocialInfoProviderManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b88e52b_7787_48d6_aa12_d8e8f47ab85a);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialInfoProviderManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateSocialFeedUpdaterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateSocialFeedUpdaterAsync: usize,
    #[cfg(feature = "winrt-")]
    pub CreateDashboardItemUpdaterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownerremoteid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateDashboardItemUpdaterAsync: usize,
    #[cfg(feature = "winrt-")]
    pub UpdateBadgeCountValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemremoteid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newcount: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    UpdateBadgeCountValue: usize,
    #[cfg(feature = "winrt-")]
    pub ReportNewContentAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactremoteid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, kind: super::SocialFeedKind) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ReportNewContentAvailable: usize,
    #[cfg(feature = "winrt-")]
    pub ProvisionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ProvisionAsync: usize,
    #[cfg(feature = "winrt-")]
    pub DeprovisionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeprovisionAsync: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialDashboardItemUpdater(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialDashboardItemUpdater {
    #[cfg(feature = "winrt-")]
    pub fn OwnerRemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerRemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Content(&self) -> ::windows_core::Result<super::SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTimestamp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestamp)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetThumbnail<'a, Param0: ::windows_core::IntoParam<'a, super::SocialItemThumbnail>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::SocialItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SocialItemThumbnail>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CommitAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommitAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialDashboardItemUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialDashboardItemUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialDashboardItemUpdater {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialDashboardItemUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialDashboardItemUpdater").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialDashboardItemUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater;{3cde9dc9-4800-46cd-869b-1973ec685bde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_Vtbl;
    const IID: ::windows_core::GUID = <ISocialDashboardItemUpdater as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialDashboardItemUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialDashboardItemUpdater> for ::windows_core::IUnknown {
    fn from(value: SocialDashboardItemUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialDashboardItemUpdater> for ::windows_core::IUnknown {
    fn from(value: &SocialDashboardItemUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialDashboardItemUpdater> for ::windows_core::IInspectable {
    fn from(value: SocialDashboardItemUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialDashboardItemUpdater> for ::windows_core::IInspectable {
    fn from(value: &SocialDashboardItemUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialDashboardItemUpdater {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialDashboardItemUpdater {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialFeedUpdater(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialFeedUpdater {
    #[cfg(feature = "winrt-")]
    pub fn OwnerRemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OwnerRemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Kind(&self) -> ::windows_core::Result<super::SocialFeedKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::SocialFeedKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SocialFeedKind>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::SocialFeedItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::SocialFeedItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CommitAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommitAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialFeedUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialFeedUpdater {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedUpdater").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater;{7a0c0aa7-ed89-4bd5-a8d9-15f4d9861c10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_Vtbl;
    const IID: ::windows_core::GUID = <ISocialFeedUpdater as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialFeedUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedUpdater> for ::windows_core::IUnknown {
    fn from(value: SocialFeedUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedUpdater> for ::windows_core::IUnknown {
    fn from(value: &SocialFeedUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialFeedUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialFeedUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedUpdater> for ::windows_core::IInspectable {
    fn from(value: SocialFeedUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedUpdater> for ::windows_core::IInspectable {
    fn from(value: &SocialFeedUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialFeedUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialFeedUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialFeedUpdater {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialFeedUpdater {}
#[cfg(feature = "winrt-")]
pub struct SocialInfoProviderManager;
#[cfg(feature = "winrt-")]
impl SocialInfoProviderManager {
    #[cfg(feature = "winrt-")]
    pub fn CreateSocialFeedUpdaterAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SocialFeedUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSocialFeedUpdaterAsync)(::windows_core::Interface::as_raw(this), kind, mode, ownerremoteid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SocialFeedUpdater>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateDashboardItemUpdaterAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(ownerremoteid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SocialDashboardItemUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDashboardItemUpdaterAsync)(::windows_core::Interface::as_raw(this), ownerremoteid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SocialDashboardItemUpdater>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn UpdateBadgeCountValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(itemremoteid: Param0, newcount: i32) -> ::windows_core::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UpdateBadgeCountValue)(::windows_core::Interface::as_raw(this), itemremoteid.into_param().abi(), newcount).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn ReportNewContentAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contactremoteid: Param0, kind: super::SocialFeedKind) -> ::windows_core::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ReportNewContentAvailable)(::windows_core::Interface::as_raw(this), contactremoteid.into_param().abi(), kind).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn ProvisionAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn DeprovisionAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeprovisionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ISocialInfoProviderManagerStatics<R, F: FnOnce(&ISocialInfoProviderManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SocialInfoProviderManager, ISocialInfoProviderManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialInfoProviderManager {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager";
}
