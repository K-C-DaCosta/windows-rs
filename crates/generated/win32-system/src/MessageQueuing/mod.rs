#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FOREIGN_STATUS(pub i32);
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(0i32);
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(1i32);
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = FOREIGN_STATUS(2i32);
impl ::core::marker::Copy for FOREIGN_STATUS {}
impl ::core::clone::Clone for FOREIGN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FOREIGN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FOREIGN_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FOREIGN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOREIGN_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQApplication(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQApplication {
    pub unsafe fn MachineIdOfMachineName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, machinename: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MachineIdOfMachineName)(::windows_core::Interface::as_raw(self), machinename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication> for ::windows_core::IUnknown {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication> for ::windows_core::IUnknown {
    fn from(value: &IMSMQApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication> for super::Com::IDispatch {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQApplication {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQApplication {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQApplication {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQApplication {
    type Vtable = IMSMQApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e085_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MachineIdOfMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrguid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQApplication2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQApplication2 {
    pub unsafe fn MachineIdOfMachineName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, machinename: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MachineIdOfMachineName)(::windows_core::Interface::as_raw(self), machinename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(externalcertificate)).ok()
    }
    pub unsafe fn MachineNameOfMachineId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MachineNameOfMachineId)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MSMQVersionMajor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MSMQVersionMinor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MSMQVersionBuild)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsDsEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication2> for ::windows_core::IUnknown {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication2> for super::Com::IDispatch {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication2> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication2> for IMSMQApplication {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication2> for IMSMQApplication {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQApplication> for IMSMQApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQApplication> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQApplication> for &'a IMSMQApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQApplication> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQApplication2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQApplication2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQApplication2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQApplication2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQApplication2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQApplication2 {
    type Vtable = IMSMQApplication2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12a30900_7300_11d2_b0e6_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication2_Vtbl {
    pub base__: IMSMQApplication_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RegisterCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RegisterCertificate: usize,
    pub MachineNameOfMachineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrmachinename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub MSMQVersionMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows_core::HRESULT,
    pub MSMQVersionMinor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows_core::HRESULT,
    pub MSMQVersionBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows_core::HRESULT,
    pub IsDsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQApplication3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQApplication3 {
    pub unsafe fn MachineIdOfMachineName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, machinename: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MachineIdOfMachineName)(::windows_core::Interface::as_raw(self), machinename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RegisterCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(externalcertificate)).ok()
    }
    pub unsafe fn MachineNameOfMachineId<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MachineNameOfMachineId)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MSMQVersionMajor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MSMQVersionMinor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MSMQVersionBuild)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsDsEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ActiveQueues(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ActiveQueues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PrivateQueues(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PrivateQueues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DirectoryServiceServer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DirectoryServiceServer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsConnected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BytesInAllQueues(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).BytesInAllQueues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetMachine<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmachine: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMachine)(::windows_core::Interface::as_raw(self), bstrmachine.into_param().abi()).ok()
    }
    pub unsafe fn Machine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Machine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Connect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Tidy(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Tidy)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication3> for ::windows_core::IUnknown {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication3> for super::Com::IDispatch {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication3> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication3> for IMSMQApplication {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication3> for IMSMQApplication {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQApplication> for IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQApplication> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQApplication> for &'a IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQApplication> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQApplication2> for IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQApplication2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQApplication2> for &'a IMSMQApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQApplication2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQApplication3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQApplication3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQApplication3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQApplication3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQApplication3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQApplication3 {
    type Vtable = IMSMQApplication3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b1f_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication3_Vtbl {
    pub base__: IMSMQApplication2_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ActiveQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ActiveQueues: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PrivateQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PrivateQueues: usize,
    pub DirectoryServiceServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BytesInAllQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BytesInAllQueues: usize,
    pub SetMachine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Machine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Tidy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQCollection {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Item(&self, index: *const super::Com::VARIANT) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCollection> for ::windows_core::IUnknown {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCollection> for ::windows_core::IUnknown {
    fn from(value: &IMSMQCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCollection> for super::Com::IDispatch {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCollection> for super::Com::IDispatch {
    fn from(value: &IMSMQCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQCollection {
    type Vtable = IMSMQCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0188ac2f_ecb3_4173_9779_635ca2039c72);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQCoordinatedTransactionDispenser {
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginTransaction(&self) -> ::windows_core::Result<IMSMQTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser> for ::windows_core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser> for ::windows_core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQCoordinatedTransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQCoordinatedTransactionDispenser {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQCoordinatedTransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCoordinatedTransactionDispenser").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQCoordinatedTransactionDispenser {
    type Vtable = IMSMQCoordinatedTransactionDispenser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e081_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginTransaction: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQCoordinatedTransactionDispenser2 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginTransaction(&self) -> ::windows_core::Result<IMSMQTransaction2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser2> for ::windows_core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQCoordinatedTransactionDispenser2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQCoordinatedTransactionDispenser2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQCoordinatedTransactionDispenser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCoordinatedTransactionDispenser2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQCoordinatedTransactionDispenser2 {
    type Vtable = IMSMQCoordinatedTransactionDispenser2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b10_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginTransaction: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQCoordinatedTransactionDispenser3 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginTransaction(&self) -> ::windows_core::Result<IMSMQTransaction3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser3> for ::windows_core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQCoordinatedTransactionDispenser3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQCoordinatedTransactionDispenser3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQCoordinatedTransactionDispenser3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCoordinatedTransactionDispenser3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQCoordinatedTransactionDispenser3 {
    type Vtable = IMSMQCoordinatedTransactionDispenser3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b14_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginTransaction: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQDestination(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQDestination {
    pub unsafe fn Open(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn IADs(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).IADs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_IADs<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, piads: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_IADs)(::windows_core::Interface::as_raw(self), piads.into_param().abi()).ok()
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ADsPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetADsPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstradspath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetADsPath)(::windows_core::Interface::as_raw(self), bstradspath.into_param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPathName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPathName)(::windows_core::Interface::as_raw(self), bstrpathname.into_param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFormatName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormatName)(::windows_core::Interface::as_raw(self), bstrformatname.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Destinations(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Destinations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Destinations<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, pdestinations: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Destinations)(::windows_core::Interface::as_raw(self), pdestinations.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQDestination> for ::windows_core::IUnknown {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQDestination> for ::windows_core::IUnknown {
    fn from(value: &IMSMQDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQDestination {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQDestination {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQDestination> for super::Com::IDispatch {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQDestination> for super::Com::IDispatch {
    fn from(value: &IMSMQDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQDestination {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQDestination {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQDestination {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQDestination {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQDestination {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQDestination {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQDestination").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQDestination {
    type Vtable = IMSMQDestination_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b16_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub IADs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiads: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    IADs: usize,
    #[cfg(feature = "win32-system")]
    pub putref_IADs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piads: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_IADs: usize,
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Destinations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestinations: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Destinations: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Destinations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinations: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Destinations: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQEvent {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent> for ::windows_core::IUnknown {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent> for ::windows_core::IUnknown {
    fn from(value: &IMSMQEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent> for super::Com::IDispatch {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQEvent {
    type Vtable = IMSMQEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e077_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQEvent2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQEvent2 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent2> for ::windows_core::IUnknown {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent2> for super::Com::IDispatch {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent2> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent2> for IMSMQEvent {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent2> for IMSMQEvent {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQEvent> for IMSMQEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQEvent> for &'a IMSMQEvent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQEvent2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQEvent2 {
    type Vtable = IMSMQEvent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b12_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent2_Vtbl {
    pub base__: IMSMQEvent_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQEvent3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQEvent3 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent3> for ::windows_core::IUnknown {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent3> for super::Com::IDispatch {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent3> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent3> for IMSMQEvent {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent3> for IMSMQEvent {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQEvent> for IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQEvent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQEvent> for &'a IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQEvent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQEvent2> for IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQEvent2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQEvent2> for &'a IMSMQEvent3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQEvent2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQEvent3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQEvent3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQEvent3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQEvent3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQEvent3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQEvent3 {
    type Vtable = IMSMQEvent3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b1c_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent3_Vtbl {
    pub base__: IMSMQEvent2_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQManagement(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQManagement {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Machine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Machine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MessageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ForeignStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).QueueType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionalStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).BytesInQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQManagement> for ::windows_core::IUnknown {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQManagement> for ::windows_core::IUnknown {
    fn from(value: &IMSMQManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQManagement> for super::Com::IDispatch {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQManagement {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQManagement {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQManagement {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQManagement").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQManagement {
    type Vtable = IMSMQManagement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe5f0241_e489_4957_8cc4_a452fcf3e23e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQManagement_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Init: usize,
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Machine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows_core::HRESULT,
    pub ForeignStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows_core::HRESULT,
    pub QueueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows_core::HRESULT,
    pub TransactionalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BytesInQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BytesInQueue: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQMessage(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQMessage {
    pub unsafe fn Class(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Class)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAuthenticated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Delivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Trace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lappspecific)).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourceMachineGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BodyLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Body(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Body)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBody)(::windows_core::Interface::as_raw(self), varbody.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Id(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CorrelationId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CorrelationId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCorrelationId)(::windows_core::Interface::as_raw(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Ack)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lack)).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SentTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SentTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ArrivedTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderCertificate)(::windows_core::Interface::as_raw(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Send<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueue>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage> for ::windows_core::IUnknown {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage> for ::windows_core::IUnknown {
    fn from(value: &IMSMQMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage> for super::Com::IDispatch {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQMessage {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQMessage {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQMessage {
    type Vtable = IMSMQMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e074_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Body: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetBody: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Id: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CorrelationId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SentTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ArrivedTime: usize,
    #[cfg(feature = "win32-system")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: ::windows_core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQMessage2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQMessage2 {
    pub unsafe fn Class(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Class)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAuthenticated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Delivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Trace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lappspecific)).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourceMachineGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BodyLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Body(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Body)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBody)(::windows_core::Interface::as_raw(self), varbody.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo_v1<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Id(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CorrelationId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CorrelationId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCorrelationId)(::windows_core::Interface::as_raw(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Ack)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lack)).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SentTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SentTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ArrivedTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderCertificate)(::windows_core::Interface::as_raw(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Send<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueue2>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Extension(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Extension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetExtension<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varextension: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExtension)(::windows_core::Interface::as_raw(self), varextension.into_param().abi()).ok()
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectorTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetConnectorTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidconnectortype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectorTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidconnectortype.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationSymmetricKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, vardestsymmkey: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(::windows_core::Interface::as_raw(self), vardestsymmkey.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Signature(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Signature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSignature<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsignature: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSignature)(::windows_core::Interface::as_raw(self), varsignature.into_param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationProviderType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationProviderType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthprovtype)).ok()
    }
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationProviderName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthenticationProviderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrauthprovname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationProviderName)(::windows_core::Interface::as_raw(self), bstrauthprovname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsenderid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderId)(::windows_core::Interface::as_raw(self), varsenderid.into_param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MsgClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMsgClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmsgclass)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn TransactionId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLastInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage2> for ::windows_core::IUnknown {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage2> for super::Com::IDispatch {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage2> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQMessage2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQMessage2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQMessage2 {
    type Vtable = IMSMQMessage2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9933be0_a567_11d2_b0f3_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Body: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetBody: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Id: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CorrelationId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SentTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ArrivedTime: usize,
    #[cfg(feature = "win32-system")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: ::windows_core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Extension: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetExtension: usize,
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Signature: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQMessage3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQMessage3 {
    pub unsafe fn Class(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Class)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAuthenticated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Delivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Trace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lappspecific)).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourceMachineGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BodyLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Body(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Body)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBody)(::windows_core::Interface::as_raw(self), varbody.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo_v1<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Id(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CorrelationId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CorrelationId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCorrelationId)(::windows_core::Interface::as_raw(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Ack)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lack)).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SentTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SentTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ArrivedTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderCertificate)(::windows_core::Interface::as_raw(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Send<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Extension(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Extension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetExtension<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varextension: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExtension)(::windows_core::Interface::as_raw(self), varextension.into_param().abi()).ok()
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectorTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetConnectorTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidconnectortype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectorTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidconnectortype.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationSymmetricKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, vardestsymmkey: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(::windows_core::Interface::as_raw(self), vardestsymmkey.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Signature(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Signature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSignature<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsignature: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSignature)(::windows_core::Interface::as_raw(self), varsignature.into_param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationProviderType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationProviderType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthprovtype)).ok()
    }
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationProviderName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthenticationProviderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrauthprovname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationProviderName)(::windows_core::Interface::as_raw(self), bstrauthprovname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsenderid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderId)(::windows_core::Interface::as_raw(self), varsenderid.into_param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MsgClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMsgClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmsgclass)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn TransactionId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLastInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo_v2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo_v2<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo_v2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo_v2<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo_v2)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo3>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo3>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseDestination(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseDestination)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseDestination<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, pdestresponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseDestination)(::windows_core::Interface::as_raw(self), pdestresponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Destination(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Destination)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAuthenticated2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstInTransaction2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLastInTransaction2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachCurrentSecurityContext2)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SoapEnvelope(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SoapEnvelope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CompoundMessage(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CompoundMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetSoapHeader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsoapheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSoapHeader)(::windows_core::Interface::as_raw(self), bstrsoapheader.into_param().abi()).ok()
    }
    pub unsafe fn SetSoapBody<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsoapbody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSoapBody)(::windows_core::Interface::as_raw(self), bstrsoapbody.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage3> for ::windows_core::IUnknown {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQMessage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQMessage3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQMessage3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage3> for super::Com::IDispatch {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage3> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQMessage3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQMessage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQMessage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQMessage3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQMessage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQMessage3 {
    type Vtable = IMSMQMessage3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b1a_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Body: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetBody: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Id: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CorrelationId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SentTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ArrivedTime: usize,
    #[cfg(feature = "win32-system")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: ::windows_core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Extension: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetExtension: usize,
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Signature: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseDestination: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestresponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "win32-system")]
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Destination: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SoapEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CompoundMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CompoundMessage: usize,
    pub SetSoapHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetSoapBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQMessage4(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQMessage4 {
    pub unsafe fn Class(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Class)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAuthenticated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Delivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Trace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAppSpecific)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lappspecific)).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourceMachineGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BodyLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Body(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Body)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBody)(::windows_core::Interface::as_raw(self), varbody.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo_v1<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Id(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CorrelationId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CorrelationId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCorrelationId)(::windows_core::Interface::as_raw(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Ack)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lack)).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxTimeToReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHashAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEncryptAlgorithm)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SentTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SentTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ArrivedTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderCertificate)(::windows_core::Interface::as_raw(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SenderId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SenderId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderIdType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Send<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SenderVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Extension(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Extension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetExtension<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varextension: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExtension)(::windows_core::Interface::as_raw(self), varextension.into_param().abi()).ok()
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ConnectorTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetConnectorTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidconnectortype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectorTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidconnectortype.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationSymmetricKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, vardestsymmkey: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(::windows_core::Interface::as_raw(self), vardestsymmkey.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Signature(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Signature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSignature<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsignature: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSignature)(::windows_core::Interface::as_raw(self), varsignature.into_param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationProviderType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationProviderType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthprovtype)).ok()
    }
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AuthenticationProviderName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthenticationProviderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrauthprovname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationProviderName)(::windows_core::Interface::as_raw(self), bstrauthprovname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSenderId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsenderid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSenderId)(::windows_core::Interface::as_raw(self), varsenderid.into_param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MsgClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMsgClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmsgclass)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn TransactionId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLastInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo_v2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo_v2<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo_v2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo_v2<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo_v2)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo4>>(&self, pqinforesponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows_core::Interface::as_raw(self), pqinforesponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AdminQueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueueInfo4>>(&self, pqinfoadmin: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_AdminQueueInfo)(::windows_core::Interface::as_raw(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResponseDestination(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResponseDestination)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_ResponseDestination<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, pdestresponse: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ResponseDestination)(::windows_core::Interface::as_raw(self), pdestresponse.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Destination(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Destination)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupId(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAuthenticated2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstInTransaction2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsLastInTransaction2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachCurrentSecurityContext2)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SoapEnvelope(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SoapEnvelope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CompoundMessage(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CompoundMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetSoapHeader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsoapheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSoapHeader)(::windows_core::Interface::as_raw(self), bstrsoapheader.into_param().abi()).ok()
    }
    pub unsafe fn SetSoapBody<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsoapbody: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSoapBody)(::windows_core::Interface::as_raw(self), bstrsoapbody.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage4> for ::windows_core::IUnknown {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage4> for ::windows_core::IUnknown {
    fn from(value: &IMSMQMessage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQMessage4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQMessage4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQMessage4> for super::Com::IDispatch {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQMessage4> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQMessage4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQMessage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQMessage4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQMessage4 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQMessage4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage4").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQMessage4 {
    type Vtable = IMSMQMessage4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b23_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Body: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetBody: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Id: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CorrelationId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SentTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ArrivedTime: usize,
    #[cfg(feature = "win32-system")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: ::windows_core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Extension: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetExtension: usize,
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Signature: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "win32-system")]
    pub ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResponseDestination: usize,
    #[cfg(feature = "win32-system")]
    pub putref_ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestresponse: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "win32-system")]
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Destination: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SoapEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CompoundMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CompoundMessage: usize,
    pub SetSoapHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetSoapBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQOutgoingQueueManagement(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQOutgoingQueueManagement {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Machine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Machine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MessageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ForeignStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueueType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransactionalStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BytesInQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn NextHops(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).NextHops)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EodGetSendInfo(&self) -> ::windows_core::Result<IMSMQCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EodGetSendInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQCollection>(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EodResend(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EodResend)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for ::windows_core::IUnknown {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for ::windows_core::IUnknown {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for super::Com::IDispatch {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQManagement> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQManagement> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQManagement> for &'a IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQManagement> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQOutgoingQueueManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQOutgoingQueueManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQOutgoingQueueManagement {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQOutgoingQueueManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQOutgoingQueueManagement").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQOutgoingQueueManagement {
    type Vtable = IMSMQOutgoingQueueManagement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64c478fb_f9b0_4695_8a7f_439ac94326d3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQOutgoingQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub NextHops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    NextHops: usize,
    #[cfg(feature = "win32-system")]
    pub EodGetSendInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EodGetSendInfo: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EodResend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQPrivateDestination(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQPrivateDestination {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Handle(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Handle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetHandle<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varhandle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHandle)(::windows_core::Interface::as_raw(self), varhandle.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQPrivateDestination> for ::windows_core::IUnknown {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQPrivateDestination> for ::windows_core::IUnknown {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQPrivateDestination {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQPrivateDestination {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQPrivateDestination> for super::Com::IDispatch {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQPrivateDestination> for super::Com::IDispatch {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQPrivateDestination {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQPrivateDestination {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQPrivateDestination {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQPrivateDestination {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQPrivateDestination {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQPrivateDestination {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQPrivateDestination").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQPrivateDestination {
    type Vtable = IMSMQPrivateDestination_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b17_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Handle: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetHandle: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQPrivateEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQPrivateEvent {
    pub unsafe fn Hwnd(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Hwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn FireArrivedEvent<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueue>>(&self, pq: Param0, msgcursor: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FireArrivedEvent)(::windows_core::Interface::as_raw(self), pq.into_param().abi(), ::core::mem::transmute(msgcursor)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn FireArrivedErrorEvent<'a, Param0: ::windows_core::IntoParam<'a, IMSMQQueue>>(&self, pq: Param0, hrstatus: ::windows_core::HRESULT, msgcursor: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FireArrivedErrorEvent)(::windows_core::Interface::as_raw(self), pq.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(msgcursor)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQPrivateEvent> for ::windows_core::IUnknown {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQPrivateEvent> for ::windows_core::IUnknown {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQPrivateEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQPrivateEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQPrivateEvent> for super::Com::IDispatch {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQPrivateEvent> for super::Com::IDispatch {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQPrivateEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQPrivateEvent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQPrivateEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQPrivateEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQPrivateEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQPrivateEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQPrivateEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQPrivateEvent {
    type Vtable = IMSMQPrivateEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7ab3341_c9d3_11d1_bb47_0080c7c5a2c0);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Hwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub FireArrivedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pq: ::windows_core::RawPtr, msgcursor: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    FireArrivedEvent: usize,
    #[cfg(feature = "win32-system")]
    pub FireArrivedErrorEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pq: ::windows_core::RawPtr, hrstatus: ::windows_core::HRESULT, msgcursor: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    FireArrivedErrorEvent: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQuery(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQuery {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LookupQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery> for ::windows_core::IUnknown {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery> for super::Com::IDispatch {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQuery {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQuery {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQuery {
    type Vtable = IMSMQQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e072_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupQueue: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQuery2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQuery2 {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LookupQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery2> for ::windows_core::IUnknown {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQuery2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQuery2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery2> for super::Com::IDispatch {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery2> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQuery2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQuery2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQuery2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQuery2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQuery2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQuery2 {
    type Vtable = IMSMQQuery2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b0e_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupQueue: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQuery3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQuery3 {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LookupQueue_v2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LookupQueue)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            ::core::mem::transmute(multicastaddress),
            ::core::mem::transmute(relmulticastaddress),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IMSMQQueueInfos3>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery3> for ::windows_core::IUnknown {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQuery3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQuery3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery3> for super::Com::IDispatch {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery3> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQuery3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQuery3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQuery3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQuery3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQuery3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQuery3 {
    type Vtable = IMSMQQuery3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b19_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupQueue_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupQueue_v2: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupQueue: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQuery4(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQuery4 {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LookupQueue_v2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos4>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LookupQueue)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            ::core::mem::transmute(multicastaddress),
            ::core::mem::transmute(relmulticastaddress),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IMSMQQueueInfos4>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery4> for ::windows_core::IUnknown {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery4> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQuery4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQuery4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQuery4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQuery4> for super::Com::IDispatch {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQuery4> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQuery4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQuery4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQuery4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQuery4 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQuery4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery4").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQuery4 {
    type Vtable = IMSMQQuery4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b24_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupQueue_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupQueue_v2: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LookupQueue: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueue(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueue {
    pub unsafe fn Access(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Access)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ShareMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn QueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Handle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows_core::IntoParam<'a, IMSMQEvent>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableNotification)(::windows_core::Interface::as_raw(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue> for super::Com::IDispatch {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueue {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueue {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueue {
    type Vtable = IMSMQQueue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e076_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: ::windows_core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueue2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueue2 {
    pub unsafe fn Access(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Access)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ShareMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn QueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Handle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows_core::IntoParam<'a, IMSMQEvent2>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableNotification)(::windows_core::Interface::as_raw(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue2> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueue2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueue2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueue2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue2> for super::Com::IDispatch {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueue2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueue2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueue2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueue2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueue2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueue2 {
    type Vtable = IMSMQQueue2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef0574e0_06d8_11d3_b100_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: ::windows_core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueue3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueue3 {
    pub unsafe fn Access(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Access)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ShareMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn QueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Handle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows_core::IntoParam<'a, IMSMQEvent3>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableNotification)(::windows_core::Interface::as_raw(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Handle2(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Handle2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveNextByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveNextByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceivePreviousByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceivePreviousByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveFirstByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveLastByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNextByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNextByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekPreviousByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekPreviousByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekFirstByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekLastByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Purge)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue3> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueue3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueue3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueue3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue3> for super::Com::IDispatch {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueue3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueue3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueue3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueue3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueue3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueue3 {
    type Vtable = IMSMQQueue3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b1b_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: ::windows_core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Handle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Handle2: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueue4(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueue4 {
    pub unsafe fn Access(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Access)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ShareMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn QueueInfo(&self) -> ::windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueueInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Handle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows_core::IntoParam<'a, IMSMQEvent3>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableNotification)(::windows_core::Interface::as_raw(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent_v1)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Receive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Peek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekCurrent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Handle2(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Handle2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveNextByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveNextByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceivePreviousByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceivePreviousByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveFirstByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveLastByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekNextByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekNextByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekPreviousByLookupId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekPreviousByLookupId)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekFirstByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PeekLastByLookupId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Purge)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpen2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ReceiveByLookupIdAllowPeek<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows_core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ReceiveByLookupIdAllowPeek)(::windows_core::Interface::as_raw(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue4> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue4> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueue4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueue4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueue4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueue4> for super::Com::IDispatch {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueue4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueue4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueue4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueue4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueue4 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueue4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue4").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueue4 {
    type Vtable = IMSMQQueue4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b20_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: ::windows_core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Receive: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Peek: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNext: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekCurrent: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Handle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Handle2: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ReceiveByLookupIdAllowPeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ReceiveByLookupIdAllowPeek: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfo(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfo {
    pub unsafe fn QueueGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).QueueGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidservicetype.into_param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPathName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPathName)(::windows_core::Interface::as_raw(self), bstrpathname.into_param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFormatName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormatName)(::windows_core::Interface::as_raw(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTransactional)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Quota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ModifyTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ModifyTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).JournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsWorldReadable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfo {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfo {
    type Vtable = IMSMQQueueInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e07b_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfo2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfo2 {
    pub unsafe fn QueueGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).QueueGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidservicetype.into_param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPathName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPathName)(::windows_core::Interface::as_raw(self), bstrpathname.into_param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFormatName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormatName)(::windows_core::Interface::as_raw(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTransactional)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Quota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ModifyTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ModifyTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).JournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsWorldReadable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PathNameDNS(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathNameDNS)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Security(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Security)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSecurity<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsecurity: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurity)(::windows_core::Interface::as_raw(self), varsecurity.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo2> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo2> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfo2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfo2 {
    type Vtable = IMSMQQueueInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd174a80_89cf_11d2_b0f2_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Security: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSecurity: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfo3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfo3 {
    pub unsafe fn QueueGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).QueueGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidservicetype.into_param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPathName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPathName)(::windows_core::Interface::as_raw(self), bstrpathname.into_param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFormatName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormatName)(::windows_core::Interface::as_raw(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTransactional)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Quota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ModifyTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ModifyTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).JournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsWorldReadable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue3>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PathNameDNS(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathNameDNS)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Security(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Security)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSecurity<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsecurity: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurity)(::windows_core::Interface::as_raw(self), varsecurity.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTransactional2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsWorldReadable2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MulticastAddress(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MulticastAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMulticastAddress<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmulticastaddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMulticastAddress)(::windows_core::Interface::as_raw(self), bstrmulticastaddress.into_param().abi()).ok()
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ADsPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo3> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo3> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfo3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfo3 {
    type Vtable = IMSMQQueueInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b1d_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Security: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT,
    pub MulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfo4(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfo4 {
    pub unsafe fn QueueGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).QueueGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceTypeGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceTypeGuid)(::windows_core::Interface::as_raw(self), bstrguidservicetype.into_param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLabel)(::windows_core::Interface::as_raw(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPathName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPathName)(::windows_core::Interface::as_raw(self), bstrpathname.into_param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFormatName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormatName)(::windows_core::Interface::as_raw(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTransactional)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Journal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Quota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).BasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBasePriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ModifyTime(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ModifyTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).JournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetJournalQuota)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsWorldReadable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue4>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PathNameDNS(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PathNameDNS)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Security(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Security)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSecurity<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varsecurity: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurity)(::windows_core::Interface::as_raw(self), varsecurity.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsTransactional2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsWorldReadable2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MulticastAddress(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MulticastAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMulticastAddress<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmulticastaddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMulticastAddress)(::windows_core::Interface::as_raw(self), bstrmulticastaddress.into_param().abi()).ok()
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ADsPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo4> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo4> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfo4> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfo4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfo4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfo4 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfo4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo4").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfo4 {
    type Vtable = IMSMQQueueInfo4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b21_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Security: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT,
    pub MulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfos(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfos {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Next(&self) -> ::windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfos {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfos {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfos {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfos {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfos {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfos {
    type Vtable = IMSMQQueueInfos_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e07d_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Next: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfos2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfos2 {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Next(&self) -> ::windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos2> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos2> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfos2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfos2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfos2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfos2 {
    type Vtable = IMSMQQueueInfos2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b0f_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Next: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfos3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfos3 {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Next(&self) -> ::windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos3> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos3> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfos3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfos3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfos3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfos3 {
    type Vtable = IMSMQQueueInfos3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b1e_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Next: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueInfos4(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueInfos4 {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Next(&self) -> ::windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos4> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos4> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueInfos4> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueInfos4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueInfos4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueInfos4 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueInfos4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos4").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueInfos4 {
    type Vtable = IMSMQQueueInfos4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b22_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Next: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQQueueManagement(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQQueueManagement {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    pub unsafe fn FormatName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Machine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Machine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MessageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ForeignStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueueType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransactionalStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BytesInQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn JournalMessageCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).JournalMessageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BytesInJournal(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).BytesInJournal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EodGetReceiveInfo(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).EodGetReceiveInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueManagement> for ::windows_core::IUnknown {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueManagement> for ::windows_core::IUnknown {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueManagement> for super::Com::IDispatch {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQManagement> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQManagement> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQManagement> for &'a IMSMQQueueManagement {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQManagement> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQQueueManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQQueueManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQQueueManagement {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQQueueManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueManagement").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQQueueManagement {
    type Vtable = IMSMQQueueManagement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fbe7759_5760_444d_b8a5_5e7ab9a84cce);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub JournalMessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BytesInJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BytesInJournal: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EodGetReceiveInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EodGetReceiveInfo: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQTransaction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQTransaction {
    pub unsafe fn Transaction(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Transaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction> for ::windows_core::IUnknown {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction> for ::windows_core::IUnknown {
    fn from(value: &IMSMQTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQTransaction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransaction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQTransaction {
    type Vtable = IMSMQTransaction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e07f_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Transaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Commit: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Abort: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQTransaction2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQTransaction2 {
    pub unsafe fn Transaction(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Transaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Abort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn InitNew<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, vartransaction: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitNew)(::windows_core::Interface::as_raw(self), vartransaction.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction2> for ::windows_core::IUnknown {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQTransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction2> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction2> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQTransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQTransaction> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQTransaction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQTransaction> for &'a IMSMQTransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQTransaction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQTransaction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQTransaction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQTransaction2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQTransaction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransaction2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQTransaction2 {
    type Vtable = IMSMQTransaction2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ce0c5b0_6e67_11d2_b0e6_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction2_Vtbl {
    pub base__: IMSMQTransaction_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    InitNew: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQTransaction3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQTransaction3 {
    pub unsafe fn Transaction(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Transaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Abort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn InitNew<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, vartransaction: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InitNew)(::windows_core::Interface::as_raw(self), vartransaction.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ITransaction(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ITransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction3> for ::windows_core::IUnknown {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction3> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction3> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQTransaction> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQTransaction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQTransaction> for &'a IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQTransaction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQTransaction2> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQTransaction2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IMSMQTransaction2> for &'a IMSMQTransaction3 {
    fn into_param(self) -> ::windows_core::Param<'a, IMSMQTransaction2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQTransaction3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQTransaction3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQTransaction3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQTransaction3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransaction3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQTransaction3 {
    type Vtable = IMSMQTransaction3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b13_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction3_Vtbl {
    pub base__: IMSMQTransaction2_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ITransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ITransaction: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQTransactionDispenser(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQTransactionDispenser {
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginTransaction(&self) -> ::windows_core::Result<IMSMQTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransactionDispenser> for ::windows_core::IUnknown {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransactionDispenser> for ::windows_core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransactionDispenser> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransactionDispenser> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQTransactionDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQTransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQTransactionDispenser {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQTransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransactionDispenser").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQTransactionDispenser {
    type Vtable = IMSMQTransactionDispenser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e083_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginTransaction: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQTransactionDispenser2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQTransactionDispenser2 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginTransaction(&self) -> ::windows_core::Result<IMSMQTransaction2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction2>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransactionDispenser2> for ::windows_core::IUnknown {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransactionDispenser2> for ::windows_core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQTransactionDispenser2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQTransactionDispenser2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQTransactionDispenser2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQTransactionDispenser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransactionDispenser2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQTransactionDispenser2 {
    type Vtable = IMSMQTransactionDispenser2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b11_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginTransaction: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMSMQTransactionDispenser3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMSMQTransactionDispenser3 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginTransaction(&self) -> ::windows_core::Result<IMSMQTransaction3> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction3>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransactionDispenser3> for ::windows_core::IUnknown {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransactionDispenser3> for ::windows_core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMSMQTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMSMQTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMSMQTransactionDispenser3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMSMQTransactionDispenser3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMSMQTransactionDispenser3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMSMQTransactionDispenser3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransactionDispenser3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMSMQTransactionDispenser3 {
    type Vtable = IMSMQTransactionDispenser3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b15_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginTransaction: usize,
    #[cfg(feature = "win32-system")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Properties: usize,
}
pub const LONG_LIVED: u32 = 4294967294u32;
pub const MACHINE_ACTION_CONNECT: &str = "CONNECT";
pub const MACHINE_ACTION_DISCONNECT: &str = "DISCONNECT";
pub const MACHINE_ACTION_TIDY: &str = "TIDY";
pub const MGMT_QUEUE_CORRECT_TYPE: &str = "YES";
pub const MGMT_QUEUE_FOREIGN_TYPE: &str = "YES";
pub const MGMT_QUEUE_INCORRECT_TYPE: &str = "NO";
pub const MGMT_QUEUE_LOCAL_LOCATION: &str = "LOCAL";
pub const MGMT_QUEUE_NOT_FOREIGN_TYPE: &str = "NO";
pub const MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE: &str = "NO";
pub const MGMT_QUEUE_REMOTE_LOCATION: &str = "REMOTE";
pub const MGMT_QUEUE_STATE_CONNECTED: &str = "CONNECTED";
pub const MGMT_QUEUE_STATE_DISCONNECTED: &str = "DISCONNECTED";
pub const MGMT_QUEUE_STATE_DISCONNECTING: &str = "DISCONNECTING";
pub const MGMT_QUEUE_STATE_LOCAL: &str = "LOCAL CONNECTION";
pub const MGMT_QUEUE_STATE_LOCKED: &str = "LOCKED";
pub const MGMT_QUEUE_STATE_NEED_VALIDATE: &str = "NEED VALIDATION";
pub const MGMT_QUEUE_STATE_NONACTIVE: &str = "INACTIVE";
pub const MGMT_QUEUE_STATE_ONHOLD: &str = "ONHOLD";
pub const MGMT_QUEUE_STATE_WAITING: &str = "WAITING";
pub const MGMT_QUEUE_TRANSACTIONAL_TYPE: &str = "YES";
pub const MGMT_QUEUE_TYPE_CONNECTOR: &str = "CONNECTOR";
pub const MGMT_QUEUE_TYPE_MACHINE: &str = "MACHINE";
pub const MGMT_QUEUE_TYPE_MULTICAST: &str = "MULTICAST";
pub const MGMT_QUEUE_TYPE_PRIVATE: &str = "PRIVATE";
pub const MGMT_QUEUE_TYPE_PUBLIC: &str = "PUBLIC";
pub const MGMT_QUEUE_UNKNOWN_TYPE: &str = "UNKNOWN";
pub const MO_MACHINE_TOKEN: &str = "MACHINE";
pub const MO_QUEUE_TOKEN: &str = "QUEUE";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQACCESS(pub i32);
pub const MQ_RECEIVE_ACCESS: MQACCESS = MQACCESS(1i32);
pub const MQ_SEND_ACCESS: MQACCESS = MQACCESS(2i32);
pub const MQ_PEEK_ACCESS: MQACCESS = MQACCESS(32i32);
pub const MQ_ADMIN_ACCESS: MQACCESS = MQACCESS(128i32);
impl ::core::marker::Copy for MQACCESS {}
impl ::core::clone::Clone for MQACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQACCESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQACCESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQACCESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQAUTHENTICATE(pub i32);
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = MQAUTHENTICATE(0i32);
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = MQAUTHENTICATE(1i32);
impl ::core::marker::Copy for MQAUTHENTICATE {}
impl ::core::clone::Clone for MQAUTHENTICATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQAUTHENTICATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQAUTHENTICATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQAUTHENTICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQAUTHENTICATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQCALG(pub i32);
pub const MQMSG_CALG_MD2: MQCALG = MQCALG(32769i32);
pub const MQMSG_CALG_MD4: MQCALG = MQCALG(32770i32);
pub const MQMSG_CALG_MD5: MQCALG = MQCALG(32771i32);
pub const MQMSG_CALG_SHA: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_SHA1: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_MAC: MQCALG = MQCALG(32773i32);
pub const MQMSG_CALG_RSA_SIGN: MQCALG = MQCALG(9216i32);
pub const MQMSG_CALG_DSS_SIGN: MQCALG = MQCALG(8704i32);
pub const MQMSG_CALG_RSA_KEYX: MQCALG = MQCALG(41984i32);
pub const MQMSG_CALG_DES: MQCALG = MQCALG(26113i32);
pub const MQMSG_CALG_RC2: MQCALG = MQCALG(26114i32);
pub const MQMSG_CALG_RC4: MQCALG = MQCALG(26625i32);
pub const MQMSG_CALG_SEAL: MQCALG = MQCALG(26626i32);
impl ::core::marker::Copy for MQCALG {}
impl ::core::clone::Clone for MQCALG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQCALG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQCALG {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQCALG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQCALG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQCERT_REGISTER(pub i32);
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
impl ::core::marker::Copy for MQCERT_REGISTER {}
impl ::core::clone::Clone for MQCERT_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQCERT_REGISTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQCERT_REGISTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQCERT_REGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQCERT_REGISTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQDEFAULT(pub i32);
pub const DEFAULT_M_PRIORITY: MQDEFAULT = MQDEFAULT(3i32);
pub const DEFAULT_M_DELIVERY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = MQDEFAULT(0i32);
impl ::core::marker::Copy for MQDEFAULT {}
impl ::core::clone::Clone for MQDEFAULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQDEFAULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQDEFAULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQDEFAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQDEFAULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQERROR(pub i32);
pub const MQ_ERROR: MQERROR = MQERROR(-1072824319i32);
pub const MQ_ERROR_PROPERTY: MQERROR = MQERROR(-1072824318i32);
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = MQERROR(-1072824317i32);
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = MQERROR(-1072824316i32);
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = MQERROR(-1072824315i32);
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = MQERROR(-1072824314i32);
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = MQERROR(-1072824313i32);
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = MQERROR(-1072824312i32);
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = MQERROR(-1072824311i32);
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824309i32);
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = MQERROR(-1072824307i32);
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = MQERROR(-1072824304i32);
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = MQERROR(-1072824303i32);
pub const MQ_ERROR_NO_DS: MQERROR = MQERROR(-1072824301i32);
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = MQERROR(-1072824300i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = MQERROR(-1072824296i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = MQERROR(-1072824295i32);
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = MQERROR(-1072824294i32);
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = MQERROR(-1072824293i32);
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = MQERROR(-1072824292i32);
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = MQERROR(-1072824291i32);
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = MQERROR(-1072824290i32);
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824289i32);
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = MQERROR(-1072824288i32);
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = MQERROR(-1072824287i32);
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824286i32);
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = MQERROR(-1072824285i32);
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = MQERROR(-1072824284i32);
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = MQERROR(-1072824283i32);
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = MQERROR(-1072824282i32);
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = MQERROR(-1072824281i32);
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824280i32);
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = MQERROR(-1072824278i32);
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824277i32);
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = MQERROR(-1072824276i32);
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = MQERROR(-1072824275i32);
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = MQERROR(-1072824274i32);
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = MQERROR(-1072824273i32);
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = MQERROR(-1072824272i32);
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = MQERROR(-1072824271i32);
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = MQERROR(-1072824269i32);
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = MQERROR(-1072824267i32);
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = MQERROR(-1072824266i32);
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = MQERROR(-1072824265i32);
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = MQERROR(-1072824264i32);
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = MQERROR(-1072824263i32);
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = MQERROR(-1072824262i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = MQERROR(-1072824261i32);
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = MQERROR(-1072824260i32);
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = MQERROR(-1072824259i32);
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = MQERROR(-1072824258i32);
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = MQERROR(-1072824257i32);
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = MQERROR(-1072824256i32);
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = MQERROR(-1072824255i32);
pub const MQ_ERROR_DS_IS_FULL: MQERROR = MQERROR(-1072824254i32);
pub const MQ_ERROR_DS_ERROR: MQERROR = MQERROR(-1072824253i32);
pub const MQ_ERROR_INVALID_OWNER: MQERROR = MQERROR(-1072824252i32);
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = MQERROR(-1072824251i32);
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824250i32);
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = MQERROR(-1072824248i32);
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = MQERROR(-1072824247i32);
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = MQERROR(-1072824246i32);
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824245i32);
pub const MQ_ERROR_DTC_CONNECT: MQERROR = MQERROR(-1072824244i32);
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = MQERROR(-1072824242i32);
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = MQERROR(-1072824240i32);
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = MQERROR(-1072824239i32);
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = MQERROR(-1072824235i32);
pub const MQ_ERROR_STALE_HANDLE: MQERROR = MQERROR(-1072824234i32);
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = MQERROR(-1072824232i32);
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = MQERROR(-1072824230i32);
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = MQERROR(-1072824229i32);
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = MQERROR(-1072824228i32);
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = MQERROR(-1072824227i32);
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824226i32);
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = MQERROR(-1072824225i32);
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = MQERROR(-1072824224i32);
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824223i32);
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824222i32);
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824221i32);
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = MQERROR(-1072824220i32);
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = MQERROR(-1072824219i32);
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = MQERROR(-1072824218i32);
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = MQERROR(-1072824217i32);
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = MQERROR(-1072824216i32);
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824215i32);
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = MQERROR(-1072824214i32);
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = MQERROR(-1072824213i32);
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = MQERROR(-1072824212i32);
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = MQERROR(-1072824211i32);
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824210i32);
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = MQERROR(-1072824207i32);
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = MQERROR(-1072824206i32);
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = MQERROR(-1072824205i32);
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = MQERROR(-1072824204i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = MQERROR(-1072824203i32);
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = MQERROR(-1072824202i32);
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = MQERROR(-1072824201i32);
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = MQERROR(-1072824200i32);
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = MQERROR(-1072824199i32);
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = MQERROR(-1072824198i32);
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = MQERROR(-1072824197i32);
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = MQERROR(-1072824196i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = MQERROR(-1072824195i32);
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = MQERROR(-1072824194i32);
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = MQERROR(-1072824193i32);
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = MQERROR(-1072824192i32);
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = MQERROR(-1072824191i32);
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = MQERROR(-1072824190i32);
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = MQERROR(-1072824189i32);
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = MQERROR(-1072824188i32);
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = MQERROR(-1072824187i32);
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = MQERROR(-1072824186i32);
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = MQERROR(-1072824185i32);
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = MQERROR(-1072824184i32);
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = MQERROR(-1072824183i32);
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = MQERROR(-1072824182i32);
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = MQERROR(-1072824181i32);
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = MQERROR(-1072824180i32);
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = MQERROR(-1072824179i32);
pub const MQ_ERROR_GC_NEEDED: MQERROR = MQERROR(-1072824178i32);
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = MQERROR(-1072824177i32);
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = MQERROR(-1072824176i32);
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824175i32);
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = MQERROR(-1072824174i32);
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = MQERROR(-1072824173i32);
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = MQERROR(-1072824172i32);
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824171i32);
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824170i32);
impl ::core::marker::Copy for MQERROR {}
impl ::core::clone::Clone for MQERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQJOURNAL(pub i32);
pub const MQ_JOURNAL_NONE: MQJOURNAL = MQJOURNAL(0i32);
pub const MQ_JOURNAL: MQJOURNAL = MQJOURNAL(1i32);
impl ::core::marker::Copy for MQJOURNAL {}
impl ::core::clone::Clone for MQJOURNAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQJOURNAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQJOURNAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQJOURNAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQJOURNAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMAX(pub i32);
pub const MQ_MAX_Q_NAME_LEN: MQMAX = MQMAX(124i32);
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = MQMAX(124i32);
impl ::core::marker::Copy for MQMAX {}
impl ::core::clone::Clone for MQMAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMAX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMAX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(0i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(1i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(2i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(8i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(5i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(12i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(14i32);
impl ::core::marker::Copy for MQMSGACKNOWLEDGEMENT {}
impl ::core::clone::Clone for MQMSGACKNOWLEDGEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGACKNOWLEDGEMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGACKNOWLEDGEMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGACKNOWLEDGEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGACKNOWLEDGEMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGAUTHENTICATION(pub i32);
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(0i32);
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(5i32);
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(9i32);
impl ::core::marker::Copy for MQMSGAUTHENTICATION {}
impl ::core::clone::Clone for MQMSGAUTHENTICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGAUTHENTICATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGAUTHENTICATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGAUTHENTICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGAUTHENTICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGAUTHLEVEL(pub i32);
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(0i32);
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(1i32);
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(8i32);
impl ::core::marker::Copy for MQMSGAUTHLEVEL {}
impl ::core::clone::Clone for MQMSGAUTHLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGAUTHLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGAUTHLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGAUTHLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGAUTHLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGCLASS(pub i32);
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = MQMSGCLASS(0i32);
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = MQMSGCLASS(1i32);
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = MQMSGCLASS(2i32);
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = MQMSGCLASS(16384i32);
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = MQMSGCLASS(32768i32);
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = MQMSGCLASS(32769i32);
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(32770i32);
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = MQMSGCLASS(32771i32);
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = MQMSGCLASS(32772i32);
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = MQMSGCLASS(32773i32);
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = MQMSGCLASS(32774i32);
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = MQMSGCLASS(32775i32);
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = MQMSGCLASS(32776i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = MQMSGCLASS(32777i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = MQMSGCLASS(32778i32);
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = MQMSGCLASS(32779i32);
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = MQMSGCLASS(32780i32);
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = MQMSGCLASS(49152i32);
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = MQMSGCLASS(49153i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(49154i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = MQMSGCLASS(49155i32);
impl ::core::marker::Copy for MQMSGCLASS {}
impl ::core::clone::Clone for MQMSGCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGCLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGCLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGCURSOR(pub i32);
pub const MQMSG_FIRST: MQMSGCURSOR = MQMSGCURSOR(0i32);
pub const MQMSG_CURRENT: MQMSGCURSOR = MQMSGCURSOR(1i32);
pub const MQMSG_NEXT: MQMSGCURSOR = MQMSGCURSOR(2i32);
impl ::core::marker::Copy for MQMSGCURSOR {}
impl ::core::clone::Clone for MQMSGCURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGCURSOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGCURSOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGCURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGCURSOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGDELIVERY(pub i32);
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = MQMSGDELIVERY(0i32);
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = MQMSGDELIVERY(1i32);
impl ::core::marker::Copy for MQMSGDELIVERY {}
impl ::core::clone::Clone for MQMSGDELIVERY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGDELIVERY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGDELIVERY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGDELIVERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGDELIVERY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGIDSIZE(pub i32);
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
impl ::core::marker::Copy for MQMSGIDSIZE {}
impl ::core::clone::Clone for MQMSGIDSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGIDSIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGIDSIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGIDSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGIDSIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGJOURNAL(pub i32);
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = MQMSGJOURNAL(0i32);
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = MQMSGJOURNAL(1i32);
pub const MQMSG_JOURNAL: MQMSGJOURNAL = MQMSGJOURNAL(2i32);
impl ::core::marker::Copy for MQMSGJOURNAL {}
impl ::core::clone::Clone for MQMSGJOURNAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGJOURNAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGJOURNAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGJOURNAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGJOURNAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGMAX(pub i32);
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = MQMSGMAX(249i32);
impl ::core::marker::Copy for MQMSGMAX {}
impl ::core::clone::Clone for MQMSGMAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGMAX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGMAX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGMAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGMAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGPRIVLEVEL(pub i32);
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(0i32);
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(1i32);
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(3i32);
impl ::core::marker::Copy for MQMSGPRIVLEVEL {}
impl ::core::clone::Clone for MQMSGPRIVLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGPRIVLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGPRIVLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGPRIVLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGPRIVLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGSENDERIDTYPE(pub i32);
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(0i32);
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(1i32);
impl ::core::marker::Copy for MQMSGSENDERIDTYPE {}
impl ::core::clone::Clone for MQMSGSENDERIDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGSENDERIDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGSENDERIDTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGSENDERIDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGSENDERIDTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQMSGTRACE(pub i32);
pub const MQMSG_TRACE_NONE: MQMSGTRACE = MQMSGTRACE(0i32);
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = MQMSGTRACE(1i32);
impl ::core::marker::Copy for MQMSGTRACE {}
impl ::core::clone::Clone for MQMSGTRACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGTRACE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQMSGTRACE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGTRACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGTRACE").field(&self.0).finish()
    }
}
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQPRIORITY(pub i32);
pub const MQ_MIN_PRIORITY: MQPRIORITY = MQPRIORITY(0i32);
pub const MQ_MAX_PRIORITY: MQPRIORITY = MQPRIORITY(7i32);
impl ::core::marker::Copy for MQPRIORITY {}
impl ::core::clone::Clone for MQPRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQPRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQPRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQPRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQPRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQPRIVLEVEL(pub i32);
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = MQPRIVLEVEL(0i32);
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = MQPRIVLEVEL(1i32);
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = MQPRIVLEVEL(2i32);
impl ::core::marker::Copy for MQPRIVLEVEL {}
impl ::core::clone::Clone for MQPRIVLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQPRIVLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQPRIVLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQPRIVLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQPRIVLEVEL").field(&self.0).finish()
    }
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: u32 = 262144u32;
pub const MQSEC_DELETE_JOURNAL_MESSAGE: u32 = 8u32;
pub const MQSEC_DELETE_MESSAGE: u32 = 1u32;
pub const MQSEC_DELETE_QUEUE: u32 = 65536u32;
pub const MQSEC_GET_QUEUE_PROPERTIES: u32 = 32u32;
pub const MQSEC_PEEK_MESSAGE: u32 = 2u32;
pub const MQSEC_QUEUE_GENERIC_EXECUTE: u32 = 0u32;
pub const MQSEC_SET_QUEUE_PROPERTIES: u32 = 16u32;
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: u32 = 524288u32;
pub const MQSEC_WRITE_MESSAGE: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQSHARE(pub i32);
pub const MQ_DENY_NONE: MQSHARE = MQSHARE(0i32);
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = MQSHARE(1i32);
impl ::core::marker::Copy for MQSHARE {}
impl ::core::clone::Clone for MQSHARE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQSHARE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQSHARE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQSHARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQSHARE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQTRANSACTION(pub i32);
pub const MQ_NO_TRANSACTION: MQTRANSACTION = MQTRANSACTION(0i32);
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = MQTRANSACTION(1i32);
pub const MQ_XA_TRANSACTION: MQTRANSACTION = MQTRANSACTION(2i32);
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = MQTRANSACTION(3i32);
impl ::core::marker::Copy for MQTRANSACTION {}
impl ::core::clone::Clone for MQTRANSACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQTRANSACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQTRANSACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQTRANSACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQTRANSACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQTRANSACTIONAL(pub i32);
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = MQTRANSACTIONAL(0i32);
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = MQTRANSACTIONAL(1i32);
impl ::core::marker::Copy for MQTRANSACTIONAL {}
impl ::core::clone::Clone for MQTRANSACTIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQTRANSACTIONAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQTRANSACTIONAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQTRANSACTIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQTRANSACTIONAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MQWARNING(pub i32);
pub const MQ_INFORMATION_PROPERTY: MQWARNING = MQWARNING(1074659329i32);
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = MQWARNING(1074659330i32);
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = MQWARNING(1074659331i32);
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = MQWARNING(1074659332i32);
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = MQWARNING(1074659333i32);
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = MQWARNING(1074659334i32);
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = MQWARNING(1074659337i32);
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = MQWARNING(1074659338i32);
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = MQWARNING(1074659339i32);
impl ::core::marker::Copy for MQWARNING {}
impl ::core::clone::Clone for MQWARNING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQWARNING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MQWARNING {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQWARNING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQWARNING").field(&self.0).finish()
    }
}
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-1072824164i32);
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1072824165i32);
pub const MQ_ERROR_RESOLVE_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1072824167i32);
pub const MQ_ERROR_TOO_MANY_PROPERTIES: ::windows_core::HRESULT = ::windows_core::HRESULT(-1072824166i32);
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
pub const MQ_MOVE_ACCESS: u32 = 4u32;
pub const MQ_OK: ::windows_core::HRESULT = ::windows_core::HRESULT(0i32);
pub const MSMQApplication: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e086_dccd_11d0_aa4b_0060970debae);
pub const MSMQCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf72b9031_2f0c_43e8_924e_e6052cdc493f);
pub const MSMQCoordinatedTransactionDispenser: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e082_dccd_11d0_aa4b_0060970debae);
pub const MSMQDestination: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba96b18_2168_11d3_898c_00e02c074f6b);
pub const MSMQEvent: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e07a_dccd_11d0_aa4b_0060970debae);
pub const MSMQManagement: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39ce96fe_f4c5_4484_a143_4c2d5d324229);
pub const MSMQMessage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e075_dccd_11d0_aa4b_0060970debae);
pub const MSMQOutgoingQueueManagement: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0188401c_247a_4fed_99c6_bf14119d7055);
pub const MSMQQuery: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e073_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e079_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e07c_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfos: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e07e_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueManagement: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33b6d07e_f27d_42fa_b2d7_bf82e11e9374);
pub const MSMQTransaction: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e080_dccd_11d0_aa4b_0060970debae);
pub const MSMQTransactionDispenser: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e084_dccd_11d0_aa4b_0060970debae);
pub const MSMQ_CONNECTED: &str = "CONNECTED";
pub const MSMQ_DISCONNECTED: &str = "DISCONNECTED";
pub const PREQ: u32 = 4u32;
pub const PRGE: u32 = 3u32;
pub const PRGT: u32 = 2u32;
pub const PRLE: u32 = 1u32;
pub const PRLT: u32 = 0u32;
pub const PRNE: u32 = 5u32;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
pub const PROPID_M_BASE: u32 = 0u32;
pub const PROPID_M_BODY: u32 = 9u32;
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
pub const PROPID_M_CLASS: u32 = 1u32;
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
pub const PROPID_M_DELIVERY: u32 = 5u32;
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
pub const PROPID_M_EXTENSION: u32 = 35u32;
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
pub const PROPID_M_HASH_ALG: u32 = 26u32;
pub const PROPID_M_JOURNAL: u32 = 7u32;
pub const PROPID_M_LABEL: u32 = 11u32;
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
pub const PROPID_M_LOOKUPID: u32 = 60u32;
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
pub const PROPID_M_MSGID: u32 = 2u32;
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
pub const PROPID_M_PRIORITY: u32 = 4u32;
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
pub const PROPID_M_PROV_NAME: u32 = 48u32;
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
pub const PROPID_M_SENDERID: u32 = 20u32;
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
pub const PROPID_M_SENTTIME: u32 = 31u32;
pub const PROPID_M_SIGNATURE: u32 = 45u32;
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
pub const PROPID_M_TRACE: u32 = 41u32;
pub const PROPID_M_VERSION: u32 = 19u32;
pub const PROPID_M_XACTID: u32 = 52u32;
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
pub const PROPID_PC_BASE: u32 = 5800u32;
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
pub const PROPID_PC_VERSION: u32 = 5801u32;
pub const PROPID_QM_BASE: u32 = 200u32;
pub const PROPID_QM_CONNECTION: u32 = 204u32;
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
pub const PROPID_QM_PATHNAME: u32 = 203u32;
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
pub const PROPID_QM_SITE_ID: u32 = 201u32;
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
pub const PROPID_Q_BASE: u32 = 100u32;
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
pub const PROPID_Q_INSTANCE: u32 = 101u32;
pub const PROPID_Q_JOURNAL: u32 = 104u32;
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
pub const PROPID_Q_LABEL: u32 = 108u32;
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
pub const PROPID_Q_PATHNAME: u32 = 103u32;
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
pub const PROPID_Q_QUOTA: u32 = 105u32;
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
pub const PROPID_Q_TYPE: u32 = 102u32;
pub const QUERY_SORTASCEND: u32 = 0u32;
pub const QUERY_SORTDESCEND: u32 = 1u32;
pub const QUEUE_ACTION_EOD_RESEND: &str = "EOD_RESEND";
pub const QUEUE_ACTION_PAUSE: &str = "PAUSE";
pub const QUEUE_ACTION_RESUME: &str = "RESUME";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QUEUE_STATE(pub i32);
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = QUEUE_STATE(0i32);
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = QUEUE_STATE(1i32);
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = QUEUE_STATE(2i32);
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = QUEUE_STATE(3i32);
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = QUEUE_STATE(4i32);
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = QUEUE_STATE(5i32);
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = QUEUE_STATE(6i32);
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = QUEUE_STATE(7i32);
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = QUEUE_STATE(8i32);
impl ::core::marker::Copy for QUEUE_STATE {}
impl ::core::clone::Clone for QUEUE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for QUEUE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUEUE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QUEUE_TYPE(pub i32);
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = QUEUE_TYPE(0i32);
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = QUEUE_TYPE(1i32);
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = QUEUE_TYPE(2i32);
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = QUEUE_TYPE(3i32);
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = QUEUE_TYPE(4i32);
impl ::core::marker::Copy for QUEUE_TYPE {}
impl ::core::clone::Clone for QUEUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for QUEUE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUEUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RELOPS(pub i32);
pub const REL_NOP: RELOPS = RELOPS(0i32);
pub const REL_EQ: RELOPS = RELOPS(1i32);
pub const REL_NEQ: RELOPS = RELOPS(2i32);
pub const REL_LT: RELOPS = RELOPS(3i32);
pub const REL_GT: RELOPS = RELOPS(4i32);
pub const REL_LE: RELOPS = RELOPS(5i32);
pub const REL_GE: RELOPS = RELOPS(6i32);
impl ::core::marker::Copy for RELOPS {}
impl ::core::clone::Clone for RELOPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RELOPS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RELOPS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RELOPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RELOPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACT_STATUS(pub i32);
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = XACT_STATUS(0i32);
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = XACT_STATUS(1i32);
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = XACT_STATUS(2i32);
impl ::core::marker::Copy for XACT_STATUS {}
impl ::core::clone::Clone for XACT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACT_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _DMSMQEventEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _DMSMQEventEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_DMSMQEventEvents> for ::windows_core::IUnknown {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_DMSMQEventEvents> for ::windows_core::IUnknown {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _DMSMQEventEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _DMSMQEventEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_DMSMQEventEvents> for super::Com::IDispatch {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_DMSMQEventEvents> for super::Com::IDispatch {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for _DMSMQEventEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a _DMSMQEventEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _DMSMQEventEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _DMSMQEventEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _DMSMQEventEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _DMSMQEventEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMSMQEventEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _DMSMQEventEvents {
    type Vtable = _DMSMQEventEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7d6e078_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _DMSMQEventEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
