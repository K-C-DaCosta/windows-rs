#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENTERPRISE_DATA_POLICIES(pub u32);
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(0u32);
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(1u32);
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(2u32);
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(4u32);
impl ::core::marker::Copy for ENTERPRISE_DATA_POLICIES {}
impl ::core::clone::Clone for ENTERPRISE_DATA_POLICIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENTERPRISE_DATA_POLICIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ENTERPRISE_DATA_POLICIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENTERPRISE_DATA_POLICIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENTERPRISE_DATA_POLICIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ENTERPRISE_DATA_POLICIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ENTERPRISE_DATA_POLICIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: bool,
}
impl ::core::marker::Copy for FILE_UNPROTECT_OPTIONS {}
impl ::core::clone::Clone for FILE_UNPROTECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_UNPROTECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_UNPROTECT_OPTIONS").field("audit", &self.audit).finish()
    }
}
unsafe impl ::windows_core::Abi for FILE_UNPROTECT_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_UNPROTECT_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_UNPROTECT_OPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_UNPROTECT_OPTIONS {}
impl ::core::default::Default for FILE_UNPROTECT_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: ::win32_foundation::HANDLE,
}
impl ::core::marker::Copy for HTHREAD_NETWORK_CONTEXT {}
impl ::core::clone::Clone for HTHREAD_NETWORK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTHREAD_NETWORK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTHREAD_NETWORK_CONTEXT").field("ThreadId", &self.ThreadId).field("ThreadContext", &self.ThreadContext).finish()
    }
}
unsafe impl ::windows_core::Abi for HTHREAD_NETWORK_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTHREAD_NETWORK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTHREAD_NETWORK_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTHREAD_NETWORK_CONTEXT {}
impl ::core::default::Default for HTHREAD_NETWORK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop(::windows_core::IUnknown);
impl IProtectionPolicyManagerInterop {
    pub unsafe fn RequestAccessForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IProtectionPolicyManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProtectionPolicyManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProtectionPolicyManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtectionPolicyManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IProtectionPolicyManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IProtectionPolicyManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IProtectionPolicyManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IProtectionPolicyManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProtectionPolicyManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectionPolicyManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectionPolicyManagerInterop {}
impl ::core::fmt::Debug for IProtectionPolicyManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectionPolicyManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerInterop {
    type Vtable = IProtectionPolicyManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4652651d_c1fe_4ba1_9f0a_c0f56596f721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop2(::windows_core::IUnknown);
impl IProtectionPolicyManagerInterop2 {
    pub unsafe fn RequestAccessForAppWithWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessForAppWithWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessWithAuditingInfoForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2, auditinfounk: Param3) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessWithAuditingInfoForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfounk.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessWithMessageForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2, auditinfounk: Param3, messagefromapp: Param4) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessWithMessageForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessForAppWithAuditingInfoForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2, auditinfounk: Param3) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessForAppWithAuditingInfoForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessForAppWithMessageForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2, auditinfounk: Param3, messagefromapp: Param4) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessForAppWithMessageForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IProtectionPolicyManagerInterop2> for ::windows_core::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop2> for ::windows_core::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProtectionPolicyManagerInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProtectionPolicyManagerInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtectionPolicyManagerInterop2> for ::windows_core::IInspectable {
    fn from(value: IProtectionPolicyManagerInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop2> for ::windows_core::IInspectable {
    fn from(value: &IProtectionPolicyManagerInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IProtectionPolicyManagerInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IProtectionPolicyManagerInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProtectionPolicyManagerInterop2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectionPolicyManagerInterop2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectionPolicyManagerInterop2 {}
impl ::core::fmt::Debug for IProtectionPolicyManagerInterop2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectionPolicyManagerInterop2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerInterop2 {
    type Vtable = IProtectionPolicyManagerInterop2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x157cfbe4_a78d_4156_b384_61fdac41e686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessForAppWithWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessWithAuditingInfoForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessWithMessageForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessForAppWithAuditingInfoForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessForAppWithMessageForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop3(::windows_core::IUnknown);
impl IProtectionPolicyManagerInterop3 {
    pub unsafe fn RequestAccessWithBehaviorForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessWithBehaviorForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessForAppWithBehaviorForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessForAppWithBehaviorForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessToFilesForAppForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, apppackagefamilyname: Param2, auditinfounk: Param3) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessToFilesForAppForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, apppackagefamilyname: Param2, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessToFilesForProcessForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, processid: u32, auditinfounk: Param3) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessToFilesForProcessForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), ::core::mem::transmute(processid), auditinfounk.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, processid: u32, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), ::core::mem::transmute(processid), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IProtectionPolicyManagerInterop3> for ::windows_core::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop3> for ::windows_core::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProtectionPolicyManagerInterop3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProtectionPolicyManagerInterop3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtectionPolicyManagerInterop3> for ::windows_core::IInspectable {
    fn from(value: IProtectionPolicyManagerInterop3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop3> for ::windows_core::IInspectable {
    fn from(value: &IProtectionPolicyManagerInterop3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IProtectionPolicyManagerInterop3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IProtectionPolicyManagerInterop3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProtectionPolicyManagerInterop3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectionPolicyManagerInterop3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectionPolicyManagerInterop3 {}
impl ::core::fmt::Debug for IProtectionPolicyManagerInterop3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectionPolicyManagerInterop3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerInterop3 {
    type Vtable = IProtectionPolicyManagerInterop3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1c03933_b398_4d93_b0fd_2972adf802c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessWithBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessForAppWithBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessToFilesForAppForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessToFilesForProcessForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: u32, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[inline]
pub unsafe fn ProtectFileToEnterpriseIdentity<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(fileorfolderpath: Param0, identity: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProtectFileToEnterpriseIdentity(fileorfolderpath: ::windows_core::PCWSTR, identity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        ProtectFileToEnterpriseIdentity(fileorfolderpath.into_param().abi(), identity.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SRPHOSTING_TYPE(pub i32);
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = SRPHOSTING_TYPE(0i32);
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = SRPHOSTING_TYPE(1i32);
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = SRPHOSTING_TYPE(2i32);
impl ::core::marker::Copy for SRPHOSTING_TYPE {}
impl ::core::clone::Clone for SRPHOSTING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SRPHOSTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SRPHOSTING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SRPHOSTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SRPHOSTING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SRPHOSTING_VERSION(pub i32);
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = SRPHOSTING_VERSION(1i32);
impl ::core::marker::Copy for SRPHOSTING_VERSION {}
impl ::core::clone::Clone for SRPHOSTING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SRPHOSTING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SRPHOSTING_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SRPHOSTING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SRPHOSTING_VERSION").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_core::HRESULT;
        }
        SrpCloseThreadNetworkContext(::core::mem::transmute(threadnetworkcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpCreateThreadNetworkContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(enterpriseid: Param0) -> ::windows_core::Result<HTHREAD_NETWORK_CONTEXT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpCreateThreadNetworkContext(enterpriseid: ::windows_core::PCWSTR, threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<HTHREAD_NETWORK_CONTEXT>::zeroed();
        SrpCreateThreadNetworkContext(enterpriseid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HTHREAD_NETWORK_CONTEXT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpDisablePermissiveModeFileEncryption() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpDisablePermissiveModeFileEncryption() -> ::windows_core::HRESULT;
        }
        SrpDisablePermissiveModeFileEncryption().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-storage")]
#[inline]
pub unsafe fn SrpDoesPolicyAllowAppExecution(packageid: *const ::win32_storage::Packaging::Appx::PACKAGE_ID) -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpDoesPolicyAllowAppExecution(packageid: *const ::win32_storage::Packaging::Appx::PACKAGE_ID, isallowed: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        SrpDoesPolicyAllowAppExecution(::core::mem::transmute(packageid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpEnablePermissiveModeFileEncryption<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(enterpriseid: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpEnablePermissiveModeFileEncryption(enterpriseid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        SrpEnablePermissiveModeFileEncryption(enterpriseid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpGetEnterpriseIds<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(tokenhandle: Param0, numberofbytes: *mut u32, enterpriseids: *mut ::windows_core::PWSTR, enterpriseidcount: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpGetEnterpriseIds(tokenhandle: ::win32_foundation::HANDLE, numberofbytes: *mut u32, enterpriseids: *mut ::windows_core::PWSTR, enterpriseidcount: *mut u32) -> ::windows_core::HRESULT;
        }
        SrpGetEnterpriseIds(tokenhandle.into_param().abi(), ::core::mem::transmute(numberofbytes), ::core::mem::transmute(enterpriseids), ::core::mem::transmute(enterpriseidcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpGetEnterprisePolicy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(tokenhandle: Param0) -> ::windows_core::Result<ENTERPRISE_DATA_POLICIES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpGetEnterprisePolicy(tokenhandle: ::win32_foundation::HANDLE, policyflags: *mut ENTERPRISE_DATA_POLICIES) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<ENTERPRISE_DATA_POLICIES>::zeroed();
        SrpGetEnterprisePolicy(tokenhandle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ENTERPRISE_DATA_POLICIES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows_core::HRESULT;
        }
        SrpHostingInitialize(::core::mem::transmute(version), ::core::mem::transmute(r#type), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE);
        }
        SrpHostingTerminate(::core::mem::transmute(r#type))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpIsTokenService<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(tokenhandle: Param0, istokenservice: *mut u8) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpIsTokenService(tokenhandle: ::win32_foundation::HANDLE, istokenservice: *mut u8) -> ::win32_foundation::NTSTATUS;
        }
        SrpIsTokenService(tokenhandle.into_param().abi(), ::core::mem::transmute(istokenservice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpSetTokenEnterpriseId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(tokenhandle: Param0, enterpriseid: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpSetTokenEnterpriseId(tokenhandle: ::win32_foundation::HANDLE, enterpriseid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        SrpSetTokenEnterpriseId(tokenhandle.into_param().abi(), enterpriseid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnprotectFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(fileorfolderpath: Param0, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnprotectFile(fileorfolderpath: ::windows_core::PCWSTR, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows_core::HRESULT;
        }
        UnprotectFile(fileorfolderpath.into_param().abi(), ::core::mem::transmute(options)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
