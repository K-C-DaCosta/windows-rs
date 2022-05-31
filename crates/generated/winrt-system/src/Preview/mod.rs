#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HingeState(pub i32);
#[cfg(feature = "winrt-")]
impl HingeState {
    pub const Unknown: Self = Self(0i32);
    pub const Closed: Self = Self(1i32);
    pub const Concave: Self = Self(2i32);
    pub const Flat: Self = Self(3i32);
    pub const Convex: Self = Self(4i32);
    pub const Full: Self = Self(5i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for HingeState {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for HingeState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for HingeState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for HingeState {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for HingeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeState").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for HingeState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Preview.HingeState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ITwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72245c31_4b39_42a6_8e73_7235ade16853);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub GetCurrentPostureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetCurrentPostureAsync: usize,
    #[cfg(feature = "winrt-")]
    pub PostureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PostureChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemovePostureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemovePostureChanged: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReading(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ITwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0251452_4ad6_4b38_8426_c59a15493a7d);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Timestamp: usize,
    #[cfg(feature = "winrt-")]
    pub HingeState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HingeState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    HingeState: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub Panel1Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Sensors::SimpleOrientation) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-")))]
    Panel1Orientation: usize,
    #[cfg(feature = "winrt-")]
    pub Panel1Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Panel1Id: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub Panel2Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Sensors::SimpleOrientation) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-")))]
    Panel2Orientation: usize,
    #[cfg(feature = "winrt-")]
    pub Panel2Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Panel2Id: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d2d1bc6_02ce_474a_a556_a75b1cf93a03);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Reading: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ITwoPanelHingedDevicePosturePreviewStatics {
    type Vtable = ITwoPanelHingedDevicePosturePreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c4733d2_57e0_4180_bd5e_f31a2138423e);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDefaultAsync: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl TwoPanelHingedDevicePosturePreview {
    #[cfg(feature = "winrt-")]
    pub fn GetCurrentPostureAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentPostureAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PostureChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PostureChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemovePostureChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePostureChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>> {
        Self::ITwoPanelHingedDevicePosturePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ITwoPanelHingedDevicePosturePreviewStatics<R, F: FnOnce(&ITwoPanelHingedDevicePosturePreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TwoPanelHingedDevicePosturePreview, ITwoPanelHingedDevicePosturePreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for TwoPanelHingedDevicePosturePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for TwoPanelHingedDevicePosturePreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for TwoPanelHingedDevicePosturePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TwoPanelHingedDevicePosturePreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for TwoPanelHingedDevicePosturePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreview;{72245c31-4b39-42a6-8e73-7235ade16853})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for TwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_Vtbl;
    const IID: ::windows_core::GUID = <ITwoPanelHingedDevicePosturePreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for TwoPanelHingedDevicePosturePreview {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows_core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows_core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows_core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows_core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreview {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreview {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReading(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl TwoPanelHingedDevicePosturePreviewReading {
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn HingeState(&self) -> ::windows_core::Result<HingeState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HingeState>::zeroed();
            (::windows_core::Interface::vtable(this).HingeState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HingeState>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub fn Panel1Orientation(&self) -> ::windows_core::Result<::winrt_devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Sensors::SimpleOrientation>::zeroed();
            (::windows_core::Interface::vtable(this).Panel1Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Panel1Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Panel1Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub fn Panel2Orientation(&self) -> ::windows_core::Result<::winrt_devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Sensors::SimpleOrientation>::zeroed();
            (::windows_core::Interface::vtable(this).Panel2Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Panel2Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Panel2Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreviewReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for TwoPanelHingedDevicePosturePreviewReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for TwoPanelHingedDevicePosturePreviewReading {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for TwoPanelHingedDevicePosturePreviewReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TwoPanelHingedDevicePosturePreviewReading").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for TwoPanelHingedDevicePosturePreviewReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading;{a0251452-4ad6-4b38-8426-c59a15493a7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for TwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_Vtbl;
    const IID: ::windows_core::GUID = <ITwoPanelHingedDevicePosturePreviewReading as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for TwoPanelHingedDevicePosturePreviewReading {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows_core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows_core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows_core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows_core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreviewReading {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreviewReading {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn Reading(&self) -> ::windows_core::Result<TwoPanelHingedDevicePosturePreviewReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TwoPanelHingedDevicePosturePreviewReading>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;{2d2d1bc6-02ce-474a-a556-a75b1cf93a03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
