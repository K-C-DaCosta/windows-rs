#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneLightingEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneLightingEffect {
    type Vtable = ISceneLightingEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AmbientAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetAmbientAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub DiffuseAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetDiffuseAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub NormalMapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    NormalMapSource: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetNormalMapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetNormalMapSource: usize,
    pub SpecularAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpecularAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub SpecularShine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpecularShine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneLightingEffect2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneLightingEffect2 {
    type Vtable = ISceneLightingEffect2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e270e81_72f0_4c5c_95f8_8a6e0024f409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReflectanceModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows_core::HRESULT,
    pub SetReflectanceModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneLightingEffectReflectanceModel) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SceneLightingEffect(::windows_core::IUnknown);
impl SceneLightingEffect {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneLightingEffect, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_graphics::Effects::IGraphicsEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_graphics::Effects::IGraphicsEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), name.into_param().abi()).ok() }
    }
    pub fn AmbientAmount(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).AmbientAmount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetAmbientAmount(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmbientAmount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DiffuseAmount(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).DiffuseAmount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetDiffuseAmount(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDiffuseAmount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn NormalMapSource(&self) -> ::windows_core::Result<::winrt_graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NormalMapSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Effects::IGraphicsEffectSource>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetNormalMapSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Effects::IGraphicsEffectSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalMapSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SpecularAmount(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).SpecularAmount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpecularAmount(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpecularAmount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpecularShine(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).SpecularShine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpecularShine(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpecularShine)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectanceModel(&self) -> ::windows_core::Result<SceneLightingEffectReflectanceModel> {
        let this = &::windows_core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneLightingEffectReflectanceModel>::zeroed();
            (::windows_core::Interface::vtable(this).ReflectanceModel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneLightingEffectReflectanceModel>(result__)
        }
    }
    pub fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReflectanceModel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SceneLightingEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneLightingEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneLightingEffect {}
impl ::core::fmt::Debug for SceneLightingEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneLightingEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneLightingEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Effects.SceneLightingEffect;{91bb5e52-95d1-4f8b-9a5a-6408b24b8c6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffect_Vtbl;
    const IID: ::windows_core::GUID = <ISceneLightingEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.SceneLightingEffect";
}
impl ::core::convert::From<SceneLightingEffect> for ::windows_core::IUnknown {
    fn from(value: SceneLightingEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows_core::IUnknown {
    fn from(value: &SceneLightingEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneLightingEffect> for ::windows_core::IInspectable {
    fn from(value: SceneLightingEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows_core::IInspectable {
    fn from(value: &SceneLightingEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-graphics")]
impl ::core::convert::TryFrom<SceneLightingEffect> for ::winrt_graphics::Effects::IGraphicsEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-graphics")]
impl ::core::convert::TryFrom<&SceneLightingEffect> for ::winrt_graphics::Effects::IGraphicsEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-graphics")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_graphics::Effects::IGraphicsEffect> for SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_graphics::Effects::IGraphicsEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-graphics")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_graphics::Effects::IGraphicsEffect> for &SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_graphics::Effects::IGraphicsEffect> {
        ::core::convert::TryInto::<::winrt_graphics::Effects::IGraphicsEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-graphics")]
impl ::core::convert::TryFrom<SceneLightingEffect> for ::winrt_graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-graphics")]
impl ::core::convert::TryFrom<&SceneLightingEffect> for ::winrt_graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-graphics")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_graphics::Effects::IGraphicsEffectSource> for SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_graphics::Effects::IGraphicsEffectSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-graphics")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_graphics::Effects::IGraphicsEffectSource> for &SceneLightingEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_graphics::Effects::IGraphicsEffectSource> {
        ::core::convert::TryInto::<::winrt_graphics::Effects::IGraphicsEffectSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneLightingEffect {}
unsafe impl ::core::marker::Sync for SceneLightingEffect {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: Self = Self(0i32);
    pub const PhysicallyBasedBlinnPhong: Self = Self(1i32);
}
impl ::core::marker::Copy for SceneLightingEffectReflectanceModel {}
impl ::core::clone::Clone for SceneLightingEffectReflectanceModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneLightingEffectReflectanceModel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SceneLightingEffectReflectanceModel {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneLightingEffectReflectanceModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneLightingEffectReflectanceModel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
