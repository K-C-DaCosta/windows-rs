#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
#[inline]
pub unsafe fn McastApiCleanup() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastApiCleanup();
    }
    McastApiCleanup()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
#[inline]
pub unsafe fn McastApiStartup(version: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastApiStartup(version: *mut u32) -> u32;
    }
    McastApiStartup(::core::mem::transmute(version))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn McastEnumerateScopes<'a, P0>(addrfamily: u16, requery: P0, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastEnumerateScopes(addrfamily: u16, requery: super::super::Foundation::BOOL, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32;
    }
    McastEnumerateScopes(addrfamily, requery.into(), ::core::mem::transmute(pscopelist), ::core::mem::transmute(pscopelen), ::core::mem::transmute(pscopecount))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
#[inline]
pub unsafe fn McastGenUID(prequestid: *mut MCAST_CLIENT_UID) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastGenUID(prequestid: *mut MCAST_CLIENT_UID) -> u32;
    }
    McastGenUID(::core::mem::transmute(prequestid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
#[inline]
pub unsafe fn McastReleaseAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastReleaseAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32;
    }
    McastReleaseAddress(addrfamily, ::core::mem::transmute(prequestid), ::core::mem::transmute(preleaserequest))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
#[inline]
pub unsafe fn McastRenewAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastRenewAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
    }
    McastRenewAddress(addrfamily, ::core::mem::transmute(prequestid), ::core::mem::transmute(prenewrequest), ::core::mem::transmute(prenewresponse))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
#[inline]
pub unsafe fn McastRequestAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn McastRequestAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
    }
    McastRequestAddress(addrfamily, ::core::mem::transmute(prequestid), ::core::mem::transmute(pscopectx), ::core::mem::transmute(paddrrequest), ::core::mem::transmute(paddrresponse))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub const MCAST_API_CURRENT_VERSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub const MCAST_API_VERSION_0: i32 = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub const MCAST_API_VERSION_1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub const MCAST_CLIENT_ID_LEN: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub union IPNG_ADDRESS {
    pub IpAddrV4: u32,
    pub IpAddrV6: [u8; 16],
}
impl ::core::marker::Copy for IPNG_ADDRESS {}
impl ::core::clone::Clone for IPNG_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPNG_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPNG_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPNG_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPNG_ADDRESS {}
impl ::core::default::Default for IPNG_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub struct MCAST_CLIENT_UID {
    pub ClientUID: *mut u8,
    pub ClientUIDLength: u32,
}
impl ::core::marker::Copy for MCAST_CLIENT_UID {}
impl ::core::clone::Clone for MCAST_CLIENT_UID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCAST_CLIENT_UID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCAST_CLIENT_UID").field("ClientUID", &self.ClientUID).field("ClientUIDLength", &self.ClientUIDLength).finish()
    }
}
unsafe impl ::windows::core::Abi for MCAST_CLIENT_UID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCAST_CLIENT_UID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCAST_CLIENT_UID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCAST_CLIENT_UID {}
impl ::core::default::Default for MCAST_CLIENT_UID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub struct MCAST_LEASE_REQUEST {
    pub LeaseStartTime: i32,
    pub MaxLeaseStartTime: i32,
    pub LeaseDuration: u32,
    pub MinLeaseDuration: u32,
    pub ServerAddress: IPNG_ADDRESS,
    pub MinAddrCount: u16,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl ::core::marker::Copy for MCAST_LEASE_REQUEST {}
impl ::core::clone::Clone for MCAST_LEASE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCAST_LEASE_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCAST_LEASE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCAST_LEASE_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCAST_LEASE_REQUEST {}
impl ::core::default::Default for MCAST_LEASE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub struct MCAST_LEASE_RESPONSE {
    pub LeaseStartTime: i32,
    pub LeaseEndTime: i32,
    pub ServerAddress: IPNG_ADDRESS,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl ::core::marker::Copy for MCAST_LEASE_RESPONSE {}
impl ::core::clone::Clone for MCAST_LEASE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCAST_LEASE_RESPONSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCAST_LEASE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCAST_LEASE_RESPONSE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCAST_LEASE_RESPONSE {}
impl ::core::default::Default for MCAST_LEASE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`*"]
pub struct MCAST_SCOPE_CTX {
    pub ScopeID: IPNG_ADDRESS,
    pub Interface: IPNG_ADDRESS,
    pub ServerID: IPNG_ADDRESS,
}
impl ::core::marker::Copy for MCAST_SCOPE_CTX {}
impl ::core::clone::Clone for MCAST_SCOPE_CTX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCAST_SCOPE_CTX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCAST_SCOPE_CTX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCAST_SCOPE_CTX>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCAST_SCOPE_CTX {}
impl ::core::default::Default for MCAST_SCOPE_CTX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Multicast\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCAST_SCOPE_ENTRY {
    pub ScopeCtx: MCAST_SCOPE_CTX,
    pub LastAddr: IPNG_ADDRESS,
    pub TTL: u32,
    pub ScopeDesc: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCAST_SCOPE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCAST_SCOPE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCAST_SCOPE_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCAST_SCOPE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCAST_SCOPE_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCAST_SCOPE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCAST_SCOPE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
