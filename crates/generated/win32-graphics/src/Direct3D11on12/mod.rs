#[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
#[inline]
pub unsafe fn D3D11On12CreateDevice<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pdevice: Param0, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], ppcommandqueues: &[::core::option::Option<::windows_core::IUnknown>], nodemask: u32, ppdevice: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>, ppimmediatecontext: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D11On12CreateDevice(pdevice: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const *mut ::core::ffi::c_void, numqueues: u32, nodemask: u32, ppdevice: *mut ::windows_core::RawPtr, ppimmediatecontext: *mut ::windows_core::RawPtr, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT;
        }
        D3D11On12CreateDevice(pdevice.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(::windows_core::as_ptr_or_null(pfeaturelevels)), pfeaturelevels.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppcommandqueues)), ppcommandqueues.len() as _, ::core::mem::transmute(nodemask), ::core::mem::transmute(ppdevice), ::core::mem::transmute(ppimmediatecontext), ::core::mem::transmute(pchosenfeaturelevel)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl ::core::marker::Copy for D3D11_RESOURCE_FLAGS {}
impl ::core::clone::Clone for D3D11_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D11_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RESOURCE_FLAGS").field("BindFlags", &self.BindFlags).field("MiscFlags", &self.MiscFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("StructureByteStride", &self.StructureByteStride).finish()
    }
}
unsafe impl ::windows_core::Abi for D3D11_RESOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D11_RESOURCE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D11_RESOURCE_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D11_RESOURCE_FLAGS {}
impl ::core::default::Default for D3D11_RESOURCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct ID3D11On12Device(::windows_core::IUnknown);
impl ID3D11On12Device {
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateWrappedResource)(::windows_core::Interface::as_raw(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows_core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).ReleaseWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).AcquireWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
}
impl ::core::convert::From<ID3D11On12Device> for ::windows_core::IUnknown {
    fn from(value: ID3D11On12Device) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device> for ::windows_core::IUnknown {
    fn from(value: &ID3D11On12Device) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ID3D11On12Device {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ID3D11On12Device {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ID3D11On12Device {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device {}
impl ::core::fmt::Debug for ID3D11On12Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device {}
unsafe impl ::core::marker::Sync for ID3D11On12Device {}
unsafe impl ::windows_core::Interface for ID3D11On12Device {
    type Vtable = ID3D11On12Device_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85611e73_70a9_490e_9614_a9e302777904);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-graphics")]
    pub CreateWrappedResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource12: *mut ::core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows_core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-graphics"))]
    CreateWrappedResource: usize,
    #[cfg(feature = "win32-graphics")]
    pub ReleaseWrappedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const ::windows_core::RawPtr, numresources: u32),
    #[cfg(not(feature = "win32-graphics"))]
    ReleaseWrappedResources: usize,
    #[cfg(feature = "win32-graphics")]
    pub AcquireWrappedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const ::windows_core::RawPtr, numresources: u32),
    #[cfg(not(feature = "win32-graphics"))]
    AcquireWrappedResources: usize,
}
#[repr(transparent)]
pub struct ID3D11On12Device1(::windows_core::IUnknown);
impl ID3D11On12Device1 {
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CreateWrappedResource)(::windows_core::Interface::as_raw(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows_core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.ReleaseWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.AcquireWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    pub unsafe fn GetD3D12Device<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetD3D12Device)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ID3D11On12Device1> for ::windows_core::IUnknown {
    fn from(value: ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ::windows_core::IUnknown {
    fn from(value: &ID3D11On12Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ID3D11On12Device1 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ID3D11On12Device1 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D11On12Device1> for ID3D11On12Device {
    fn from(value: ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ID3D11On12Device {
    fn from(value: &ID3D11On12Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ID3D11On12Device> for ID3D11On12Device1 {
    fn into_param(self) -> ::windows_core::Param<'a, ID3D11On12Device> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ID3D11On12Device> for &'a ID3D11On12Device1 {
    fn into_param(self) -> ::windows_core::Param<'a, ID3D11On12Device> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ID3D11On12Device1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device1 {}
impl ::core::fmt::Debug for ID3D11On12Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device1").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device1 {}
unsafe impl ::core::marker::Sync for ID3D11On12Device1 {}
unsafe impl ::windows_core::Interface for ID3D11On12Device1 {
    type Vtable = ID3D11On12Device1_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdb64df4_ea2f_4c70_b861_aaab1258bb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device1_Vtbl {
    pub base__: ID3D11On12Device_Vtbl,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ID3D11On12Device2(::windows_core::IUnknown);
impl ID3D11On12Device2 {
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CreateWrappedResource)(::windows_core::Interface::as_raw(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows_core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.base__.ReleaseWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows_core::Interface::vtable(self).base__.base__.AcquireWrappedResources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    pub unsafe fn GetD3D12Device<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetD3D12Device)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows_core::IntoParam<'a, super::Direct3D11::ID3D11Resource>, Param1: ::windows_core::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>, T: ::windows_core::Interface>(&self, presource11: Param0, pcommandqueue: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).UnwrapUnderlyingResource)(::windows_core::Interface::as_raw(self), presource11.into_param().abi(), pcommandqueue.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows_core::IntoParam<'a, super::Direct3D11::ID3D11Resource>>(&self, presource11: Param0, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReturnUnderlyingResource)(::windows_core::Interface::as_raw(self), presource11.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ::windows_core::IUnknown {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ::windows_core::IUnknown {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ID3D11On12Device2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ID3D11On12Device {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ID3D11On12Device {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ID3D11On12Device> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows_core::Param<'a, ID3D11On12Device> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ID3D11On12Device> for &'a ID3D11On12Device2 {
    fn into_param(self) -> ::windows_core::Param<'a, ID3D11On12Device> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ID3D11On12Device1 {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ID3D11On12Device1 {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ID3D11On12Device1> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows_core::Param<'a, ID3D11On12Device1> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ID3D11On12Device1> for &'a ID3D11On12Device2 {
    fn into_param(self) -> ::windows_core::Param<'a, ID3D11On12Device1> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ID3D11On12Device2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device2 {}
impl ::core::fmt::Debug for ID3D11On12Device2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device2").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device2 {}
unsafe impl ::core::marker::Sync for ID3D11On12Device2 {}
unsafe impl ::windows_core::Interface for ID3D11On12Device2 {
    type Vtable = ID3D11On12Device2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc90f331_4740_43fa_866e_67f12cb58223);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device2_Vtbl {
    pub base__: ID3D11On12Device1_Vtbl,
    #[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource11: ::windows_core::RawPtr, pcommandqueue: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-graphics", feature = "win32-graphics")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource11: ::windows_core::RawPtr, numsync: u32, psignalvalues: *const u64, ppfences: *const ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-graphics", feature = "win32-graphics")))]
    ReturnUnderlyingResource: usize,
}
#[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<::windows_core::IUnknown>, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const ::core::option::Option<::windows_core::IUnknown>, numqueues: u32, param6: u32, param7: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>, param8: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT>;
