pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPrintDocumentPackageStatusEvent(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPrintDocumentPackageStatusEvent {
    pub unsafe fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PackageStatusUpdated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(packagestatus)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPrintDocumentPackageStatusEvent> for ::windows_core::IUnknown {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPrintDocumentPackageStatusEvent> for ::windows_core::IUnknown {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPrintDocumentPackageStatusEvent> for ::win32_system::Com::IDispatch {
    fn from(value: IPrintDocumentPackageStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPrintDocumentPackageStatusEvent> for ::win32_system::Com::IDispatch {
    fn from(value: &IPrintDocumentPackageStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IPrintDocumentPackageStatusEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPrintDocumentPackageStatusEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPrintDocumentPackageStatusEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPrintDocumentPackageStatusEvent {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPrintDocumentPackageStatusEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageStatusEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPrintDocumentPackageStatusEvent {
    type Vtable = IPrintDocumentPackageStatusEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed90c8ad_5c34_4d05_a1ec_0e8a9b3ad7af);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageStatusEvent_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub PackageStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrintDocumentPackageTarget(::windows_core::IUnknown);
impl IPrintDocumentPackageTarget {
    pub unsafe fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPackageTargetTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(targetcount), ::core::mem::transmute(targettypes)).ok()
    }
    pub unsafe fn GetPackageTarget<T: ::windows_core::Interface>(&self, guidtargettype: *const ::windows_core::GUID) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetPackageTarget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtargettype), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPrintDocumentPackageTarget> for ::windows_core::IUnknown {
    fn from(value: IPrintDocumentPackageTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintDocumentPackageTarget> for ::windows_core::IUnknown {
    fn from(value: &IPrintDocumentPackageTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintDocumentPackageTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintDocumentPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintDocumentPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentPackageTarget {}
impl ::core::fmt::Debug for IPrintDocumentPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintDocumentPackageTarget {
    type Vtable = IPrintDocumentPackageTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b8efec4_3019_4c27_964e_367202156906);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTarget_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPackageTargetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetPackageTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrintDocumentPackageTargetFactory(::windows_core::IUnknown);
impl IPrintDocumentPackageTargetFactory {
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateDocumentPackageTargetForPrintJob<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param3: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, printername: Param0, jobname: Param1, joboutputstream: Param2, jobprintticketstream: Param3) -> ::windows_core::Result<IPrintDocumentPackageTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDocumentPackageTargetForPrintJob)(::windows_core::Interface::as_raw(self), printername.into_param().abi(), jobname.into_param().abi(), joboutputstream.into_param().abi(), jobprintticketstream.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPrintDocumentPackageTarget>(result__)
    }
}
impl ::core::convert::From<IPrintDocumentPackageTargetFactory> for ::windows_core::IUnknown {
    fn from(value: IPrintDocumentPackageTargetFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintDocumentPackageTargetFactory> for ::windows_core::IUnknown {
    fn from(value: &IPrintDocumentPackageTargetFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintDocumentPackageTargetFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintDocumentPackageTargetFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintDocumentPackageTargetFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentPackageTargetFactory {}
impl ::core::fmt::Debug for IPrintDocumentPackageTargetFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentPackageTargetFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintDocumentPackageTargetFactory {
    type Vtable = IPrintDocumentPackageTargetFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2959bf7_b31b_4a3d_9600_712eb1335ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPackageTargetFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub CreateDocumentPackageTargetForPrintJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printername: ::windows_core::PCWSTR, jobname: ::windows_core::PCWSTR, joboutputstream: ::windows_core::RawPtr, jobprintticketstream: ::windows_core::RawPtr, docpackagetarget: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateDocumentPackageTargetForPrintJob: usize,
}
#[repr(transparent)]
pub struct IXpsPrintJob(::windows_core::IUnknown);
impl IXpsPrintJob {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetJobStatus(&self) -> ::windows_core::Result<XPS_JOB_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<XPS_JOB_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).GetJobStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XPS_JOB_STATUS>(result__)
    }
}
impl ::core::convert::From<IXpsPrintJob> for ::windows_core::IUnknown {
    fn from(value: IXpsPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXpsPrintJob> for ::windows_core::IUnknown {
    fn from(value: &IXpsPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXpsPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXpsPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXpsPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXpsPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsPrintJob {}
impl ::core::fmt::Debug for IXpsPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXpsPrintJob {
    type Vtable = IXpsPrintJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ab89b06_8194_425f_ab3b_d7a96e350161);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJob_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IXpsPrintJobStream(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IXpsPrintJobStream {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)))
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)))
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IXpsPrintJobStream> for ::windows_core::IUnknown {
    fn from(value: IXpsPrintJobStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IXpsPrintJobStream> for ::windows_core::IUnknown {
    fn from(value: &IXpsPrintJobStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXpsPrintJobStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IXpsPrintJobStream> for ::win32_system::Com::ISequentialStream {
    fn from(value: IXpsPrintJobStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IXpsPrintJobStream> for ::win32_system::Com::ISequentialStream {
    fn from(value: &IXpsPrintJobStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::ISequentialStream> for IXpsPrintJobStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::ISequentialStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::ISequentialStream> for &'a IXpsPrintJobStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::ISequentialStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IXpsPrintJobStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IXpsPrintJobStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IXpsPrintJobStream {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IXpsPrintJobStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsPrintJobStream").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IXpsPrintJobStream {
    type Vtable = IXpsPrintJobStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a77dc5f_45d6_4dff_9307_d8cb846347ca);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPrintJobStream_Vtbl {
    pub base__: ::win32_system::Com::ISequentialStream_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintDocumentPackageCompletion(pub i32);
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(0i32);
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(1i32);
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(2i32);
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = PrintDocumentPackageCompletion(3i32);
impl ::core::marker::Copy for PrintDocumentPackageCompletion {}
impl ::core::clone::Clone for PrintDocumentPackageCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintDocumentPackageCompletion {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintDocumentPackageCompletion {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintDocumentPackageCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDocumentPackageCompletion").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for PrintDocumentPackageStatus {}
impl ::core::clone::Clone for PrintDocumentPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PrintDocumentPackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrintDocumentPackageStatus").field("JobId", &self.JobId).field("CurrentDocument", &self.CurrentDocument).field("CurrentPage", &self.CurrentPage).field("CurrentPageTotal", &self.CurrentPageTotal).field("Completion", &self.Completion).field("PackageStatus", &self.PackageStatus).finish()
    }
}
unsafe impl ::windows_core::Abi for PrintDocumentPackageStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PrintDocumentPackageStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PrintDocumentPackageStatus>()) == 0 }
    }
}
impl ::core::cmp::Eq for PrintDocumentPackageStatus {}
impl ::core::default::Default for PrintDocumentPackageStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PrintDocumentPackageTarget: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
pub const PrintDocumentPackageTargetFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn StartXpsPrintJob<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(printername: Param0, jobname: Param1, outputfilename: Param2, progressevent: Param3, completionevent: Param4, printablepageson: &[u8], xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>, documentstream: *mut ::core::option::Option<IXpsPrintJobStream>, printticketstream: *mut ::core::option::Option<IXpsPrintJobStream>) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartXpsPrintJob(printername: ::windows_core::PCWSTR, jobname: ::windows_core::PCWSTR, outputfilename: ::windows_core::PCWSTR, progressevent: ::win32_foundation::HANDLE, completionevent: ::win32_foundation::HANDLE, printablepageson: *const u8, printablepagesoncount: u32, xpsprintjob: *mut ::windows_core::RawPtr, documentstream: *mut ::windows_core::RawPtr, printticketstream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        StartXpsPrintJob(printername.into_param().abi(), jobname.into_param().abi(), outputfilename.into_param().abi(), progressevent.into_param().abi(), completionevent.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(printablepageson)), printablepageson.len() as _, ::core::mem::transmute(xpsprintjob), ::core::mem::transmute(documentstream), ::core::mem::transmute(printticketstream)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StartXpsPrintJob1<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(printername: Param0, jobname: Param1, outputfilename: Param2, progressevent: Param3, completionevent: Param4, xpsprintjob: *mut ::core::option::Option<IXpsPrintJob>, printcontentreceiver: *mut ::core::option::Option<super::IXpsOMPackageTarget>) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartXpsPrintJob1(printername: ::windows_core::PCWSTR, jobname: ::windows_core::PCWSTR, outputfilename: ::windows_core::PCWSTR, progressevent: ::win32_foundation::HANDLE, completionevent: ::win32_foundation::HANDLE, xpsprintjob: *mut ::windows_core::RawPtr, printcontentreceiver: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        StartXpsPrintJob1(printername.into_param().abi(), jobname.into_param().abi(), outputfilename.into_param().abi(), progressevent.into_param().abi(), completionevent.into_param().abi(), ::core::mem::transmute(xpsprintjob), ::core::mem::transmute(printcontentreceiver)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XPS_JOB_COMPLETION(pub i32);
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(0i32);
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(1i32);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(2i32);
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = XPS_JOB_COMPLETION(3i32);
impl ::core::marker::Copy for XPS_JOB_COMPLETION {}
impl ::core::clone::Clone for XPS_JOB_COMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_JOB_COMPLETION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XPS_JOB_COMPLETION {
    type Abi = Self;
}
impl ::core::fmt::Debug for XPS_JOB_COMPLETION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_JOB_COMPLETION").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for XPS_JOB_STATUS {}
impl ::core::clone::Clone for XPS_JOB_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPS_JOB_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_JOB_STATUS").field("jobId", &self.jobId).field("currentDocument", &self.currentDocument).field("currentPage", &self.currentPage).field("currentPageTotal", &self.currentPageTotal).field("completion", &self.completion).field("jobStatus", &self.jobStatus).finish()
    }
}
unsafe impl ::windows_core::Abi for XPS_JOB_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XPS_JOB_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XPS_JOB_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for XPS_JOB_STATUS {}
impl ::core::default::Default for XPS_JOB_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
