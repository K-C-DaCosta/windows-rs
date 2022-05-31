#[repr(transparent)]
pub struct CoreIncrementalInkStroke(::windows_core::IUnknown);
impl CoreIncrementalInkStroke {
    #[cfg(feature = "winrt-foundation")]
    pub fn AppendInkPoints<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::InkPoint>>>(&self, inkpoints: Param0) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).AppendInkPoints)(::windows_core::Interface::as_raw(this), inkpoints.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn CreateInkStroke(&self) -> ::windows_core::Result<super::InkStroke> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInkStroke)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::InkStroke>(result__)
        }
    }
    pub fn DrawingAttributes(&self) -> ::windows_core::Result<super::InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DrawingAttributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::InkDrawingAttributes>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PointTransform(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Matrix3x2>::zeroed();
            (::windows_core::Interface::vtable(this).PointTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Matrix3x2>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::InkDrawingAttributes>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Matrix3x2>>(drawingattributes: Param0, pointtransform: Param1) -> ::windows_core::Result<CoreIncrementalInkStroke> {
        Self::ICoreIncrementalInkStrokeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), drawingattributes.into_param().abi(), pointtransform.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreIncrementalInkStroke>(result__)
        })
    }
    pub fn ICoreIncrementalInkStrokeFactory<R, F: FnOnce(&ICoreIncrementalInkStrokeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreIncrementalInkStroke, ICoreIncrementalInkStrokeFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreIncrementalInkStroke {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreIncrementalInkStroke {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreIncrementalInkStroke {}
impl ::core::fmt::Debug for CoreIncrementalInkStroke {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIncrementalInkStroke").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreIncrementalInkStroke {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke;{fda015d3-9d66-4f7d-a57f-cc70b9cfaa76})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_Vtbl;
    const IID: ::windows_core::GUID = <ICoreIncrementalInkStroke as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows_core::IUnknown {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows_core::IUnknown {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows_core::IInspectable {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows_core::IInspectable {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreIncrementalInkStroke {}
unsafe impl ::core::marker::Sync for CoreIncrementalInkStroke {}
#[repr(transparent)]
pub struct CoreInkIndependentInputSource(::windows_core::IUnknown);
impl CoreInkIndependentInputSource {
    #[cfg(feature = "winrt-ui")]
    pub fn PointerEntering<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerEntering)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerEntering<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerEntering)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerHovering<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerHovering)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerHovering<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerHovering)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerExiting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerExiting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerExiting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerExiting)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerPressing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerPressing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerPressing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerPressing)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerMoving<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerMoving)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerMoving<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerMoving)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerReleasing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerReleasing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerReleasing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerReleasing)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PointerLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePointerLost)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows_core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InkPresenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PointerCursor(&self) -> ::windows_core::Result<super::super::super::Core::CoreCursor> {
        let this = &::windows_core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerCursor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Core::CoreCursor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetPointerCursor<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Core::CoreCursor>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerCursor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows_core::Result<CoreInkIndependentInputSource> {
        Self::ICoreInkIndependentInputSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), inkpresenter.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreInkIndependentInputSource>(result__)
        })
    }
    pub fn ICoreInkIndependentInputSourceStatics<R, F: FnOnce(&ICoreInkIndependentInputSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreInkIndependentInputSource, ICoreInkIndependentInputSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreInkIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInkIndependentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInkIndependentInputSource {}
impl ::core::fmt::Debug for CoreInkIndependentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInkIndependentInputSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInkIndependentInputSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource;{39b38da9-7639-4499-a5b5-191d00e35b16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInkIndependentInputSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows_core::IUnknown {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows_core::IUnknown {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows_core::IInspectable {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows_core::IInspectable {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInkIndependentInputSource {}
unsafe impl ::core::marker::Sync for CoreInkIndependentInputSource {}
#[repr(transparent)]
pub struct CoreInkPresenterHost(::windows_core::IUnknown);
impl CoreInkPresenterHost {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreInkPresenterHost, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn InkPresenter(&self) -> ::windows_core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InkPresenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn RootVisual(&self) -> ::windows_core::Result<super::super::super::Composition::ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RootVisual)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Composition::ContainerVisual>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetRootVisual<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Composition::ContainerVisual>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRootVisual)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreInkPresenterHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInkPresenterHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInkPresenterHost {}
impl ::core::fmt::Debug for CoreInkPresenterHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInkPresenterHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInkPresenterHost {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkPresenterHost;{396e89e6-7d55-4617-9e58-68c70c9169b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInkPresenterHost as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows_core::IUnknown {
    fn from(value: CoreInkPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows_core::IUnknown {
    fn from(value: &CoreInkPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInkPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows_core::IInspectable {
    fn from(value: CoreInkPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows_core::IInspectable {
    fn from(value: &CoreInkPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInkPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInkPresenterHost {}
unsafe impl ::core::marker::Sync for CoreInkPresenterHost {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWetStrokeDisposition {}
impl ::core::clone::Clone for CoreWetStrokeDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWetStrokeDisposition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CoreWetStrokeDisposition {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreWetStrokeDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWetStrokeDisposition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreWetStrokeDisposition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CoreWetStrokeUpdateEventArgs(::windows_core::IUnknown);
impl CoreWetStrokeUpdateEventArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn NewInkPoints(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::InkPoint>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewInkPoints)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::InkPoint>>(result__)
        }
    }
    pub fn PointerId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PointerId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Disposition(&self) -> ::windows_core::Result<CoreWetStrokeDisposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CoreWetStrokeDisposition>::zeroed();
            (::windows_core::Interface::vtable(this).Disposition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreWetStrokeDisposition>(result__)
        }
    }
    pub fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisposition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreWetStrokeUpdateEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWetStrokeUpdateEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWetStrokeUpdateEventArgs {}
impl ::core::fmt::Debug for CoreWetStrokeUpdateEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWetStrokeUpdateEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreWetStrokeUpdateEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs;{fb07d14c-3380-457a-a987-991357896c1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreWetStrokeUpdateEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateEventArgs {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateEventArgs {}
#[repr(transparent)]
pub struct CoreWetStrokeUpdateSource(::windows_core::IUnknown);
impl CoreWetStrokeUpdateSource {
    pub fn WetStrokeStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WetStrokeStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWetStrokeStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWetStrokeStarting)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn WetStrokeContinuing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WetStrokeContinuing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWetStrokeContinuing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWetStrokeContinuing)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn WetStrokeStopping<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WetStrokeStopping)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWetStrokeStopping<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWetStrokeStopping)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn WetStrokeCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WetStrokeCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWetStrokeCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWetStrokeCompleted)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn WetStrokeCanceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WetStrokeCanceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWetStrokeCanceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWetStrokeCanceled)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows_core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InkPresenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::InkPresenter>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows_core::Result<CoreWetStrokeUpdateSource> {
        Self::ICoreWetStrokeUpdateSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), inkpresenter.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreWetStrokeUpdateSource>(result__)
        })
    }
    pub fn ICoreWetStrokeUpdateSourceStatics<R, F: FnOnce(&ICoreWetStrokeUpdateSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreWetStrokeUpdateSource, ICoreWetStrokeUpdateSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreWetStrokeUpdateSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWetStrokeUpdateSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWetStrokeUpdateSource {}
impl ::core::fmt::Debug for CoreWetStrokeUpdateSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWetStrokeUpdateSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreWetStrokeUpdateSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource;{1f718e22-ee52-4e00-8209-4c3e5b21a3cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_Vtbl;
    const IID: ::windows_core::GUID = <ICoreWetStrokeUpdateSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows_core::IUnknown {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows_core::IUnknown {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows_core::IInspectable {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows_core::IInspectable {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateSource {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateSource {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreIncrementalInkStroke(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfda015d3_9d66_4f7d_a57f_cc70b9cfaa76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStroke_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AppendInkPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AppendInkPoints: usize,
    pub CreateInkStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub PointTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PointTransform: usize,
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreIncrementalInkStrokeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreIncrementalInkStrokeFactory {
    type Vtable = ICoreIncrementalInkStrokeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7c59f46_8da8_4f70_9751_e53bb6df4596);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: ::windows_core::RawPtr, pointtransform: ::winrt_foundation::Numerics::Matrix3x2, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39b38da9_7639_4499_a5b5_191d00e35b16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub PointerEntering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerEntering: usize,
    pub RemovePointerEntering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PointerHovering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerHovering: usize,
    pub RemovePointerHovering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PointerExiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerExiting: usize,
    pub RemovePointerExiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PointerPressing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerPressing: usize,
    pub RemovePointerPressing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PointerMoving: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerMoving: usize,
    pub RemovePointerMoving: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PointerReleasing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerReleasing: usize,
    pub RemovePointerReleasing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PointerLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerLost: usize,
    pub RemovePointerLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub InkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInkIndependentInputSource2 {
    type Vtable = ICoreInkIndependentInputSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2846b012_0b59_5bb9_a3c5_becb7cf03a33);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub PointerCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PointerCursor: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetPointerCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetPointerCursor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkIndependentInputSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInkIndependentInputSourceStatics {
    type Vtable = ICoreInkIndependentInputSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73e6011b_80c0_4dfb_9b66_10ba7f3f9c84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkPresenterHost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x396e89e6_7d55_4617_9e58_68c70c9169b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkPresenterHost_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub RootVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    RootVisual: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetRootVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetRootVisual: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb07d14c_3380_457a_a987_991357896c1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub NewInkPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    NewInkPoints: usize,
    pub PointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Disposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreWetStrokeDisposition) -> ::windows_core::HRESULT,
    pub SetDisposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreWetStrokeDisposition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f718e22_ee52_4e00_8209_4c3e5b21a3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WetStrokeStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveWetStrokeStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub WetStrokeContinuing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveWetStrokeContinuing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub WetStrokeStopping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveWetStrokeStopping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub WetStrokeCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveWetStrokeCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub WetStrokeCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveWetStrokeCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub InkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWetStrokeUpdateSourceStatics {
    type Vtable = ICoreWetStrokeUpdateSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3dad9cba_1d3d_46ae_ab9d_8647486c6f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
