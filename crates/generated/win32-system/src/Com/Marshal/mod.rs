#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const ::win32_foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree(param0: *const u32, param1: *const ::win32_foundation::BSTR);
        }
        BSTR_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const ::win32_foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree64(param0: *const u32, param1: *const ::win32_foundation::BSTR);
        }
        BSTR_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_foundation::BSTR) -> u32;
        }
        ::core::mem::transmute(BSTR_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_foundation::BSTR) -> u32;
        }
        ::core::mem::transmute(BSTR_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16);
        }
        CLIPFORMAT_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16);
        }
        CLIPFORMAT_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::core::mem::transmute(CLIPFORMAT_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::core::mem::transmute(CLIPFORMAT_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetMarshalSizeMax<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pulsize: *mut u32, riid: *const ::windows_core::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMarshalSizeMax(pulsize: *mut u32, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::HRESULT;
        }
        CoGetMarshalSizeMax(::core::mem::transmute(pulsize), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetStandardMarshal<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(riid: *const ::windows_core::GUID, punk: Param1, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<IMarshal> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStandardMarshal(riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, ppmarshal: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CoGetStandardMarshal(::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMarshal>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetStdMarshalEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkouter: Param0, smexflags: u32) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStdMarshalEx(punkouter: *mut ::core::ffi::c_void, smexflags: u32, ppunkinner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CoGetStdMarshalEx(punkouter.into_param().abi(), ::core::mem::transmute(smexflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalHresult<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(pstm: Param0, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalHresult(pstm: ::windows_core::RawPtr, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT;
        }
        CoMarshalHresult(pstm.into_param().abi(), ::core::mem::transmute(hresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(riid: *const ::windows_core::GUID, punk: Param1) -> ::windows_core::Result<super::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterThreadInterfaceInStream(riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, ppstm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CoMarshalInterThreadInterfaceInStream(::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalInterface<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pstm: Param0, riid: *const ::windows_core::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterface(pstm: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::HRESULT;
        }
        CoMarshalInterface(pstm.into_param().abi(), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoReleaseMarshalData<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseMarshalData(pstm: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CoReleaseMarshalData(pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUnmarshalHresult<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows_core::Result<::windows_core::HRESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalHresult(pstm: ::windows_core::RawPtr, phresult: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        CoUnmarshalHresult(pstm.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUnmarshalInterface<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>, T: ::windows_core::Interface>(pstm: Param0) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalInterface(pstm: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoUnmarshalInterface(pstm.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HACCEL);
        }
        HACCEL_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree64(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HACCEL);
        }
        HACCEL_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::core::mem::transmute(HACCEL_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::core::mem::transmute(HACCEL_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const ::win32_graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree(param0: *const u32, param1: *const ::win32_graphics::Gdi::HBITMAP);
        }
        HBITMAP_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const ::win32_graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree64(param0: *const u32, param1: *const ::win32_graphics::Gdi::HBITMAP);
        }
        HBITMAP_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HBITMAP) -> u32;
        }
        ::core::mem::transmute(HBITMAP_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HBITMAP) -> u32;
        }
        ::core::mem::transmute(HBITMAP_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const ::win32_graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree(param0: *const u32, param1: *const ::win32_graphics::Gdi::HDC);
        }
        HDC_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const ::win32_graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree64(param0: *const u32, param1: *const ::win32_graphics::Gdi::HDC);
        }
        HDC_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HDC) -> u32;
        }
        ::core::mem::transmute(HDC_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HDC) -> u32;
        }
        ::core::mem::transmute(HDC_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize);
        }
        HGLOBAL_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize);
        }
        HGLOBAL_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::core::mem::transmute(HGLOBAL_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::core::mem::transmute(HGLOBAL_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HICON);
        }
        HICON_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree64(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HICON);
        }
        HICON_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> u32;
        }
        ::core::mem::transmute(HICON_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HICON) -> u32;
        }
        ::core::mem::transmute(HICON_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HMENU);
        }
        HMENU_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree64(param0: *const u32, param1: *const ::win32_ui::WindowsAndMessaging::HMENU);
        }
        HMENU_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> u32;
        }
        ::core::mem::transmute(HMENU_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_ui::WindowsAndMessaging::HMENU) -> u32;
        }
        ::core::mem::transmute(HMENU_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_ui::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const ::win32_graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree(param0: *const u32, param1: *const ::win32_graphics::Gdi::HPALETTE);
        }
        HPALETTE_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const ::win32_graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree64(param0: *const u32, param1: *const ::win32_graphics::Gdi::HPALETTE);
        }
        HPALETTE_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HPALETTE) -> u32;
        }
        ::core::mem::transmute(HPALETTE_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics::Gdi::HPALETTE) -> u32;
        }
        ::core::mem::transmute(HPALETTE_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const ::win32_foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree(param0: *const u32, param1: *const ::win32_foundation::HWND);
        }
        HWND_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const ::win32_foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree64(param0: *const u32, param1: *const ::win32_foundation::HWND);
        }
        HWND_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_foundation::HWND) -> u32;
        }
        ::core::mem::transmute(HWND_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_foundation::HWND) -> u32;
        }
        ::core::mem::transmute(HWND_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IMarshal(::windows_core::IUnknown);
impl IMarshal {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUnmarshalClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pcid)).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMarshalSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(psize)).ok()
    }
    pub unsafe fn MarshalInterface<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MarshalInterface)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnmarshalInterface)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMarshalData)(::windows_core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
impl ::core::convert::From<IMarshal> for ::windows_core::IUnknown {
    fn from(value: IMarshal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal> for ::windows_core::IUnknown {
    fn from(value: &IMarshal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMarshal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMarshal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal {}
impl ::core::fmt::Debug for IMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMarshal {
    type Vtable = IMarshal_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000003_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetUnmarshalClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMarshalSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows_core::HRESULT,
    pub MarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisconnectObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMarshal2(::windows_core::IUnknown);
impl IMarshal2 {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetUnmarshalClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pcid)).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMarshalSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(psize)).ok()
    }
    pub unsafe fn MarshalInterface<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows_core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MarshalInterface)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnmarshalInterface)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReleaseMarshalData)(::windows_core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisconnectObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
impl ::core::convert::From<IMarshal2> for ::windows_core::IUnknown {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal2> for ::windows_core::IUnknown {
    fn from(value: &IMarshal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMarshal2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMarshal2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshal2> for IMarshal {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal2> for IMarshal {
    fn from(value: &IMarshal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMarshal> for IMarshal2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMarshal> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMarshal> for &'a IMarshal2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMarshal> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMarshal2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMarshal2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal2 {}
impl ::core::fmt::Debug for IMarshal2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMarshal2 {
    type Vtable = IMarshal2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000001cf_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal2_Vtbl {
    pub base__: IMarshal_Vtbl,
}
#[repr(transparent)]
pub struct IMarshalingStream(::windows_core::IUnknown);
impl IMarshalingStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)))
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::STREAM_SEEK) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(libnewsize)).ok()
    }
    pub unsafe fn CopyTo<'a, Param0: ::windows_core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: super::STGC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnlockRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<super::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IStream>(result__)
    }
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows_core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::<usize>::zeroed();
        (::windows_core::Interface::vtable(self).GetMarshalingContextAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(attribute), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IMarshalingStream> for ::windows_core::IUnknown {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for ::windows_core::IUnknown {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMarshalingStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshalingStream> for super::ISequentialStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for super::ISequentialStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ISequentialStream> for IMarshalingStream {
    fn into_param(self) -> ::windows_core::Param<'a, super::ISequentialStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ISequentialStream> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows_core::Param<'a, super::ISequentialStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshalingStream> for super::IStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for super::IStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStream> for IMarshalingStream {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStream> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMarshalingStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMarshalingStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshalingStream {}
impl ::core::fmt::Debug for IMarshalingStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshalingStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMarshalingStream {
    type Vtable = IMarshalingStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8f2f5e6_6102_4863_9f26_389a4676efde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshalingStream_Vtbl {
    pub base__: super::IStream_Vtbl,
    pub GetMarshalingContextAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows_core::HRESULT,
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        LPSAFEARRAY_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        LPSAFEARRAY_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16);
        }
        SNB_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16);
        }
        SNB_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::core::mem::transmute(SNB_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::core::mem::transmute(SNB_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STDMSHLFLAGS(pub i32);
pub const SMEXF_SERVER: STDMSHLFLAGS = STDMSHLFLAGS(1i32);
pub const SMEXF_HANDLER: STDMSHLFLAGS = STDMSHLFLAGS(2i32);
impl ::core::marker::Copy for STDMSHLFLAGS {}
impl ::core::clone::Clone for STDMSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STDMSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STDMSHLFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STDMSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STDMSHLFLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM);
        }
        STGMEDIUM_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM);
        }
        STGMEDIUM_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32;
        }
        ::core::mem::transmute(STGMEDIUM_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32;
        }
        ::core::mem::transmute(STGMEDIUM_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-system"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT);
        }
        VARIANT_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT);
        }
        VARIANT_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32;
        }
        ::core::mem::transmute(VARIANT_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32;
        }
        ::core::mem::transmute(VARIANT_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
