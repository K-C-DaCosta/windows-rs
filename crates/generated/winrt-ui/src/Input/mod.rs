#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Inking")]
pub mod Inking;
#[cfg(feature = "Preview")]
pub mod Preview;
#[cfg(feature = "Spatial")]
pub mod Spatial;
#[repr(transparent)]
pub struct AttachableInputObject(::windows_core::IUnknown);
impl AttachableInputObject {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AttachableInputObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AttachableInputObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AttachableInputObject {}
impl ::core::fmt::Debug for AttachableInputObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AttachableInputObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AttachableInputObject {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.AttachableInputObject;{9b822734-a3c1-542a-b2f4-0e32b773fb07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AttachableInputObject {
    type Vtable = IAttachableInputObject_Vtbl;
    const IID: ::windows_core::GUID = <IAttachableInputObject as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AttachableInputObject {
    const NAME: &'static str = "Windows.UI.Input.AttachableInputObject";
}
impl ::core::convert::From<AttachableInputObject> for ::windows_core::IUnknown {
    fn from(value: AttachableInputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AttachableInputObject> for ::windows_core::IUnknown {
    fn from(value: &AttachableInputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AttachableInputObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AttachableInputObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AttachableInputObject> for ::windows_core::IInspectable {
    fn from(value: AttachableInputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AttachableInputObject> for ::windows_core::IInspectable {
    fn from(value: &AttachableInputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AttachableInputObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AttachableInputObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AttachableInputObject> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AttachableInputObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AttachableInputObject> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AttachableInputObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AttachableInputObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AttachableInputObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AttachableInputObject {}
unsafe impl ::core::marker::Sync for AttachableInputObject {}
#[repr(C)]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl ::core::marker::Copy for CrossSlideThresholds {}
impl ::core::clone::Clone for CrossSlideThresholds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CrossSlideThresholds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CrossSlideThresholds").field("SelectionStart", &self.SelectionStart).field("SpeedBumpStart", &self.SpeedBumpStart).field("SpeedBumpEnd", &self.SpeedBumpEnd).field("RearrangeStart", &self.RearrangeStart).finish()
    }
}
unsafe impl ::windows_core::Abi for CrossSlideThresholds {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CrossSlideThresholds {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CrossSlideThresholds>()) == 0 }
    }
}
impl ::core::cmp::Eq for CrossSlideThresholds {}
impl ::core::default::Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct CrossSlidingEventArgs(::windows_core::IUnknown);
impl CrossSlidingEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn CrossSlidingState(&self) -> ::windows_core::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CrossSlidingState>::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlidingState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CrossSlidingState>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICrossSlidingEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for CrossSlidingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CrossSlidingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CrossSlidingEventArgs {}
impl ::core::fmt::Debug for CrossSlidingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.CrossSlidingEventArgs;{e9374738-6f88-41d9-8720-78e08e398349})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICrossSlidingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.CrossSlidingEventArgs";
}
impl ::core::convert::From<CrossSlidingEventArgs> for ::windows_core::IUnknown {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CrossSlidingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CrossSlidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CrossSlidingEventArgs> for ::windows_core::IInspectable {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CrossSlidingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CrossSlidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for CrossSlidingState {}
impl ::core::clone::Clone for CrossSlidingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CrossSlidingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CrossSlidingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CrossSlidingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CrossSlidingState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.CrossSlidingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DraggingEventArgs(::windows_core::IUnknown);
impl DraggingEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn DraggingState(&self) -> ::windows_core::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DraggingState>::zeroed();
            (::windows_core::Interface::vtable(this).DraggingState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DraggingState>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IDraggingEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for DraggingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DraggingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DraggingEventArgs {}
impl ::core::fmt::Debug for DraggingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.DraggingEventArgs;{1c905384-083c-4bd3-b559-179cddeb33ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDraggingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.DraggingEventArgs";
}
impl ::core::convert::From<DraggingEventArgs> for ::windows_core::IUnknown {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DraggingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DraggingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DraggingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DraggingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DraggingEventArgs> for ::windows_core::IInspectable {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DraggingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DraggingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DraggingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DraggingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for DraggingState {}
impl ::core::clone::Clone for DraggingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DraggingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DraggingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DraggingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DraggingState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.DraggingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EdgeGesture(::windows_core::IUnknown);
impl EdgeGesture {
    pub fn Starting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Starting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Completed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Canceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<EdgeGesture> {
        Self::IEdgeGestureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EdgeGesture>(result__)
        })
    }
    pub fn IEdgeGestureStatics<R, F: FnOnce(&IEdgeGestureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EdgeGesture, IEdgeGestureStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EdgeGesture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EdgeGesture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EdgeGesture {}
impl ::core::fmt::Debug for EdgeGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeGesture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EdgeGesture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.EdgeGesture;{580d5292-2ab1-49aa-a7f0-33bd3f8df9f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EdgeGesture {
    type Vtable = IEdgeGesture_Vtbl;
    const IID: ::windows_core::GUID = <IEdgeGesture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EdgeGesture {
    const NAME: &'static str = "Windows.UI.Input.EdgeGesture";
}
impl ::core::convert::From<EdgeGesture> for ::windows_core::IUnknown {
    fn from(value: EdgeGesture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGesture> for ::windows_core::IUnknown {
    fn from(value: &EdgeGesture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EdgeGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EdgeGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeGesture> for ::windows_core::IInspectable {
    fn from(value: EdgeGesture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGesture> for ::windows_core::IInspectable {
    fn from(value: &EdgeGesture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EdgeGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EdgeGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct EdgeGestureEventArgs(::windows_core::IUnknown);
impl EdgeGestureEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<EdgeGestureKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EdgeGestureKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EdgeGestureKind>(result__)
        }
    }
}
impl ::core::clone::Clone for EdgeGestureEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EdgeGestureEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EdgeGestureEventArgs {}
impl ::core::fmt::Debug for EdgeGestureEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeGestureEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EdgeGestureEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.EdgeGestureEventArgs;{44fa4a24-2d09-42e1-8b5e-368208796a4c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IEdgeGestureEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EdgeGestureEventArgs {
    const NAME: &'static str = "Windows.UI.Input.EdgeGestureEventArgs";
}
impl ::core::convert::From<EdgeGestureEventArgs> for ::windows_core::IUnknown {
    fn from(value: EdgeGestureEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGestureEventArgs> for ::windows_core::IUnknown {
    fn from(value: &EdgeGestureEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EdgeGestureEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EdgeGestureEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeGestureEventArgs> for ::windows_core::IInspectable {
    fn from(value: EdgeGestureEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGestureEventArgs> for ::windows_core::IInspectable {
    fn from(value: &EdgeGestureEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EdgeGestureEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EdgeGestureEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for EdgeGestureKind {}
impl ::core::clone::Clone for EdgeGestureKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EdgeGestureKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EdgeGestureKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EdgeGestureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeGestureKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EdgeGestureKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.EdgeGestureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for GazeInputAccessStatus {}
impl ::core::clone::Clone for GazeInputAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GazeInputAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GazeInputAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GazeInputAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeInputAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeInputAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GazeInputAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GestureRecognizer(::windows_core::IUnknown);
impl GestureRecognizer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GestureRecognizer, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GestureSettings(&self) -> ::windows_core::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GestureSettings>::zeroed();
            (::windows_core::Interface::vtable(this).GestureSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GestureSettings>(result__)
        }
    }
    pub fn SetGestureSettings(&self, value: GestureSettings) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGestureSettings)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsInertial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInertial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ShowGestureFeedback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowGestureFeedback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShowGestureFeedback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowGestureFeedback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PivotCenter(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).PivotCenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPivotCenter<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPivotCenter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PivotRadius(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PivotRadius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPivotRadius(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPivotRadius)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaTranslationDeceleration(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).InertiaTranslationDeceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInertiaTranslationDeceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaRotationDeceleration(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).InertiaRotationDeceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInertiaRotationDeceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaExpansionDeceleration(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).InertiaExpansionDeceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInertiaExpansionDeceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaTranslationDisplacement(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).InertiaTranslationDisplacement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInertiaTranslationDisplacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaRotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).InertiaRotationAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInertiaRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaExpansion(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).InertiaExpansion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaExpansion(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInertiaExpansion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ManipulationExact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationExact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetManipulationExact(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetManipulationExact)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrossSlideThresholds(&self) -> ::windows_core::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CrossSlideThresholds>::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlideThresholds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CrossSlideThresholds>(result__)
        }
    }
    pub fn SetCrossSlideThresholds<'a, Param0: ::windows_core::IntoParam<'a, CrossSlideThresholds>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCrossSlideThresholds)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CrossSlideHorizontally(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlideHorizontally)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCrossSlideHorizontally)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrossSlideExact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlideExact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCrossSlideExact(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCrossSlideExact)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoProcessInertia(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoProcessInertia)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoProcessInertia(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoProcessInertia)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MouseWheelParameters(&self) -> ::windows_core::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MouseWheelParameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MouseWheelParameters>(result__)
        }
    }
    pub fn CanBeDoubleTap<'a, Param0: ::windows_core::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanBeDoubleTap)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ProcessDownEvent<'a, Param0: ::windows_core::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessDownEvent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ProcessMoveEvents<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PointerPoint>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessMoveEvents)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProcessUpEvent<'a, Param0: ::windows_core::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessUpEvent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProcessMouseWheelEvent<'a, Param0: ::windows_core::IntoParam<'a, PointerPoint>>(&self, value: Param0, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessMouseWheelEvent)(::windows_core::Interface::as_raw(this), value.into_param().abi(), isshiftkeydown, iscontrolkeydown).ok() }
    }
    pub fn ProcessInertia(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessInertia)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CompleteGesture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CompleteGesture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Tapped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Tapped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTapped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTapped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RightTapped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RightTapped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRightTapped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRightTapped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Holding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Holding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHolding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHolding)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Dragging<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Dragging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragging<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDragging)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ManipulationStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ManipulationUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ManipulationInertiaStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationInertiaStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ManipulationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CrossSliding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CrossSliding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCrossSliding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCrossSliding)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TapMinContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TapMinContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTapMinContactCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTapMinContactCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TapMaxContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TapMaxContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTapMaxContactCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTapMaxContactCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldMinContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HoldMinContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetHoldMinContactCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetHoldMinContactCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldMaxContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HoldMaxContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetHoldMaxContactCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetHoldMaxContactCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldRadius(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).HoldRadius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetHoldRadius(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetHoldRadius)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldStartDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).HoldStartDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetHoldStartDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetHoldStartDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TranslationMinContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TranslationMinContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTranslationMinContactCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslationMinContactCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslationMaxContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TranslationMaxContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTranslationMaxContactCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslationMaxContactCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for GestureRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GestureRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GestureRecognizer {}
impl ::core::fmt::Debug for GestureRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GestureRecognizer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.GestureRecognizer;{b47a37bf-3d6b-4f88-83e8-6dcb4012ffb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows_core::GUID = <IGestureRecognizer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.GestureRecognizer";
}
impl ::core::convert::From<GestureRecognizer> for ::windows_core::IUnknown {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GestureRecognizer> for ::windows_core::IUnknown {
    fn from(value: &GestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GestureRecognizer> for ::windows_core::IInspectable {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GestureRecognizer> for ::windows_core::IInspectable {
    fn from(value: &GestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
impl ::core::marker::Copy for GestureSettings {}
impl ::core::clone::Clone for GestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GestureSettings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GestureSettings {
    type Abi = Self;
}
impl ::core::fmt::Debug for GestureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureSettings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for GestureSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GestureSettings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HoldingEventArgs(::windows_core::IUnknown);
impl HoldingEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn HoldingState(&self) -> ::windows_core::Result<HoldingState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HoldingState>::zeroed();
            (::windows_core::Interface::vtable(this).HoldingState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HoldingState>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CurrentContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for HoldingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingEventArgs {}
impl ::core::fmt::Debug for HoldingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.HoldingEventArgs;{2bf755c5-e799-41b4-bb40-242f40959b71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHoldingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.HoldingEventArgs";
}
impl ::core::convert::From<HoldingEventArgs> for ::windows_core::IUnknown {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingEventArgs> for ::windows_core::IInspectable {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for HoldingState {}
impl ::core::clone::Clone for HoldingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HoldingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HoldingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for HoldingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HoldingState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.HoldingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAttachableInputObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAttachableInputObject {
    type Vtable = IAttachableInputObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b822734_a3c1_542a_b2f4_0e32b773fb07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttachableInputObject_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAttachableInputObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAttachableInputObjectFactory {
    type Vtable = IAttachableInputObjectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4c54c4e_42bc_58fa_a640_ea1516f4c06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttachableInputObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICrossSlidingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9374738_6f88_41d9_8720_78e08e398349);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub CrossSlidingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CrossSlidingState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICrossSlidingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICrossSlidingEventArgs2 {
    type Vtable = ICrossSlidingEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeefb7d48_c070_59f3_8dab_bcaf621d8687);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDraggingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c905384_083c_4bd3_b559_179cddeb33ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub DraggingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DraggingState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDraggingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDraggingEventArgs2 {
    type Vtable = IDraggingEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71efdbf9_382a_55ca_b4b9_008123c1bf1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeGesture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEdgeGesture {
    type Vtable = IEdgeGesture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x580d5292_2ab1_49aa_a7f0_33bd3f8df9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGesture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Starting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeGestureEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44fa4a24_2d09_42e1_8b5e_368208796a4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGestureEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EdgeGestureKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeGestureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEdgeGestureStatics {
    type Vtable = IEdgeGestureStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc6a8519_18ee_4043_9839_4fc584d60a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGestureStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGestureRecognizer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb47a37bf_3d6b_4f88_83e8_6dcb4012ffb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GestureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GestureSettings) -> ::windows_core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GestureSettings) -> ::windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PivotCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPivotCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CrossSlideThresholds) -> ::windows_core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CrossSlideThresholds) -> ::windows_core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutoProcessInertia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ProcessMoveEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProcessMoveEvents: usize,
    pub ProcessUpEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows_core::HRESULT,
    pub ProcessInertia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CompleteGesture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Tapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Holding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Dragging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDragging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ManipulationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveManipulationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ManipulationInertiaStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CrossSliding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCrossSliding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGestureRecognizer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGestureRecognizer2 {
    type Vtable = IGestureRecognizer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd646097f_6ef7_5746_8ba8_8ff2206e6f3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TapMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTapMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub TapMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTapMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HoldMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHoldMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HoldMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHoldMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HoldRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetHoldRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub HoldStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetHoldStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub TranslationMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTranslationMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub TranslationMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTranslationMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bf755c5_e799_41b4_bb40_242f40959b71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub HoldingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HoldingState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHoldingEventArgs2 {
    type Vtable = IHoldingEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x141da9ea_4c79_5674_afea_493fdeb91f19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d6d4ed2_28c7_5ae3_aa74_c918a9f243ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows_core::HRESULT,
    pub InputActivationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInputActivationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListenerActivationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7699b465_1dcf_5791_b4b9_6cafbeed2056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4baf068_8f49_446c_8db5_8c0ffe85cc9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    KeyDown: usize,
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    KeyUp: usize,
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardDeliveryInterceptorStatics {
    type Vtable = IKeyboardDeliveryInterceptorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9f63ba2_ceba_4755_8a7e_14c0ffecd239);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb34ab22b_d19b_46ff_9f38_dec7754bb9e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationCompletedEventArgs2 {
    type Vtable = IManipulationCompletedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0c0dce7_30a9_5b96_886f_6560a85e4757);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd37a898_26bf_467a_9ce5_ccf3fb11371e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationInertiaStartingEventArgs2 {
    type Vtable = IManipulationInertiaStartingEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc25409b8_f9fa_5a45_bd97_dcbbb2201860);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddec873e_cfce_4932_8c1d_3c3d011a34c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedEventArgs2 {
    type Vtable = IManipulationStartedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2da3db4e_e583_5055_afaa_16fd986531a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb354ce5_abb8_4f9f_b3ce_8181aa61ad82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationUpdatedEventArgs2 {
    type Vtable = IManipulationUpdatedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3dfb96a_3306_5903_a1c5_ff9757a8689e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseWheelParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xead0ca44_9ded_4037_8149_5e4cc2564468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CharTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetCharTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub DeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub PageTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPageTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe995317d_7296_42d9_8233_c5be73b74a4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDevice: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub RawPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub PointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc79d8a4b_c163_4ee7_803f_67ce79f9972d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub XTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub YTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub ContactRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub ContactRectRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub TouchConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PointerUpdateKind) -> ::windows_core::HRESULT,
    pub HasUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetUsageValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointProperties2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPointProperties2 {
    type Vtable = IPointerPointProperties2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22c3433a_c83b_41c0_a296_5e232d64d6af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ZDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPointStatics {
    type Vtable = IPointerPointStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa506638d_2a1a_413e_bc75_9f38381cc069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetIntermediatePoints: usize,
    pub GetCurrentPointTransformed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetIntermediatePointsTransformed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetIntermediatePointsTransformed: usize,
}
#[repr(transparent)]
pub struct IPointerPointTransform(::windows_core::IUnknown);
impl IPointerPointTransform {
    pub fn Inverse(&self) -> ::windows_core::Result<IPointerPointTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPointerPointTransform>(result__)
        }
    }
    pub fn TryTransform<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, inpoint: Param0, outpoint: &mut ::winrt_foundation::Point) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(::windows_core::Interface::as_raw(this), inpoint.into_param().abi(), outpoint, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TransformBounds<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, rect: Param0) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(::windows_core::Interface::as_raw(this), rect.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
}
impl ::core::convert::From<IPointerPointTransform> for ::windows_core::IUnknown {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPointerPointTransform> for ::windows_core::IUnknown {
    fn from(value: &IPointerPointTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPointerPointTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPointerPointTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPointerPointTransform> for ::windows_core::IInspectable {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPointerPointTransform> for ::windows_core::IInspectable {
    fn from(value: &IPointerPointTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPointerPointTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPointerPointTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPointerPointTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPointerPointTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPointerPointTransform {}
impl ::core::fmt::Debug for IPointerPointTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPointerPointTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4d5fe14f-b87c-4028-bc9c-59e9947fb056}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPointerPointTransform {
    type Vtable = IPointerPointTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d5fe14f_b87c_4028_bc9c_59e9947fb056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Inverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inpoint: ::winrt_foundation::Point, outpoint: *mut ::winrt_foundation::Point, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TransformBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: ::winrt_foundation::Rect, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerVisualizationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d1e6461_84f7_499d_bd91_2a36e2b7aaa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerVisualizationSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerVisualizationSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerVisualizationSettingsStatics {
    type Vtable = IPointerVisualizationSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68870edb_165b_4214_b4f3_584eca8c8a69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerVisualizationSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialController {
    type Vtable = IRadialController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3055d1c8_df51_43d4_b23b_0e1037467a09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Menu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationResolutionInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationResolutionInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub UseAutomaticHapticFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetUseAutomaticHapticFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ScreenContactStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScreenContactStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ScreenContactEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScreenContactEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ScreenContactContinued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScreenContactContinued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RotationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRotationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ControlAcquired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveControlAcquired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialController2 {
    type Vtable = IRadialController2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialController2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ButtonHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveButtonHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x206aa438_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerButtonClickedEventArgs2 {
    type Vtable = IRadialControllerButtonClickedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef3_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonHoldingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577eee_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonHoldingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577eed_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonReleasedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577eef_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonReleasedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6b79ecb_6a52_4430_910c_56370a9d6b42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SetDefaultMenuItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttons: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetDefaultMenuItems: usize,
    pub ResetToDefaultMenuItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TrySelectDefaultMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: RadialControllerSystemMenuItemKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerConfiguration2 {
    type Vtable = IRadialControllerConfiguration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef7_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIsMenuSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMenuSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerConfigurationStatics {
    type Vtable = IRadialControllerConfigurationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79b6b0e5_069a_4486_a99d_8db772b9642f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerConfigurationStatics2 {
    type Vtable = IRadialControllerConfigurationStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53e08b17_e205_48d3_9caf_80ff47c4d7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAppController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIsAppControllerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAppControllerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x206aa439_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerControlAcquiredEventArgs2 {
    type Vtable = IRadialControllerControlAcquiredEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef4_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenu(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerMenu {
    type Vtable = IRadialControllerMenu_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8506b35d_f640_4412_aba0_bad077e5ea8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenu_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Items: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetSelectedMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitem: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySelectPreviouslySelectedMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenuItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc80fc98d_ad0b_4c9c_8f2f_136a2373a6ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerMenuItemStatics {
    type Vtable = IRadialControllerMenuItemStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x249e0887_d842_4524_9df8_e0d647edc887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, icon: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromIcon: usize,
    pub CreateFromKnownIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: RadialControllerMenuKnownIcon, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerMenuItemStatics2 {
    type Vtable = IRadialControllerMenuItemStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cbb70be_7e3e_48bd_be04_2c7fcaa9c1ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromFontGlyph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromFontGlyphWithUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, fonturi: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x206aa435_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RotationDeltaInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerRotationChangedEventArgs2 {
    type Vtable = IRadialControllerRotationChangedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577eec_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContact(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x206aa434_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContact_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x206aa437_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerScreenContactContinuedEventArgs2 {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef1_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactEndedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef2_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactEndedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x206aa436_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerScreenContactStartedEventArgs2 {
    type Vtable = IRadialControllerScreenContactStartedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef0_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerStatics {
    type Vtable = IRadialControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaded0b7_b84c_4894_87aa_8f25aa5f288b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CreateForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cbf40bd_af7a_4a36_9476_b1dce141709a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRightTappedEventArgs2 {
    type Vtable = IRightTappedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61c7b7bb_9f57_5857_a33c_c58c3dfa959e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemButtonEventController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemButtonEventController {
    type Vtable = ISystemButtonEventController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59b893a9_73bc_52b5_ba41_82511b2cb46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemButtonEventController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SystemFunctionButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSystemFunctionButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SystemFunctionButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSystemFunctionButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SystemFunctionLockChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSystemFunctionLockChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SystemFunctionLockIndicatorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSystemFunctionLockIndicatorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemButtonEventControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemButtonEventControllerStatics {
    type Vtable = ISystemButtonEventControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x632fb07b_20bd_5e15_af4a_00dbf2064ffa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemButtonEventControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub CreateForDispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queue: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    CreateForDispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemFunctionButtonEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4833896f_80d1_5dd6_92a7_62a508ffef5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionButtonEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemFunctionLockChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd040608_fcf9_585c_beab_f1d2eaf364ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionLockChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IsLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb212b94e_7a6f_58ae_b304_bae61d0371b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IsIndicatorOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfa126e4_253a_4c3c_953b_395c37aed309);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub TapCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITappedEventArgs2 {
    type Vtable = ITappedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x294388f2_177e_51d5_be56_ee0866fa968c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct InputActivationListener(::windows_core::IUnknown);
impl InputActivationListener {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InputActivationState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InputActivationState>(result__)
        }
    }
    pub fn InputActivationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).InputActivationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInputActivationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInputActivationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for InputActivationListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputActivationListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputActivationListener {}
impl ::core::fmt::Debug for InputActivationListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputActivationListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.InputActivationListener;{5d6d4ed2-28c7-5ae3-aa74-c918a9f243ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
    const IID: ::windows_core::GUID = <IInputActivationListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListener";
}
impl ::core::convert::From<InputActivationListener> for ::windows_core::IUnknown {
    fn from(value: InputActivationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListener> for ::windows_core::IUnknown {
    fn from(value: &InputActivationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputActivationListener> for ::windows_core::IInspectable {
    fn from(value: InputActivationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListener> for ::windows_core::IInspectable {
    fn from(value: &InputActivationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InputActivationListener> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InputActivationListener) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InputActivationListener> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InputActivationListener) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InputActivationListener> for AttachableInputObject {
    fn from(value: InputActivationListener) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputActivationListener> for AttachableInputObject {
    fn from(value: &InputActivationListener) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, AttachableInputObject> for InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, AttachableInputObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, AttachableInputObject> for &InputActivationListener {
    fn into_param(self) -> ::windows_core::Param<'a, AttachableInputObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<AttachableInputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputActivationListener {}
unsafe impl ::core::marker::Sync for InputActivationListener {}
#[repr(transparent)]
pub struct InputActivationListenerActivationChangedEventArgs(::windows_core::IUnknown);
impl InputActivationListenerActivationChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InputActivationState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InputActivationState>(result__)
        }
    }
}
impl ::core::clone::Clone for InputActivationListenerActivationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputActivationListenerActivationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputActivationListenerActivationChangedEventArgs {}
impl ::core::fmt::Debug for InputActivationListenerActivationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationListenerActivationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.InputActivationListenerActivationChangedEventArgs;{7699b465-1dcf-5791-b4b9-6cafbeed2056})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInputActivationListenerActivationChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
impl ::core::convert::From<InputActivationListenerActivationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: InputActivationListenerActivationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListenerActivationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &InputActivationListenerActivationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputActivationListenerActivationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: InputActivationListenerActivationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListenerActivationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &InputActivationListenerActivationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl ::core::marker::Sync for InputActivationListenerActivationChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for InputActivationState {}
impl ::core::clone::Clone for InputActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputActivationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InputActivationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputActivationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputActivationState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.InputActivationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct KeyboardDeliveryInterceptor(::windows_core::IUnknown);
impl KeyboardDeliveryInterceptor {
    pub fn IsInterceptionEnabledWhenInForeground(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInterceptionEnabledWhenInForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInterceptionEnabledWhenInForeground(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsInterceptionEnabledWhenInForeground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn KeyDown<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).KeyDown)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveKeyDown<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveKeyDown)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn KeyUp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).KeyUp)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveKeyUp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveKeyUp)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<KeyboardDeliveryInterceptor> {
        Self::IKeyboardDeliveryInterceptorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyboardDeliveryInterceptor>(result__)
        })
    }
    pub fn IKeyboardDeliveryInterceptorStatics<R, F: FnOnce(&IKeyboardDeliveryInterceptorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyboardDeliveryInterceptor, IKeyboardDeliveryInterceptorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyboardDeliveryInterceptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardDeliveryInterceptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardDeliveryInterceptor {}
impl ::core::fmt::Debug for KeyboardDeliveryInterceptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardDeliveryInterceptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyboardDeliveryInterceptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.KeyboardDeliveryInterceptor;{b4baf068-8f49-446c-8db5-8c0ffe85cc9e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_Vtbl;
    const IID: ::windows_core::GUID = <IKeyboardDeliveryInterceptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardDeliveryInterceptor {
    const NAME: &'static str = "Windows.UI.Input.KeyboardDeliveryInterceptor";
}
impl ::core::convert::From<KeyboardDeliveryInterceptor> for ::windows_core::IUnknown {
    fn from(value: KeyboardDeliveryInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardDeliveryInterceptor> for ::windows_core::IUnknown {
    fn from(value: &KeyboardDeliveryInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardDeliveryInterceptor> for ::windows_core::IInspectable {
    fn from(value: KeyboardDeliveryInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardDeliveryInterceptor> for ::windows_core::IInspectable {
    fn from(value: &KeyboardDeliveryInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyboardDeliveryInterceptor {}
unsafe impl ::core::marker::Sync for KeyboardDeliveryInterceptor {}
#[repr(transparent)]
pub struct ManipulationCompletedEventArgs(::windows_core::IUnknown);
impl ManipulationCompletedEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn Velocities(&self) -> ::windows_core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationVelocities>::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationVelocities>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CurrentContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedEventArgs {}
impl ::core::fmt::Debug for ManipulationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationCompletedEventArgs;{b34ab22b-d19b-46ff-9f38-dec7754bb9e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationCompletedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
pub struct ManipulationDelta {
    pub Translation: ::winrt_foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
impl ::core::marker::Copy for ManipulationDelta {}
impl ::core::clone::Clone for ManipulationDelta {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ManipulationDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationDelta").field("Translation", &self.Translation).field("Scale", &self.Scale).field("Rotation", &self.Rotation).field("Expansion", &self.Expansion).finish()
    }
}
unsafe impl ::windows_core::Abi for ManipulationDelta {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for ManipulationDelta {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ManipulationDelta>()) == 0 }
    }
}
impl ::core::cmp::Eq for ManipulationDelta {}
impl ::core::default::Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventArgs(::windows_core::IUnknown);
impl ManipulationInertiaStartingEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Delta(&self) -> ::windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn Velocities(&self) -> ::windows_core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationVelocities>::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationVelocities>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IManipulationInertiaStartingEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingEventArgs {}
impl ::core::fmt::Debug for ManipulationInertiaStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationInertiaStartingEventArgs;{dd37a898-26bf-467a-9ce5-ccf3fb11371e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationInertiaStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationInertiaStartingEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ManipulationStartedEventArgs(::windows_core::IUnknown);
impl ManipulationStartedEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IManipulationStartedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationStartedEventArgs;{ddec873e-cfce-4932-8c1d-3c3d011a34c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationStartedEventArgs";
}
impl ::core::convert::From<ManipulationStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ManipulationUpdatedEventArgs(::windows_core::IUnknown);
impl ManipulationUpdatedEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Delta(&self) -> ::windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn Velocities(&self) -> ::windows_core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationVelocities>::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationVelocities>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CurrentContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationUpdatedEventArgs {}
impl ::core::fmt::Debug for ManipulationUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationUpdatedEventArgs;{cb354ce5-abb8-4f9f-b3ce-8181aa61ad82})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationUpdatedEventArgs";
}
impl ::core::convert::From<ManipulationUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
pub struct ManipulationVelocities {
    pub Linear: ::winrt_foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
impl ::core::marker::Copy for ManipulationVelocities {}
impl ::core::clone::Clone for ManipulationVelocities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ManipulationVelocities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationVelocities").field("Linear", &self.Linear).field("Angular", &self.Angular).field("Expansion", &self.Expansion).finish()
    }
}
unsafe impl ::windows_core::Abi for ManipulationVelocities {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for ManipulationVelocities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ManipulationVelocities {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ManipulationVelocities>()) == 0 }
    }
}
impl ::core::cmp::Eq for ManipulationVelocities {}
impl ::core::default::Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct MouseWheelParameters(::windows_core::IUnknown);
impl MouseWheelParameters {
    pub fn CharTranslation(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).CharTranslation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetCharTranslation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharTranslation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DeltaScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).DeltaScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetDeltaScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeltaScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaRotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).DeltaRotationAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetDeltaRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeltaRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PageTranslation(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).PageTranslation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPageTranslation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPageTranslation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MouseWheelParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MouseWheelParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseWheelParameters {}
impl ::core::fmt::Debug for MouseWheelParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseWheelParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.MouseWheelParameters;{ead0ca44-9ded-4037-8149-5e4cc2564468})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: ::windows_core::GUID = <IMouseWheelParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Windows.UI.Input.MouseWheelParameters";
}
impl ::core::convert::From<MouseWheelParameters> for ::windows_core::IUnknown {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseWheelParameters> for ::windows_core::IUnknown {
    fn from(value: &MouseWheelParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MouseWheelParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MouseWheelParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MouseWheelParameters> for ::windows_core::IInspectable {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseWheelParameters> for ::windows_core::IInspectable {
    fn from(value: &MouseWheelParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MouseWheelParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MouseWheelParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PointerPoint(::windows_core::IUnknown);
impl PointerPoint {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDevice(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDevice>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn RawPosition(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).RawPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn PointerId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PointerId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn IsInContact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInContact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointerPointProperties>(result__)
        }
    }
    pub fn GetCurrentPoint(pointerid: u32) -> ::windows_core::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentPoint)(::windows_core::Interface::as_raw(this), pointerid, result__.as_mut_ptr()).from_abi::<PointerPoint>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetIntermediatePoints(pointerid: u32) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePoints)(::windows_core::Interface::as_raw(this), pointerid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<PointerPoint>>(result__)
        })
    }
    pub fn GetCurrentPointTransformed<'a, Param1: ::windows_core::IntoParam<'a, IPointerPointTransform>>(pointerid: u32, transform: Param1) -> ::windows_core::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentPointTransformed)(::windows_core::Interface::as_raw(this), pointerid, transform.into_param().abi(), result__.as_mut_ptr()).from_abi::<PointerPoint>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetIntermediatePointsTransformed<'a, Param1: ::windows_core::IntoParam<'a, IPointerPointTransform>>(pointerid: u32, transform: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePointsTransformed)(::windows_core::Interface::as_raw(this), pointerid, transform.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<PointerPoint>>(result__)
        })
    }
    pub fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerPoint, IPointerPointStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPoint {}
impl ::core::fmt::Debug for PointerPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerPoint;{e995317d-7296-42d9-8233-c5be73b74a4a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: ::windows_core::GUID = <IPointerPoint as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Windows.UI.Input.PointerPoint";
}
impl ::core::convert::From<PointerPoint> for ::windows_core::IUnknown {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPoint> for ::windows_core::IUnknown {
    fn from(value: &PointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerPoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerPoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPoint> for ::windows_core::IInspectable {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPoint> for ::windows_core::IInspectable {
    fn from(value: &PointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerPoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerPoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PointerPointProperties(::windows_core::IUnknown);
impl PointerPointProperties {
    pub fn Pressure(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Pressure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn IsInverted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInverted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsEraser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEraser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn XTilt(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).XTilt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn YTilt(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).YTilt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Twist(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Twist)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn ContactRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ContactRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn ContactRectRaw(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ContactRectRaw)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn TouchConfidence(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TouchConfidence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLeftButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLeftButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRightButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRightButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMiddleButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMiddleButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MouseWheelDelta(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MouseWheelDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsHorizontalMouseWheel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHorizontalMouseWheel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPrimary(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsInRange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCanceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBarrelButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBarrelButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsXButton1Pressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsXButton1Pressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsXButton2Pressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsXButton2Pressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> ::windows_core::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PointerUpdateKind>::zeroed();
            (::windows_core::Interface::vtable(this).PointerUpdateKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointerUpdateKind>(result__)
        }
    }
    pub fn HasUsage(&self, usagepage: u32, usageid: u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasUsage)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetUsageValue)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ZDistance(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = &::windows_core::Interface::cast::<IPointerPointProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ZDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerPointProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPointProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPointProperties {}
impl ::core::fmt::Debug for PointerPointProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPointProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerPointProperties;{c79d8a4b-c163-4ee7-803f-67ce79f9972d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: ::windows_core::GUID = <IPointerPointProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.PointerPointProperties";
}
impl ::core::convert::From<PointerPointProperties> for ::windows_core::IUnknown {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPointProperties> for ::windows_core::IUnknown {
    fn from(value: &PointerPointProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerPointProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerPointProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPointProperties> for ::windows_core::IInspectable {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPointProperties> for ::windows_core::IInspectable {
    fn from(value: &PointerPointProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerPointProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerPointProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl ::core::marker::Copy for PointerUpdateKind {}
impl ::core::clone::Clone for PointerUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PointerUpdateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PointerUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerUpdateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.PointerUpdateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PointerVisualizationSettings(::windows_core::IUnknown);
impl PointerVisualizationSettings {
    pub fn SetIsContactFeedbackEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsContactFeedbackEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsContactFeedbackEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsContactFeedbackEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsBarrelButtonFeedbackEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBarrelButtonFeedbackEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsBarrelButtonFeedbackEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBarrelButtonFeedbackEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<PointerVisualizationSettings> {
        Self::IPointerVisualizationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointerVisualizationSettings>(result__)
        })
    }
    pub fn IPointerVisualizationSettingsStatics<R, F: FnOnce(&IPointerVisualizationSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerVisualizationSettings, IPointerVisualizationSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerVisualizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerVisualizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerVisualizationSettings {}
impl ::core::fmt::Debug for PointerVisualizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerVisualizationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerVisualizationSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerVisualizationSettings;{4d1e6461-84f7-499d-bd91-2a36e2b7aaa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_Vtbl;
    const IID: ::windows_core::GUID = <IPointerVisualizationSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerVisualizationSettings {
    const NAME: &'static str = "Windows.UI.Input.PointerVisualizationSettings";
}
impl ::core::convert::From<PointerVisualizationSettings> for ::windows_core::IUnknown {
    fn from(value: PointerVisualizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerVisualizationSettings> for ::windows_core::IUnknown {
    fn from(value: &PointerVisualizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerVisualizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerVisualizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerVisualizationSettings> for ::windows_core::IInspectable {
    fn from(value: PointerVisualizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerVisualizationSettings> for ::windows_core::IInspectable {
    fn from(value: &PointerVisualizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerVisualizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerVisualizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PointerVisualizationSettings {}
unsafe impl ::core::marker::Sync for PointerVisualizationSettings {}
#[repr(transparent)]
pub struct RadialController(::windows_core::IUnknown);
impl RadialController {
    pub fn Menu(&self) -> ::windows_core::Result<RadialControllerMenu> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Menu)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerMenu>(result__)
        }
    }
    pub fn RotationResolutionInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationResolutionInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationResolutionInDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationResolutionInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UseAutomaticHapticFeedback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UseAutomaticHapticFeedback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseAutomaticHapticFeedback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUseAutomaticHapticFeedback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScreenContactStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenContactStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScreenContactStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScreenContactStarted)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn ScreenContactEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenContactEnded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScreenContactEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScreenContactEnded)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn ScreenContactContinued<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenContactContinued)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScreenContactContinued<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScreenContactContinued)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn ControlLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ControlLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveControlLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveControlLost)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn RotationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RotationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRotationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRotationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveButtonClicked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ControlAcquired<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ControlAcquired)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveControlAcquired<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveControlAcquired)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn ButtonPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveButtonPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveButtonPressed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ButtonHolding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonHolding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveButtonHolding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveButtonHolding)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ButtonReleased<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonReleased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveButtonReleased<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveButtonReleased)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CreateForCurrentView() -> ::windows_core::Result<RadialController> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialController>(result__)
        })
    }
    pub fn IRadialControllerStatics<R, F: FnOnce(&IRadialControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RadialController, IRadialControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialController {}
impl ::core::fmt::Debug for RadialController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialController;{3055d1c8-df51-43d4-b23b-0e1037467a09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialController {
    type Vtable = IRadialController_Vtbl;
    const IID: ::windows_core::GUID = <IRadialController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialController {
    const NAME: &'static str = "Windows.UI.Input.RadialController";
}
impl ::core::convert::From<RadialController> for ::windows_core::IUnknown {
    fn from(value: RadialController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialController> for ::windows_core::IUnknown {
    fn from(value: &RadialController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialController> for ::windows_core::IInspectable {
    fn from(value: RadialController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialController> for ::windows_core::IInspectable {
    fn from(value: &RadialController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialController {}
unsafe impl ::core::marker::Sync for RadialController {}
#[repr(transparent)]
pub struct RadialControllerButtonClickedEventArgs(::windows_core::IUnknown);
impl RadialControllerButtonClickedEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = &::windows_core::Interface::cast::<IRadialControllerButtonClickedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonClickedEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonClickedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerButtonClickedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonClickedEventArgs;{206aa438-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerButtonClickedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerButtonClickedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonClickedEventArgs";
}
impl ::core::convert::From<RadialControllerButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonClickedEventArgs {}
#[repr(transparent)]
pub struct RadialControllerButtonHoldingEventArgs(::windows_core::IUnknown);
impl RadialControllerButtonHoldingEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonHoldingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonHoldingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonHoldingEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonHoldingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonHoldingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerButtonHoldingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonHoldingEventArgs;{3d577eee-3cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerButtonHoldingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerButtonHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonHoldingEventArgs";
}
impl ::core::convert::From<RadialControllerButtonHoldingEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerButtonHoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonHoldingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerButtonHoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonHoldingEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerButtonHoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonHoldingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerButtonHoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonHoldingEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonHoldingEventArgs {}
#[repr(transparent)]
pub struct RadialControllerButtonPressedEventArgs(::windows_core::IUnknown);
impl RadialControllerButtonPressedEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonPressedEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonPressedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerButtonPressedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonPressedEventArgs;{3d577eed-4cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerButtonPressedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerButtonPressedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonPressedEventArgs";
}
impl ::core::convert::From<RadialControllerButtonPressedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonPressedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonPressedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonPressedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonPressedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonPressedEventArgs {}
#[repr(transparent)]
pub struct RadialControllerButtonReleasedEventArgs(::windows_core::IUnknown);
impl RadialControllerButtonReleasedEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonReleasedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonReleasedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonReleasedEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonReleasedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonReleasedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerButtonReleasedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonReleasedEventArgs;{3d577eef-3cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerButtonReleasedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerButtonReleasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonReleasedEventArgs";
}
impl ::core::convert::From<RadialControllerButtonReleasedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerButtonReleasedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonReleasedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerButtonReleasedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonReleasedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerButtonReleasedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonReleasedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerButtonReleasedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonReleasedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonReleasedEventArgs {}
#[repr(transparent)]
pub struct RadialControllerConfiguration(::windows_core::IUnknown);
impl RadialControllerConfiguration {
    #[cfg(feature = "winrt-foundation")]
    pub fn SetDefaultMenuItems<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>>(&self, buttons: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultMenuItems)(::windows_core::Interface::as_raw(this), buttons.into_param().abi()).ok() }
    }
    pub fn ResetToDefaultMenuItems(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResetToDefaultMenuItems)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TrySelectDefaultMenuItem(&self, r#type: RadialControllerSystemMenuItemKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySelectDefaultMenuItem)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetActiveControllerWhenMenuIsSuppressed<'a, Param0: ::windows_core::IntoParam<'a, RadialController>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetActiveControllerWhenMenuIsSuppressed)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ActiveControllerWhenMenuIsSuppressed(&self) -> ::windows_core::Result<RadialController> {
        let this = &::windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveControllerWhenMenuIsSuppressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialController>(result__)
        }
    }
    pub fn SetIsMenuSuppressed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMenuSuppressed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMenuSuppressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMenuSuppressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<RadialControllerConfiguration> {
        Self::IRadialControllerConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerConfiguration>(result__)
        })
    }
    pub fn SetAppController<'a, Param0: ::windows_core::IntoParam<'a, RadialController>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetAppController)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn AppController() -> ::windows_core::Result<RadialController> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialController>(result__)
        })
    }
    pub fn SetIsAppControllerEnabled(value: bool) -> ::windows_core::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsAppControllerEnabled)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsAppControllerEnabled() -> ::windows_core::Result<bool> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAppControllerEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IRadialControllerConfigurationStatics<R, F: FnOnce(&IRadialControllerConfigurationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRadialControllerConfigurationStatics2<R, F: FnOnce(&IRadialControllerConfigurationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialControllerConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerConfiguration {}
impl ::core::fmt::Debug for RadialControllerConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerConfiguration;{a6b79ecb-6a52-4430-910c-56370a9d6b42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerConfiguration {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerConfiguration";
}
impl ::core::convert::From<RadialControllerConfiguration> for ::windows_core::IUnknown {
    fn from(value: RadialControllerConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerConfiguration> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerConfiguration> for ::windows_core::IInspectable {
    fn from(value: RadialControllerConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerConfiguration> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerConfiguration {}
unsafe impl ::core::marker::Sync for RadialControllerConfiguration {}
#[repr(transparent)]
pub struct RadialControllerControlAcquiredEventArgs(::windows_core::IUnknown);
impl RadialControllerControlAcquiredEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    pub fn IsButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = &::windows_core::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerControlAcquiredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerControlAcquiredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerControlAcquiredEventArgs {}
impl ::core::fmt::Debug for RadialControllerControlAcquiredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerControlAcquiredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerControlAcquiredEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerControlAcquiredEventArgs;{206aa439-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerControlAcquiredEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerControlAcquiredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerControlAcquiredEventArgs";
}
impl ::core::convert::From<RadialControllerControlAcquiredEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerControlAcquiredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerControlAcquiredEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerControlAcquiredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerControlAcquiredEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerControlAcquiredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerControlAcquiredEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerControlAcquiredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerControlAcquiredEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerControlAcquiredEventArgs {}
#[repr(transparent)]
pub struct RadialControllerMenu(::windows_core::IUnknown);
impl RadialControllerMenu {
    #[cfg(feature = "winrt-foundation")]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<RadialControllerMenuItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<RadialControllerMenuItem>>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetSelectedMenuItem(&self) -> ::windows_core::Result<RadialControllerMenuItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSelectedMenuItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerMenuItem>(result__)
        }
    }
    pub fn SelectMenuItem<'a, Param0: ::windows_core::IntoParam<'a, RadialControllerMenuItem>>(&self, menuitem: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SelectMenuItem)(::windows_core::Interface::as_raw(this), menuitem.into_param().abi()).ok() }
    }
    pub fn TrySelectPreviouslySelectedMenuItem(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySelectPreviouslySelectedMenuItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerMenu {}
impl ::core::fmt::Debug for RadialControllerMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerMenu {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerMenu;{8506b35d-f640-4412-aba0-bad077e5ea8a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerMenu {
    type Vtable = IRadialControllerMenu_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerMenu as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerMenu {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenu";
}
impl ::core::convert::From<RadialControllerMenu> for ::windows_core::IUnknown {
    fn from(value: RadialControllerMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenu> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerMenu> for ::windows_core::IInspectable {
    fn from(value: RadialControllerMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenu> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerMenu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerMenu {}
unsafe impl ::core::marker::Sync for RadialControllerMenu {}
#[repr(transparent)]
pub struct RadialControllerMenuItem(::windows_core::IUnknown);
impl RadialControllerMenuItem {
    pub fn DisplayText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Invoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<RadialControllerMenuItem, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Invoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInvoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInvoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromIcon<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::RandomAccessStreamReference>>(displaytext: Param0, icon: Param1) -> ::windows_core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIcon)(::windows_core::Interface::as_raw(this), displaytext.into_param().abi(), icon.into_param().abi(), result__.as_mut_ptr()).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    pub fn CreateFromKnownIcon<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(displaytext: Param0, value: RadialControllerMenuKnownIcon) -> ::windows_core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromKnownIcon)(::windows_core::Interface::as_raw(this), displaytext.into_param().abi(), value, result__.as_mut_ptr()).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    pub fn CreateFromFontGlyph<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(displaytext: Param0, glyph: Param1, fontfamily: Param2) -> ::windows_core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromFontGlyph)(::windows_core::Interface::as_raw(this), displaytext.into_param().abi(), glyph.into_param().abi(), fontfamily.into_param().abi(), result__.as_mut_ptr()).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    pub fn CreateFromFontGlyphWithUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(displaytext: Param0, glyph: Param1, fontfamily: Param2, fonturi: Param3) -> ::windows_core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromFontGlyphWithUri)(::windows_core::Interface::as_raw(this), displaytext.into_param().abi(), glyph.into_param().abi(), fontfamily.into_param().abi(), fonturi.into_param().abi(), result__.as_mut_ptr()).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    pub fn IRadialControllerMenuItemStatics<R, F: FnOnce(&IRadialControllerMenuItemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRadialControllerMenuItemStatics2<R, F: FnOnce(&IRadialControllerMenuItemStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialControllerMenuItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerMenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerMenuItem {}
impl ::core::fmt::Debug for RadialControllerMenuItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerMenuItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerMenuItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerMenuItem;{c80fc98d-ad0b-4c9c-8f2f-136a2373a6ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerMenuItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerMenuItem {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenuItem";
}
impl ::core::convert::From<RadialControllerMenuItem> for ::windows_core::IUnknown {
    fn from(value: RadialControllerMenuItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenuItem> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerMenuItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerMenuItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerMenuItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerMenuItem> for ::windows_core::IInspectable {
    fn from(value: RadialControllerMenuItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenuItem> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerMenuItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerMenuItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerMenuItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerMenuItem {}
unsafe impl ::core::marker::Sync for RadialControllerMenuItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadialControllerMenuKnownIcon(pub i32);
impl RadialControllerMenuKnownIcon {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
    pub const Ruler: Self = Self(5i32);
    pub const InkColor: Self = Self(6i32);
    pub const InkThickness: Self = Self(7i32);
    pub const PenType: Self = Self(8i32);
}
impl ::core::marker::Copy for RadialControllerMenuKnownIcon {}
impl ::core::clone::Clone for RadialControllerMenuKnownIcon {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadialControllerMenuKnownIcon {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RadialControllerMenuKnownIcon {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadialControllerMenuKnownIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerMenuKnownIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerMenuKnownIcon {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerMenuKnownIcon;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RadialControllerRotationChangedEventArgs(::windows_core::IUnknown);
impl RadialControllerRotationChangedEventArgs {
    pub fn RotationDeltaInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationDeltaInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    pub fn IsButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = &::windows_core::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerRotationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerRotationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerRotationChangedEventArgs {}
impl ::core::fmt::Debug for RadialControllerRotationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerRotationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerRotationChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerRotationChangedEventArgs;{206aa435-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerRotationChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerRotationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerRotationChangedEventArgs";
}
impl ::core::convert::From<RadialControllerRotationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerRotationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerRotationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerRotationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerRotationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerRotationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerRotationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerRotationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerRotationChangedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerRotationChangedEventArgs {}
#[repr(transparent)]
pub struct RadialControllerScreenContact(::windows_core::IUnknown);
impl RadialControllerScreenContact {
    pub fn Bounds(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContact {}
impl ::core::fmt::Debug for RadialControllerScreenContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContact").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerScreenContact {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContact;{206aa434-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerScreenContact as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerScreenContact {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContact";
}
impl ::core::convert::From<RadialControllerScreenContact> for ::windows_core::IUnknown {
    fn from(value: RadialControllerScreenContact) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContact> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerScreenContact) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerScreenContact {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerScreenContact {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContact> for ::windows_core::IInspectable {
    fn from(value: RadialControllerScreenContact) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContact> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerScreenContact) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerScreenContact {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerScreenContact {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContact {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContact {}
#[repr(transparent)]
pub struct RadialControllerScreenContactContinuedEventArgs(::windows_core::IUnknown);
impl RadialControllerScreenContactContinuedEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    pub fn IsButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = &::windows_core::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContactContinuedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContactContinuedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContactContinuedEventArgs {}
impl ::core::fmt::Debug for RadialControllerScreenContactContinuedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContactContinuedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerScreenContactContinuedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs;{206aa437-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerScreenContactContinuedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerScreenContactContinuedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs";
}
impl ::core::convert::From<RadialControllerScreenContactContinuedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerScreenContactContinuedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactContinuedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerScreenContactContinuedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContactContinuedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerScreenContactContinuedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactContinuedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerScreenContactContinuedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContactContinuedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContactContinuedEventArgs {}
#[repr(transparent)]
pub struct RadialControllerScreenContactEndedEventArgs(::windows_core::IUnknown);
impl RadialControllerScreenContactEndedEventArgs {
    pub fn IsButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContactEndedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContactEndedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContactEndedEventArgs {}
impl ::core::fmt::Debug for RadialControllerScreenContactEndedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContactEndedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerScreenContactEndedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactEndedEventArgs;{3d577ef2-3cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerScreenContactEndedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerScreenContactEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactEndedEventArgs";
}
impl ::core::convert::From<RadialControllerScreenContactEndedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerScreenContactEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactEndedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerScreenContactEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContactEndedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerScreenContactEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactEndedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerScreenContactEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContactEndedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContactEndedEventArgs {}
#[repr(transparent)]
pub struct RadialControllerScreenContactStartedEventArgs(::windows_core::IUnknown);
impl RadialControllerScreenContactStartedEventArgs {
    pub fn Contact(&self) -> ::windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    pub fn IsButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsButtonPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<::winrt_devices::Haptics::SimpleHapticsController> {
        let this = &::windows_core::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContactStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContactStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContactStartedEventArgs {}
impl ::core::fmt::Debug for RadialControllerScreenContactStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContactStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerScreenContactStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactStartedEventArgs;{206aa436-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerScreenContactStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerScreenContactStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactStartedEventArgs";
}
impl ::core::convert::From<RadialControllerScreenContactStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RadialControllerScreenContactStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerScreenContactStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContactStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RadialControllerScreenContactStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerScreenContactStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContactStartedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContactStartedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
}
impl ::core::marker::Copy for RadialControllerSystemMenuItemKind {}
impl ::core::clone::Clone for RadialControllerSystemMenuItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadialControllerSystemMenuItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RadialControllerSystemMenuItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadialControllerSystemMenuItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerSystemMenuItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerSystemMenuItemKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerSystemMenuItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RightTappedEventArgs(::windows_core::IUnknown);
impl RightTappedEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IRightTappedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for RightTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedEventArgs {}
impl ::core::fmt::Debug for RightTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RightTappedEventArgs;{4cbf40bd-af7a-4a36-9476-b1dce141709a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRightTappedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RightTappedEventArgs";
}
impl ::core::convert::From<RightTappedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RightTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RightTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RightTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RightTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct SystemButtonEventController(::windows_core::IUnknown);
impl SystemButtonEventController {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SystemFunctionButtonPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SystemFunctionButtonPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSystemFunctionButtonPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSystemFunctionButtonPressed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SystemFunctionButtonReleased<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SystemFunctionButtonReleased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSystemFunctionButtonReleased<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSystemFunctionButtonReleased)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SystemFunctionLockChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SystemFunctionLockChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSystemFunctionLockChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSystemFunctionLockChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SystemFunctionLockIndicatorChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SystemFunctionLockIndicatorChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSystemFunctionLockIndicatorChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSystemFunctionLockIndicatorChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn CreateForDispatcherQueue<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::DispatcherQueue>>(queue: Param0) -> ::windows_core::Result<SystemButtonEventController> {
        Self::ISystemButtonEventControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForDispatcherQueue)(::windows_core::Interface::as_raw(this), queue.into_param().abi(), result__.as_mut_ptr()).from_abi::<SystemButtonEventController>(result__)
        })
    }
    pub fn ISystemButtonEventControllerStatics<R, F: FnOnce(&ISystemButtonEventControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemButtonEventController, ISystemButtonEventControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemButtonEventController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemButtonEventController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemButtonEventController {}
impl ::core::fmt::Debug for SystemButtonEventController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemButtonEventController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemButtonEventController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemButtonEventController;{59b893a9-73bc-52b5-ba41-82511b2cb46c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemButtonEventController {
    type Vtable = ISystemButtonEventController_Vtbl;
    const IID: ::windows_core::GUID = <ISystemButtonEventController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemButtonEventController {
    const NAME: &'static str = "Windows.UI.Input.SystemButtonEventController";
}
impl ::core::convert::From<SystemButtonEventController> for ::windows_core::IUnknown {
    fn from(value: SystemButtonEventController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemButtonEventController> for ::windows_core::IUnknown {
    fn from(value: &SystemButtonEventController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemButtonEventController> for ::windows_core::IInspectable {
    fn from(value: SystemButtonEventController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemButtonEventController> for ::windows_core::IInspectable {
    fn from(value: &SystemButtonEventController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SystemButtonEventController> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SystemButtonEventController) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SystemButtonEventController> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SystemButtonEventController) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SystemButtonEventController> for AttachableInputObject {
    fn from(value: SystemButtonEventController) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SystemButtonEventController> for AttachableInputObject {
    fn from(value: &SystemButtonEventController) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, AttachableInputObject> for SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, AttachableInputObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, AttachableInputObject> for &SystemButtonEventController {
    fn into_param(self) -> ::windows_core::Param<'a, AttachableInputObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<AttachableInputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SystemButtonEventController {}
unsafe impl ::core::marker::Sync for SystemButtonEventController {}
#[repr(transparent)]
pub struct SystemFunctionButtonEventArgs(::windows_core::IUnknown);
impl SystemFunctionButtonEventArgs {
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SystemFunctionButtonEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemFunctionButtonEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemFunctionButtonEventArgs {}
impl ::core::fmt::Debug for SystemFunctionButtonEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemFunctionButtonEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemFunctionButtonEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionButtonEventArgs;{4833896f-80d1-5dd6-92a7-62a508ffef5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISystemFunctionButtonEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemFunctionButtonEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionButtonEventArgs";
}
impl ::core::convert::From<SystemFunctionButtonEventArgs> for ::windows_core::IUnknown {
    fn from(value: SystemFunctionButtonEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionButtonEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SystemFunctionButtonEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemFunctionButtonEventArgs> for ::windows_core::IInspectable {
    fn from(value: SystemFunctionButtonEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionButtonEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SystemFunctionButtonEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemFunctionButtonEventArgs {}
unsafe impl ::core::marker::Sync for SystemFunctionButtonEventArgs {}
#[repr(transparent)]
pub struct SystemFunctionLockChangedEventArgs(::windows_core::IUnknown);
impl SystemFunctionLockChangedEventArgs {
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn IsLocked(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLocked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SystemFunctionLockChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemFunctionLockChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemFunctionLockChangedEventArgs {}
impl ::core::fmt::Debug for SystemFunctionLockChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemFunctionLockChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemFunctionLockChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionLockChangedEventArgs;{cd040608-fcf9-585c-beab-f1d2eaf364ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISystemFunctionLockChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemFunctionLockChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockChangedEventArgs";
}
impl ::core::convert::From<SystemFunctionLockChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SystemFunctionLockChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SystemFunctionLockChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemFunctionLockChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SystemFunctionLockChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SystemFunctionLockChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemFunctionLockChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemFunctionLockChangedEventArgs {}
#[repr(transparent)]
pub struct SystemFunctionLockIndicatorChangedEventArgs(::windows_core::IUnknown);
impl SystemFunctionLockIndicatorChangedEventArgs {
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn IsIndicatorOn(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIndicatorOn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SystemFunctionLockIndicatorChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemFunctionLockIndicatorChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemFunctionLockIndicatorChangedEventArgs {}
impl ::core::fmt::Debug for SystemFunctionLockIndicatorChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemFunctionLockIndicatorChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemFunctionLockIndicatorChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs;{b212b94e-7a6f-58ae-b304-bae61d0371b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISystemFunctionLockIndicatorChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemFunctionLockIndicatorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs";
}
impl ::core::convert::From<SystemFunctionLockIndicatorChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockIndicatorChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemFunctionLockIndicatorChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockIndicatorChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemFunctionLockIndicatorChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemFunctionLockIndicatorChangedEventArgs {}
#[repr(transparent)]
pub struct TappedEventArgs(::windows_core::IUnknown);
impl TappedEventArgs {
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn TapCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TapCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ContactCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ITappedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContactCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for TappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedEventArgs {}
impl ::core::fmt::Debug for TappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TappedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.TappedEventArgs;{cfa126e4-253a-4c3c-953b-395c37aed309})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITappedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.TappedEventArgs";
}
impl ::core::convert::From<TappedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
