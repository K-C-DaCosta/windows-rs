#[repr(transparent)]
pub struct IDummyHICONIncluder(::windows_core::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(all(feature = "win32-graphics", feature = "win32-ui"))]
    pub unsafe fn Dummy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_ui::WindowsAndMessaging::HICON>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(&self, h1: Param0, h2: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Dummy)(::windows_core::Interface::as_raw(self), h1.into_param().abi(), h2.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDummyHICONIncluder> for ::windows_core::IUnknown {
    fn from(value: IDummyHICONIncluder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDummyHICONIncluder> for ::windows_core::IUnknown {
    fn from(value: &IDummyHICONIncluder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDummyHICONIncluder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDummyHICONIncluder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDummyHICONIncluder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDummyHICONIncluder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDummyHICONIncluder {}
impl ::core::fmt::Debug for IDummyHICONIncluder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDummyHICONIncluder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x947990de_cc28_11d2_a0f7_00805f858fb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-graphics", feature = "win32-ui"))]
    pub Dummy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, h1: ::win32_ui::WindowsAndMessaging::HICON, h2: ::win32_graphics::Gdi::HDC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-graphics", feature = "win32-ui")))]
    Dummy: usize,
}
#[repr(transparent)]
pub struct IThumbnailExtractor(::windows_core::IUnknown);
impl IThumbnailExtractor {
    #[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
    pub unsafe fn ExtractThumbnail<'a, Param0: ::windows_core::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut ::win32_graphics::Gdi::HBITMAP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExtractThumbnail)(::windows_core::Interface::as_raw(self), pstg.into_param().abi(), ::core::mem::transmute(ullength), ::core::mem::transmute(ulheight), ::core::mem::transmute(puloutputlength), ::core::mem::transmute(puloutputheight), ::core::mem::transmute(phoutputbitmap)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn OnFileUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::StructuredStorage::IStorage>>(&self, pstg: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnFileUpdated)(::windows_core::Interface::as_raw(self), pstg.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IThumbnailExtractor> for ::windows_core::IUnknown {
    fn from(value: IThumbnailExtractor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IThumbnailExtractor> for ::windows_core::IUnknown {
    fn from(value: &IThumbnailExtractor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IThumbnailExtractor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IThumbnailExtractor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IThumbnailExtractor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IThumbnailExtractor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailExtractor {}
impl ::core::fmt::Debug for IThumbnailExtractor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailExtractor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x969dc708_5c76_11d1_8d86_0000f804b057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
    pub ExtractThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: ::windows_core::RawPtr, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut ::win32_graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-graphics", feature = "win32-system")))]
    ExtractThumbnail: usize,
    #[cfg(feature = "win32-system")]
    pub OnFileUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    OnFileUpdated: usize,
}
