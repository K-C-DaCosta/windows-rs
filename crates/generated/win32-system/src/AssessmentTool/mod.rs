pub const CAccessiblityWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e18f9c6_a3eb_495a_89b7_956482e19f7a);
pub const CInitiateWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x489331dc_f5e0_4528_9fda_45331bf4a571);
pub const CProvideWinSATVisuals: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f377d7e_e551_44f8_9f94_9db392b03b7b);
pub const CQueryAllWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05df8d13_c355_47f4_a11e_851b338cefb8);
pub const CQueryOEMWinSATCustomization: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc47a41b7_b729_424f_9af9_5cb3934f2dfa);
pub const CQueryWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3bdfad3_f276_49e9_9b17_c474f48f0764);
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
#[repr(transparent)]
pub struct IAccessibleWinSAT(::windows_core::IUnknown);
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl IAccessibleWinSAT {
    #[cfg(all(feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accParent(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.accParent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn accChildCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.accChildCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accChild<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accChild)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accName<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accName)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accValue<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accValue)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accDescription<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accDescription)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accRole<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accRole)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accState<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accState)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accHelp<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accHelp)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accHelpTopic<'a, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, pszhelpfile: *mut ::win32_foundation::BSTR, varchild: Param1, pidtopic: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.get_accHelpTopic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszhelpfile), varchild.into_param().abi(), ::core::mem::transmute(pidtopic)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accKeyboardShortcut<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accKeyboardShortcut)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accFocus(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.accFocus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accSelection(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.accSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn get_accDefaultAction<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accDefaultAction)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accSelect<'a, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, flagsselect: i32, varchild: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.accSelect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flagsselect), varchild.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accLocation<'a, Param4: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.accLocation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxleft), ::core::mem::transmute(pytop), ::core::mem::transmute(pcxwidth), ::core::mem::transmute(pcyheight), varchild.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accNavigate<'a, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, navdir: i32, varstart: Param1) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.accNavigate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(navdir), varstart.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.accHitTest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xleft), ::core::mem::transmute(ytop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn accDoDefaultAction<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.accDoDefaultAction)(::windows_core::Interface::as_raw(self), varchild.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn put_accName<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, varchild: Param0, szname: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_accName)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), szname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
    pub unsafe fn put_accValue<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, varchild: Param0, szvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_accValue)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), szvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetAccessiblityData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wsname: Param0, wsvalue: Param1, wsdesc: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAccessiblityData)(::windows_core::Interface::as_raw(self), wsname.into_param().abi(), wsvalue.into_param().abi(), wsdesc.into_param().abi()).ok()
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::convert::From<IAccessibleWinSAT> for ::windows_core::IUnknown {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::convert::From<&IAccessibleWinSAT> for ::windows_core::IUnknown {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAccessibleWinSAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::convert::From<IAccessibleWinSAT> for super::Com::IDispatch {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::convert::From<&IAccessibleWinSAT> for super::Com::IDispatch {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAccessibleWinSAT {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::convert::From<IAccessibleWinSAT> for ::win32_ui::Accessibility::IAccessible {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::convert::From<&IAccessibleWinSAT> for ::win32_ui::Accessibility::IAccessible {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl<'a> ::windows_core::IntoParam<'a, ::win32_ui::Accessibility::IAccessible> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_ui::Accessibility::IAccessible> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl<'a> ::windows_core::IntoParam<'a, ::win32_ui::Accessibility::IAccessible> for &'a IAccessibleWinSAT {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_ui::Accessibility::IAccessible> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::clone::Clone for IAccessibleWinSAT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::PartialEq for IAccessibleWinSAT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::Eq for IAccessibleWinSAT {}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::fmt::Debug for IAccessibleWinSAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleWinSAT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
unsafe impl ::windows_core::Interface for IAccessibleWinSAT {
    type Vtable = IAccessibleWinSAT_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30e6018a_94a8_4ff8_a69a_71b67413f07b);
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWinSAT_Vtbl {
    pub base__: ::win32_ui::Accessibility::IAccessible_Vtbl,
    pub SetAccessiblityData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsname: ::windows_core::PCWSTR, wsvalue: ::windows_core::PCWSTR, wsdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInitiateWinSATAssessment(::windows_core::IUnknown);
impl IInitiateWinSATAssessment {
    pub unsafe fn InitiateAssessment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWinSATInitiateEvents>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, cmdline: Param0, pcallbacks: Param1, callerhwnd: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitiateAssessment)(::windows_core::Interface::as_raw(self), cmdline.into_param().abi(), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    pub unsafe fn InitiateFormalAssessment<'a, Param0: ::windows_core::IntoParam<'a, IWinSATInitiateEvents>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, pcallbacks: Param0, callerhwnd: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitiateFormalAssessment)(::windows_core::Interface::as_raw(self), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    pub unsafe fn CancelAssessment(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAssessment)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IInitiateWinSATAssessment> for ::windows_core::IUnknown {
    fn from(value: IInitiateWinSATAssessment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitiateWinSATAssessment> for ::windows_core::IUnknown {
    fn from(value: &IInitiateWinSATAssessment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInitiateWinSATAssessment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInitiateWinSATAssessment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitiateWinSATAssessment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitiateWinSATAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitiateWinSATAssessment {}
impl ::core::fmt::Debug for IInitiateWinSATAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitiateWinSATAssessment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitiateWinSATAssessment {
    type Vtable = IInitiateWinSATAssessment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd983fc50_f5bf_49d5_b5ed_cccb18aa7fc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitiateWinSATAssessment_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InitiateAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmdline: ::windows_core::PCWSTR, pcallbacks: ::windows_core::RawPtr, callerhwnd: ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub InitiateFormalAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallbacks: ::windows_core::RawPtr, callerhwnd: ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub CancelAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IProvideWinSATAssessmentInfo(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IProvideWinSATAssessmentInfo {
    pub unsafe fn Score(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).Score)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IProvideWinSATAssessmentInfo> for ::windows_core::IUnknown {
    fn from(value: IProvideWinSATAssessmentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IProvideWinSATAssessmentInfo> for ::windows_core::IUnknown {
    fn from(value: &IProvideWinSATAssessmentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IProvideWinSATAssessmentInfo> for super::Com::IDispatch {
    fn from(value: IProvideWinSATAssessmentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IProvideWinSATAssessmentInfo> for super::Com::IDispatch {
    fn from(value: &IProvideWinSATAssessmentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IProvideWinSATAssessmentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IProvideWinSATAssessmentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IProvideWinSATAssessmentInfo {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IProvideWinSATAssessmentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideWinSATAssessmentInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IProvideWinSATAssessmentInfo {
    type Vtable = IProvideWinSATAssessmentInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cd1c380_52d3_4678_ac6f_e929e480be9e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATAssessmentInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Score: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IProvideWinSATResultsInfo(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IProvideWinSATResultsInfo {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows_core::Result<IProvideWinSATAssessmentInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAssessmentInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(assessment), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProvideWinSATAssessmentInfo>(result__)
    }
    pub unsafe fn AssessmentState(&self) -> ::windows_core::Result<WINSAT_ASSESSMENT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WINSAT_ASSESSMENT_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).AssessmentState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WINSAT_ASSESSMENT_STATE>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn AssessmentDateTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AssessmentDateTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SystemRating(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).SystemRating)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn RatingStateDesc(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RatingStateDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IProvideWinSATResultsInfo> for ::windows_core::IUnknown {
    fn from(value: IProvideWinSATResultsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IProvideWinSATResultsInfo> for ::windows_core::IUnknown {
    fn from(value: &IProvideWinSATResultsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IProvideWinSATResultsInfo> for super::Com::IDispatch {
    fn from(value: IProvideWinSATResultsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IProvideWinSATResultsInfo> for super::Com::IDispatch {
    fn from(value: &IProvideWinSATResultsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IProvideWinSATResultsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IProvideWinSATResultsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IProvideWinSATResultsInfo {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IProvideWinSATResultsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideWinSATResultsInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IProvideWinSATResultsInfo {
    type Vtable = IProvideWinSATResultsInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8334d5d_568e_4075_875f_9df341506640);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATResultsInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetAssessmentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetAssessmentInfo: usize,
    pub AssessmentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub AssessmentDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    AssessmentDateTime: usize,
    pub SystemRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows_core::HRESULT,
    pub RatingStateDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProvideWinSATVisuals(::windows_core::IUnknown);
impl IProvideWinSATVisuals {
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows_core::Result<::win32_graphics::Gdi::HBITMAP> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::HBITMAP>::zeroed();
        (::windows_core::Interface::vtable(self).get_Bitmap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bitmapsize), ::core::mem::transmute(state), ::core::mem::transmute(rating), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::HBITMAP>(result__)
    }
}
impl ::core::convert::From<IProvideWinSATVisuals> for ::windows_core::IUnknown {
    fn from(value: IProvideWinSATVisuals) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvideWinSATVisuals> for ::windows_core::IUnknown {
    fn from(value: &IProvideWinSATVisuals) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProvideWinSATVisuals {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProvideWinSATVisuals {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProvideWinSATVisuals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideWinSATVisuals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideWinSATVisuals {}
impl ::core::fmt::Debug for IProvideWinSATVisuals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideWinSATVisuals").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProvideWinSATVisuals {
    type Vtable = IProvideWinSATVisuals_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9f4ade0_871a_42a3_b813_3078d25162c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATVisuals_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-graphics")]
    pub get_Bitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut ::win32_graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-graphics"))]
    get_Bitmap: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IQueryAllWinSATAssessments(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IQueryAllWinSATAssessments {
    #[cfg(all(feature = "win32-data", feature = "win32-system"))]
    pub unsafe fn get_AllXML<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<::win32_data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_AllXML)(::windows_core::Interface::as_raw(self), xpath.into_param().abi(), namespaces.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IQueryAllWinSATAssessments> for ::windows_core::IUnknown {
    fn from(value: IQueryAllWinSATAssessments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IQueryAllWinSATAssessments> for ::windows_core::IUnknown {
    fn from(value: &IQueryAllWinSATAssessments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IQueryAllWinSATAssessments> for super::Com::IDispatch {
    fn from(value: IQueryAllWinSATAssessments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IQueryAllWinSATAssessments> for super::Com::IDispatch {
    fn from(value: &IQueryAllWinSATAssessments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IQueryAllWinSATAssessments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IQueryAllWinSATAssessments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IQueryAllWinSATAssessments {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IQueryAllWinSATAssessments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryAllWinSATAssessments").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IQueryAllWinSATAssessments {
    type Vtable = IQueryAllWinSATAssessments_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b89ed1d_6398_4fea_87fc_567d8d19176f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryAllWinSATAssessments_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-data", feature = "win32-system"))]
    pub get_AllXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdomnodelist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-data", feature = "win32-system")))]
    get_AllXML: usize,
}
#[repr(transparent)]
pub struct IQueryOEMWinSATCustomization(::windows_core::IUnknown);
impl IQueryOEMWinSATCustomization {
    pub unsafe fn GetOEMPrePopulationInfo(&self) -> ::windows_core::Result<WINSAT_OEM_DATA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<WINSAT_OEM_DATA_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetOEMPrePopulationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WINSAT_OEM_DATA_TYPE>(result__)
    }
}
impl ::core::convert::From<IQueryOEMWinSATCustomization> for ::windows_core::IUnknown {
    fn from(value: IQueryOEMWinSATCustomization) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IQueryOEMWinSATCustomization> for ::windows_core::IUnknown {
    fn from(value: &IQueryOEMWinSATCustomization) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IQueryOEMWinSATCustomization {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IQueryOEMWinSATCustomization {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IQueryOEMWinSATCustomization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IQueryOEMWinSATCustomization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryOEMWinSATCustomization {}
impl ::core::fmt::Debug for IQueryOEMWinSATCustomization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryOEMWinSATCustomization").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IQueryOEMWinSATCustomization {
    type Vtable = IQueryOEMWinSATCustomization_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc9a6a9f_ad4e_420e_9953_b34671e9df22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOEMWinSATCustomization_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetOEMPrePopulationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_DATA_TYPE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IQueryRecentWinSATAssessment(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IQueryRecentWinSATAssessment {
    #[cfg(all(feature = "win32-data", feature = "win32-system"))]
    pub unsafe fn get_XML<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, xpath: Param0, namespaces: Param1) -> ::windows_core::Result<::win32_data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_XML)(::windows_core::Interface::as_raw(self), xpath.into_param().abi(), namespaces.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Info(&self) -> ::windows_core::Result<IProvideWinSATResultsInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Info)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProvideWinSATResultsInfo>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IQueryRecentWinSATAssessment> for ::windows_core::IUnknown {
    fn from(value: IQueryRecentWinSATAssessment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IQueryRecentWinSATAssessment> for ::windows_core::IUnknown {
    fn from(value: &IQueryRecentWinSATAssessment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IQueryRecentWinSATAssessment> for super::Com::IDispatch {
    fn from(value: IQueryRecentWinSATAssessment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IQueryRecentWinSATAssessment> for super::Com::IDispatch {
    fn from(value: &IQueryRecentWinSATAssessment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IQueryRecentWinSATAssessment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IQueryRecentWinSATAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IQueryRecentWinSATAssessment {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IQueryRecentWinSATAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryRecentWinSATAssessment").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IQueryRecentWinSATAssessment {
    type Vtable = IQueryRecentWinSATAssessment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8ad5d1f_3b47_4bdc_9375_7c6b1da4eca7);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryRecentWinSATAssessment_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-data", feature = "win32-system"))]
    pub get_XML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdomnodelist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-data", feature = "win32-system")))]
    get_XML: usize,
    #[cfg(feature = "win32-system")]
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Info: usize,
}
#[repr(transparent)]
pub struct IWinSATInitiateEvents(::windows_core::IUnknown);
impl IWinSATInitiateEvents {
    pub unsafe fn WinSATComplete<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hresult: ::windows_core::HRESULT, strdescription: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WinSATComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hresult), strdescription.into_param().abi()).ok()
    }
    pub unsafe fn WinSATUpdate<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WinSATUpdate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ucurrenttick), ::core::mem::transmute(uticktotal), strcurrentstate.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWinSATInitiateEvents> for ::windows_core::IUnknown {
    fn from(value: IWinSATInitiateEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinSATInitiateEvents> for ::windows_core::IUnknown {
    fn from(value: &IWinSATInitiateEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWinSATInitiateEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWinSATInitiateEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWinSATInitiateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWinSATInitiateEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinSATInitiateEvents {}
impl ::core::fmt::Debug for IWinSATInitiateEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinSATInitiateEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWinSATInitiateEvents {
    type Vtable = IWinSATInitiateEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x262a1918_ba0d_41d5_92c2_fab4633ee74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinSATInitiateEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub WinSATComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, strdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WinSATUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(2i32);
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(3i32);
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
impl ::core::marker::Copy for WINSAT_ASSESSMENT_STATE {}
impl ::core::clone::Clone for WINSAT_ASSESSMENT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_ASSESSMENT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WINSAT_ASSESSMENT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
impl ::core::marker::Copy for WINSAT_ASSESSMENT_TYPE {}
impl ::core::clone::Clone for WINSAT_ASSESSMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_ASSESSMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WINSAT_ASSESSMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
impl ::core::marker::Copy for WINSAT_BITMAP_SIZE {}
impl ::core::clone::Clone for WINSAT_BITMAP_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_BITMAP_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WINSAT_BITMAP_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_BITMAP_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_BITMAP_SIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_OEM_DATA_TYPE(pub i32);
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(0i32);
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(1i32);
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(2i32);
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(3i32);
impl ::core::marker::Copy for WINSAT_OEM_DATA_TYPE {}
impl ::core::clone::Clone for WINSAT_OEM_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_OEM_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WINSAT_OEM_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_OEM_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_OEM_DATA_TYPE").field(&self.0).finish()
    }
}
