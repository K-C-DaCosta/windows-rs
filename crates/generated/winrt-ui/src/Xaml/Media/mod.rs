#[cfg(feature = "Animation")]
pub mod Animation;
#[cfg(feature = "Imaging")]
pub mod Imaging;
#[cfg(feature = "Media3D")]
pub mod Media3D;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AcrylicBackgroundSource(pub i32);
impl AcrylicBackgroundSource {
    pub const HostBackdrop: Self = Self(0i32);
    pub const Backdrop: Self = Self(1i32);
}
impl ::core::marker::Copy for AcrylicBackgroundSource {}
impl ::core::clone::Clone for AcrylicBackgroundSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AcrylicBackgroundSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AcrylicBackgroundSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for AcrylicBackgroundSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AcrylicBackgroundSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AcrylicBackgroundSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AcrylicBackgroundSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AcrylicBrush(::windows_core::IUnknown);
impl AcrylicBrush {
    pub fn BackgroundSource(&self) -> ::windows_core::Result<AcrylicBackgroundSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AcrylicBackgroundSource>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AcrylicBackgroundSource>(result__)
        }
    }
    pub fn SetBackgroundSource(&self, value: AcrylicBackgroundSource) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundSource)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TintColor(&self) -> ::windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).TintColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetTintColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTintColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TintOpacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TintOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTintOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TintTransitionDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TintTransitionDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetTintTransitionDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTintTransitionDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AlwaysUseFallback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysUseFallback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysUseFallback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysUseFallback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TintLuminosityOpacity(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<IAcrylicBrush2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TintLuminosityOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetTintLuminosityOpacity<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAcrylicBrush2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTintLuminosityOpacity)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<AcrylicBrush> {
        Self::IAcrylicBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<AcrylicBrush>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<AcrylicBrush> {
        Self::IAcrylicBrushFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<AcrylicBrush>(result__)
        })
    }
    pub fn BackgroundSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundSourceProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TintColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintOpacityProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TintOpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintTransitionDurationProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TintTransitionDurationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AlwaysUseFallbackProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysUseFallbackProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TintLuminosityOpacityProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TintLuminosityOpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IAcrylicBrushFactory<R, F: FnOnce(&IAcrylicBrushFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AcrylicBrush, IAcrylicBrushFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAcrylicBrushStatics<R, F: FnOnce(&IAcrylicBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AcrylicBrush, IAcrylicBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAcrylicBrushStatics2<R, F: FnOnce(&IAcrylicBrushStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AcrylicBrush, IAcrylicBrushStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AcrylicBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AcrylicBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AcrylicBrush {}
impl ::core::fmt::Debug for AcrylicBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AcrylicBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AcrylicBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.AcrylicBrush;{79bbcf4e-cd66-4f1b-a8b6-cd6d2977c18d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AcrylicBrush {
    type Vtable = IAcrylicBrush_Vtbl;
    const IID: ::windows_core::GUID = <IAcrylicBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AcrylicBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.AcrylicBrush";
}
impl ::core::convert::From<AcrylicBrush> for ::windows_core::IUnknown {
    fn from(value: AcrylicBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AcrylicBrush> for ::windows_core::IUnknown {
    fn from(value: &AcrylicBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AcrylicBrush> for ::windows_core::IInspectable {
    fn from(value: AcrylicBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AcrylicBrush> for ::windows_core::IInspectable {
    fn from(value: &AcrylicBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<AcrylicBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: AcrylicBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&AcrylicBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &AcrylicBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<AcrylicBrush> for XamlCompositionBrushBase {
    fn from(value: AcrylicBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AcrylicBrush> for XamlCompositionBrushBase {
    fn from(value: &AcrylicBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for &AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl ::core::convert::From<AcrylicBrush> for Brush {
    fn from(value: AcrylicBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AcrylicBrush> for Brush {
    fn from(value: &AcrylicBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<AcrylicBrush> for super::DependencyObject {
    fn from(value: AcrylicBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AcrylicBrush> for super::DependencyObject {
    fn from(value: &AcrylicBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &AcrylicBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AcrylicBrush {}
unsafe impl ::core::marker::Sync for AcrylicBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentX {}
impl ::core::clone::Clone for AlignmentX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlignmentX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AlignmentX {
    type Abi = Self;
}
impl ::core::fmt::Debug for AlignmentX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlignmentX").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AlignmentX {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AlignmentX;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentY {}
impl ::core::clone::Clone for AlignmentY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlignmentY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AlignmentY {
    type Abi = Self;
}
impl ::core::fmt::Debug for AlignmentY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlignmentY").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AlignmentY {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AlignmentY;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ArcSegment(::windows_core::IUnknown);
impl ArcSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ArcSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Point(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Size(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SetSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RotationAngle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLargeArc(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLargeArc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLargeArc(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLargeArc)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SweepDirection(&self) -> ::windows_core::Result<SweepDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SweepDirection>::zeroed();
            (::windows_core::Interface::vtable(this).SweepDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SweepDirection>(result__)
        }
    }
    pub fn SetSweepDirection(&self, value: SweepDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSweepDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SizeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationAngleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsLargeArcProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsLargeArcProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SweepDirectionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SweepDirectionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IArcSegmentStatics<R, F: FnOnce(&IArcSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ArcSegment, IArcSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ArcSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ArcSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ArcSegment {}
impl ::core::fmt::Debug for ArcSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ArcSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ArcSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ArcSegment;{07940c5f-63fb-4469-91be-f1097c168052})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ArcSegment {
    type Vtable = IArcSegment_Vtbl;
    const IID: ::windows_core::GUID = <IArcSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ArcSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ArcSegment";
}
impl ::core::convert::From<ArcSegment> for ::windows_core::IUnknown {
    fn from(value: ArcSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ArcSegment> for ::windows_core::IUnknown {
    fn from(value: &ArcSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ArcSegment> for ::windows_core::IInspectable {
    fn from(value: ArcSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ArcSegment> for ::windows_core::IInspectable {
    fn from(value: &ArcSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ArcSegment> for PathSegment {
    fn from(value: ArcSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ArcSegment> for PathSegment {
    fn from(value: &ArcSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<ArcSegment> for super::DependencyObject {
    fn from(value: ArcSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ArcSegment> for super::DependencyObject {
    fn from(value: &ArcSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &ArcSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ArcSegment {}
unsafe impl ::core::marker::Sync for ArcSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioCategory(pub i32);
impl AudioCategory {
    pub const Other: Self = Self(0i32);
    pub const ForegroundOnlyMedia: Self = Self(1i32);
    pub const BackgroundCapableMedia: Self = Self(2i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for AudioCategory {}
impl ::core::clone::Clone for AudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AudioCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioDeviceType(pub i32);
impl AudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDeviceType {}
impl ::core::clone::Clone for AudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioDeviceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.AudioDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BezierSegment(::windows_core::IUnknown);
impl BezierSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BezierSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Point1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint1<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint1)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Point2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint2<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint2)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Point3(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint3<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint3)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Point1Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Point1Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Point2Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Point2Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Point3Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Point3Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IBezierSegmentStatics<R, F: FnOnce(&IBezierSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BezierSegment, IBezierSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BezierSegment {}
impl ::core::fmt::Debug for BezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BezierSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BezierSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.BezierSegment;{af4bb9ee-8984-49b7-81df-3f35994b95eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BezierSegment {
    type Vtable = IBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = <IBezierSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BezierSegment";
}
impl ::core::convert::From<BezierSegment> for ::windows_core::IUnknown {
    fn from(value: BezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BezierSegment> for ::windows_core::IUnknown {
    fn from(value: &BezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BezierSegment> for ::windows_core::IInspectable {
    fn from(value: BezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BezierSegment> for ::windows_core::IInspectable {
    fn from(value: &BezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BezierSegment> for PathSegment {
    fn from(value: BezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BezierSegment> for PathSegment {
    fn from(value: &BezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<BezierSegment> for super::DependencyObject {
    fn from(value: BezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BezierSegment> for super::DependencyObject {
    fn from(value: &BezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &BezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BezierSegment {}
unsafe impl ::core::marker::Sync for BezierSegment {}
#[repr(transparent)]
pub struct BitmapCache(::windows_core::IUnknown);
impl BitmapCache {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapCache, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BitmapCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapCache {}
impl ::core::fmt::Debug for BitmapCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapCache").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapCache {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.BitmapCache;{79c2219e-44d2-4610-9735-9bec83809ecf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapCache {
    type Vtable = IBitmapCache_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapCache as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapCache {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BitmapCache";
}
impl ::core::convert::From<BitmapCache> for ::windows_core::IUnknown {
    fn from(value: BitmapCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapCache> for ::windows_core::IUnknown {
    fn from(value: &BitmapCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapCache> for ::windows_core::IInspectable {
    fn from(value: BitmapCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapCache> for ::windows_core::IInspectable {
    fn from(value: &BitmapCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapCache> for CacheMode {
    fn from(value: BitmapCache) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapCache> for CacheMode {
    fn from(value: &BitmapCache) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, CacheMode> for BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, CacheMode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, CacheMode> for &BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, CacheMode> {
        ::windows_core::Param::Owned(::core::convert::Into::<CacheMode>::into(self))
    }
}
impl ::core::convert::From<BitmapCache> for super::DependencyObject {
    fn from(value: BitmapCache) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapCache> for super::DependencyObject {
    fn from(value: &BitmapCache) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &BitmapCache {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BitmapCache {}
unsafe impl ::core::marker::Sync for BitmapCache {}
#[repr(transparent)]
pub struct Brush(::windows_core::IUnknown);
impl Brush {
    #[cfg(feature = "winrt-ui")]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::Composition::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Composition::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Opacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Transform>(result__)
        }
    }
    pub fn SetTransform<'a, Param0: ::windows_core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransform)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Transform>(result__)
        }
    }
    pub fn SetRelativeTransform<'a, Param0: ::windows_core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTransform)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OpacityProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TransformProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransformProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RelativeTransformProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransformProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IBrushStatics<R, F: FnOnce(&IBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Brush, IBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Brush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Brush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Brush {}
impl ::core::fmt::Debug for Brush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Brush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Brush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Brush;{8806a321-1e06-422c-a1cc-01696559e021})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Brush {
    type Vtable = IBrush_Vtbl;
    const IID: ::windows_core::GUID = <IBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Brush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Brush";
}
impl ::core::convert::From<Brush> for ::windows_core::IUnknown {
    fn from(value: Brush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Brush> for ::windows_core::IUnknown {
    fn from(value: &Brush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Brush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Brush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Brush> for ::windows_core::IInspectable {
    fn from(value: Brush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Brush> for ::windows_core::IInspectable {
    fn from(value: &Brush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Brush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Brush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<Brush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: Brush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&Brush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &Brush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for Brush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Brush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<Brush> for super::DependencyObject {
    fn from(value: Brush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Brush> for super::DependencyObject {
    fn from(value: &Brush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for Brush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &Brush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Brush {}
unsafe impl ::core::marker::Sync for Brush {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct BrushCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl BrushCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BrushCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<Brush>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<Brush>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<Brush>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<Brush>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Brush>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Brush>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, Brush>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, Brush>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, Brush>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Brush>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Brush>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for BrushCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for BrushCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for BrushCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for BrushCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrushCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for BrushCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.BrushCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Brush;{8806a321-1e06-422c-a1cc-01696559e021})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for BrushCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<Brush>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<Brush> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for BrushCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.BrushCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for BrushCollection {
    type Item = Brush;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &BrushCollection {
    type Item = Brush;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<BrushCollection> for ::windows_core::IUnknown {
    fn from(value: BrushCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&BrushCollection> for ::windows_core::IUnknown {
    fn from(value: &BrushCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<BrushCollection> for ::windows_core::IInspectable {
    fn from(value: BrushCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&BrushCollection> for ::windows_core::IInspectable {
    fn from(value: &BrushCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<BrushCollection> for ::winrt_foundation::Collections::IIterable<Brush> {
    type Error = ::windows_core::Error;
    fn try_from(value: BrushCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&BrushCollection> for ::winrt_foundation::Collections::IIterable<Brush> {
    type Error = ::windows_core::Error;
    fn try_from(value: &BrushCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Brush>> for BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Brush>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Brush>> for &BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Brush>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<Brush>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<BrushCollection> for ::winrt_foundation::Collections::IVector<Brush> {
    type Error = ::windows_core::Error;
    fn try_from(value: BrushCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&BrushCollection> for ::winrt_foundation::Collections::IVector<Brush> {
    type Error = ::windows_core::Error;
    fn try_from(value: &BrushCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Brush>> for BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Brush>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Brush>> for &BrushCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Brush>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<Brush>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for BrushCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for BrushCollection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const RelativeToBoundingBox: Self = Self(1i32);
}
impl ::core::marker::Copy for BrushMappingMode {}
impl ::core::clone::Clone for BrushMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BrushMappingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BrushMappingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BrushMappingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrushMappingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BrushMappingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.BrushMappingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CacheMode(::windows_core::IUnknown);
impl CacheMode {}
impl ::core::clone::Clone for CacheMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CacheMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CacheMode {}
impl ::core::fmt::Debug for CacheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CacheMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CacheMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.CacheMode;{98dc8b11-c6f9-4dab-b838-5fd5ec8c7350})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CacheMode {
    type Vtable = ICacheMode_Vtbl;
    const IID: ::windows_core::GUID = <ICacheMode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CacheMode {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CacheMode";
}
impl ::core::convert::From<CacheMode> for ::windows_core::IUnknown {
    fn from(value: CacheMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CacheMode> for ::windows_core::IUnknown {
    fn from(value: &CacheMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CacheMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CacheMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CacheMode> for ::windows_core::IInspectable {
    fn from(value: CacheMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CacheMode> for ::windows_core::IInspectable {
    fn from(value: &CacheMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CacheMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CacheMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CacheMode> for super::DependencyObject {
    fn from(value: CacheMode) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CacheMode> for super::DependencyObject {
    fn from(value: &CacheMode) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for CacheMode {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &CacheMode {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CacheMode {}
unsafe impl ::core::marker::Sync for CacheMode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: Self = Self(0i32);
    pub const SRgbLinearInterpolation: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorInterpolationMode {}
impl ::core::clone::Clone for ColorInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ColorInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ColorInterpolationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ColorInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorInterpolationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorInterpolationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.ColorInterpolationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CompositeTransform(::windows_core::IUnknown);
impl CompositeTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositeTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SkewX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SkewX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSkewX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSkewX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SkewY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SkewY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSkewY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSkewY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslateX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslateX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslateY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslateY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SkewXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SkewXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SkewYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SkewYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ICompositeTransformStatics<R, F: FnOnce(&ICompositeTransformStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositeTransform, ICompositeTransformStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositeTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositeTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositeTransform {}
impl ::core::fmt::Debug for CompositeTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositeTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositeTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.CompositeTransform;{c8a4385b-f24a-4701-a265-a78846f142b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositeTransform {
    type Vtable = ICompositeTransform_Vtbl;
    const IID: ::windows_core::GUID = <ICompositeTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositeTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositeTransform";
}
impl ::core::convert::From<CompositeTransform> for ::windows_core::IUnknown {
    fn from(value: CompositeTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeTransform> for ::windows_core::IUnknown {
    fn from(value: &CompositeTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeTransform> for ::windows_core::IInspectable {
    fn from(value: CompositeTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeTransform> for ::windows_core::IInspectable {
    fn from(value: &CompositeTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeTransform> for Transform {
    fn from(value: CompositeTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform> for Transform {
    fn from(value: &CompositeTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<CompositeTransform> for GeneralTransform {
    fn from(value: CompositeTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform> for GeneralTransform {
    fn from(value: &CompositeTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<CompositeTransform> for super::DependencyObject {
    fn from(value: CompositeTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform> for super::DependencyObject {
    fn from(value: &CompositeTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &CompositeTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositeTransform {}
unsafe impl ::core::marker::Sync for CompositeTransform {}
#[repr(transparent)]
pub struct CompositionTarget(::windows_core::IUnknown);
impl CompositionTarget {
    pub fn Rendering<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Rendering)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRendering<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRendering)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn SurfaceContentsLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SurfaceContentsLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSurfaceContentsLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSurfaceContentsLost)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn Rendered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<RenderedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICompositionTargetStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Rendered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRendered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICompositionTargetStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRendered)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ICompositionTargetStatics<R, F: FnOnce(&ICompositionTargetStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositionTarget, ICompositionTargetStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICompositionTargetStatics3<R, F: FnOnce(&ICompositionTargetStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositionTarget, ICompositionTargetStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositionTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionTarget {}
impl ::core::fmt::Debug for CompositionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.CompositionTarget;{26cfbff0-713c-4bec-8803-e101f7b14ed3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositionTarget {
    type Vtable = ICompositionTarget_Vtbl;
    const IID: ::windows_core::GUID = <ICompositionTarget as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositionTarget";
}
impl ::core::convert::From<CompositionTarget> for ::windows_core::IUnknown {
    fn from(value: CompositionTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionTarget> for ::windows_core::IUnknown {
    fn from(value: &CompositionTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositionTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositionTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionTarget> for ::windows_core::IInspectable {
    fn from(value: CompositionTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionTarget> for ::windows_core::IInspectable {
    fn from(value: &CompositionTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositionTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositionTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompositionTarget {}
unsafe impl ::core::marker::Sync for CompositionTarget {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct DoubleCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl DoubleCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<f64>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<f64>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<f64>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<f64>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf(&self, value: f64, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value, index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt(&self, index: u32, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt(&self, index: u32, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [f64]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[f64]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for DoubleCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for DoubleCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for DoubleCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for DoubleCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for DoubleCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.DoubleCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};f8))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for DoubleCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<f64>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<f64> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for DoubleCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.DoubleCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for DoubleCollection {
    type Item = f64;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &DoubleCollection {
    type Item = f64;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<DoubleCollection> for ::windows_core::IUnknown {
    fn from(value: DoubleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&DoubleCollection> for ::windows_core::IUnknown {
    fn from(value: &DoubleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<DoubleCollection> for ::windows_core::IInspectable {
    fn from(value: DoubleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&DoubleCollection> for ::windows_core::IInspectable {
    fn from(value: &DoubleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<DoubleCollection> for ::winrt_foundation::Collections::IIterable<f64> {
    type Error = ::windows_core::Error;
    fn try_from(value: DoubleCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&DoubleCollection> for ::winrt_foundation::Collections::IIterable<f64> {
    type Error = ::windows_core::Error;
    fn try_from(value: &DoubleCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<f64>> for DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<f64>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<f64>> for &DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<f64>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<f64>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<DoubleCollection> for ::winrt_foundation::Collections::IVector<f64> {
    type Error = ::windows_core::Error;
    fn try_from(value: DoubleCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&DoubleCollection> for ::winrt_foundation::Collections::IVector<f64> {
    type Error = ::windows_core::Error;
    fn try_from(value: &DoubleCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<f64>> for DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<f64>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<f64>> for &DoubleCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<f64>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<f64>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for DoubleCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for DoubleCollection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const MinBlend: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementCompositeMode {}
impl ::core::clone::Clone for ElementCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementCompositeMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ElementCompositeMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ElementCompositeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositeMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ElementCompositeMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.ElementCompositeMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EllipseGeometry(::windows_core::IUnknown);
impl EllipseGeometry {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EllipseGeometry, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Center(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetCenter<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RadiusX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RadiusX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadiusX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRadiusX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RadiusY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RadiusY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadiusY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRadiusY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RadiusXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RadiusXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RadiusYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RadiusYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IEllipseGeometryStatics<R, F: FnOnce(&IEllipseGeometryStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EllipseGeometry, IEllipseGeometryStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EllipseGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EllipseGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EllipseGeometry {}
impl ::core::fmt::Debug for EllipseGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EllipseGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EllipseGeometry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.EllipseGeometry;{d4f61bba-4ea2-40d6-aa6c-8d38aa87651f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EllipseGeometry {
    type Vtable = IEllipseGeometry_Vtbl;
    const IID: ::windows_core::GUID = <IEllipseGeometry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EllipseGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.EllipseGeometry";
}
impl ::core::convert::From<EllipseGeometry> for ::windows_core::IUnknown {
    fn from(value: EllipseGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EllipseGeometry> for ::windows_core::IUnknown {
    fn from(value: &EllipseGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EllipseGeometry> for ::windows_core::IInspectable {
    fn from(value: EllipseGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EllipseGeometry> for ::windows_core::IInspectable {
    fn from(value: &EllipseGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EllipseGeometry> for Geometry {
    fn from(value: EllipseGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EllipseGeometry> for Geometry {
    fn from(value: &EllipseGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for &EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl ::core::convert::From<EllipseGeometry> for super::DependencyObject {
    fn from(value: EllipseGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EllipseGeometry> for super::DependencyObject {
    fn from(value: &EllipseGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &EllipseGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EllipseGeometry {}
unsafe impl ::core::marker::Sync for EllipseGeometry {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: Self = Self(0i32);
    pub const Hide: Self = Self(1i32);
    pub const Disable: Self = Self(2i32);
}
impl ::core::marker::Copy for FastPlayFallbackBehaviour {}
impl ::core::clone::Clone for FastPlayFallbackBehaviour {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FastPlayFallbackBehaviour {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FastPlayFallbackBehaviour {
    type Abi = Self;
}
impl ::core::fmt::Debug for FastPlayFallbackBehaviour {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FastPlayFallbackBehaviour").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FastPlayFallbackBehaviour {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.FastPlayFallbackBehaviour;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: Self = Self(0i32);
    pub const Nonzero: Self = Self(1i32);
}
impl ::core::marker::Copy for FillRule {}
impl ::core::clone::Clone for FillRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FillRule {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FillRule {
    type Abi = Self;
}
impl ::core::fmt::Debug for FillRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FillRule").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FillRule {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.FillRule;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FontFamily(::windows_core::IUnknown);
impl FontFamily {
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateInstanceWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(familyname: Param0) -> ::windows_core::Result<FontFamily> {
        Self::IFontFamilyFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithName)(::windows_core::Interface::as_raw(this), familyname.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<FontFamily>(result__)
        })
    }
    pub fn CreateInstanceWithName_compose<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Compose>(familyname: Param0, compose: T) -> ::windows_core::Result<FontFamily> {
        Self::IFontFamilyFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithName)(::windows_core::Interface::as_raw(this), familyname.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<FontFamily>(result__)
        })
    }
    pub fn XamlAutoFontFamily() -> ::windows_core::Result<FontFamily> {
        Self::IFontFamilyStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).XamlAutoFontFamily)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FontFamily>(result__)
        })
    }
    pub fn IFontFamilyFactory<R, F: FnOnce(&IFontFamilyFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FontFamily, IFontFamilyFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFontFamilyStatics2<R, F: FnOnce(&IFontFamilyStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FontFamily, IFontFamilyStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FontFamily {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FontFamily {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FontFamily {}
impl ::core::fmt::Debug for FontFamily {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontFamily").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FontFamily {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.FontFamily;{92467e64-d66a-4cf4-9322-3d23b3c0c361})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FontFamily {
    type Vtable = IFontFamily_Vtbl;
    const IID: ::windows_core::GUID = <IFontFamily as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FontFamily {
    const NAME: &'static str = "Windows.UI.Xaml.Media.FontFamily";
}
impl ::core::convert::From<FontFamily> for ::windows_core::IUnknown {
    fn from(value: FontFamily) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FontFamily> for ::windows_core::IUnknown {
    fn from(value: &FontFamily) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FontFamily {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FontFamily {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FontFamily> for ::windows_core::IInspectable {
    fn from(value: FontFamily) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FontFamily> for ::windows_core::IInspectable {
    fn from(value: &FontFamily) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FontFamily {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FontFamily {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FontFamily {}
unsafe impl ::core::marker::Sync for FontFamily {}
#[repr(transparent)]
pub struct GeneralTransform(::windows_core::IUnknown);
impl GeneralTransform {
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeneralTransform>(result__)
        }
    }
    pub fn TransformPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, point: Param0) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(::windows_core::Interface::as_raw(this), point.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
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
impl ::core::clone::Clone for GeneralTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeneralTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeneralTransform {}
impl ::core::fmt::Debug for GeneralTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneralTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeneralTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GeneralTransform;{a06798b7-a2ec-415f-ade2-eade9333f2c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeneralTransform {
    type Vtable = IGeneralTransform_Vtbl;
    const IID: ::windows_core::GUID = <IGeneralTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeneralTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeneralTransform";
}
impl ::core::convert::From<GeneralTransform> for ::windows_core::IUnknown {
    fn from(value: GeneralTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeneralTransform> for ::windows_core::IUnknown {
    fn from(value: &GeneralTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeneralTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeneralTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeneralTransform> for ::windows_core::IInspectable {
    fn from(value: GeneralTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeneralTransform> for ::windows_core::IInspectable {
    fn from(value: &GeneralTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeneralTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeneralTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeneralTransform> for super::DependencyObject {
    fn from(value: GeneralTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GeneralTransform> for super::DependencyObject {
    fn from(value: &GeneralTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for GeneralTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &GeneralTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for GeneralTransform {}
unsafe impl ::core::marker::Sync for GeneralTransform {}
#[repr(transparent)]
pub struct Geometry(::windows_core::IUnknown);
impl Geometry {
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Transform>(result__)
        }
    }
    pub fn SetTransform<'a, Param0: ::windows_core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransform)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Bounds(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn Empty() -> ::windows_core::Result<Geometry> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Empty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geometry>(result__)
        })
    }
    pub fn StandardFlatteningTolerance() -> ::windows_core::Result<f64> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).StandardFlatteningTolerance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        })
    }
    pub fn TransformProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransformProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGeometryStatics<R, F: FnOnce(&IGeometryStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geometry, IGeometryStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geometry {}
impl ::core::fmt::Debug for Geometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geometry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geometry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Geometry;{fa123889-0acd-417b-b62d-5ca1bf4dfc0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geometry {
    type Vtable = IGeometry_Vtbl;
    const IID: ::windows_core::GUID = <IGeometry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Geometry";
}
impl ::core::convert::From<Geometry> for ::windows_core::IUnknown {
    fn from(value: Geometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geometry> for ::windows_core::IUnknown {
    fn from(value: &Geometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geometry> for ::windows_core::IInspectable {
    fn from(value: Geometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geometry> for ::windows_core::IInspectable {
    fn from(value: &Geometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geometry> for super::DependencyObject {
    fn from(value: Geometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Geometry> for super::DependencyObject {
    fn from(value: &Geometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for Geometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &Geometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Geometry {}
unsafe impl ::core::marker::Sync for Geometry {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct GeometryCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl GeometryCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeometryCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<Geometry>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<Geometry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<Geometry>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Geometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<Geometry>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Geometry>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Geometry>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, Geometry>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, Geometry>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, Geometry>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, Geometry>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Geometry>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Geometry>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for GeometryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for GeometryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for GeometryCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for GeometryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeometryCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for GeometryCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GeometryCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Geometry;{fa123889-0acd-417b-b62d-5ca1bf4dfc0e})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for GeometryCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<Geometry>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<Geometry> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for GeometryCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeometryCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for GeometryCollection {
    type Item = Geometry;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &GeometryCollection {
    type Item = Geometry;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<GeometryCollection> for ::windows_core::IUnknown {
    fn from(value: GeometryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&GeometryCollection> for ::windows_core::IUnknown {
    fn from(value: &GeometryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<GeometryCollection> for ::windows_core::IInspectable {
    fn from(value: GeometryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&GeometryCollection> for ::windows_core::IInspectable {
    fn from(value: &GeometryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<GeometryCollection> for ::winrt_foundation::Collections::IIterable<Geometry> {
    type Error = ::windows_core::Error;
    fn try_from(value: GeometryCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&GeometryCollection> for ::winrt_foundation::Collections::IIterable<Geometry> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GeometryCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Geometry>> for GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Geometry>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Geometry>> for &GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Geometry>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<Geometry>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<GeometryCollection> for ::winrt_foundation::Collections::IVector<Geometry> {
    type Error = ::windows_core::Error;
    fn try_from(value: GeometryCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&GeometryCollection> for ::winrt_foundation::Collections::IVector<Geometry> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GeometryCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Geometry>> for GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Geometry>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Geometry>> for &GeometryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Geometry>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<Geometry>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for GeometryCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for GeometryCollection {}
#[repr(transparent)]
pub struct GeometryGroup(::windows_core::IUnknown);
impl GeometryGroup {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeometryGroup, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FillRule(&self) -> ::windows_core::Result<FillRule> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FillRule>::zeroed();
            (::windows_core::Interface::vtable(this).FillRule)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FillRule>(result__)
        }
    }
    pub fn SetFillRule(&self, value: FillRule) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFillRule)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<GeometryCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeometryCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetChildren<'a, Param0: ::windows_core::IntoParam<'a, GeometryCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChildren)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FillRuleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGeometryGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillRuleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ChildrenProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGeometryGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildrenProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGeometryGroupStatics<R, F: FnOnce(&IGeometryGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeometryGroup, IGeometryGroupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GeometryGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeometryGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeometryGroup {}
impl ::core::fmt::Debug for GeometryGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeometryGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeometryGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GeometryGroup;{55225a61-8677-4c8c-8e46-ee3dc355114b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeometryGroup {
    type Vtable = IGeometryGroup_Vtbl;
    const IID: ::windows_core::GUID = <IGeometryGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeometryGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GeometryGroup";
}
impl ::core::convert::From<GeometryGroup> for ::windows_core::IUnknown {
    fn from(value: GeometryGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeometryGroup> for ::windows_core::IUnknown {
    fn from(value: &GeometryGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeometryGroup> for ::windows_core::IInspectable {
    fn from(value: GeometryGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeometryGroup> for ::windows_core::IInspectable {
    fn from(value: &GeometryGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeometryGroup> for Geometry {
    fn from(value: GeometryGroup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GeometryGroup> for Geometry {
    fn from(value: &GeometryGroup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for &GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl ::core::convert::From<GeometryGroup> for super::DependencyObject {
    fn from(value: GeometryGroup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GeometryGroup> for super::DependencyObject {
    fn from(value: &GeometryGroup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &GeometryGroup {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for GeometryGroup {}
unsafe impl ::core::marker::Sync for GeometryGroup {}
#[repr(transparent)]
pub struct GradientBrush(::windows_core::IUnknown);
impl GradientBrush {
    pub fn SpreadMethod(&self) -> ::windows_core::Result<GradientSpreadMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GradientSpreadMethod>::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GradientSpreadMethod>(result__)
        }
    }
    pub fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpreadMethod)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MappingMode(&self) -> ::windows_core::Result<BrushMappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BrushMappingMode>::zeroed();
            (::windows_core::Interface::vtable(this).MappingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BrushMappingMode>(result__)
        }
    }
    pub fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMappingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorInterpolationMode(&self) -> ::windows_core::Result<ColorInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ColorInterpolationMode>::zeroed();
            (::windows_core::Interface::vtable(this).ColorInterpolationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ColorInterpolationMode>(result__)
        }
    }
    pub fn SetColorInterpolationMode(&self, value: ColorInterpolationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorInterpolationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GradientStops(&self) -> ::windows_core::Result<GradientStopCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GradientStops)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GradientStopCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetGradientStops<'a, Param0: ::windows_core::IntoParam<'a, GradientStopCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGradientStops)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SpreadMethodProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethodProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn MappingModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MappingModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ColorInterpolationModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorInterpolationModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GradientStopsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GradientStopsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGradientBrushStatics<R, F: FnOnce(&IGradientBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GradientBrush, IGradientBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GradientBrush {}
impl ::core::fmt::Debug for GradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GradientBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GradientBrush;{2166e69f-935a-4191-8e3c-1c8dfdfcdc78})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GradientBrush {
    type Vtable = IGradientBrush_Vtbl;
    const IID: ::windows_core::GUID = <IGradientBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientBrush";
}
impl ::core::convert::From<GradientBrush> for ::windows_core::IUnknown {
    fn from(value: GradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GradientBrush> for ::windows_core::IUnknown {
    fn from(value: &GradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GradientBrush> for ::windows_core::IInspectable {
    fn from(value: GradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GradientBrush> for ::windows_core::IInspectable {
    fn from(value: &GradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<GradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: GradientBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&GradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &GradientBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<GradientBrush> for Brush {
    fn from(value: GradientBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GradientBrush> for Brush {
    fn from(value: &GradientBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<GradientBrush> for super::DependencyObject {
    fn from(value: GradientBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GradientBrush> for super::DependencyObject {
    fn from(value: &GradientBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &GradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for GradientBrush {}
unsafe impl ::core::marker::Sync for GradientBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: Self = Self(0i32);
    pub const Reflect: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for GradientSpreadMethod {}
impl ::core::clone::Clone for GradientSpreadMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GradientSpreadMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GradientSpreadMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for GradientSpreadMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientSpreadMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GradientSpreadMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.GradientSpreadMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GradientStop(::windows_core::IUnknown);
impl GradientStop {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GradientStop, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Color(&self) -> ::windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Offset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Offset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientStopStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientStopStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IGradientStopStatics<R, F: FnOnce(&IGradientStopStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GradientStop, IGradientStopStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GradientStop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GradientStop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GradientStop {}
impl ::core::fmt::Debug for GradientStop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientStop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GradientStop {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GradientStop;{665f44fe-2e59-4c4a-ab53-076a100ccd81})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GradientStop {
    type Vtable = IGradientStop_Vtbl;
    const IID: ::windows_core::GUID = <IGradientStop as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GradientStop {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientStop";
}
impl ::core::convert::From<GradientStop> for ::windows_core::IUnknown {
    fn from(value: GradientStop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GradientStop> for ::windows_core::IUnknown {
    fn from(value: &GradientStop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GradientStop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GradientStop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GradientStop> for ::windows_core::IInspectable {
    fn from(value: GradientStop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GradientStop> for ::windows_core::IInspectable {
    fn from(value: &GradientStop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GradientStop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GradientStop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GradientStop> for super::DependencyObject {
    fn from(value: GradientStop) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GradientStop> for super::DependencyObject {
    fn from(value: &GradientStop) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for GradientStop {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &GradientStop {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for GradientStop {}
unsafe impl ::core::marker::Sync for GradientStop {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct GradientStopCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl GradientStopCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GradientStopCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<GradientStop>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<GradientStop>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<GradientStop>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<GradientStop> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<GradientStop>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GradientStop>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GradientStop>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, GradientStop>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, GradientStop>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, GradientStop>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, GradientStop>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<GradientStop>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<GradientStop>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for GradientStopCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for GradientStopCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for GradientStopCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for GradientStopCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientStopCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for GradientStopCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.GradientStopCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.GradientStop;{665f44fe-2e59-4c4a-ab53-076a100ccd81})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for GradientStopCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<GradientStop>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<GradientStop> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for GradientStopCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.GradientStopCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<GradientStopCollection> for ::windows_core::IUnknown {
    fn from(value: GradientStopCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&GradientStopCollection> for ::windows_core::IUnknown {
    fn from(value: &GradientStopCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<GradientStopCollection> for ::windows_core::IInspectable {
    fn from(value: GradientStopCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&GradientStopCollection> for ::windows_core::IInspectable {
    fn from(value: &GradientStopCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<GradientStopCollection> for ::winrt_foundation::Collections::IIterable<GradientStop> {
    type Error = ::windows_core::Error;
    fn try_from(value: GradientStopCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&GradientStopCollection> for ::winrt_foundation::Collections::IIterable<GradientStop> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GradientStopCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<GradientStop>> for GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<GradientStop>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<GradientStop>> for &GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<GradientStop>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<GradientStop>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<GradientStopCollection> for ::winrt_foundation::Collections::IVector<GradientStop> {
    type Error = ::windows_core::Error;
    fn try_from(value: GradientStopCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&GradientStopCollection> for ::winrt_foundation::Collections::IVector<GradientStop> {
    type Error = ::windows_core::Error;
    fn try_from(value: &GradientStopCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<GradientStop>> for GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<GradientStop>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<GradientStop>> for &GradientStopCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<GradientStop>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<GradientStop>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for GradientStopCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for GradientStopCollection {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrush {
    type Vtable = IAcrylicBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79bbcf4e_cd66_4f1b_a8b6_cd6d2977c18d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BackgroundSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AcrylicBackgroundSource) -> ::windows_core::HRESULT,
    pub SetBackgroundSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AcrylicBackgroundSource) -> ::windows_core::HRESULT,
    pub TintColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TintTransitionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetTintTransitionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AlwaysUseFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrush2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrush2 {
    type Vtable = IAcrylicBrush2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9645383_b19e_5ac0_86ff_3d90506dbcda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TintLuminosityOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTintLuminosityOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushFactory {
    type Vtable = IAcrylicBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81a32568_f6cc_4013_8363_928ae23b7a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushStatics {
    type Vtable = IAcrylicBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2787fd79_a3da_423f_b81a_599147971523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BackgroundSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TintColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TintOpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TintTransitionDurationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAcrylicBrushStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushStatics2 {
    type Vtable = IAcrylicBrushStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x129188a8_bf11_5bbc_8445_8c510e5926c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TintLuminosityOpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcSegment {
    type Vtable = IArcSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07940c5f_63fb_4469_91be_f1097c168052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub RotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IsLargeArc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLargeArc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SweepDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SweepDirection) -> ::windows_core::HRESULT,
    pub SetSweepDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SweepDirection) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcSegmentStatics {
    type Vtable = IArcSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82348f6e_8a69_4204_9c12_7207df317643);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationAngleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsLargeArcProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SweepDirectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBezierSegment {
    type Vtable = IBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf4bb9ee_8984_49b7_81df_3f35994b95eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Point2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Point3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBezierSegmentStatics {
    type Vtable = IBezierSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0287bac_1410_4530_8452_1c9d0ad1f341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point1Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Point3Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapCache(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapCache {
    type Vtable = IBitmapCache_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79c2219e_44d2_4610_9735_9bec83809ecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCache_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrush {
    type Vtable = IBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8806a321_1e06_422c_a1cc_01696559e021);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Opacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RelativeTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRelativeTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushFactory {
    type Vtable = IBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x399658a2_14fb_4b8f_83e6_6e3dab12069b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrushOverrides2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushOverrides2 {
    type Vtable = IBrushOverrides2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd092b151_d83b_5a81_a71e_a1c7f8ad6963);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushOverrides2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub PopulatePropertyInfoOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, animationpropertyinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PopulatePropertyInfoOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushStatics {
    type Vtable = IBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe70c3102_0225_47f5_b22e_0467619f6a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RelativeTransformProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICacheMode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICacheMode {
    type Vtable = ICacheMode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98dc8b11_c6f9_4dab_b838_5fd5ec8c7350);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheMode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICacheModeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICacheModeFactory {
    type Vtable = ICacheModeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb1f8c5b_0abb_4e70_b8a8_620d0d953ab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheModeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransform {
    type Vtable = ICompositeTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8a4385b_f24a_4701_a265_a78846f142b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SkewX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSkewX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SkewY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSkewY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TranslateX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTranslateX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TranslateY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTranslateY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransformStatics {
    type Vtable = ICompositeTransformStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f190c08_8266_496f_9653_a18bd4f836aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SkewXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SkewYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TranslateXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TranslateYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTarget {
    type Vtable = ICompositionTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26cfbff0_713c_4bec_8803_e101f7b14ed3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTarget_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionTargetStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTargetStatics {
    type Vtable = ICompositionTargetStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b1af03d_1ed2_4b59_bd00_7594ee92832b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Rendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SurfaceContentsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSurfaceContentsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionTargetStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTargetStatics3 {
    type Vtable = ICompositionTargetStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc0a7cd9_6750_4708_994c_2028e0312ac8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Rendered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRendered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEllipseGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEllipseGeometry {
    type Vtable = IEllipseGeometry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4f61bba_4ea2_40d6_aa6c_8d38aa87651f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub RadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEllipseGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEllipseGeometryStatics {
    type Vtable = IEllipseGeometryStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1744db47_f635_4b16_aee6_e052a65defb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RadiusXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontFamily(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamily {
    type Vtable = IFontFamily_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92467e64_d66a_4cf4_9322_3d23b3c0c361);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamily_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontFamilyFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamilyFactory {
    type Vtable = IFontFamilyFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5603377_3dae_4dcd_af09_f9498e9ec659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, familyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontFamilyStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamilyStatics2 {
    type Vtable = IFontFamilyStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52ad7af9_37e6_4297_a238_97fb6a408d9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub XamlAutoFontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneralTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransform {
    type Vtable = IGeneralTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa06798b7_a2ec_415f_ade2_eade9333f2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Inverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TransformPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: ::winrt_foundation::Point, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inpoint: ::winrt_foundation::Point, outpoint: *mut ::winrt_foundation::Point, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TransformBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: ::winrt_foundation::Rect, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneralTransformFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransformFactory {
    type Vtable = IGeneralTransformFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a25c930_29c4_4e31_b6f9_dedd52e4df1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneralTransformOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransformOverrides {
    type Vtable = IGeneralTransformOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f121083_24cf_4524_90ad_8a42b1c12783);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InverseCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryTransformCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inpoint: ::winrt_foundation::Point, outpoint: *mut ::winrt_foundation::Point, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TransformBoundsCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: ::winrt_foundation::Rect, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometry {
    type Vtable = IGeometry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa123889_0acd_417b_b62d_5ca1bf4dfc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryFactory {
    type Vtable = IGeometryFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf65daf23_d5fd_42f9_b32a_929c5a4b54e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryGroup {
    type Vtable = IGeometryGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55225a61_8677_4c8c_8e46_ee3dc355114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FillRule) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FillRule) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Children: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetChildren: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryGroupStatics {
    type Vtable = IGeometryGroupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56c955f4_8496_4bb6_abf0_617b1fe78b45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryStatics {
    type Vtable = IGeometryStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a70aa8c_0b06_465f_b637_9a47e5a70111);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Empty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StandardFlatteningTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrush {
    type Vtable = IGradientBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2166e69f_935a_4191_8e3c_1c8dfdfcdc78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GradientSpreadMethod) -> ::windows_core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GradientSpreadMethod) -> ::windows_core::HRESULT,
    pub MappingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BrushMappingMode) -> ::windows_core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BrushMappingMode) -> ::windows_core::HRESULT,
    pub ColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ColorInterpolationMode) -> ::windows_core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ColorInterpolationMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GradientStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GradientStops: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetGradientStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetGradientStops: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrushFactory {
    type Vtable = IGradientBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed4779ca_45bd_4131_b625_be86e07c6112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrushStatics {
    type Vtable = IGradientBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x961661f9_8bb4_4e6c_b923_b5d787e0f1a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SpreadMethodProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MappingModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ColorInterpolationModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GradientStopsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientStop(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientStop {
    type Vtable = IGradientStop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x665f44fe_2e59_4c4a_ab53_076a100ccd81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGradientStopStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientStopStatics {
    type Vtable = IGradientStopStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x602a6d75_6193_4fe5_8e82_c7c6f6febafd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStopStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageBrush {
    type Vtable = IImageBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fd11377_c12a_4493_bf7d_f3a8ad74b554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ImageSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetImageSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ImageFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveImageFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ImageOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveImageOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageBrushStatics {
    type Vtable = IImageBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1255b1b2_dd18_42e5_892c_eae30c305b8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ImageSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageSource {
    type Vtable = IImageSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x737ef309_ea41_4d96_a71c_98e98efcab07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageSourceFactory {
    type Vtable = IImageSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x297ec001_2540_4e5a_ab66_88035dd3ddb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineGeometry {
    type Vtable = ILineGeometry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30edd4a2_8fc5_40af_a7a2_c27fe7aa1363);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub EndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineGeometryStatics {
    type Vtable = ILineGeometryStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x578ae763_5562_4ee4_8703_ea4036d891e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineSegment {
    type Vtable = ILineSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef6a2e25_3ff0_4420_a411_7182a4cecb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineSegmentStatics {
    type Vtable = ILineSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fcab141_04c0_4afb_87b3_e800b969b894);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrush {
    type Vtable = ILinearGradientBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e96d16b_bb84_4c6f_9dbf_9d6c5c6d9c39);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub EndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrushFactory {
    type Vtable = ILinearGradientBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ae0861c_1e7a_4fed_9857_ea8caa798490);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateInstanceWithGradientStopCollectionAndAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstopcollection: ::windows_core::RawPtr, angle: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateInstanceWithGradientStopCollectionAndAngle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrushStatics {
    type Vtable = ILinearGradientBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7af6e504_2dc3_40e3_be0b_b314c13cb991);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoadedImageSourceLoadCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ac60b1e_7837_4489_b3e5_d0d5ad0a56c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSourceLoadCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LoadedImageSourceLoadStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoadedImageSurface(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSurface {
    type Vtable = ILoadedImageSurface_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x050c8313_6737_45ba_8531_33094febef55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurface_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DecodedPhysicalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub DecodedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub NaturalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub LoadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLoadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoadedImageSurfaceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSurfaceStatics {
    type Vtable = ILoadedImageSurfaceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22b8edf6_84ad_40ab_937d_4871613e765d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurfaceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartLoadFromUriWithSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, desiredmaxsize: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartLoadFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub StartLoadFromStreamWithSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, desiredmaxsize: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    StartLoadFromStreamWithSize: usize,
    #[cfg(feature = "winrt-storage")]
    pub StartLoadFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    StartLoadFromStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DProjection {
    type Vtable = IMatrix3DProjection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f03e149_bfc9_4c01_b578_50338cec97fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub ProjectionMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Media3D::Matrix3D) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ProjectionMatrix: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetProjectionMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Media3D::Matrix3D) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetProjectionMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DProjectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DProjectionStatics {
    type Vtable = IMatrix3DProjectionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae9d5895_41ec_4e37_abaa_69f41d2f876b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixHelper {
    type Vtable = IMatrixHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3cf4882_06b5_48c8_9eb2_1763e9364038);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixHelperStatics {
    type Vtable = IMatrixHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc18606a6_39f4_4b8a_8403_28e5e5f033b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows_core::HRESULT,
    pub FromElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64, result__: *mut Matrix) -> ::windows_core::HRESULT,
    pub GetIsIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix, point: ::winrt_foundation::Point, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixTransform {
    type Vtable = IMatrixTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedfdd551_5fed_45fc_ae62_92a4b6cf9707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Matrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Matrix) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrixTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixTransformStatics {
    type Vtable = IMatrixTransformStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43e02e47_15b8_4758_bb97_7d52420acc5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MatrixProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4a8b21c_e3c2_485c_ae69_f1537b76755a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub SetThumbnailImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetThumbnailImage: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPartialMediaFailureDetectedEventArgs {
    type Vtable = IPartialMediaFailureDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02b65a91_e5a1_442b_88d3_2dc127bfc59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub StreamKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_media::Playback::FailedMediaStreamKind) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    StreamKind: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPartialMediaFailureDetectedEventArgs2 {
    type Vtable = IPartialMediaFailureDetectedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73074875_890d_416b_b9ae_e84dfd9c4b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartialMediaFailureDetectedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathFigure(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathFigure {
    type Vtable = IPathFigure_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d955c8c_5fa9_4dda_a3cc_10fcdcaa20d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigure_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Segments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Segments: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetSegments: usize,
    pub StartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathFigureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathFigureStatics {
    type Vtable = IPathFigureStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb60591d9_2395_4317_9552_3a58526f8c7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigureStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SegmentsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsClosedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsFilledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathGeometry {
    type Vtable = IPathGeometry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x081b9df8_bae6_4bcb_813c_bde0e46dc8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FillRule) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FillRule) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Figures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Figures: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetFigures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetFigures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathGeometryStatics {
    type Vtable = IPathGeometryStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9e58bba_2cba_4741_8f8d_3198cf5186b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FiguresProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathSegment {
    type Vtable = IPathSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcfa71cf_9ce3_474f_8157_10b6435a616b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathSegmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathSegmentFactory {
    type Vtable = IPathSegmentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a1c0aae_eccd_4464_a148_6ffdb3aa281f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaneProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaneProjection {
    type Vtable = IPlaneProjection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6f82bfa_6726_469a_b259_a5188347ca8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLocalOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub LocalOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLocalOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub LocalOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLocalOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RotationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RotationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RotationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterOfRotationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterOfRotationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterOfRotationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub GlobalOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub GlobalOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub GlobalOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ProjectionMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Media3D::Matrix3D) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ProjectionMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaneProjectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaneProjectionStatics {
    type Vtable = IPlaneProjectionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad919c67_3bdc_4855_8969_d1f9a3adc27d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalOffsetXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalOffsetYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalOffsetZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterOfRotationXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterOfRotationYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterOfRotationZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GlobalOffsetXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GlobalOffsetYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GlobalOffsetZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyBezierSegment {
    type Vtable = IPolyBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36805271_38c4_4bcf_96cd_028a6d38af25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Points: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyBezierSegmentStatics {
    type Vtable = IPolyBezierSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d91a6da_1492_4acc_bd66_a496f3d829d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyLineSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyLineSegment {
    type Vtable = IPolyLineSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b397f87_a2e6_479d_bdc8_6f4464646887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Points: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyLineSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyLineSegmentStatics {
    type Vtable = IPolyLineSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd64a2c87_33f1_4e70_a47f_b4981ef648a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd5ced7d_e6db_4c96_b6a1_3fce96e987a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Points: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyQuadraticBezierSegmentStatics {
    type Vtable = IPolyQuadraticBezierSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdf5eb75_7ad5_4c89_8169_8c9786abd9eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProjection {
    type Vtable = IProjection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3443557_7f39_4d04_a89c_844338cac897);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProjectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProjectionFactory {
    type Vtable = IProjectionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4f29cab_60ad_4f24_bd27_9d69c3127c9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuadraticBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c509a5b_bf18_455a_a078_914b5232d8af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Point2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPoint2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuadraticBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticBezierSegmentStatics {
    type Vtable = IQuadraticBezierSegmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69c78278_3c0b_4b4f_b7a2_f003ded41bb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point1Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRateChangedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRateChangedRoutedEventArgs {
    type Vtable = IRateChangedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9016aa6f_3ca8_4c80_8e2f_8851a68f131f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRateChangedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRectangleGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectangleGeometry {
    type Vtable = IRectangleGeometry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa25a1f58_c575_4196_91cf_9fdfb10445c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Rect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRectangleGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectangleGeometryStatics {
    type Vtable = IRectangleGeometryStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x377f8dba_7902_48e3_83be_7c8002a6653c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRenderedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderedEventArgs {
    type Vtable = IRenderedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe349817d_81c7_4938_828c_a7e2797b35a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRenderingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderingEventArgs {
    type Vtable = IRenderingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bf7d30d_9748_4aed_8380_d7890eb776a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RenderingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBackgroundBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBackgroundBrush {
    type Vtable = IRevealBackgroundBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x261dcc0e_1991_4cdf_aee0_6350a3f90bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBackgroundBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBackgroundBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBackgroundBrushFactory {
    type Vtable = IRevealBackgroundBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c56bcaa_02a5_4f45_8506_8d39228f5d3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBackgroundBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBorderBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBorderBrush {
    type Vtable = IRevealBorderBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x060ba115_c542_483c_8202_5f03331866c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBorderBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBorderBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBorderBrushFactory {
    type Vtable = IRevealBorderBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94c25298_f5f8_4482_a25c_6758501a8626);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBorderBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBrush {
    type Vtable = IRevealBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2036a0ed_8271_4398_9019_25872093f13f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT,
    pub TargetTheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ApplicationTheme) -> ::windows_core::HRESULT,
    pub SetTargetTheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ApplicationTheme) -> ::windows_core::HRESULT,
    pub AlwaysUseFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBrushFactory {
    type Vtable = IRevealBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d9379ce_e3a0_4aaf_be37_ea9d9dd43105);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevealBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevealBrushStatics {
    type Vtable = IRevealBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x190f2625_7209_4d42_a847_1ac4bbbb3499);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevealBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TargetThemeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: RevealBrushState) -> ::windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut RevealBrushState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRotateTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRotateTransform {
    type Vtable = IRotateTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x688ea9b9_1e4e_4596_86e3_428b27334faf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Angle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRotateTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRotateTransformStatics {
    type Vtable = IRotateTransformStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa131eb8a_51a3_41b6_b9d3_a10e429054ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AngleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScaleTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleTransform {
    type Vtable = IScaleTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed67f18d_936e_43ab_929a_e9cd0a511e52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScaleTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleTransformStatics {
    type Vtable = IScaleTransformStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d9436f4_40a7_46dd_975a_07d337cd852e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShadow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShadow {
    type Vtable = IShadow_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6813a583_f3b4_5fcf_8694_2cd0aefc2fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadow_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShadowFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShadowFactory {
    type Vtable = IShadowFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19899f25_d28b_51e6_94b0_d7e709686305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadowFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISkewTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISkewTransform {
    type Vtable = ISkewTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e8a3b15_7a0f_4617_9e98_1e65bdc92115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub AngleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAngleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub AngleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISkewTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISkewTransformStatics {
    type Vtable = ISkewTransformStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecd11d73_5614_4b31_b6af_beae10105624);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AngleXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AngleYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISolidColorBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrush {
    type Vtable = ISolidColorBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d850850_66f3_48df_9a8f_824bd5e070af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISolidColorBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrushFactory {
    type Vtable = ISolidColorBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd935ce0c_86f5_4da6_8a27_b1619ef7f92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: super::super::Color, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISolidColorBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrushStatics {
    type Vtable = ISolidColorBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1a65efa_2b23_41ba_b9ba_7094ec8e4e9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThemeShadow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeShadow {
    type Vtable = IThemeShadow_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3eccad09_7985_5f39_8b62_6c10696dca6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadow_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Receivers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThemeShadowFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeShadowFactory {
    type Vtable = IThemeShadowFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e71465d_0f67_590e_831b_7e5e2a32b778);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadowFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrush {
    type Vtable = ITileBrush_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc201cf06_cd84_48a5_9607_664d7361cd61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrush_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlignmentX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlignmentX) -> ::windows_core::HRESULT,
    pub SetAlignmentX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AlignmentX) -> ::windows_core::HRESULT,
    pub AlignmentY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlignmentY) -> ::windows_core::HRESULT,
    pub SetAlignmentY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AlignmentY) -> ::windows_core::HRESULT,
    pub Stretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Stretch) -> ::windows_core::HRESULT,
    pub SetStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Stretch) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrushFactory {
    type Vtable = ITileBrushFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa159f7c_ed6a_4fb3_b014_b5c7e379a4de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrushStatics {
    type Vtable = ITileBrushStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3497c25b_b562_4e68_8435_2399f6eb94d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlignmentXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AlignmentYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineMarker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineMarker {
    type Vtable = ITimelineMarker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa68ef02d_45ba_4e50_8cad_aaea3a227af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineMarkerRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineMarkerRoutedEventArgs {
    type Vtable = ITimelineMarkerRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c3b3ef3_2c88_4d9c_99b6_46cdbd48d4c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarkerRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Marker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineMarkerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineMarkerStatics {
    type Vtable = ITimelineMarkerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4aef0c6_16a3_484b_87f5_6528b8f04a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineMarkerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransform {
    type Vtable = ITransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4df74078_bfd6_4ed1_9682_d2fd8bf2fe6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformFactory {
    type Vtable = ITransformFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a955a66_7cf4_4320_b416_6181192fcc6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformGroup {
    type Vtable = ITransformGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63418ccc_8d2d_4737_b951_2afce1ddc4c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Children: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetChildren: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Matrix) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformGroupStatics {
    type Vtable = ITransformGroupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25312f2a_cfab_4b24_9713_5bdead1929c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITranslateTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITranslateTransform {
    type Vtable = ITranslateTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc975905c_3c36_4229_817b_178f64c0e113);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub X: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITranslateTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITranslateTransformStatics {
    type Vtable = ITranslateTransformStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf419aa91_e042_4111_9c2f_d201304123dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub XProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub YProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelper {
    type Vtable = IVisualTreeHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24b935e3_52c7_4141_8bac_a73d06130569);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics {
    type Vtable = IVisualTreeHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe75758c4_d25d_4b1d_971f_596f17f12baa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub FindElementsInHostCoordinatesPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intersectingpoint: ::winrt_foundation::Point, subtree: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindElementsInHostCoordinatesPoint: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindElementsInHostCoordinatesRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intersectingrect: ::winrt_foundation::Rect, subtree: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindElementsInHostCoordinatesRect: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllElementsInHostCoordinatesPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intersectingpoint: ::winrt_foundation::Point, subtree: ::windows_core::RawPtr, includeallelements: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllElementsInHostCoordinatesPoint: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllElementsInHostCoordinatesRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intersectingrect: ::winrt_foundation::Rect, subtree: ::windows_core::RawPtr, includeallelements: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllElementsInHostCoordinatesRect: usize,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: ::windows_core::RawPtr, childindex: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetChildrenCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisconnectChildrenRecursive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelperStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics2 {
    type Vtable = IVisualTreeHelperStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07bcd176_869f_44a7_8797_2103a4c3e47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-ui"))]
    pub GetOpenPopups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-ui")))]
    GetOpenPopups: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualTreeHelperStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics3 {
    type Vtable = IVisualTreeHelperStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40420d50_ca16_57da_8aac_944c8af577fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-ui"))]
    pub GetOpenPopupsForXamlRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamlroot: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-ui")))]
    GetOpenPopupsForXamlRoot: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03e432d9_b35c_4a79_811c_c5652004da0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FallbackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseFactory {
    type Vtable = IXamlCompositionBrushBaseFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x394f0823_2451_4ed8_bd24_488149b3428d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseOverrides {
    type Vtable = IXamlCompositionBrushBaseOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd19127f1_38b4_4ea1_8f33_849629a4c9c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseProtected {
    type Vtable = IXamlCompositionBrushBaseProtected_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1513f3d8_0457_4e1c_ad77_11c1d9879743);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseProtected_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub CompositionBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CompositionBrush: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetCompositionBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetCompositionBrush: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseStatics {
    type Vtable = IXamlCompositionBrushBaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fd49b06_061a_441f_b97a_adfbd41ae681);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FallbackColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLight(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLight {
    type Vtable = IXamlLight_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cc3fc1f_b327_4a18_9648_7c84db26ce22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLight_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightFactory {
    type Vtable = IXamlLightFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87ded768_3055_43b8_8ef6_798dc4c2329a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightOverrides {
    type Vtable = IXamlLightOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c6296c7_0173_48e1_b73d_7fa216a9ac28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OnConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newelement: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldelement: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightProtected {
    type Vtable = IXamlLightProtected_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ecf220b_1252_43d0_9729_6ea692046838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightProtected_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub CompositionLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CompositionLight: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetCompositionLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetCompositionLight: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlLightStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightStatics {
    type Vtable = IXamlLightStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5ea9d69_b508_4e9c_bd27_6b044b5f78a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddTargetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, element: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveTargetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, element: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddTargetBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, brush: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveTargetBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, brush: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ImageBrush(::windows_core::IUnknown);
impl ImageBrush {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ImageBrush, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ImageSource(&self) -> ::windows_core::Result<ImageSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageSource>(result__)
        }
    }
    pub fn SetImageSource<'a, Param0: ::windows_core::IntoParam<'a, ImageSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImageSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImageFailed<'a, Param0: ::windows_core::IntoParam<'a, super::ExceptionRoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ImageFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImageFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveImageFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ImageOpened<'a, Param0: ::windows_core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ImageOpened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImageOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveImageOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ImageSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IImageBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageSourceProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IImageBrushStatics<R, F: FnOnce(&IImageBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ImageBrush, IImageBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageBrush {}
impl ::core::fmt::Debug for ImageBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ImageBrush;{9fd11377-c12a-4493-bf7d-f3a8ad74b554})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageBrush {
    type Vtable = IImageBrush_Vtbl;
    const IID: ::windows_core::GUID = <IImageBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageBrush";
}
impl ::core::convert::From<ImageBrush> for ::windows_core::IUnknown {
    fn from(value: ImageBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageBrush> for ::windows_core::IUnknown {
    fn from(value: &ImageBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageBrush> for ::windows_core::IInspectable {
    fn from(value: ImageBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageBrush> for ::windows_core::IInspectable {
    fn from(value: &ImageBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<ImageBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&ImageBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ImageBrush> for TileBrush {
    fn from(value: ImageBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ImageBrush> for TileBrush {
    fn from(value: &ImageBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, TileBrush> for ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, TileBrush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, TileBrush> for &ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, TileBrush> {
        ::windows_core::Param::Owned(::core::convert::Into::<TileBrush>::into(self))
    }
}
impl ::core::convert::From<ImageBrush> for Brush {
    fn from(value: ImageBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ImageBrush> for Brush {
    fn from(value: &ImageBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<ImageBrush> for super::DependencyObject {
    fn from(value: ImageBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ImageBrush> for super::DependencyObject {
    fn from(value: &ImageBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &ImageBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ImageBrush {}
unsafe impl ::core::marker::Sync for ImageBrush {}
#[repr(transparent)]
pub struct ImageSource(::windows_core::IUnknown);
impl ImageSource {}
impl ::core::clone::Clone for ImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageSource {}
impl ::core::fmt::Debug for ImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ImageSource;{737ef309-ea41-4d96-a71c-98e98efcab07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageSource {
    type Vtable = IImageSource_Vtbl;
    const IID: ::windows_core::GUID = <IImageSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageSource";
}
impl ::core::convert::From<ImageSource> for ::windows_core::IUnknown {
    fn from(value: ImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageSource> for ::windows_core::IUnknown {
    fn from(value: &ImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageSource> for ::windows_core::IInspectable {
    fn from(value: ImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageSource> for ::windows_core::IInspectable {
    fn from(value: &ImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageSource> for super::DependencyObject {
    fn from(value: ImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ImageSource> for super::DependencyObject {
    fn from(value: &ImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for ImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &ImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ImageSource {}
unsafe impl ::core::marker::Sync for ImageSource {}
#[repr(transparent)]
pub struct LineGeometry(::windows_core::IUnknown);
impl LineGeometry {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LineGeometry, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StartPoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).StartPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetStartPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndPoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).EndPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetEndPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILineGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn EndPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILineGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndPointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILineGeometryStatics<R, F: FnOnce(&ILineGeometryStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LineGeometry, ILineGeometryStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LineGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineGeometry {}
impl ::core::fmt::Debug for LineGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineGeometry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LineGeometry;{30edd4a2-8fc5-40af-a7a2-c27fe7aa1363})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineGeometry {
    type Vtable = ILineGeometry_Vtbl;
    const IID: ::windows_core::GUID = <ILineGeometry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LineGeometry";
}
impl ::core::convert::From<LineGeometry> for ::windows_core::IUnknown {
    fn from(value: LineGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineGeometry> for ::windows_core::IUnknown {
    fn from(value: &LineGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineGeometry> for ::windows_core::IInspectable {
    fn from(value: LineGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineGeometry> for ::windows_core::IInspectable {
    fn from(value: &LineGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineGeometry> for Geometry {
    fn from(value: LineGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineGeometry> for Geometry {
    fn from(value: &LineGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for &LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl ::core::convert::From<LineGeometry> for super::DependencyObject {
    fn from(value: LineGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineGeometry> for super::DependencyObject {
    fn from(value: &LineGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &LineGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LineGeometry {}
unsafe impl ::core::marker::Sync for LineGeometry {}
#[repr(transparent)]
pub struct LineSegment(::windows_core::IUnknown);
impl LineSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LineSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Point(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILineSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILineSegmentStatics<R, F: FnOnce(&ILineSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LineSegment, ILineSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LineSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineSegment {}
impl ::core::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LineSegment;{ef6a2e25-3ff0-4420-a411-7182a4cecb15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineSegment {
    type Vtable = ILineSegment_Vtbl;
    const IID: ::windows_core::GUID = <ILineSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LineSegment";
}
impl ::core::convert::From<LineSegment> for ::windows_core::IUnknown {
    fn from(value: LineSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineSegment> for ::windows_core::IUnknown {
    fn from(value: &LineSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineSegment> for ::windows_core::IInspectable {
    fn from(value: LineSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineSegment> for ::windows_core::IInspectable {
    fn from(value: &LineSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineSegment> for PathSegment {
    fn from(value: LineSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineSegment> for PathSegment {
    fn from(value: &LineSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<LineSegment> for super::DependencyObject {
    fn from(value: LineSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineSegment> for super::DependencyObject {
    fn from(value: &LineSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &LineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LineSegment {}
unsafe impl ::core::marker::Sync for LineSegment {}
#[repr(transparent)]
pub struct LinearGradientBrush(::windows_core::IUnknown);
impl LinearGradientBrush {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LinearGradientBrush, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StartPoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).StartPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetStartPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndPoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).EndPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetEndPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateInstanceWithGradientStopCollectionAndAngle<'a, Param0: ::windows_core::IntoParam<'a, GradientStopCollection>>(gradientstopcollection: Param0, angle: f64) -> ::windows_core::Result<LinearGradientBrush> {
        Self::ILinearGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithGradientStopCollectionAndAngle)(::windows_core::Interface::as_raw(this), gradientstopcollection.into_param().abi(), angle, result__.as_mut_ptr()).from_abi::<LinearGradientBrush>(result__)
        })
    }
    pub fn StartPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn EndPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndPointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ILinearGradientBrushFactory<R, F: FnOnce(&ILinearGradientBrushFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LinearGradientBrush, ILinearGradientBrushFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILinearGradientBrushStatics<R, F: FnOnce(&ILinearGradientBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LinearGradientBrush, ILinearGradientBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearGradientBrush {}
impl ::core::fmt::Debug for LinearGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearGradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LinearGradientBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LinearGradientBrush;{8e96d16b-bb84-4c6f-9dbf-9d6c5c6d9c39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LinearGradientBrush {
    type Vtable = ILinearGradientBrush_Vtbl;
    const IID: ::windows_core::GUID = <ILinearGradientBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LinearGradientBrush";
}
impl ::core::convert::From<LinearGradientBrush> for ::windows_core::IUnknown {
    fn from(value: LinearGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearGradientBrush> for ::windows_core::IUnknown {
    fn from(value: &LinearGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearGradientBrush> for ::windows_core::IInspectable {
    fn from(value: LinearGradientBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearGradientBrush> for ::windows_core::IInspectable {
    fn from(value: &LinearGradientBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<LinearGradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: LinearGradientBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&LinearGradientBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &LinearGradientBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<LinearGradientBrush> for GradientBrush {
    fn from(value: LinearGradientBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearGradientBrush> for GradientBrush {
    fn from(value: &LinearGradientBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GradientBrush> for LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, GradientBrush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GradientBrush> for &LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, GradientBrush> {
        ::windows_core::Param::Owned(::core::convert::Into::<GradientBrush>::into(self))
    }
}
impl ::core::convert::From<LinearGradientBrush> for Brush {
    fn from(value: LinearGradientBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearGradientBrush> for Brush {
    fn from(value: &LinearGradientBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<LinearGradientBrush> for super::DependencyObject {
    fn from(value: LinearGradientBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearGradientBrush> for super::DependencyObject {
    fn from(value: &LinearGradientBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &LinearGradientBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearGradientBrush {}
unsafe impl ::core::marker::Sync for LinearGradientBrush {}
#[repr(transparent)]
pub struct LoadedImageSourceLoadCompletedEventArgs(::windows_core::IUnknown);
impl LoadedImageSourceLoadCompletedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<LoadedImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LoadedImageSourceLoadStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LoadedImageSourceLoadStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for LoadedImageSourceLoadCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LoadedImageSourceLoadCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoadedImageSourceLoadCompletedEventArgs {}
impl ::core::fmt::Debug for LoadedImageSourceLoadCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSourceLoadCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LoadedImageSourceLoadCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs;{1ac60b1e-7837-4489-b3e5-d0d5ad0a56c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILoadedImageSourceLoadCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LoadedImageSourceLoadCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs";
}
impl ::core::convert::From<LoadedImageSourceLoadCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LoadedImageSourceLoadCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoadedImageSourceLoadCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LoadedImageSourceLoadCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoadedImageSourceLoadCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LoadedImageSourceLoadCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoadedImageSourceLoadCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LoadedImageSourceLoadCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LoadedImageSourceLoadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LoadedImageSourceLoadCompletedEventArgs {}
unsafe impl ::core::marker::Sync for LoadedImageSourceLoadCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for LoadedImageSourceLoadStatus {}
impl ::core::clone::Clone for LoadedImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoadedImageSourceLoadStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LoadedImageSourceLoadStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LoadedImageSourceLoadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSourceLoadStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LoadedImageSourceLoadStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.LoadedImageSourceLoadStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LoadedImageSurface(::windows_core::IUnknown);
impl LoadedImageSurface {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DecodedPhysicalSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).DecodedPhysicalSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn DecodedSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).DecodedSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn NaturalSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn LoadCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LoadCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLoadCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoadCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StartLoadFromUriWithSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(uri: Param0, desiredmaxsize: Param1) -> ::windows_core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromUriWithSize)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), desiredmaxsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<LoadedImageSurface>(result__)
        })
    }
    pub fn StartLoadFromUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<LoadedImageSurface>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn StartLoadFromStreamWithSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(stream: Param0, desiredmaxsize: Param1) -> ::windows_core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromStreamWithSize)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), desiredmaxsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<LoadedImageSurface>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn StartLoadFromStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows_core::Result<LoadedImageSurface> {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<LoadedImageSurface>(result__)
        })
    }
    pub fn ILoadedImageSurfaceStatics<R, F: FnOnce(&ILoadedImageSurfaceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LoadedImageSurface, ILoadedImageSurfaceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoadedImageSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LoadedImageSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoadedImageSurface {}
impl ::core::fmt::Debug for LoadedImageSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LoadedImageSurface {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.LoadedImageSurface;{050c8313-6737-45ba-8531-33094febef55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LoadedImageSurface {
    type Vtable = ILoadedImageSurface_Vtbl;
    const IID: ::windows_core::GUID = <ILoadedImageSurface as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LoadedImageSurface {
    const NAME: &'static str = "Windows.UI.Xaml.Media.LoadedImageSurface";
}
impl ::core::convert::From<LoadedImageSurface> for ::windows_core::IUnknown {
    fn from(value: LoadedImageSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoadedImageSurface> for ::windows_core::IUnknown {
    fn from(value: &LoadedImageSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoadedImageSurface> for ::windows_core::IInspectable {
    fn from(value: LoadedImageSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoadedImageSurface> for ::windows_core::IInspectable {
    fn from(value: &LoadedImageSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LoadedImageSurface> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: LoadedImageSurface) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoadedImageSurface> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoadedImageSurface) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<LoadedImageSurface> for super::super::Composition::ICompositionSurface {
    type Error = ::windows_core::Error;
    fn try_from(value: LoadedImageSurface) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&LoadedImageSurface> for super::super::Composition::ICompositionSurface {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoadedImageSurface) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::ICompositionSurface> for LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::ICompositionSurface> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::ICompositionSurface> for &LoadedImageSurface {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::ICompositionSurface> {
        ::core::convert::TryInto::<super::super::Composition::ICompositionSurface>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoadedImageSurface {}
unsafe impl ::core::marker::Sync for LoadedImageSurface {}
#[repr(C)]
pub struct Matrix {
    pub M11: f64,
    pub M12: f64,
    pub M21: f64,
    pub M22: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
}
impl ::core::marker::Copy for Matrix {}
impl ::core::clone::Clone for Matrix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Matrix").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("OffsetX", &self.OffsetX).field("OffsetY", &self.OffsetY).finish()
    }
}
unsafe impl ::windows_core::Abi for Matrix {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for Matrix {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Matrix;f8;f8;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Matrix>()) == 0 }
    }
}
impl ::core::cmp::Eq for Matrix {}
impl ::core::default::Default for Matrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct Matrix3DProjection(::windows_core::IUnknown);
impl Matrix3DProjection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Matrix3DProjection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ProjectionMatrix(&self) -> ::windows_core::Result<Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Media3D::Matrix3D>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Media3D::Matrix3D>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetProjectionMatrix<'a, Param0: ::windows_core::IntoParam<'a, Media3D::Matrix3D>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProjectionMatrix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProjectionMatrixProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IMatrix3DProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrixProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IMatrix3DProjectionStatics<R, F: FnOnce(&IMatrix3DProjectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Matrix3DProjection, IMatrix3DProjectionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Matrix3DProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Matrix3DProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Matrix3DProjection {}
impl ::core::fmt::Debug for Matrix3DProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Matrix3DProjection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Matrix3DProjection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Matrix3DProjection;{6f03e149-bfc9-4c01-b578-50338cec97fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Matrix3DProjection {
    type Vtable = IMatrix3DProjection_Vtbl;
    const IID: ::windows_core::GUID = <IMatrix3DProjection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Matrix3DProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Matrix3DProjection";
}
impl ::core::convert::From<Matrix3DProjection> for ::windows_core::IUnknown {
    fn from(value: Matrix3DProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Matrix3DProjection> for ::windows_core::IUnknown {
    fn from(value: &Matrix3DProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Matrix3DProjection> for ::windows_core::IInspectable {
    fn from(value: Matrix3DProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Matrix3DProjection> for ::windows_core::IInspectable {
    fn from(value: &Matrix3DProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Matrix3DProjection> for Projection {
    fn from(value: Matrix3DProjection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Matrix3DProjection> for Projection {
    fn from(value: &Matrix3DProjection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Projection> for Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, Projection> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Projection> for &Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, Projection> {
        ::windows_core::Param::Owned(::core::convert::Into::<Projection>::into(self))
    }
}
impl ::core::convert::From<Matrix3DProjection> for super::DependencyObject {
    fn from(value: Matrix3DProjection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Matrix3DProjection> for super::DependencyObject {
    fn from(value: &Matrix3DProjection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &Matrix3DProjection {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Matrix3DProjection {}
unsafe impl ::core::marker::Sync for Matrix3DProjection {}
#[repr(transparent)]
pub struct MatrixHelper(::windows_core::IUnknown);
impl MatrixHelper {
    pub fn Identity() -> ::windows_core::Result<Matrix> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix>::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Matrix>(result__)
        })
    }
    pub fn FromElements(m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64) -> ::windows_core::Result<Matrix> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix>::zeroed();
            (::windows_core::Interface::vtable(this).FromElements)(::windows_core::Interface::as_raw(this), m11, m12, m21, m22, offsetx, offsety, result__.as_mut_ptr()).from_abi::<Matrix>(result__)
        })
    }
    pub fn GetIsIdentity<'a, Param0: ::windows_core::IntoParam<'a, Matrix>>(target: Param0) -> ::windows_core::Result<bool> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsIdentity)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn Transform<'a, Param0: ::windows_core::IntoParam<'a, Matrix>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(target: Param0, point: Param1) -> ::windows_core::Result<::winrt_foundation::Point> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(::windows_core::Interface::as_raw(this), target.into_param().abi(), point.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        })
    }
    pub fn IMatrixHelperStatics<R, F: FnOnce(&IMatrixHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MatrixHelper, IMatrixHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MatrixHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MatrixHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MatrixHelper {}
impl ::core::fmt::Debug for MatrixHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MatrixHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MatrixHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.MatrixHelper;{f3cf4882-06b5-48c8-9eb2-1763e9364038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MatrixHelper {
    type Vtable = IMatrixHelper_Vtbl;
    const IID: ::windows_core::GUID = <IMatrixHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MatrixHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MatrixHelper";
}
impl ::core::convert::From<MatrixHelper> for ::windows_core::IUnknown {
    fn from(value: MatrixHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MatrixHelper> for ::windows_core::IUnknown {
    fn from(value: &MatrixHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MatrixHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MatrixHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MatrixHelper> for ::windows_core::IInspectable {
    fn from(value: MatrixHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MatrixHelper> for ::windows_core::IInspectable {
    fn from(value: &MatrixHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MatrixHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MatrixHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MatrixHelper {}
unsafe impl ::core::marker::Sync for MatrixHelper {}
#[repr(transparent)]
pub struct MatrixTransform(::windows_core::IUnknown);
impl MatrixTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MatrixTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Matrix(&self) -> ::windows_core::Result<Matrix> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix>::zeroed();
            (::windows_core::Interface::vtable(this).Matrix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Matrix>(result__)
        }
    }
    pub fn SetMatrix<'a, Param0: ::windows_core::IntoParam<'a, Matrix>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMatrix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MatrixProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IMatrixTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MatrixProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IMatrixTransformStatics<R, F: FnOnce(&IMatrixTransformStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MatrixTransform, IMatrixTransformStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MatrixTransform {}
impl ::core::fmt::Debug for MatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MatrixTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MatrixTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.MatrixTransform;{edfdd551-5fed-45fc-ae62-92a4b6cf9707})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MatrixTransform {
    type Vtable = IMatrixTransform_Vtbl;
    const IID: ::windows_core::GUID = <IMatrixTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MatrixTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MatrixTransform";
}
impl ::core::convert::From<MatrixTransform> for ::windows_core::IUnknown {
    fn from(value: MatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MatrixTransform> for ::windows_core::IUnknown {
    fn from(value: &MatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MatrixTransform> for ::windows_core::IInspectable {
    fn from(value: MatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MatrixTransform> for ::windows_core::IInspectable {
    fn from(value: &MatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MatrixTransform> for Transform {
    fn from(value: MatrixTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MatrixTransform> for Transform {
    fn from(value: &MatrixTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<MatrixTransform> for GeneralTransform {
    fn from(value: MatrixTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MatrixTransform> for GeneralTransform {
    fn from(value: &MatrixTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<MatrixTransform> for super::DependencyObject {
    fn from(value: MatrixTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MatrixTransform> for super::DependencyObject {
    fn from(value: &MatrixTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &MatrixTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MatrixTransform {}
unsafe impl ::core::marker::Sync for MatrixTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCanPlayResponse(pub i32);
impl MediaCanPlayResponse {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCanPlayResponse {}
impl ::core::clone::Clone for MediaCanPlayResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCanPlayResponse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCanPlayResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCanPlayResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCanPlayResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCanPlayResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.MediaCanPlayResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaElementState(pub i32);
impl MediaElementState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaElementState {}
impl ::core::clone::Clone for MediaElementState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaElementState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaElementState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaElementState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaElementState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaElementState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.MediaElementState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaTransportControlsThumbnailRequestedEventArgs(::windows_core::IUnknown);
impl MediaTransportControlsThumbnailRequestedEventArgs {
    #[cfg(feature = "winrt-storage")]
    pub fn SetThumbnailImage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailImage)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaTransportControlsThumbnailRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaTransportControlsThumbnailRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTransportControlsThumbnailRequestedEventArgs {}
impl ::core::fmt::Debug for MediaTransportControlsThumbnailRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTransportControlsThumbnailRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaTransportControlsThumbnailRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs;{e4a8b21c-e3c2-485c-ae69-f1537b76755a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTransportControlsThumbnailRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaTransportControlsThumbnailRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs";
}
impl ::core::convert::From<MediaTransportControlsThumbnailRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTransportControlsThumbnailRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaTransportControlsThumbnailRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTransportControlsThumbnailRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaTransportControlsThumbnailRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaTransportControlsThumbnailRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaTransportControlsThumbnailRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTransportControlsThumbnailRequestedEventArgs {}
#[repr(transparent)]
pub struct PartialMediaFailureDetectedEventArgs(::windows_core::IUnknown);
impl PartialMediaFailureDetectedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PartialMediaFailureDetectedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-media")]
    pub fn StreamKind(&self) -> ::windows_core::Result<::winrt_media::Playback::FailedMediaStreamKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_media::Playback::FailedMediaStreamKind>::zeroed();
            (::windows_core::Interface::vtable(this).StreamKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::Playback::FailedMediaStreamKind>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPartialMediaFailureDetectedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PartialMediaFailureDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PartialMediaFailureDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PartialMediaFailureDetectedEventArgs {}
impl ::core::fmt::Debug for PartialMediaFailureDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PartialMediaFailureDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PartialMediaFailureDetectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PartialMediaFailureDetectedEventArgs;{02b65a91-e5a1-442b-88d3-2dc127bfc59b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PartialMediaFailureDetectedEventArgs {
    type Vtable = IPartialMediaFailureDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPartialMediaFailureDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PartialMediaFailureDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PartialMediaFailureDetectedEventArgs";
}
impl ::core::convert::From<PartialMediaFailureDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PartialMediaFailureDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PartialMediaFailureDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PartialMediaFailureDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PartialMediaFailureDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PartialMediaFailureDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PartialMediaFailureDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PartialMediaFailureDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PartialMediaFailureDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PartialMediaFailureDetectedEventArgs {}
unsafe impl ::core::marker::Sync for PartialMediaFailureDetectedEventArgs {}
#[repr(transparent)]
pub struct PathFigure(::windows_core::IUnknown);
impl PathFigure {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PathFigure, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Segments(&self) -> ::windows_core::Result<PathSegmentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Segments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PathSegmentCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetSegments<'a, Param0: ::windows_core::IntoParam<'a, PathSegmentCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSegments)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartPoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).StartPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetStartPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsClosed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsClosed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFilled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFilled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsFilled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsFilled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SegmentsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SegmentsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StartPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsClosedProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsClosedProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsFilledProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsFilledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPathFigureStatics<R, F: FnOnce(&IPathFigureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PathFigure, IPathFigureStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PathFigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PathFigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathFigure {}
impl ::core::fmt::Debug for PathFigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathFigure").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PathFigure {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathFigure;{5d955c8c-5fa9-4dda-a3cc-10fcdcaa20d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PathFigure {
    type Vtable = IPathFigure_Vtbl;
    const IID: ::windows_core::GUID = <IPathFigure as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PathFigure {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathFigure";
}
impl ::core::convert::From<PathFigure> for ::windows_core::IUnknown {
    fn from(value: PathFigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PathFigure> for ::windows_core::IUnknown {
    fn from(value: &PathFigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PathFigure {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PathFigure {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PathFigure> for ::windows_core::IInspectable {
    fn from(value: PathFigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PathFigure> for ::windows_core::IInspectable {
    fn from(value: &PathFigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PathFigure {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PathFigure {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PathFigure> for super::DependencyObject {
    fn from(value: PathFigure) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PathFigure> for super::DependencyObject {
    fn from(value: &PathFigure) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PathFigure {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PathFigure {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PathFigure {}
unsafe impl ::core::marker::Sync for PathFigure {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PathFigureCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PathFigureCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PathFigureCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<PathFigure>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<PathFigure>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<PathFigure>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<PathFigure> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<PathFigure>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PathFigure>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PathFigure>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, PathFigure>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, PathFigure>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, PathFigure>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, PathFigure>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<PathFigure>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<PathFigure>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PathFigureCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PathFigureCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PathFigureCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PathFigureCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathFigureCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PathFigureCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathFigureCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.PathFigure;{5d955c8c-5fa9-4dda-a3cc-10fcdcaa20d7})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PathFigureCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<PathFigure>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<PathFigure> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PathFigureCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathFigureCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PathFigureCollection> for ::windows_core::IUnknown {
    fn from(value: PathFigureCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PathFigureCollection> for ::windows_core::IUnknown {
    fn from(value: &PathFigureCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PathFigureCollection> for ::windows_core::IInspectable {
    fn from(value: PathFigureCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PathFigureCollection> for ::windows_core::IInspectable {
    fn from(value: &PathFigureCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PathFigureCollection> for ::winrt_foundation::Collections::IIterable<PathFigure> {
    type Error = ::windows_core::Error;
    fn try_from(value: PathFigureCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PathFigureCollection> for ::winrt_foundation::Collections::IIterable<PathFigure> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PathFigureCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PathFigure>> for PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PathFigure>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PathFigure>> for &PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PathFigure>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<PathFigure>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PathFigureCollection> for ::winrt_foundation::Collections::IVector<PathFigure> {
    type Error = ::windows_core::Error;
    fn try_from(value: PathFigureCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PathFigureCollection> for ::winrt_foundation::Collections::IVector<PathFigure> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PathFigureCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PathFigure>> for PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<PathFigure>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PathFigure>> for &PathFigureCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<PathFigure>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<PathFigure>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for PathFigureCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for PathFigureCollection {}
#[repr(transparent)]
pub struct PathGeometry(::windows_core::IUnknown);
impl PathGeometry {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PathGeometry, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FillRule(&self) -> ::windows_core::Result<FillRule> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FillRule>::zeroed();
            (::windows_core::Interface::vtable(this).FillRule)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FillRule>(result__)
        }
    }
    pub fn SetFillRule(&self, value: FillRule) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFillRule)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Figures(&self) -> ::windows_core::Result<PathFigureCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Figures)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PathFigureCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetFigures<'a, Param0: ::windows_core::IntoParam<'a, PathFigureCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFigures)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FillRuleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillRuleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FiguresProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FiguresProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPathGeometryStatics<R, F: FnOnce(&IPathGeometryStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PathGeometry, IPathGeometryStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PathGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PathGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathGeometry {}
impl ::core::fmt::Debug for PathGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PathGeometry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathGeometry;{081b9df8-bae6-4bcb-813c-bde0e46dc8b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PathGeometry {
    type Vtable = IPathGeometry_Vtbl;
    const IID: ::windows_core::GUID = <IPathGeometry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PathGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathGeometry";
}
impl ::core::convert::From<PathGeometry> for ::windows_core::IUnknown {
    fn from(value: PathGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PathGeometry> for ::windows_core::IUnknown {
    fn from(value: &PathGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PathGeometry> for ::windows_core::IInspectable {
    fn from(value: PathGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PathGeometry> for ::windows_core::IInspectable {
    fn from(value: &PathGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PathGeometry> for Geometry {
    fn from(value: PathGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PathGeometry> for Geometry {
    fn from(value: &PathGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for &PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl ::core::convert::From<PathGeometry> for super::DependencyObject {
    fn from(value: PathGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PathGeometry> for super::DependencyObject {
    fn from(value: &PathGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PathGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PathGeometry {}
unsafe impl ::core::marker::Sync for PathGeometry {}
#[repr(transparent)]
pub struct PathSegment(::windows_core::IUnknown);
impl PathSegment {}
impl ::core::clone::Clone for PathSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PathSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PathSegment {}
impl ::core::fmt::Debug for PathSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PathSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathSegment;{fcfa71cf-9ce3-474f-8157-10b6435a616b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PathSegment {
    type Vtable = IPathSegment_Vtbl;
    const IID: ::windows_core::GUID = <IPathSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PathSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathSegment";
}
impl ::core::convert::From<PathSegment> for ::windows_core::IUnknown {
    fn from(value: PathSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PathSegment> for ::windows_core::IUnknown {
    fn from(value: &PathSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PathSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PathSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PathSegment> for ::windows_core::IInspectable {
    fn from(value: PathSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PathSegment> for ::windows_core::IInspectable {
    fn from(value: &PathSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PathSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PathSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PathSegment> for super::DependencyObject {
    fn from(value: PathSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PathSegment> for super::DependencyObject {
    fn from(value: &PathSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PathSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PathSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PathSegment {}
unsafe impl ::core::marker::Sync for PathSegment {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PathSegmentCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PathSegmentCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PathSegmentCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<PathSegment>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<PathSegment>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<PathSegment>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<PathSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<PathSegment>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PathSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PathSegment>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, PathSegment>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, PathSegment>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, PathSegment>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, PathSegment>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<PathSegment>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<PathSegment>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PathSegmentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PathSegmentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PathSegmentCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PathSegmentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PathSegmentCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PathSegmentCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PathSegmentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.PathSegment;{fcfa71cf-9ce3-474f-8157-10b6435a616b})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PathSegmentCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<PathSegment>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<PathSegment> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PathSegmentCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PathSegmentCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PathSegmentCollection> for ::windows_core::IUnknown {
    fn from(value: PathSegmentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PathSegmentCollection> for ::windows_core::IUnknown {
    fn from(value: &PathSegmentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PathSegmentCollection> for ::windows_core::IInspectable {
    fn from(value: PathSegmentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PathSegmentCollection> for ::windows_core::IInspectable {
    fn from(value: &PathSegmentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PathSegmentCollection> for ::winrt_foundation::Collections::IIterable<PathSegment> {
    type Error = ::windows_core::Error;
    fn try_from(value: PathSegmentCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PathSegmentCollection> for ::winrt_foundation::Collections::IIterable<PathSegment> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PathSegmentCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PathSegment>> for PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PathSegment>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PathSegment>> for &PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PathSegment>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<PathSegment>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PathSegmentCollection> for ::winrt_foundation::Collections::IVector<PathSegment> {
    type Error = ::windows_core::Error;
    fn try_from(value: PathSegmentCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PathSegmentCollection> for ::winrt_foundation::Collections::IVector<PathSegment> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PathSegmentCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PathSegment>> for PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<PathSegment>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PathSegment>> for &PathSegmentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<PathSegment>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<PathSegment>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for PathSegmentCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for PathSegmentCollection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for PenLineCap {}
impl ::core::clone::Clone for PenLineCap {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenLineCap {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PenLineCap {
    type Abi = Self;
}
impl ::core::fmt::Debug for PenLineCap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenLineCap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenLineCap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.PenLineCap;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
}
impl ::core::marker::Copy for PenLineJoin {}
impl ::core::clone::Clone for PenLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenLineJoin {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PenLineJoin {
    type Abi = Self;
}
impl ::core::fmt::Debug for PenLineJoin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenLineJoin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenLineJoin {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.PenLineJoin;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PlaneProjection(::windows_core::IUnknown);
impl PlaneProjection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlaneProjection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn LocalOffsetX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLocalOffsetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalOffsetX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LocalOffsetY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLocalOffsetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalOffsetY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LocalOffsetZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLocalOffsetZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalOffsetZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterOfRotationX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterOfRotationX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterOfRotationX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterOfRotationY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterOfRotationY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterOfRotationY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterOfRotationZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterOfRotationZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterOfRotationZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GlobalOffsetX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGlobalOffsetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGlobalOffsetX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GlobalOffsetY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGlobalOffsetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGlobalOffsetY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GlobalOffsetZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGlobalOffsetZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGlobalOffsetZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ProjectionMatrix(&self) -> ::windows_core::Result<Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Media3D::Matrix3D>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Media3D::Matrix3D>(result__)
        }
    }
    pub fn LocalOffsetXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LocalOffsetYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LocalOffsetZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn RotationZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterOfRotationXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterOfRotationYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterOfRotationZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GlobalOffsetXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GlobalOffsetYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GlobalOffsetZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ProjectionMatrixProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrixProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPlaneProjectionStatics<R, F: FnOnce(&IPlaneProjectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlaneProjection, IPlaneProjectionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PlaneProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaneProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaneProjection {}
impl ::core::fmt::Debug for PlaneProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaneProjection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaneProjection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PlaneProjection;{e6f82bfa-6726-469a-b259-a5188347ca8f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaneProjection {
    type Vtable = IPlaneProjection_Vtbl;
    const IID: ::windows_core::GUID = <IPlaneProjection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaneProjection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PlaneProjection";
}
impl ::core::convert::From<PlaneProjection> for ::windows_core::IUnknown {
    fn from(value: PlaneProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaneProjection> for ::windows_core::IUnknown {
    fn from(value: &PlaneProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaneProjection> for ::windows_core::IInspectable {
    fn from(value: PlaneProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaneProjection> for ::windows_core::IInspectable {
    fn from(value: &PlaneProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaneProjection> for Projection {
    fn from(value: PlaneProjection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PlaneProjection> for Projection {
    fn from(value: &PlaneProjection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Projection> for PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, Projection> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Projection> for &PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, Projection> {
        ::windows_core::Param::Owned(::core::convert::Into::<Projection>::into(self))
    }
}
impl ::core::convert::From<PlaneProjection> for super::DependencyObject {
    fn from(value: PlaneProjection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PlaneProjection> for super::DependencyObject {
    fn from(value: &PlaneProjection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PlaneProjection {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PlaneProjection {}
unsafe impl ::core::marker::Sync for PlaneProjection {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PointCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PointCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Point>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::winrt_foundation::Point]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::winrt_foundation::Point]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PointCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PointCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PointCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PointCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PointCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PointCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};struct(Windows.Foundation.Point;f4;f4)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PointCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<::winrt_foundation::Point>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<::winrt_foundation::Point> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PointCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PointCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PointCollection {
    type Item = ::winrt_foundation::Point;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PointCollection {
    type Item = ::winrt_foundation::Point;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PointCollection> for ::windows_core::IUnknown {
    fn from(value: PointCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PointCollection> for ::windows_core::IUnknown {
    fn from(value: &PointCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PointCollection> for ::windows_core::IInspectable {
    fn from(value: PointCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PointCollection> for ::windows_core::IInspectable {
    fn from(value: &PointCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PointCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Point> {
    type Error = ::windows_core::Error;
    fn try_from(value: PointCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PointCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Point> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PointCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Point>> for PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Point>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Point>> for &PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Point>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Point>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PointCollection> for ::winrt_foundation::Collections::IVector<::winrt_foundation::Point> {
    type Error = ::windows_core::Error;
    fn try_from(value: PointCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PointCollection> for ::winrt_foundation::Collections::IVector<::winrt_foundation::Point> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PointCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::winrt_foundation::Point>> for PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<::winrt_foundation::Point>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::winrt_foundation::Point>> for &PointCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<::winrt_foundation::Point>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<::winrt_foundation::Point>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for PointCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for PointCollection {}
#[repr(transparent)]
pub struct PolyBezierSegment(::windows_core::IUnknown);
impl PolyBezierSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PolyBezierSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Points(&self) -> ::windows_core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Points)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPoints<'a, Param0: ::windows_core::IntoParam<'a, PointCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoints)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PointsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPolyBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolyBezierSegmentStatics<R, F: FnOnce(&IPolyBezierSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PolyBezierSegment, IPolyBezierSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PolyBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PolyBezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PolyBezierSegment {}
impl ::core::fmt::Debug for PolyBezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolyBezierSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PolyBezierSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PolyBezierSegment;{36805271-38c4-4bcf-96cd-028a6d38af25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PolyBezierSegment {
    type Vtable = IPolyBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = <IPolyBezierSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PolyBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyBezierSegment";
}
impl ::core::convert::From<PolyBezierSegment> for ::windows_core::IUnknown {
    fn from(value: PolyBezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PolyBezierSegment> for ::windows_core::IUnknown {
    fn from(value: &PolyBezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PolyBezierSegment> for ::windows_core::IInspectable {
    fn from(value: PolyBezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PolyBezierSegment> for ::windows_core::IInspectable {
    fn from(value: &PolyBezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PolyBezierSegment> for PathSegment {
    fn from(value: PolyBezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PolyBezierSegment> for PathSegment {
    fn from(value: &PolyBezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<PolyBezierSegment> for super::DependencyObject {
    fn from(value: PolyBezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PolyBezierSegment> for super::DependencyObject {
    fn from(value: &PolyBezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PolyBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PolyBezierSegment {}
unsafe impl ::core::marker::Sync for PolyBezierSegment {}
#[repr(transparent)]
pub struct PolyLineSegment(::windows_core::IUnknown);
impl PolyLineSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PolyLineSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Points(&self) -> ::windows_core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Points)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPoints<'a, Param0: ::windows_core::IntoParam<'a, PointCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoints)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PointsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPolyLineSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolyLineSegmentStatics<R, F: FnOnce(&IPolyLineSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PolyLineSegment, IPolyLineSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PolyLineSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PolyLineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PolyLineSegment {}
impl ::core::fmt::Debug for PolyLineSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolyLineSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PolyLineSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PolyLineSegment;{4b397f87-a2e6-479d-bdc8-6f4464646887})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PolyLineSegment {
    type Vtable = IPolyLineSegment_Vtbl;
    const IID: ::windows_core::GUID = <IPolyLineSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PolyLineSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyLineSegment";
}
impl ::core::convert::From<PolyLineSegment> for ::windows_core::IUnknown {
    fn from(value: PolyLineSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PolyLineSegment> for ::windows_core::IUnknown {
    fn from(value: &PolyLineSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PolyLineSegment> for ::windows_core::IInspectable {
    fn from(value: PolyLineSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PolyLineSegment> for ::windows_core::IInspectable {
    fn from(value: &PolyLineSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PolyLineSegment> for PathSegment {
    fn from(value: PolyLineSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PolyLineSegment> for PathSegment {
    fn from(value: &PolyLineSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<PolyLineSegment> for super::DependencyObject {
    fn from(value: PolyLineSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PolyLineSegment> for super::DependencyObject {
    fn from(value: &PolyLineSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PolyLineSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PolyLineSegment {}
unsafe impl ::core::marker::Sync for PolyLineSegment {}
#[repr(transparent)]
pub struct PolyQuadraticBezierSegment(::windows_core::IUnknown);
impl PolyQuadraticBezierSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PolyQuadraticBezierSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Points(&self) -> ::windows_core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Points)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPoints<'a, Param0: ::windows_core::IntoParam<'a, PointCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoints)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PointsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPolyQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPolyQuadraticBezierSegmentStatics<R, F: FnOnce(&IPolyQuadraticBezierSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PolyQuadraticBezierSegment, IPolyQuadraticBezierSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PolyQuadraticBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PolyQuadraticBezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PolyQuadraticBezierSegment {}
impl ::core::fmt::Debug for PolyQuadraticBezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolyQuadraticBezierSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PolyQuadraticBezierSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.PolyQuadraticBezierSegment;{dd5ced7d-e6db-4c96-b6a1-3fce96e987a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = <IPolyQuadraticBezierSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PolyQuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.PolyQuadraticBezierSegment";
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for ::windows_core::IUnknown {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for ::windows_core::IUnknown {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for ::windows_core::IInspectable {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for ::windows_core::IInspectable {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for PathSegment {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for PathSegment {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<PolyQuadraticBezierSegment> for super::DependencyObject {
    fn from(value: PolyQuadraticBezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PolyQuadraticBezierSegment> for super::DependencyObject {
    fn from(value: &PolyQuadraticBezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PolyQuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PolyQuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for PolyQuadraticBezierSegment {}
#[repr(transparent)]
pub struct Projection(::windows_core::IUnknown);
impl Projection {}
impl ::core::clone::Clone for Projection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Projection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Projection {}
impl ::core::fmt::Debug for Projection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Projection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Projection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Projection;{b3443557-7f39-4d04-a89c-844338cac897})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Projection {
    type Vtable = IProjection_Vtbl;
    const IID: ::windows_core::GUID = <IProjection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Projection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Projection";
}
impl ::core::convert::From<Projection> for ::windows_core::IUnknown {
    fn from(value: Projection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Projection> for ::windows_core::IUnknown {
    fn from(value: &Projection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Projection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Projection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Projection> for ::windows_core::IInspectable {
    fn from(value: Projection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Projection> for ::windows_core::IInspectable {
    fn from(value: &Projection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Projection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Projection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Projection> for super::DependencyObject {
    fn from(value: Projection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Projection> for super::DependencyObject {
    fn from(value: &Projection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for Projection {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &Projection {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Projection {}
unsafe impl ::core::marker::Sync for Projection {}
#[repr(transparent)]
pub struct QuadraticBezierSegment(::windows_core::IUnknown);
impl QuadraticBezierSegment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QuadraticBezierSegment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Point1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint1<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint1)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Point2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Point2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPoint2<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPoint2)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Point1Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Point1Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Point2Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Point2Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IQuadraticBezierSegmentStatics<R, F: FnOnce(&IQuadraticBezierSegmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QuadraticBezierSegment, IQuadraticBezierSegmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuadraticBezierSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuadraticBezierSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuadraticBezierSegment {}
impl ::core::fmt::Debug for QuadraticBezierSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuadraticBezierSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for QuadraticBezierSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.QuadraticBezierSegment;{2c509a5b-bf18-455a-a078-914b5232d8af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for QuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_Vtbl;
    const IID: ::windows_core::GUID = <IQuadraticBezierSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for QuadraticBezierSegment {
    const NAME: &'static str = "Windows.UI.Xaml.Media.QuadraticBezierSegment";
}
impl ::core::convert::From<QuadraticBezierSegment> for ::windows_core::IUnknown {
    fn from(value: QuadraticBezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for ::windows_core::IUnknown {
    fn from(value: &QuadraticBezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuadraticBezierSegment> for ::windows_core::IInspectable {
    fn from(value: QuadraticBezierSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for ::windows_core::IInspectable {
    fn from(value: &QuadraticBezierSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuadraticBezierSegment> for PathSegment {
    fn from(value: QuadraticBezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for PathSegment {
    fn from(value: &QuadraticBezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PathSegment> for &QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, PathSegment> {
        ::windows_core::Param::Owned(::core::convert::Into::<PathSegment>::into(self))
    }
}
impl ::core::convert::From<QuadraticBezierSegment> for super::DependencyObject {
    fn from(value: QuadraticBezierSegment) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuadraticBezierSegment> for super::DependencyObject {
    fn from(value: &QuadraticBezierSegment) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &QuadraticBezierSegment {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for QuadraticBezierSegment {}
#[repr(transparent)]
pub struct RateChangedRoutedEventArgs(::windows_core::IUnknown);
impl RateChangedRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RateChangedRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RateChangedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RateChangedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RateChangedRoutedEventArgs {}
impl ::core::fmt::Debug for RateChangedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RateChangedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RateChangedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RateChangedRoutedEventArgs;{9016aa6f-3ca8-4c80-8e2f-8851a68f131f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RateChangedRoutedEventArgs {
    type Vtable = IRateChangedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRateChangedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RateChangedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RateChangedRoutedEventArgs";
}
impl ::core::convert::From<RateChangedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RateChangedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RateChangedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RateChangedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RateChangedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RateChangedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RateChangedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RateChangedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RateChangedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RateChangedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RateChangedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RateChangedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &RateChangedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for RateChangedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RateChangedRoutedEventArgs {}
#[repr(transparent)]
pub struct RateChangedRoutedEventHandler(pub ::windows_core::IUnknown);
impl RateChangedRoutedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RateChangedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RateChangedRoutedEventHandlerBox::<F> { vtable: &RateChangedRoutedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, RateChangedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RateChangedRoutedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RateChangedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RateChangedRoutedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RateChangedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RateChangedRoutedEventHandlerBox<F> {
    const VTABLE: RateChangedRoutedEventHandler_Vtbl = RateChangedRoutedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<RateChangedRoutedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for RateChangedRoutedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RateChangedRoutedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RateChangedRoutedEventHandler {}
impl ::core::fmt::Debug for RateChangedRoutedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RateChangedRoutedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RateChangedRoutedEventHandler {
    type Vtable = RateChangedRoutedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08e9a257_ae05_489b_8839_28c6225d2349);
}
unsafe impl ::windows_core::RuntimeType for RateChangedRoutedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{08e9a257-ae05-489b-8839-28c6225d2349}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RateChangedRoutedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct RectangleGeometry(::windows_core::IUnknown);
impl RectangleGeometry {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RectangleGeometry, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Rect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).Rect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RectProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRectangleGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RectProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IRectangleGeometryStatics<R, F: FnOnce(&IRectangleGeometryStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RectangleGeometry, IRectangleGeometryStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RectangleGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RectangleGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RectangleGeometry {}
impl ::core::fmt::Debug for RectangleGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RectangleGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RectangleGeometry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RectangleGeometry;{a25a1f58-c575-4196-91cf-9fdfb10445c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RectangleGeometry {
    type Vtable = IRectangleGeometry_Vtbl;
    const IID: ::windows_core::GUID = <IRectangleGeometry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RectangleGeometry {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RectangleGeometry";
}
impl ::core::convert::From<RectangleGeometry> for ::windows_core::IUnknown {
    fn from(value: RectangleGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RectangleGeometry> for ::windows_core::IUnknown {
    fn from(value: &RectangleGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RectangleGeometry> for ::windows_core::IInspectable {
    fn from(value: RectangleGeometry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RectangleGeometry> for ::windows_core::IInspectable {
    fn from(value: &RectangleGeometry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RectangleGeometry> for Geometry {
    fn from(value: RectangleGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RectangleGeometry> for Geometry {
    fn from(value: &RectangleGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Geometry> for &RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, Geometry> {
        ::windows_core::Param::Owned(::core::convert::Into::<Geometry>::into(self))
    }
}
impl ::core::convert::From<RectangleGeometry> for super::DependencyObject {
    fn from(value: RectangleGeometry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RectangleGeometry> for super::DependencyObject {
    fn from(value: &RectangleGeometry) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &RectangleGeometry {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RectangleGeometry {}
unsafe impl ::core::marker::Sync for RectangleGeometry {}
#[repr(transparent)]
pub struct RenderedEventArgs(::windows_core::IUnknown);
impl RenderedEventArgs {
    pub fn FrameDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).FrameDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for RenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RenderedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RenderedEventArgs {}
impl ::core::fmt::Debug for RenderedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenderedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RenderedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RenderedEventArgs;{e349817d-81c7-4938-828c-a7e2797b35a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RenderedEventArgs {
    type Vtable = IRenderedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRenderedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RenderedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RenderedEventArgs";
}
impl ::core::convert::From<RenderedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RenderedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RenderedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RenderedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RenderedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RenderedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RenderedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RenderedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RenderedEventArgs {}
unsafe impl ::core::marker::Sync for RenderedEventArgs {}
#[repr(transparent)]
pub struct RenderingEventArgs(::windows_core::IUnknown);
impl RenderingEventArgs {
    pub fn RenderingTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RenderingTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for RenderingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RenderingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RenderingEventArgs {}
impl ::core::fmt::Debug for RenderingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenderingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RenderingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RenderingEventArgs;{5bf7d30d-9748-4aed-8380-d7890eb776a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RenderingEventArgs {
    type Vtable = IRenderingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRenderingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RenderingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RenderingEventArgs";
}
impl ::core::convert::From<RenderingEventArgs> for ::windows_core::IUnknown {
    fn from(value: RenderingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RenderingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RenderingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RenderingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RenderingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RenderingEventArgs> for ::windows_core::IInspectable {
    fn from(value: RenderingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RenderingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RenderingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RenderingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RenderingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RenderingEventArgs {}
unsafe impl ::core::marker::Sync for RenderingEventArgs {}
#[repr(transparent)]
pub struct RevealBackgroundBrush(::windows_core::IUnknown);
impl RevealBackgroundBrush {
    pub fn new() -> ::windows_core::Result<RevealBackgroundBrush> {
        Self::IRevealBackgroundBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<RevealBackgroundBrush>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<RevealBackgroundBrush> {
        Self::IRevealBackgroundBrushFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<RevealBackgroundBrush>(result__)
        })
    }
    pub fn IRevealBackgroundBrushFactory<R, F: FnOnce(&IRevealBackgroundBrushFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RevealBackgroundBrush, IRevealBackgroundBrushFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RevealBackgroundBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevealBackgroundBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevealBackgroundBrush {}
impl ::core::fmt::Debug for RevealBackgroundBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBackgroundBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RevealBackgroundBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RevealBackgroundBrush;{261dcc0e-1991-4cdf-aee0-6350a3f90bb9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RevealBackgroundBrush {
    type Vtable = IRevealBackgroundBrush_Vtbl;
    const IID: ::windows_core::GUID = <IRevealBackgroundBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RevealBackgroundBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBackgroundBrush";
}
impl ::core::convert::From<RevealBackgroundBrush> for ::windows_core::IUnknown {
    fn from(value: RevealBackgroundBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for ::windows_core::IUnknown {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for ::windows_core::IInspectable {
    fn from(value: RevealBackgroundBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for ::windows_core::IInspectable {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<RevealBackgroundBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: RevealBackgroundBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&RevealBackgroundBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &RevealBackgroundBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for RevealBrush {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for RevealBrush {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, RevealBrush> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, RevealBrush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, RevealBrush> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, RevealBrush> {
        ::windows_core::Param::Owned(::core::convert::Into::<RevealBrush>::into(self))
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for XamlCompositionBrushBase {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for XamlCompositionBrushBase {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for Brush {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for Brush {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<RevealBackgroundBrush> for super::DependencyObject {
    fn from(value: RevealBackgroundBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBackgroundBrush> for super::DependencyObject {
    fn from(value: &RevealBackgroundBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &RevealBackgroundBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RevealBackgroundBrush {}
unsafe impl ::core::marker::Sync for RevealBackgroundBrush {}
#[repr(transparent)]
pub struct RevealBorderBrush(::windows_core::IUnknown);
impl RevealBorderBrush {
    pub fn new() -> ::windows_core::Result<RevealBorderBrush> {
        Self::IRevealBorderBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<RevealBorderBrush>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<RevealBorderBrush> {
        Self::IRevealBorderBrushFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<RevealBorderBrush>(result__)
        })
    }
    pub fn IRevealBorderBrushFactory<R, F: FnOnce(&IRevealBorderBrushFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RevealBorderBrush, IRevealBorderBrushFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RevealBorderBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevealBorderBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevealBorderBrush {}
impl ::core::fmt::Debug for RevealBorderBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBorderBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RevealBorderBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RevealBorderBrush;{060ba115-c542-483c-8202-5f03331866c9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RevealBorderBrush {
    type Vtable = IRevealBorderBrush_Vtbl;
    const IID: ::windows_core::GUID = <IRevealBorderBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RevealBorderBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBorderBrush";
}
impl ::core::convert::From<RevealBorderBrush> for ::windows_core::IUnknown {
    fn from(value: RevealBorderBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevealBorderBrush> for ::windows_core::IUnknown {
    fn from(value: &RevealBorderBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RevealBorderBrush> for ::windows_core::IInspectable {
    fn from(value: RevealBorderBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevealBorderBrush> for ::windows_core::IInspectable {
    fn from(value: &RevealBorderBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<RevealBorderBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: RevealBorderBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&RevealBorderBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &RevealBorderBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<RevealBorderBrush> for RevealBrush {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for RevealBrush {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, RevealBrush> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, RevealBrush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, RevealBrush> for &RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, RevealBrush> {
        ::windows_core::Param::Owned(::core::convert::Into::<RevealBrush>::into(self))
    }
}
impl ::core::convert::From<RevealBorderBrush> for XamlCompositionBrushBase {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for XamlCompositionBrushBase {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for &RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl ::core::convert::From<RevealBorderBrush> for Brush {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for Brush {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<RevealBorderBrush> for super::DependencyObject {
    fn from(value: RevealBorderBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBorderBrush> for super::DependencyObject {
    fn from(value: &RevealBorderBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &RevealBorderBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RevealBorderBrush {}
unsafe impl ::core::marker::Sync for RevealBorderBrush {}
#[repr(transparent)]
pub struct RevealBrush(::windows_core::IUnknown);
impl RevealBrush {
    pub fn Color(&self) -> ::windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetTheme(&self) -> ::windows_core::Result<super::ApplicationTheme> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ApplicationTheme>::zeroed();
            (::windows_core::Interface::vtable(this).TargetTheme)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ApplicationTheme>(result__)
        }
    }
    pub fn SetTargetTheme(&self, value: super::ApplicationTheme) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetTheme)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysUseFallback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysUseFallback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysUseFallback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysUseFallback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TargetThemeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetThemeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AlwaysUseFallbackProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysUseFallbackProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StateProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn SetState<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0, value: RevealBrushState) -> ::windows_core::Result<()> {
        Self::IRevealBrushStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn GetState<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows_core::Result<RevealBrushState> {
        Self::IRevealBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RevealBrushState>::zeroed();
            (::windows_core::Interface::vtable(this).GetState)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<RevealBrushState>(result__)
        })
    }
    pub fn IRevealBrushStatics<R, F: FnOnce(&IRevealBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RevealBrush, IRevealBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RevealBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevealBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevealBrush {}
impl ::core::fmt::Debug for RevealBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RevealBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RevealBrush;{2036a0ed-8271-4398-9019-25872093f13f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RevealBrush {
    type Vtable = IRevealBrush_Vtbl;
    const IID: ::windows_core::GUID = <IRevealBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RevealBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RevealBrush";
}
impl ::core::convert::From<RevealBrush> for ::windows_core::IUnknown {
    fn from(value: RevealBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevealBrush> for ::windows_core::IUnknown {
    fn from(value: &RevealBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RevealBrush> for ::windows_core::IInspectable {
    fn from(value: RevealBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevealBrush> for ::windows_core::IInspectable {
    fn from(value: &RevealBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<RevealBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: RevealBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&RevealBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &RevealBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<RevealBrush> for XamlCompositionBrushBase {
    fn from(value: RevealBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBrush> for XamlCompositionBrushBase {
    fn from(value: &RevealBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlCompositionBrushBase> for &RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, XamlCompositionBrushBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<XamlCompositionBrushBase>::into(self))
    }
}
impl ::core::convert::From<RevealBrush> for Brush {
    fn from(value: RevealBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBrush> for Brush {
    fn from(value: &RevealBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<RevealBrush> for super::DependencyObject {
    fn from(value: RevealBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RevealBrush> for super::DependencyObject {
    fn from(value: &RevealBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &RevealBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RevealBrush {}
unsafe impl ::core::marker::Sync for RevealBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RevealBrushState(pub i32);
impl RevealBrushState {
    pub const Normal: Self = Self(0i32);
    pub const PointerOver: Self = Self(1i32);
    pub const Pressed: Self = Self(2i32);
}
impl ::core::marker::Copy for RevealBrushState {}
impl ::core::clone::Clone for RevealBrushState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RevealBrushState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RevealBrushState {
    type Abi = Self;
}
impl ::core::fmt::Debug for RevealBrushState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevealBrushState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RevealBrushState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.RevealBrushState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RotateTransform(::windows_core::IUnknown);
impl RotateTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RotateTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Angle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Angle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAngle(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AngleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AngleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IRotateTransformStatics<R, F: FnOnce(&IRotateTransformStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RotateTransform, IRotateTransformStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RotateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RotateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RotateTransform {}
impl ::core::fmt::Debug for RotateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RotateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RotateTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.RotateTransform;{688ea9b9-1e4e-4596-86e3-428b27334faf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RotateTransform {
    type Vtable = IRotateTransform_Vtbl;
    const IID: ::windows_core::GUID = <IRotateTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RotateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.RotateTransform";
}
impl ::core::convert::From<RotateTransform> for ::windows_core::IUnknown {
    fn from(value: RotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RotateTransform> for ::windows_core::IUnknown {
    fn from(value: &RotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RotateTransform> for ::windows_core::IInspectable {
    fn from(value: RotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RotateTransform> for ::windows_core::IInspectable {
    fn from(value: &RotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RotateTransform> for Transform {
    fn from(value: RotateTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RotateTransform> for Transform {
    fn from(value: &RotateTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<RotateTransform> for GeneralTransform {
    fn from(value: RotateTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RotateTransform> for GeneralTransform {
    fn from(value: &RotateTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<RotateTransform> for super::DependencyObject {
    fn from(value: RotateTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RotateTransform> for super::DependencyObject {
    fn from(value: &RotateTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &RotateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RotateTransform {}
unsafe impl ::core::marker::Sync for RotateTransform {}
#[repr(transparent)]
pub struct ScaleTransform(::windows_core::IUnknown);
impl ScaleTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScaleTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IScaleTransformStatics<R, F: FnOnce(&IScaleTransformStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScaleTransform, IScaleTransformStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScaleTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScaleTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScaleTransform {}
impl ::core::fmt::Debug for ScaleTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScaleTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScaleTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ScaleTransform;{ed67f18d-936e-43ab-929a-e9cd0a511e52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScaleTransform {
    type Vtable = IScaleTransform_Vtbl;
    const IID: ::windows_core::GUID = <IScaleTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScaleTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ScaleTransform";
}
impl ::core::convert::From<ScaleTransform> for ::windows_core::IUnknown {
    fn from(value: ScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScaleTransform> for ::windows_core::IUnknown {
    fn from(value: &ScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScaleTransform> for ::windows_core::IInspectable {
    fn from(value: ScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScaleTransform> for ::windows_core::IInspectable {
    fn from(value: &ScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScaleTransform> for Transform {
    fn from(value: ScaleTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScaleTransform> for Transform {
    fn from(value: &ScaleTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<ScaleTransform> for GeneralTransform {
    fn from(value: ScaleTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScaleTransform> for GeneralTransform {
    fn from(value: &ScaleTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<ScaleTransform> for super::DependencyObject {
    fn from(value: ScaleTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScaleTransform> for super::DependencyObject {
    fn from(value: &ScaleTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &ScaleTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ScaleTransform {}
unsafe impl ::core::marker::Sync for ScaleTransform {}
#[repr(transparent)]
pub struct Shadow(::windows_core::IUnknown);
impl Shadow {}
impl ::core::clone::Clone for Shadow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Shadow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Shadow {}
impl ::core::fmt::Debug for Shadow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Shadow").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Shadow {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Shadow;{6813a583-f3b4-5fcf-8694-2cd0aefc2fad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Shadow {
    type Vtable = IShadow_Vtbl;
    const IID: ::windows_core::GUID = <IShadow as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Shadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Shadow";
}
impl ::core::convert::From<Shadow> for ::windows_core::IUnknown {
    fn from(value: Shadow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Shadow> for ::windows_core::IUnknown {
    fn from(value: &Shadow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Shadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Shadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Shadow> for ::windows_core::IInspectable {
    fn from(value: Shadow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Shadow> for ::windows_core::IInspectable {
    fn from(value: &Shadow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Shadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Shadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Shadow> for super::DependencyObject {
    fn from(value: Shadow) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Shadow> for super::DependencyObject {
    fn from(value: &Shadow) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for Shadow {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &Shadow {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Shadow {}
unsafe impl ::core::marker::Sync for Shadow {}
#[repr(transparent)]
pub struct SkewTransform(::windows_core::IUnknown);
impl SkewTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SkewTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AngleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AngleX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAngleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAngleX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AngleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AngleY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAngleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAngleY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AngleXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AngleXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AngleYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AngleYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ISkewTransformStatics<R, F: FnOnce(&ISkewTransformStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SkewTransform, ISkewTransformStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SkewTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SkewTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SkewTransform {}
impl ::core::fmt::Debug for SkewTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SkewTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SkewTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.SkewTransform;{4e8a3b15-7a0f-4617-9e98-1e65bdc92115})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SkewTransform {
    type Vtable = ISkewTransform_Vtbl;
    const IID: ::windows_core::GUID = <ISkewTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SkewTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SkewTransform";
}
impl ::core::convert::From<SkewTransform> for ::windows_core::IUnknown {
    fn from(value: SkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SkewTransform> for ::windows_core::IUnknown {
    fn from(value: &SkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SkewTransform> for ::windows_core::IInspectable {
    fn from(value: SkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SkewTransform> for ::windows_core::IInspectable {
    fn from(value: &SkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SkewTransform> for Transform {
    fn from(value: SkewTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SkewTransform> for Transform {
    fn from(value: &SkewTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<SkewTransform> for GeneralTransform {
    fn from(value: SkewTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SkewTransform> for GeneralTransform {
    fn from(value: &SkewTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<SkewTransform> for super::DependencyObject {
    fn from(value: SkewTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SkewTransform> for super::DependencyObject {
    fn from(value: &SkewTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &SkewTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SkewTransform {}
unsafe impl ::core::marker::Sync for SkewTransform {}
#[repr(transparent)]
pub struct SolidColorBrush(::windows_core::IUnknown);
impl SolidColorBrush {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SolidColorBrush, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Color(&self) -> ::windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateInstanceWithColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Color>>(color: Param0) -> ::windows_core::Result<SolidColorBrush> {
        Self::ISolidColorBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithColor)(::windows_core::Interface::as_raw(this), color.into_param().abi(), result__.as_mut_ptr()).from_abi::<SolidColorBrush>(result__)
        })
    }
    pub fn ColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISolidColorBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ISolidColorBrushFactory<R, F: FnOnce(&ISolidColorBrushFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SolidColorBrush, ISolidColorBrushFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISolidColorBrushStatics<R, F: FnOnce(&ISolidColorBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SolidColorBrush, ISolidColorBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SolidColorBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SolidColorBrush {}
impl ::core::fmt::Debug for SolidColorBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SolidColorBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SolidColorBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.SolidColorBrush;{9d850850-66f3-48df-9a8f-824bd5e070af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SolidColorBrush {
    type Vtable = ISolidColorBrush_Vtbl;
    const IID: ::windows_core::GUID = <ISolidColorBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SolidColorBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SolidColorBrush";
}
impl ::core::convert::From<SolidColorBrush> for ::windows_core::IUnknown {
    fn from(value: SolidColorBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SolidColorBrush> for ::windows_core::IUnknown {
    fn from(value: &SolidColorBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SolidColorBrush> for ::windows_core::IInspectable {
    fn from(value: SolidColorBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SolidColorBrush> for ::windows_core::IInspectable {
    fn from(value: &SolidColorBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<SolidColorBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SolidColorBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&SolidColorBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SolidColorBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SolidColorBrush> for Brush {
    fn from(value: SolidColorBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SolidColorBrush> for Brush {
    fn from(value: &SolidColorBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<SolidColorBrush> for super::DependencyObject {
    fn from(value: SolidColorBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SolidColorBrush> for super::DependencyObject {
    fn from(value: &SolidColorBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &SolidColorBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SolidColorBrush {}
unsafe impl ::core::marker::Sync for SolidColorBrush {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Stereo3DVideoPackingMode(pub i32);
impl Stereo3DVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for Stereo3DVideoPackingMode {}
impl ::core::clone::Clone for Stereo3DVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stereo3DVideoPackingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Stereo3DVideoPackingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for Stereo3DVideoPackingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stereo3DVideoPackingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Stereo3DVideoPackingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stereo3DVideoPackingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Stereo3DVideoRenderMode(pub i32);
impl Stereo3DVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for Stereo3DVideoRenderMode {}
impl ::core::clone::Clone for Stereo3DVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stereo3DVideoRenderMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Stereo3DVideoRenderMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for Stereo3DVideoRenderMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stereo3DVideoRenderMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Stereo3DVideoRenderMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stereo3DVideoRenderMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for Stretch {}
impl ::core::clone::Clone for Stretch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stretch {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Stretch {
    type Abi = Self;
}
impl ::core::fmt::Debug for Stretch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stretch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Stretch {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stretch;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: Self = Self(0i32);
    pub const BoldSimulation: Self = Self(1i32);
    pub const ItalicSimulation: Self = Self(2i32);
    pub const BoldItalicSimulation: Self = Self(3i32);
}
impl ::core::marker::Copy for StyleSimulations {}
impl ::core::clone::Clone for StyleSimulations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StyleSimulations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StyleSimulations {
    type Abi = Self;
}
impl ::core::fmt::Debug for StyleSimulations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StyleSimulations").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StyleSimulations {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.StyleSimulations;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
impl ::core::marker::Copy for SweepDirection {}
impl ::core::clone::Clone for SweepDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SweepDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SweepDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for SweepDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SweepDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SweepDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.SweepDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ThemeShadow(::windows_core::IUnknown);
impl ThemeShadow {
    pub fn Receivers(&self) -> ::windows_core::Result<super::UIElementWeakCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Receivers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElementWeakCollection>(result__)
        }
    }
    pub fn new() -> ::windows_core::Result<ThemeShadow> {
        Self::IThemeShadowFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<ThemeShadow>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<ThemeShadow> {
        Self::IThemeShadowFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<ThemeShadow>(result__)
        })
    }
    pub fn IThemeShadowFactory<R, F: FnOnce(&IThemeShadowFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ThemeShadow, IThemeShadowFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ThemeShadow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ThemeShadow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ThemeShadow {}
impl ::core::fmt::Debug for ThemeShadow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThemeShadow").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ThemeShadow {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.ThemeShadow;{3eccad09-7985-5f39-8b62-6c10696dca6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ThemeShadow {
    type Vtable = IThemeShadow_Vtbl;
    const IID: ::windows_core::GUID = <IThemeShadow as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ThemeShadow {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ThemeShadow";
}
impl ::core::convert::From<ThemeShadow> for ::windows_core::IUnknown {
    fn from(value: ThemeShadow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThemeShadow> for ::windows_core::IUnknown {
    fn from(value: &ThemeShadow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ThemeShadow> for ::windows_core::IInspectable {
    fn from(value: ThemeShadow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThemeShadow> for ::windows_core::IInspectable {
    fn from(value: &ThemeShadow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ThemeShadow> for Shadow {
    fn from(value: ThemeShadow) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ThemeShadow> for Shadow {
    fn from(value: &ThemeShadow) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Shadow> for ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, Shadow> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Shadow> for &ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, Shadow> {
        ::windows_core::Param::Owned(::core::convert::Into::<Shadow>::into(self))
    }
}
impl ::core::convert::From<ThemeShadow> for super::DependencyObject {
    fn from(value: ThemeShadow) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ThemeShadow> for super::DependencyObject {
    fn from(value: &ThemeShadow) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &ThemeShadow {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ThemeShadow {}
unsafe impl ::core::marker::Sync for ThemeShadow {}
#[repr(transparent)]
pub struct TileBrush(::windows_core::IUnknown);
impl TileBrush {
    pub fn AlignmentX(&self) -> ::windows_core::Result<AlignmentX> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AlignmentX>::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AlignmentX>(result__)
        }
    }
    pub fn SetAlignmentX(&self, value: AlignmentX) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlignmentX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlignmentY(&self) -> ::windows_core::Result<AlignmentY> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AlignmentY>::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AlignmentY>(result__)
        }
    }
    pub fn SetAlignmentY(&self, value: AlignmentY) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlignmentY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Stretch(&self) -> ::windows_core::Result<Stretch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Stretch>::zeroed();
            (::windows_core::Interface::vtable(this).Stretch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Stretch>(result__)
        }
    }
    pub fn SetStretch(&self, value: Stretch) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStretch)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlignmentXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AlignmentYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StretchProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StretchProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITileBrushStatics<R, F: FnOnce(&ITileBrushStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TileBrush, ITileBrushStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TileBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileBrush {}
impl ::core::fmt::Debug for TileBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileBrush {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TileBrush;{c201cf06-cd84-48a5-9607-664d7361cd61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileBrush {
    type Vtable = ITileBrush_Vtbl;
    const IID: ::windows_core::GUID = <ITileBrush as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TileBrush";
}
impl ::core::convert::From<TileBrush> for ::windows_core::IUnknown {
    fn from(value: TileBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileBrush> for ::windows_core::IUnknown {
    fn from(value: &TileBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileBrush> for ::windows_core::IInspectable {
    fn from(value: TileBrush) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileBrush> for ::windows_core::IInspectable {
    fn from(value: &TileBrush) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<TileBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: TileBrush) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&TileBrush> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &TileBrush) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<TileBrush> for Brush {
    fn from(value: TileBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TileBrush> for Brush {
    fn from(value: &TileBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<TileBrush> for super::DependencyObject {
    fn from(value: TileBrush) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TileBrush> for super::DependencyObject {
    fn from(value: &TileBrush) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &TileBrush {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TileBrush {}
unsafe impl ::core::marker::Sync for TileBrush {}
#[repr(transparent)]
pub struct TimelineMarker(::windows_core::IUnknown);
impl TimelineMarker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimelineMarker, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Time(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Time)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TimeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITimelineMarkerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITimelineMarkerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TextProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITimelineMarkerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITimelineMarkerStatics<R, F: FnOnce(&ITimelineMarkerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimelineMarker, ITimelineMarkerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimelineMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimelineMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarker {}
impl ::core::fmt::Debug for TimelineMarker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimelineMarker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TimelineMarker;{a68ef02d-45ba-4e50-8cad-aaea3a227af5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimelineMarker {
    type Vtable = ITimelineMarker_Vtbl;
    const IID: ::windows_core::GUID = <ITimelineMarker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimelineMarker {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarker";
}
impl ::core::convert::From<TimelineMarker> for ::windows_core::IUnknown {
    fn from(value: TimelineMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelineMarker> for ::windows_core::IUnknown {
    fn from(value: &TimelineMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimelineMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimelineMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimelineMarker> for ::windows_core::IInspectable {
    fn from(value: TimelineMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelineMarker> for ::windows_core::IInspectable {
    fn from(value: &TimelineMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimelineMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimelineMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimelineMarker> for super::DependencyObject {
    fn from(value: TimelineMarker) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TimelineMarker> for super::DependencyObject {
    fn from(value: &TimelineMarker) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for TimelineMarker {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &TimelineMarker {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TimelineMarker {}
unsafe impl ::core::marker::Sync for TimelineMarker {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct TimelineMarkerCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl TimelineMarkerCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimelineMarkerCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<TimelineMarker>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<TimelineMarker>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<TimelineMarker>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<TimelineMarker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<TimelineMarker>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TimelineMarker>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TimelineMarker>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, TimelineMarker>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, TimelineMarker>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, TimelineMarker>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, TimelineMarker>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<TimelineMarker>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<TimelineMarker>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for TimelineMarkerCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for TimelineMarkerCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for TimelineMarkerCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for TimelineMarkerCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarkerCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for TimelineMarkerCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TimelineMarkerCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.TimelineMarker;{a68ef02d-45ba-4e50-8cad-aaea3a227af5})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for TimelineMarkerCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<TimelineMarker>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<TimelineMarker> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for TimelineMarkerCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarkerCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for TimelineMarkerCollection {
    type Item = TimelineMarker;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &TimelineMarkerCollection {
    type Item = TimelineMarker;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TimelineMarkerCollection> for ::windows_core::IUnknown {
    fn from(value: TimelineMarkerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TimelineMarkerCollection> for ::windows_core::IUnknown {
    fn from(value: &TimelineMarkerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TimelineMarkerCollection> for ::windows_core::IInspectable {
    fn from(value: TimelineMarkerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TimelineMarkerCollection> for ::windows_core::IInspectable {
    fn from(value: &TimelineMarkerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TimelineMarkerCollection> for ::winrt_foundation::Collections::IIterable<TimelineMarker> {
    type Error = ::windows_core::Error;
    fn try_from(value: TimelineMarkerCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TimelineMarkerCollection> for ::winrt_foundation::Collections::IIterable<TimelineMarker> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimelineMarkerCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<TimelineMarker>> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<TimelineMarker>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<TimelineMarker>> for &TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<TimelineMarker>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<TimelineMarker>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TimelineMarkerCollection> for ::winrt_foundation::Collections::IVector<TimelineMarker> {
    type Error = ::windows_core::Error;
    fn try_from(value: TimelineMarkerCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TimelineMarkerCollection> for ::winrt_foundation::Collections::IVector<TimelineMarker> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimelineMarkerCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<TimelineMarker>> for TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<TimelineMarker>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<TimelineMarker>> for &TimelineMarkerCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<TimelineMarker>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<TimelineMarker>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for TimelineMarkerCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for TimelineMarkerCollection {}
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventArgs(::windows_core::IUnknown);
impl TimelineMarkerRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimelineMarkerRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Marker(&self) -> ::windows_core::Result<TimelineMarker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Marker)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimelineMarker>(result__)
        }
    }
    pub fn SetMarker<'a, Param0: ::windows_core::IntoParam<'a, TimelineMarker>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMarker)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for TimelineMarkerRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimelineMarkerRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarkerRoutedEventArgs {}
impl ::core::fmt::Debug for TimelineMarkerRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarkerRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimelineMarkerRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TimelineMarkerRoutedEventArgs;{7c3b3ef3-2c88-4d9c-99b6-46cdbd48d4c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimelineMarkerRoutedEventArgs {
    type Vtable = ITimelineMarkerRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimelineMarkerRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimelineMarkerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TimelineMarkerRoutedEventArgs";
}
impl ::core::convert::From<TimelineMarkerRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TimelineMarkerRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelineMarkerRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TimelineMarkerRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimelineMarkerRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TimelineMarkerRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelineMarkerRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TimelineMarkerRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimelineMarkerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TimelineMarkerRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TimelineMarkerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TimelineMarkerRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &TimelineMarkerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for TimelineMarkerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TimelineMarkerRoutedEventArgs {}
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventHandler(pub ::windows_core::IUnknown);
impl TimelineMarkerRoutedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<TimelineMarkerRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = TimelineMarkerRoutedEventHandlerBox::<F> { vtable: &TimelineMarkerRoutedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, TimelineMarkerRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct TimelineMarkerRoutedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<TimelineMarkerRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const TimelineMarkerRoutedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<TimelineMarkerRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> TimelineMarkerRoutedEventHandlerBox<F> {
    const VTABLE: TimelineMarkerRoutedEventHandler_Vtbl = TimelineMarkerRoutedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<TimelineMarkerRoutedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for TimelineMarkerRoutedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimelineMarkerRoutedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelineMarkerRoutedEventHandler {}
impl ::core::fmt::Debug for TimelineMarkerRoutedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineMarkerRoutedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for TimelineMarkerRoutedEventHandler {
    type Vtable = TimelineMarkerRoutedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72e2fa9c_6dea_4cbe_a159_06ce95fbeced);
}
unsafe impl ::windows_core::RuntimeType for TimelineMarkerRoutedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{72e2fa9c-6dea-4cbe-a159-06ce95fbeced}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TimelineMarkerRoutedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct Transform(::windows_core::IUnknown);
impl Transform {}
impl ::core::clone::Clone for Transform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Transform {}
impl ::core::fmt::Debug for Transform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Transform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Transform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Transform;{4df74078-bfd6-4ed1-9682-d2fd8bf2fe6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Transform {
    type Vtable = ITransform_Vtbl;
    const IID: ::windows_core::GUID = <ITransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Transform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Transform";
}
impl ::core::convert::From<Transform> for ::windows_core::IUnknown {
    fn from(value: Transform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transform> for ::windows_core::IUnknown {
    fn from(value: &Transform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Transform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Transform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transform> for ::windows_core::IInspectable {
    fn from(value: Transform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transform> for ::windows_core::IInspectable {
    fn from(value: &Transform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Transform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Transform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transform> for GeneralTransform {
    fn from(value: Transform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Transform> for GeneralTransform {
    fn from(value: &Transform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for Transform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &Transform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<Transform> for super::DependencyObject {
    fn from(value: Transform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Transform> for super::DependencyObject {
    fn from(value: &Transform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for Transform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &Transform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Transform {}
unsafe impl ::core::marker::Sync for Transform {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct TransformCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl TransformCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TransformCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<Transform>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<Transform>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<Transform>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<Transform>(result__)
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
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Transform>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Transform>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, Transform>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, Transform>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, Transform>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, Transform>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Transform>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Transform>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for TransformCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for TransformCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for TransformCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for TransformCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for TransformCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TransformCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Transform;{4df74078-bfd6-4ed1-9682-d2fd8bf2fe6f})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for TransformCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<Transform>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<Transform> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for TransformCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TransformCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for TransformCollection {
    type Item = Transform;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &TransformCollection {
    type Item = Transform;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TransformCollection> for ::windows_core::IUnknown {
    fn from(value: TransformCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TransformCollection> for ::windows_core::IUnknown {
    fn from(value: &TransformCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TransformCollection> for ::windows_core::IInspectable {
    fn from(value: TransformCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TransformCollection> for ::windows_core::IInspectable {
    fn from(value: &TransformCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TransformCollection> for ::winrt_foundation::Collections::IIterable<Transform> {
    type Error = ::windows_core::Error;
    fn try_from(value: TransformCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TransformCollection> for ::winrt_foundation::Collections::IIterable<Transform> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TransformCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Transform>> for TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Transform>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Transform>> for &TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Transform>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<Transform>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TransformCollection> for ::winrt_foundation::Collections::IVector<Transform> {
    type Error = ::windows_core::Error;
    fn try_from(value: TransformCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TransformCollection> for ::winrt_foundation::Collections::IVector<Transform> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TransformCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Transform>> for TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Transform>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Transform>> for &TransformCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Transform>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<Transform>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for TransformCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for TransformCollection {}
#[repr(transparent)]
pub struct TransformGroup(::windows_core::IUnknown);
impl TransformGroup {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TransformGroup, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<TransformCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TransformCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetChildren<'a, Param0: ::windows_core::IntoParam<'a, TransformCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChildren)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<Matrix> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Matrix>(result__)
        }
    }
    pub fn ChildrenProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITransformGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildrenProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITransformGroupStatics<R, F: FnOnce(&ITransformGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TransformGroup, ITransformGroupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TransformGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TransformGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformGroup {}
impl ::core::fmt::Debug for TransformGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TransformGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TransformGroup;{63418ccc-8d2d-4737-b951-2afce1ddc4c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TransformGroup {
    type Vtable = ITransformGroup_Vtbl;
    const IID: ::windows_core::GUID = <ITransformGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TransformGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TransformGroup";
}
impl ::core::convert::From<TransformGroup> for ::windows_core::IUnknown {
    fn from(value: TransformGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformGroup> for ::windows_core::IUnknown {
    fn from(value: &TransformGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TransformGroup> for ::windows_core::IInspectable {
    fn from(value: TransformGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformGroup> for ::windows_core::IInspectable {
    fn from(value: &TransformGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TransformGroup> for Transform {
    fn from(value: TransformGroup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TransformGroup> for Transform {
    fn from(value: &TransformGroup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<TransformGroup> for GeneralTransform {
    fn from(value: TransformGroup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TransformGroup> for GeneralTransform {
    fn from(value: &TransformGroup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<TransformGroup> for super::DependencyObject {
    fn from(value: TransformGroup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TransformGroup> for super::DependencyObject {
    fn from(value: &TransformGroup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &TransformGroup {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TransformGroup {}
unsafe impl ::core::marker::Sync for TransformGroup {}
#[repr(transparent)]
pub struct TranslateTransform(::windows_core::IUnknown);
impl TranslateTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TranslateTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn X(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).X)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Y(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Y)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITranslateTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).XProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn YProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITranslateTransformStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).YProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ITranslateTransformStatics<R, F: FnOnce(&ITranslateTransformStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TranslateTransform, ITranslateTransformStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TranslateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TranslateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TranslateTransform {}
impl ::core::fmt::Debug for TranslateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranslateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TranslateTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.TranslateTransform;{c975905c-3c36-4229-817b-178f64c0e113})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TranslateTransform {
    type Vtable = ITranslateTransform_Vtbl;
    const IID: ::windows_core::GUID = <ITranslateTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TranslateTransform {
    const NAME: &'static str = "Windows.UI.Xaml.Media.TranslateTransform";
}
impl ::core::convert::From<TranslateTransform> for ::windows_core::IUnknown {
    fn from(value: TranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TranslateTransform> for ::windows_core::IUnknown {
    fn from(value: &TranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TranslateTransform> for ::windows_core::IInspectable {
    fn from(value: TranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TranslateTransform> for ::windows_core::IInspectable {
    fn from(value: &TranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TranslateTransform> for Transform {
    fn from(value: TranslateTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TranslateTransform> for Transform {
    fn from(value: &TranslateTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform> for &TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, Transform> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform>::into(self))
    }
}
impl ::core::convert::From<TranslateTransform> for GeneralTransform {
    fn from(value: TranslateTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TranslateTransform> for GeneralTransform {
    fn from(value: &TranslateTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, GeneralTransform> for &TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, GeneralTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<GeneralTransform>::into(self))
    }
}
impl ::core::convert::From<TranslateTransform> for super::DependencyObject {
    fn from(value: TranslateTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TranslateTransform> for super::DependencyObject {
    fn from(value: &TranslateTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &TranslateTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TranslateTransform {}
unsafe impl ::core::marker::Sync for TranslateTransform {}
#[repr(transparent)]
pub struct VisualTreeHelper(::windows_core::IUnknown);
impl VisualTreeHelper {
    #[cfg(feature = "winrt-foundation")]
    pub fn FindElementsInHostCoordinatesPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(intersectingpoint: Param0, subtree: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindElementsInHostCoordinatesPoint)(::windows_core::Interface::as_raw(this), intersectingpoint.into_param().abi(), subtree.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindElementsInHostCoordinatesRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(intersectingrect: Param0, subtree: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindElementsInHostCoordinatesRect)(::windows_core::Interface::as_raw(this), intersectingrect.into_param().abi(), subtree.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllElementsInHostCoordinatesPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(intersectingpoint: Param0, subtree: Param1, includeallelements: bool) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllElementsInHostCoordinatesPoint)(::windows_core::Interface::as_raw(this), intersectingpoint.into_param().abi(), subtree.into_param().abi(), includeallelements, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllElementsInHostCoordinatesRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(intersectingrect: Param0, subtree: Param1, includeallelements: bool) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<super::UIElement>> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllElementsInHostCoordinatesRect)(::windows_core::Interface::as_raw(this), intersectingrect.into_param().abi(), subtree.into_param().abi(), includeallelements, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<super::UIElement>>(result__)
        })
    }
    pub fn GetChild<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(reference: Param0, childindex: i32) -> ::windows_core::Result<super::DependencyObject> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetChild)(::windows_core::Interface::as_raw(this), reference.into_param().abi(), childindex, result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn GetChildrenCount<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(reference: Param0) -> ::windows_core::Result<i32> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetChildrenCount)(::windows_core::Interface::as_raw(this), reference.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn GetParent<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(reference: Param0) -> ::windows_core::Result<super::DependencyObject> {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetParent)(::windows_core::Interface::as_raw(this), reference.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn DisconnectChildrenRecursive<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows_core::Result<()> {
        Self::IVisualTreeHelperStatics(|this| unsafe { (::windows_core::Interface::vtable(this).DisconnectChildrenRecursive)(::windows_core::Interface::as_raw(this), element.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-ui"))]
    pub fn GetOpenPopups<'a, Param0: ::windows_core::IntoParam<'a, super::Window>>(window: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::Controls::Primitives::Popup>> {
        Self::IVisualTreeHelperStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOpenPopups)(::windows_core::Interface::as_raw(this), window.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-ui"))]
    pub fn GetOpenPopupsForXamlRoot<'a, Param0: ::windows_core::IntoParam<'a, super::XamlRoot>>(xamlroot: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::Controls::Primitives::Popup>> {
        Self::IVisualTreeHelperStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOpenPopupsForXamlRoot)(::windows_core::Interface::as_raw(this), xamlroot.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>(result__)
        })
    }
    pub fn IVisualTreeHelperStatics<R, F: FnOnce(&IVisualTreeHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualTreeHelperStatics2<R, F: FnOnce(&IVisualTreeHelperStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualTreeHelperStatics3<R, F: FnOnce(&IVisualTreeHelperStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VisualTreeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VisualTreeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualTreeHelper {}
impl ::core::fmt::Debug for VisualTreeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualTreeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VisualTreeHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.VisualTreeHelper;{24b935e3-52c7-4141-8bac-a73d06130569})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VisualTreeHelper {
    type Vtable = IVisualTreeHelper_Vtbl;
    const IID: ::windows_core::GUID = <IVisualTreeHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VisualTreeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.VisualTreeHelper";
}
impl ::core::convert::From<VisualTreeHelper> for ::windows_core::IUnknown {
    fn from(value: VisualTreeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VisualTreeHelper> for ::windows_core::IUnknown {
    fn from(value: &VisualTreeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VisualTreeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VisualTreeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VisualTreeHelper> for ::windows_core::IInspectable {
    fn from(value: VisualTreeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VisualTreeHelper> for ::windows_core::IInspectable {
    fn from(value: &VisualTreeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VisualTreeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VisualTreeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VisualTreeHelper {}
unsafe impl ::core::marker::Sync for VisualTreeHelper {}
#[repr(transparent)]
pub struct XamlCompositionBrushBase(::windows_core::IUnknown);
impl XamlCompositionBrushBase {
    pub fn FallbackColor(&self) -> ::windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetFallbackColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFallbackColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CompositionBrush(&self) -> ::windows_core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows_core::Interface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompositionBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Composition::CompositionBrush>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetCompositionBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Composition::CompositionBrush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompositionBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FallbackColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlCompositionBrushBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IXamlCompositionBrushBaseStatics<R, F: FnOnce(&IXamlCompositionBrushBaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlCompositionBrushBase, IXamlCompositionBrushBaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlCompositionBrushBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlCompositionBrushBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlCompositionBrushBase {}
impl ::core::fmt::Debug for XamlCompositionBrushBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlCompositionBrushBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlCompositionBrushBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.XamlCompositionBrushBase;{03e432d9-b35c-4a79-811c-c5652004da0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_Vtbl;
    const IID: ::windows_core::GUID = <IXamlCompositionBrushBase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlCompositionBrushBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.XamlCompositionBrushBase";
}
impl ::core::convert::From<XamlCompositionBrushBase> for ::windows_core::IUnknown {
    fn from(value: XamlCompositionBrushBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for ::windows_core::IUnknown {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlCompositionBrushBase> for ::windows_core::IInspectable {
    fn from(value: XamlCompositionBrushBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for ::windows_core::IInspectable {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<XamlCompositionBrushBase> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: XamlCompositionBrushBase) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-ui")]
impl ::core::convert::TryFrom<&XamlCompositionBrushBase> for super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &XamlCompositionBrushBase) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-ui")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Composition::IAnimationObject> for &XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<XamlCompositionBrushBase> for Brush {
    fn from(value: XamlCompositionBrushBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for Brush {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Brush> for &XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, Brush> {
        ::windows_core::Param::Owned(::core::convert::Into::<Brush>::into(self))
    }
}
impl ::core::convert::From<XamlCompositionBrushBase> for super::DependencyObject {
    fn from(value: XamlCompositionBrushBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&XamlCompositionBrushBase> for super::DependencyObject {
    fn from(value: &XamlCompositionBrushBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &XamlCompositionBrushBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for XamlCompositionBrushBase {}
unsafe impl ::core::marker::Sync for XamlCompositionBrushBase {}
#[repr(transparent)]
pub struct XamlLight(::windows_core::IUnknown);
impl XamlLight {
    pub fn new() -> ::windows_core::Result<XamlLight> {
        Self::IXamlLightFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<XamlLight>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<XamlLight> {
        Self::IXamlLightFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<XamlLight>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CompositionLight(&self) -> ::windows_core::Result<super::super::Composition::CompositionLight> {
        let this = &::windows_core::Interface::cast::<IXamlLightProtected>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompositionLight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Composition::CompositionLight>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetCompositionLight<'a, Param0: ::windows_core::IntoParam<'a, super::super::Composition::CompositionLight>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IXamlLightProtected>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompositionLight)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AddTargetElement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(lightid: Param0, element: Param1) -> ::windows_core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows_core::Interface::vtable(this).AddTargetElement)(::windows_core::Interface::as_raw(this), lightid.into_param().abi(), element.into_param().abi()).ok() })
    }
    pub fn RemoveTargetElement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(lightid: Param0, element: Param1) -> ::windows_core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveTargetElement)(::windows_core::Interface::as_raw(this), lightid.into_param().abi(), element.into_param().abi()).ok() })
    }
    pub fn AddTargetBrush<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Brush>>(lightid: Param0, brush: Param1) -> ::windows_core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows_core::Interface::vtable(this).AddTargetBrush)(::windows_core::Interface::as_raw(this), lightid.into_param().abi(), brush.into_param().abi()).ok() })
    }
    pub fn RemoveTargetBrush<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, Brush>>(lightid: Param0, brush: Param1) -> ::windows_core::Result<()> {
        Self::IXamlLightStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveTargetBrush)(::windows_core::Interface::as_raw(this), lightid.into_param().abi(), brush.into_param().abi()).ok() })
    }
    pub fn IXamlLightFactory<R, F: FnOnce(&IXamlLightFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlLight, IXamlLightFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlLightStatics<R, F: FnOnce(&IXamlLightStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlLight, IXamlLightStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlLight {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlLight {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlLight {}
impl ::core::fmt::Debug for XamlLight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlLight").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlLight {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.XamlLight;{0cc3fc1f-b327-4a18-9648-7c84db26ce22})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlLight {
    type Vtable = IXamlLight_Vtbl;
    const IID: ::windows_core::GUID = <IXamlLight as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlLight {
    const NAME: &'static str = "Windows.UI.Xaml.Media.XamlLight";
}
impl ::core::convert::From<XamlLight> for ::windows_core::IUnknown {
    fn from(value: XamlLight) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlLight> for ::windows_core::IUnknown {
    fn from(value: &XamlLight) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlLight {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlLight {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlLight> for ::windows_core::IInspectable {
    fn from(value: XamlLight) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlLight> for ::windows_core::IInspectable {
    fn from(value: &XamlLight) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlLight {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlLight {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlLight> for super::DependencyObject {
    fn from(value: XamlLight) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&XamlLight> for super::DependencyObject {
    fn from(value: &XamlLight) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for XamlLight {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &XamlLight {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for XamlLight {}
unsafe impl ::core::marker::Sync for XamlLight {}
