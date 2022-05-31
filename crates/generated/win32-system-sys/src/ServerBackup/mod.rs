pub type IWsbApplicationAsync = *mut ::core::ffi::c_void;
pub type IWsbApplicationBackupSupport = *mut ::core::ffi::c_void;
pub type IWsbApplicationRestoreSupport = *mut ::core::ffi::c_void;
pub const WSBAPP_ASYNC_IN_PROGRESS: ::windows_core_sys::HRESULT = 7995396i32;
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WSB_OB_REGISTRATION_INFO {
    pub m_wszResourceDLL: ::windows_core_sys::PWSTR,
    pub m_guidSnapinId: ::windows_core_sys::GUID,
    pub m_dwProviderName: u32,
    pub m_dwProviderIcon: u32,
    pub m_bSupportsRemoting: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSB_OB_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSB_OB_STATUS_ENTRY {
    pub m_dwIcon: u32,
    pub m_dwStatusEntryName: u32,
    pub m_dwStatusEntryValue: u32,
    pub m_cValueTypePair: u32,
    pub m_rgValueTypePair: *mut WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR,
}
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WSB_OB_STATUS_ENTRY_PAIR_TYPE = i32;
pub const WSB_OB_ET_UNDEFINED: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 0i32;
pub const WSB_OB_ET_STRING: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 1i32;
pub const WSB_OB_ET_NUMBER: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 2i32;
pub const WSB_OB_ET_DATETIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 3i32;
pub const WSB_OB_ET_TIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 4i32;
pub const WSB_OB_ET_SIZE: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 5i32;
pub const WSB_OB_ET_MAX: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 6i32;
#[repr(C)]
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    pub m_wszObStatusEntryPairValue: ::windows_core_sys::PWSTR,
    pub m_ObStatusEntryPairType: WSB_OB_STATUS_ENTRY_PAIR_TYPE,
}
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSB_OB_STATUS_INFO {
    pub m_guidSnapinId: ::windows_core_sys::GUID,
    pub m_cStatusEntry: u32,
    pub m_rgStatusEntry: *mut WSB_OB_STATUS_ENTRY,
}
impl ::core::marker::Copy for WSB_OB_STATUS_INFO {}
impl ::core::clone::Clone for WSB_OB_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
