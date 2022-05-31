
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::windows_core_sys::HSTRING, policy: ::windows_core_sys::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilentlyForUser(user: ::windows_core_sys::IInspectable, privilegeid: u32, scope: ::windows_core_sys::HSTRING, policy: ::windows_core_sys::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::windows_core_sys::HSTRING, policy: ::windows_core_sys::HSTRING, friendlymessage: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn CheckGamingPrivilegeWithUIForUser(user: ::windows_core_sys::IInspectable, privilegeid: u32, scope: ::windows_core_sys::HSTRING, policy: ::windows_core_sys::HSTRING, friendlymessage: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn ReleaseExclusiveCpuSets() -> ::windows_core_sys::HRESULT;
    pub fn ShowChangeFriendRelationshipUI(targetuserxuid: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowChangeFriendRelationshipUIForUser(user: ::windows_core_sys::IInspectable, targetuserxuid: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowCustomizeUserProfileUIForUser(user: ::windows_core_sys::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowFindFriendsUIForUser(user: ::windows_core_sys::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowGameInfoUIForUser(user: ::windows_core_sys::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowGameInviteUI(serviceconfigurationid: ::windows_core_sys::HSTRING, sessiontemplatename: ::windows_core_sys::HSTRING, sessionid: ::windows_core_sys::HSTRING, invitationdisplaytext: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowGameInviteUIForUser(user: ::windows_core_sys::IInspectable, serviceconfigurationid: ::windows_core_sys::HSTRING, sessiontemplatename: ::windows_core_sys::HSTRING, sessionid: ::windows_core_sys::HSTRING, invitationdisplaytext: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowGameInviteUIWithContext(serviceconfigurationid: ::windows_core_sys::HSTRING, sessiontemplatename: ::windows_core_sys::HSTRING, sessionid: ::windows_core_sys::HSTRING, invitationdisplaytext: ::windows_core_sys::HSTRING, customactivationcontext: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowGameInviteUIWithContextForUser(user: ::windows_core_sys::IInspectable, serviceconfigurationid: ::windows_core_sys::HSTRING, sessiontemplatename: ::windows_core_sys::HSTRING, sessionid: ::windows_core_sys::HSTRING, invitationdisplaytext: ::windows_core_sys::HSTRING, customactivationcontext: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowPlayerPickerUI(promptdisplaytext: ::windows_core_sys::HSTRING, xuids: *const ::windows_core_sys::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_core_sys::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowPlayerPickerUIForUser(user: ::windows_core_sys::IInspectable, promptdisplaytext: ::windows_core_sys::HSTRING, xuids: *const ::windows_core_sys::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_core_sys::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowProfileCardUI(targetuserxuid: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowProfileCardUIForUser(user: ::windows_core_sys::IInspectable, targetuserxuid: ::windows_core_sys::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowTitleAchievementsUIForUser(user: ::windows_core_sys::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn ShowUserSettingsUIForUser(user: ::windows_core_sys::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
}
pub type GAMESTATS_OPEN_RESULT = i32;
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = 0i32;
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = 1i32;
pub type GAMESTATS_OPEN_TYPE = i32;
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = 0i32;
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = 1i32;
pub type GAME_INSTALL_SCOPE = i32;
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = 1i32;
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = 2i32;
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = 3i32;
pub type GAMING_DEVICE_DEVICE_ID = i32;
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = 0i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = 1988865574i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = 712204761i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = 1523980231i32;
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = 284675555i32;
#[repr(C)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl ::core::marker::Copy for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::clone::Clone for GAMING_DEVICE_MODEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GAMING_DEVICE_VENDOR_ID = i32;
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = 0i32;
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = -1024700366i32;
pub const GameExplorer: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2589895056, data2: 12340, data3: 19823, data4: [145, 40, 1, 243, 198, 16, 34, 188] };
pub const GameStatistics: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3687340588, data2: 49372, data3: 18785, data4: [182, 226, 210, 139, 98, 193, 26, 212] };
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_core_sys::HRESULT, context: *const ::core::ffi::c_void)>;
pub const ID_GDF_THUMBNAIL_STR: &str = "__GDF_THUMBNAIL";
pub const ID_GDF_XML_STR: &str = "__GDF_XML";
pub type IGameExplorer = *mut ::core::ffi::c_void;
pub type IGameExplorer2 = *mut ::core::ffi::c_void;
pub type IGameStatistics = *mut ::core::ffi::c_void;
pub type IGameStatisticsMgr = *mut ::core::ffi::c_void;
pub type IXblIdpAuthManager = *mut ::core::ffi::c_void;
pub type IXblIdpAuthTokenResult = *mut ::core::ffi::c_void;
pub type IXblIdpAuthTokenResult2 = *mut ::core::ffi::c_void;
pub type KnownGamingPrivileges = i32;
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = 190i32;
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = 197i32;
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = 198i32;
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = 199i32;
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = 203i32;
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = 205i32;
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = 206i32;
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = 207i32;
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = 208i32;
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = 209i32;
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = 211i32;
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = 214i32;
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = 219i32;
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = 220i32;
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = 224i32;
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = 235i32;
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = 245i32;
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = 247i32;
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = 249i32;
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = 252i32;
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = 254i32;
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = 255i32;
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_core_sys::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_core_sys::HSTRING, selectedxuidscount: usize)>;
pub type XBL_IDP_AUTH_TOKEN_STATUS = i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = 0i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = 1i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = 2i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = 3i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = 4i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = 5i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = 6i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = 7i32;
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = -1i32;
pub const XblIdpAuthManager: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3458421579, data2: 22232, data3: 18808, data4: [134, 162, 126, 229, 112, 100, 4, 104] };
pub const XblIdpAuthTokenResult: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2672374849, data2: 29770, data3: 16652, data4: [174, 43, 154, 34, 247, 199, 115, 31] };
