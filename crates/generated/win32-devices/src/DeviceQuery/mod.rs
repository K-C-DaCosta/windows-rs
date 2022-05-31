#[repr(C)]
#[cfg(feature = "win32-devices")]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::Properties::DEVPROPERTY,
}
#[cfg(feature = "win32-devices")]
impl ::core::marker::Copy for DEVPROP_FILTER_EXPRESSION {}
#[cfg(feature = "win32-devices")]
impl ::core::clone::Clone for DEVPROP_FILTER_EXPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::fmt::Debug for DEVPROP_FILTER_EXPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVPROP_FILTER_EXPRESSION").field("Operator", &self.Operator).field("Property", &self.Property).finish()
    }
}
#[cfg(feature = "win32-devices")]
unsafe impl ::windows_core::Abi for DEVPROP_FILTER_EXPRESSION {
    type Abi = Self;
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::PartialEq for DEVPROP_FILTER_EXPRESSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVPROP_FILTER_EXPRESSION>()) == 0 }
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::Eq for DEVPROP_FILTER_EXPRESSION {}
#[cfg(feature = "win32-devices")]
impl ::core::default::Default for DEVPROP_FILTER_EXPRESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVPROP_OPERATOR(pub u32);
pub const DEVPROP_OPERATOR_MODIFIER_NOT: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65536u32);
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131072u32);
pub const DEVPROP_OPERATOR_NONE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(0u32);
pub const DEVPROP_OPERATOR_EXISTS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(1u32);
pub const DEVPROP_OPERATOR_NOT_EXISTS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65537u32);
pub const DEVPROP_OPERATOR_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(2u32);
pub const DEVPROP_OPERATOR_NOT_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65538u32);
pub const DEVPROP_OPERATOR_GREATER_THAN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(3u32);
pub const DEVPROP_OPERATOR_LESS_THAN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4u32);
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(5u32);
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(6u32);
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131074u32);
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(196610u32);
pub const DEVPROP_OPERATOR_BITWISE_AND: DEVPROP_OPERATOR = DEVPROP_OPERATOR(7u32);
pub const DEVPROP_OPERATOR_BITWISE_OR: DEVPROP_OPERATOR = DEVPROP_OPERATOR(8u32);
pub const DEVPROP_OPERATOR_BEGINS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(9u32);
pub const DEVPROP_OPERATOR_ENDS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(10u32);
pub const DEVPROP_OPERATOR_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(11u32);
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131081u32);
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131082u32);
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131083u32);
pub const DEVPROP_OPERATOR_LIST_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4096u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(8192u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(12288u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(16384u32);
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(135168u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(139264u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(143360u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(147456u32);
pub const DEVPROP_OPERATOR_AND_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(1048576u32);
pub const DEVPROP_OPERATOR_AND_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(2097152u32);
pub const DEVPROP_OPERATOR_OR_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(3145728u32);
pub const DEVPROP_OPERATOR_OR_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4194304u32);
pub const DEVPROP_OPERATOR_NOT_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(5242880u32);
pub const DEVPROP_OPERATOR_NOT_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(6291456u32);
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(268435456u32);
pub const DEVPROP_OPERATOR_MASK_EVAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4095u32);
pub const DEVPROP_OPERATOR_MASK_LIST: DEVPROP_OPERATOR = DEVPROP_OPERATOR(61440u32);
pub const DEVPROP_OPERATOR_MASK_MODIFIER: DEVPROP_OPERATOR = DEVPROP_OPERATOR(983040u32);
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4027580415u32);
pub const DEVPROP_OPERATOR_MASK_LOGICAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(267386880u32);
pub const DEVPROP_OPERATOR_MASK_ARRAY: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4026531840u32);
impl ::core::marker::Copy for DEVPROP_OPERATOR {}
impl ::core::clone::Clone for DEVPROP_OPERATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVPROP_OPERATOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEVPROP_OPERATOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVPROP_OPERATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVPROP_OPERATOR").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DEVPROP_OPERATOR {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DEVPROP_OPERATOR {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DEVPROP_OPERATOR {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DEVPROP_OPERATOR {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DEVPROP_OPERATOR {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[cfg(feature = "win32-devices")]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: ::windows_core::PCWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *const super::Properties::DEVPROPERTY,
}
#[cfg(feature = "win32-devices")]
impl ::core::marker::Copy for DEV_OBJECT {}
#[cfg(feature = "win32-devices")]
impl ::core::clone::Clone for DEV_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::fmt::Debug for DEV_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_OBJECT").field("ObjectType", &self.ObjectType).field("pszObjectId", &self.pszObjectId).field("cPropertyCount", &self.cPropertyCount).field("pProperties", &self.pProperties).finish()
    }
}
#[cfg(feature = "win32-devices")]
unsafe impl ::windows_core::Abi for DEV_OBJECT {
    type Abi = Self;
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::PartialEq for DEV_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_OBJECT>()) == 0 }
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::Eq for DEV_OBJECT {}
#[cfg(feature = "win32-devices")]
impl ::core::default::Default for DEV_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEV_OBJECT_TYPE(pub i32);
pub const DevObjectTypeUnknown: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(0i32);
pub const DevObjectTypeDeviceInterface: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(1i32);
pub const DevObjectTypeDeviceContainer: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(2i32);
pub const DevObjectTypeDevice: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(3i32);
pub const DevObjectTypeDeviceInterfaceClass: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(4i32);
pub const DevObjectTypeAEP: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(5i32);
pub const DevObjectTypeAEPContainer: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(6i32);
pub const DevObjectTypeDeviceInstallerClass: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(7i32);
pub const DevObjectTypeDeviceInterfaceDisplay: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(8i32);
pub const DevObjectTypeDeviceContainerDisplay: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(9i32);
pub const DevObjectTypeAEPService: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(10i32);
pub const DevObjectTypeDevicePanel: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(11i32);
impl ::core::marker::Copy for DEV_OBJECT_TYPE {}
impl ::core::clone::Clone for DEV_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEV_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEV_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEV_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEV_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEV_QUERY_FLAGS(pub i32);
pub const DevQueryFlagNone: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(0i32);
pub const DevQueryFlagUpdateResults: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(1i32);
pub const DevQueryFlagAllProperties: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(2i32);
pub const DevQueryFlagLocalize: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(4i32);
pub const DevQueryFlagAsyncClose: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(8i32);
impl ::core::marker::Copy for DEV_QUERY_FLAGS {}
impl ::core::clone::Clone for DEV_QUERY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEV_QUERY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEV_QUERY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEV_QUERY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEV_QUERY_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "win32-devices")]
pub struct DEV_QUERY_PARAMETER {
    pub Key: super::Properties::DEVPROPKEY,
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "win32-devices")]
impl ::core::marker::Copy for DEV_QUERY_PARAMETER {}
#[cfg(feature = "win32-devices")]
impl ::core::clone::Clone for DEV_QUERY_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::fmt::Debug for DEV_QUERY_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_QUERY_PARAMETER").field("Key", &self.Key).field("Type", &self.Type).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "win32-devices")]
unsafe impl ::windows_core::Abi for DEV_QUERY_PARAMETER {
    type Abi = Self;
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::PartialEq for DEV_QUERY_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_QUERY_PARAMETER>()) == 0 }
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::Eq for DEV_QUERY_PARAMETER {}
#[cfg(feature = "win32-devices")]
impl ::core::default::Default for DEV_QUERY_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEV_QUERY_RESULT_ACTION(pub i32);
pub const DevQueryResultStateChange: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(0i32);
pub const DevQueryResultAdd: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(1i32);
pub const DevQueryResultUpdate: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(2i32);
pub const DevQueryResultRemove: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(3i32);
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION {}
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEV_QUERY_RESULT_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEV_QUERY_RESULT_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEV_QUERY_RESULT_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEV_QUERY_RESULT_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "win32-devices")]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(feature = "win32-devices")]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(feature = "win32-devices")]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-devices")]
unsafe impl ::windows_core::Abi for DEV_QUERY_RESULT_ACTION_DATA {
    type Abi = Self;
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::PartialEq for DEV_QUERY_RESULT_ACTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_QUERY_RESULT_ACTION_DATA>()) == 0 }
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::Eq for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(feature = "win32-devices")]
impl ::core::default::Default for DEV_QUERY_RESULT_ACTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "win32-devices")]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(feature = "win32-devices")]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(feature = "win32-devices")]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-devices")]
unsafe impl ::windows_core::Abi for DEV_QUERY_RESULT_ACTION_DATA_0 {
    type Abi = Self;
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::PartialEq for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_QUERY_RESULT_ACTION_DATA_0>()) == 0 }
    }
}
#[cfg(feature = "win32-devices")]
impl ::core::cmp::Eq for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(feature = "win32-devices")]
impl ::core::default::Default for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEV_QUERY_STATE(pub i32);
pub const DevQueryStateInitialized: DEV_QUERY_STATE = DEV_QUERY_STATE(0i32);
pub const DevQueryStateEnumCompleted: DEV_QUERY_STATE = DEV_QUERY_STATE(1i32);
pub const DevQueryStateAborted: DEV_QUERY_STATE = DEV_QUERY_STATE(2i32);
pub const DevQueryStateClosed: DEV_QUERY_STATE = DEV_QUERY_STATE(3i32);
impl ::core::marker::Copy for DEV_QUERY_STATE {}
impl ::core::clone::Clone for DEV_QUERY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEV_QUERY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEV_QUERY_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEV_QUERY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEV_QUERY_STATE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__);
        }
        DevCloseObjectQuery(::core::mem::transmute(hdevquery))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut HDEVQUERY__>::zeroed();
        DevCreateObjectQuery(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), pfilter.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pextendedparameters: &[DEV_QUERY_PARAMETER], pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut HDEVQUERY__>::zeroed();
        DevCreateObjectQueryEx(
            ::core::mem::transmute(objecttype),
            ::core::mem::transmute(queryflags),
            prequestedproperties.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)),
            pfilter.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)),
            pextendedparameters.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pextendedparameters)),
            ::core::mem::transmute(pcallback),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromId<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromId(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut HDEVQUERY__>::zeroed();
        DevCreateObjectQueryFromId(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), pfilter.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIdEx<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pextendedparameters: &[DEV_QUERY_PARAMETER], pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromIdEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut HDEVQUERY__>::zeroed();
        DevCreateObjectQueryFromIdEx(
            ::core::mem::transmute(objecttype),
            pszobjectid.into_param().abi(),
            ::core::mem::transmute(queryflags),
            prequestedproperties.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)),
            pfilter.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)),
            pextendedparameters.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pextendedparameters)),
            ::core::mem::transmute(pcallback),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIds<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(objecttype: DEV_OBJECT_TYPE, pszzobjectids: Param1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromIds(objecttype: DEV_OBJECT_TYPE, pszzobjectids: ::windows_core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut HDEVQUERY__>::zeroed();
        DevCreateObjectQueryFromIds(::core::mem::transmute(objecttype), pszzobjectids.into_param().abi(), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), pfilter.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIdsEx<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(objecttype: DEV_OBJECT_TYPE, pszzobjectids: Param1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pextendedparameters: &[DEV_QUERY_PARAMETER], pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromIdsEx(objecttype: DEV_OBJECT_TYPE, pszzobjectids: ::windows_core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: ::windows_core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut HDEVQUERY__>::zeroed();
        DevCreateObjectQueryFromIdsEx(
            ::core::mem::transmute(objecttype),
            pszzobjectids.into_param().abi(),
            ::core::mem::transmute(queryflags),
            prequestedproperties.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)),
            pfilter.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)),
            pextendedparameters.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pextendedparameters)),
            ::core::mem::transmute(pcallback),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevFindProperty<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: Param2, pproperties: &[super::Properties::DEVPROPERTY]) -> *mut super::Properties::DEVPROPERTY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevFindProperty(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: ::windows_core::PCWSTR, cproperties: u32, pproperties: *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY;
        }
        ::core::mem::transmute(DevFindProperty(::core::mem::transmute(pkey), ::core::mem::transmute(store), pszlocalename.into_param().abi(), pproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pproperties))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevFreeObjectProperties(pproperties: &[super::Properties::DEVPROPERTY]) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevFreeObjectProperties(cpropertycount: u32, pproperties: *const super::Properties::DEVPROPERTY);
        }
        DevFreeObjectProperties(pproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pproperties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevFreeObjects(pobjects: &[DEV_OBJECT]) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT);
        }
        DevFreeObjects(pobjects.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pobjects)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevGetObjectProperties<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjectProperties(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_core::HRESULT;
        }
        DevGetObjectProperties(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), ::core::mem::transmute(pcpropertycount), ::core::mem::transmute(ppproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevGetObjectPropertiesEx<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pextendedparameters: &[DEV_QUERY_PARAMETER], pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjectPropertiesEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_core::HRESULT;
        }
        DevGetObjectPropertiesEx(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), pextendedparameters.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pextendedparameters)), ::core::mem::transmute(pcpropertycount), ::core::mem::transmute(ppproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_core::HRESULT;
        }
        DevGetObjects(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), pfilter.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)), ::core::mem::transmute(pcobjectcount), ::core::mem::transmute(ppobjects)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-devices")]
#[inline]
pub unsafe fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pfilter: &[DEVPROP_FILTER_EXPRESSION], pextendedparameters: &[DEV_QUERY_PARAMETER], pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_core::HRESULT;
        }
        DevGetObjectsEx(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), prequestedproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(prequestedproperties)), pfilter.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pfilter)), pextendedparameters.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pextendedparameters)), ::core::mem::transmute(pcobjectcount), ::core::mem::transmute(ppobjects)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct HDEVQUERY__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDEVQUERY__ {}
impl ::core::clone::Clone for HDEVQUERY__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HDEVQUERY__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDEVQUERY__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows_core::Abi for HDEVQUERY__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HDEVQUERY__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HDEVQUERY__>()) == 0 }
    }
}
impl ::core::cmp::Eq for HDEVQUERY__ {}
impl ::core::default::Default for HDEVQUERY__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "win32-devices")]
pub type PDEV_QUERY_RESULT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevquery: *const HDEVQUERY__, pcontext: *const ::core::ffi::c_void, pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA)>;
