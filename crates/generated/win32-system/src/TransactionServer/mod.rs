pub const Catalog: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22881_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22883_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22882_8a19_11d0_81b6_00a0c9231c29);
pub const ComponentUtil: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22884_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ICatalog(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ICatalog {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetCollection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCollection)(::windows_core::Interface::as_raw(self), bstrcollname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrconnectstring: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), bstrconnectstring.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MajorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(retval)).ok()
    }
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MinorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(retval)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ICatalog> for ::windows_core::IUnknown {
    fn from(value: ICatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ICatalog> for ::windows_core::IUnknown {
    fn from(value: &ICatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ICatalog> for super::Com::IDispatch {
    fn from(value: ICatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ICatalog> for super::Com::IDispatch {
    fn from(value: &ICatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICatalog {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICatalog {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ICatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ICatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ICatalog {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ICatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalog").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ICatalog {
    type Vtable = ICatalog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22870_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetCollection: usize,
    #[cfg(feature = "win32-system")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IComponentUtil(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IComponentUtil {
    pub unsafe fn InstallComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, bstrproxystubdllfile: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallComponent)(::windows_core::Interface::as_raw(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), bstrproxystubdllfile.into_param().abi()).ok()
    }
    pub unsafe fn ImportComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImportComponent)(::windows_core::Interface::as_raw(self), bstrclsid.into_param().abi()).ok()
    }
    pub unsafe fn ImportComponentByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprogid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImportComponentByName)(::windows_core::Interface::as_raw(self), bstrprogid.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetCLSIDs<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdllfile: Param0, bstrtypelibfile: Param1, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCLSIDs)(::windows_core::Interface::as_raw(self), bstrdllfile.into_param().abi(), bstrtypelibfile.into_param().abi(), ::core::mem::transmute(aclsids)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IComponentUtil> for ::windows_core::IUnknown {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IComponentUtil> for ::windows_core::IUnknown {
    fn from(value: &IComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IComponentUtil> for super::Com::IDispatch {
    fn from(value: IComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IComponentUtil> for super::Com::IDispatch {
    fn from(value: &IComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IComponentUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IComponentUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IComponentUtil {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IComponentUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IComponentUtil {
    type Vtable = IComponentUtil_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22873_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrproxystubdllfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ImportComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ImportComponentByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetCLSIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetCLSIDs: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPackageUtil(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPackageUtil {
    pub unsafe fn InstallPackage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpackagefile: Param0, bstrinstallpath: Param1, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallPackage)(::windows_core::Interface::as_raw(self), bstrpackagefile.into_param().abi(), bstrinstallpath.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    pub unsafe fn ExportPackage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpackageid: Param0, bstrpackagefile: Param1, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExportPackage)(::windows_core::Interface::as_raw(self), bstrpackageid.into_param().abi(), bstrpackagefile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    pub unsafe fn ShutdownPackage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpackageid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShutdownPackage)(::windows_core::Interface::as_raw(self), bstrpackageid.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPackageUtil> for ::windows_core::IUnknown {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPackageUtil> for ::windows_core::IUnknown {
    fn from(value: &IPackageUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPackageUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPackageUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPackageUtil> for super::Com::IDispatch {
    fn from(value: IPackageUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPackageUtil> for super::Com::IDispatch {
    fn from(value: &IPackageUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IPackageUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IPackageUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPackageUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPackageUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPackageUtil {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPackageUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPackageUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPackageUtil {
    type Vtable = IPackageUtil_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22874_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackagefile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrinstallpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: i32) -> ::windows_core::HRESULT,
    pub ExportPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpackagefile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: i32) -> ::windows_core::HRESULT,
    pub ShutdownPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRemoteComponentUtil(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRemoteComponentUtil {
    pub unsafe fn InstallRemoteComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrserver: Param0, bstrpackageid: Param1, bstrclsid: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallRemoteComponent)(::windows_core::Interface::as_raw(self), bstrserver.into_param().abi(), bstrpackageid.into_param().abi(), bstrclsid.into_param().abi()).ok()
    }
    pub unsafe fn InstallRemoteComponentByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrserver: Param0, bstrpackagename: Param1, bstrprogid: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallRemoteComponentByName)(::windows_core::Interface::as_raw(self), bstrserver.into_param().abi(), bstrpackagename.into_param().abi(), bstrprogid.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRemoteComponentUtil> for ::windows_core::IUnknown {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRemoteComponentUtil> for ::windows_core::IUnknown {
    fn from(value: &IRemoteComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRemoteComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRemoteComponentUtil> for super::Com::IDispatch {
    fn from(value: IRemoteComponentUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRemoteComponentUtil> for super::Com::IDispatch {
    fn from(value: &IRemoteComponentUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRemoteComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRemoteComponentUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRemoteComponentUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRemoteComponentUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRemoteComponentUtil {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRemoteComponentUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteComponentUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRemoteComponentUtil {
    type Vtable = IRemoteComponentUtil_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22875_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallRemoteComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpackageid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub InstallRemoteComponentByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpackagename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IRoleAssociationUtil(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IRoleAssociationUtil {
    pub unsafe fn AssociateRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AssociateRole)(::windows_core::Interface::as_raw(self), bstrroleid.into_param().abi()).ok()
    }
    pub unsafe fn AssociateRoleByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrolename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AssociateRoleByName)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRoleAssociationUtil> for ::windows_core::IUnknown {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRoleAssociationUtil> for ::windows_core::IUnknown {
    fn from(value: &IRoleAssociationUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRoleAssociationUtil {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IRoleAssociationUtil> for super::Com::IDispatch {
    fn from(value: IRoleAssociationUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IRoleAssociationUtil> for super::Com::IDispatch {
    fn from(value: &IRoleAssociationUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IRoleAssociationUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IRoleAssociationUtil {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IRoleAssociationUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IRoleAssociationUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IRoleAssociationUtil {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IRoleAssociationUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoleAssociationUtil").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IRoleAssociationUtil {
    type Vtable = IRoleAssociationUtil_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22876_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IRoleAssociationUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AssociateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AssociateRoleByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
pub const PackageUtil: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
pub const RemoteComponentUtil: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
pub const RoleAssociationUtil: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0001(pub i32);
pub const mtsInstallUsers: __MIDL___MIDL_itf_mtxadmin_0107_0001 = __MIDL___MIDL_itf_mtxadmin_0107_0001(1i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mtxadmin_0107_0001 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mtxadmin_0107_0001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mtxadmin_0107_0001").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0002(pub i32);
pub const mtsExportUsers: __MIDL___MIDL_itf_mtxadmin_0107_0002 = __MIDL___MIDL_itf_mtxadmin_0107_0002(1i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mtxadmin_0107_0002 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mtxadmin_0107_0002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mtxadmin_0107_0002").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mtxadmin_0107_0003(pub i32);
pub const mtsErrObjectErrors: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368511i32);
pub const mtsErrObjectInvalid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368510i32);
pub const mtsErrKeyMissing: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368509i32);
pub const mtsErrAlreadyInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368508i32);
pub const mtsErrDownloadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368507i32);
pub const mtsErrPDFWriteFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368505i32);
pub const mtsErrPDFReadFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368504i32);
pub const mtsErrPDFVersion: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368503i32);
pub const mtsErrCoReqCompInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
pub const mtsErrBadPath: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368502i32);
pub const mtsErrPackageExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368501i32);
pub const mtsErrRoleExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368500i32);
pub const mtsErrCantCopyFile: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368499i32);
pub const mtsErrNoTypeLib: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368498i32);
pub const mtsErrNoUser: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368497i32);
pub const mtsErrInvalidUserids: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368496i32);
pub const mtsErrNoRegistryCLSID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368495i32);
pub const mtsErrBadRegistryProgID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368494i32);
pub const mtsErrAuthenticationLevel: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368493i32);
pub const mtsErrUserPasswdNotValid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368492i32);
pub const mtsErrNoRegistryRead: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368491i32);
pub const mtsErrNoRegistryWrite: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368490i32);
pub const mtsErrNoRegistryRepair: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368489i32);
pub const mtsErrCLSIDOrIIDMismatch: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368488i32);
pub const mtsErrRemoteInterface: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368487i32);
pub const mtsErrDllRegisterServer: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368486i32);
pub const mtsErrNoServerShare: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368485i32);
pub const mtsErrNoAccessToUNC: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368484i32);
pub const mtsErrDllLoadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368483i32);
pub const mtsErrBadRegistryLibID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368482i32);
pub const mtsErrPackDirNotFound: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368481i32);
pub const mtsErrTreatAs: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368480i32);
pub const mtsErrBadForward: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368479i32);
pub const mtsErrBadIID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368478i32);
pub const mtsErrRegistrarFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368477i32);
pub const mtsErrCompFileDoesNotExist: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368476i32);
pub const mtsErrCompFileLoadDLLFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368475i32);
pub const mtsErrCompFileGetClassObj: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368474i32);
pub const mtsErrCompFileClassNotAvail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368473i32);
pub const mtsErrCompFileBadTLB: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368472i32);
pub const mtsErrCompFileNotInstallable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368471i32);
pub const mtsErrNotChangeable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368470i32);
pub const mtsErrNotDeletable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368469i32);
pub const mtsErrSession: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368468i32);
pub const mtsErrCompFileNoRegistrar: __MIDL___MIDL_itf_mtxadmin_0107_0003 = __MIDL___MIDL_itf_mtxadmin_0107_0003(-2146368460i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mtxadmin_0107_0003 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mtxadmin_0107_0003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mtxadmin_0107_0003").field(&self.0).finish()
    }
}
