pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
pub type INetworkTransportSettings = *mut ::core::ffi::c_void;
pub type INotificationTransportSync = *mut ::core::ffi::c_void;
pub type IRTCBuddy = *mut ::core::ffi::c_void;
pub type IRTCBuddy2 = *mut ::core::ffi::c_void;
pub type IRTCBuddyEvent = *mut ::core::ffi::c_void;
pub type IRTCBuddyEvent2 = *mut ::core::ffi::c_void;
pub type IRTCBuddyGroup = *mut ::core::ffi::c_void;
pub type IRTCBuddyGroupEvent = *mut ::core::ffi::c_void;
pub type IRTCClient = *mut ::core::ffi::c_void;
pub type IRTCClient2 = *mut ::core::ffi::c_void;
pub type IRTCClientEvent = *mut ::core::ffi::c_void;
pub type IRTCClientPortManagement = *mut ::core::ffi::c_void;
pub type IRTCClientPresence = *mut ::core::ffi::c_void;
pub type IRTCClientPresence2 = *mut ::core::ffi::c_void;
pub type IRTCClientProvisioning = *mut ::core::ffi::c_void;
pub type IRTCClientProvisioning2 = *mut ::core::ffi::c_void;
pub type IRTCCollection = *mut ::core::ffi::c_void;
pub type IRTCDispatchEventNotification = *mut ::core::ffi::c_void;
pub type IRTCEnumBuddies = *mut ::core::ffi::c_void;
pub type IRTCEnumGroups = *mut ::core::ffi::c_void;
pub type IRTCEnumParticipants = *mut ::core::ffi::c_void;
pub type IRTCEnumPresenceDevices = *mut ::core::ffi::c_void;
pub type IRTCEnumProfiles = *mut ::core::ffi::c_void;
pub type IRTCEnumUserSearchResults = *mut ::core::ffi::c_void;
pub type IRTCEnumWatchers = *mut ::core::ffi::c_void;
pub type IRTCEventNotification = *mut ::core::ffi::c_void;
pub type IRTCInfoEvent = *mut ::core::ffi::c_void;
pub type IRTCIntensityEvent = *mut ::core::ffi::c_void;
pub type IRTCMediaEvent = *mut ::core::ffi::c_void;
pub type IRTCMediaRequestEvent = *mut ::core::ffi::c_void;
pub type IRTCMessagingEvent = *mut ::core::ffi::c_void;
pub type IRTCParticipant = *mut ::core::ffi::c_void;
pub type IRTCParticipantStateChangeEvent = *mut ::core::ffi::c_void;
pub type IRTCPortManager = *mut ::core::ffi::c_void;
pub type IRTCPresenceContact = *mut ::core::ffi::c_void;
pub type IRTCPresenceDataEvent = *mut ::core::ffi::c_void;
pub type IRTCPresenceDevice = *mut ::core::ffi::c_void;
pub type IRTCPresencePropertyEvent = *mut ::core::ffi::c_void;
pub type IRTCPresenceStatusEvent = *mut ::core::ffi::c_void;
pub type IRTCProfile = *mut ::core::ffi::c_void;
pub type IRTCProfile2 = *mut ::core::ffi::c_void;
pub type IRTCProfileEvent = *mut ::core::ffi::c_void;
pub type IRTCProfileEvent2 = *mut ::core::ffi::c_void;
pub type IRTCReInviteEvent = *mut ::core::ffi::c_void;
pub type IRTCRegistrationStateChangeEvent = *mut ::core::ffi::c_void;
pub type IRTCRoamingEvent = *mut ::core::ffi::c_void;
pub type IRTCSession = *mut ::core::ffi::c_void;
pub type IRTCSession2 = *mut ::core::ffi::c_void;
pub type IRTCSessionCallControl = *mut ::core::ffi::c_void;
pub type IRTCSessionDescriptionManager = *mut ::core::ffi::c_void;
pub type IRTCSessionOperationCompleteEvent = *mut ::core::ffi::c_void;
pub type IRTCSessionOperationCompleteEvent2 = *mut ::core::ffi::c_void;
pub type IRTCSessionPortManagement = *mut ::core::ffi::c_void;
pub type IRTCSessionReferStatusEvent = *mut ::core::ffi::c_void;
pub type IRTCSessionReferredEvent = *mut ::core::ffi::c_void;
pub type IRTCSessionStateChangeEvent = *mut ::core::ffi::c_void;
pub type IRTCSessionStateChangeEvent2 = *mut ::core::ffi::c_void;
pub type IRTCUserSearch = *mut ::core::ffi::c_void;
pub type IRTCUserSearchQuery = *mut ::core::ffi::c_void;
pub type IRTCUserSearchResult = *mut ::core::ffi::c_void;
pub type IRTCUserSearchResultsEvent = *mut ::core::ffi::c_void;
pub type IRTCWatcher = *mut ::core::ffi::c_void;
pub type IRTCWatcher2 = *mut ::core::ffi::c_void;
pub type IRTCWatcherEvent = *mut ::core::ffi::c_void;
pub type IRTCWatcherEvent2 = *mut ::core::ffi::c_void;
pub type ITransportSettingsInternal = *mut ::core::ffi::c_void;
pub const RTCAU_BASIC: u32 = 1u32;
pub const RTCAU_DIGEST: u32 = 2u32;
pub const RTCAU_KERBEROS: u32 = 8u32;
pub const RTCAU_NTLM: u32 = 4u32;
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
pub const RTCClient: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2051205673, data2: 41655, data3: 16580, data4: [176, 145, 246, 240, 36, 170, 137, 190] };
pub const RTCEF_ALL: u32 = 33554431u32;
pub const RTCEF_BUDDY: u32 = 256u32;
pub const RTCEF_BUDDY2: u32 = 262144u32;
pub const RTCEF_CLIENT: u32 = 1u32;
pub const RTCEF_GROUP: u32 = 8192u32;
pub const RTCEF_INFO: u32 = 4096u32;
pub const RTCEF_INTENSITY: u32 = 64u32;
pub const RTCEF_MEDIA: u32 = 32u32;
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
pub const RTCEF_MESSAGING: u32 = 128u32;
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
pub const RTCEF_PROFILE: u32 = 1024u32;
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
pub const RTCEF_REINVITE: u32 = 4194304u32;
pub const RTCEF_ROAMING: u32 = 65536u32;
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
pub const RTCEF_USERSEARCH: u32 = 2048u32;
pub const RTCEF_WATCHER: u32 = 512u32;
pub const RTCEF_WATCHER2: u32 = 524288u32;
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
pub const RTCSI_APPLICATION: u32 = 32u32;
pub const RTCSI_IM: u32 = 8u32;
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
pub const RTCSI_PC_TO_PC: u32 = 1u32;
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
pub const RTCTR_TCP: u32 = 2u32;
pub const RTCTR_TLS: u32 = 4u32;
pub const RTCTR_UDP: u32 = 1u32;
pub type RTC_ACE_SCOPE = i32;
pub const RTCAS_SCOPE_USER: RTC_ACE_SCOPE = 0i32;
pub const RTCAS_SCOPE_DOMAIN: RTC_ACE_SCOPE = 1i32;
pub const RTCAS_SCOPE_ALL: RTC_ACE_SCOPE = 2i32;
pub type RTC_ANSWER_MODE = i32;
pub const RTCAM_OFFER_SESSION_EVENT: RTC_ANSWER_MODE = 0i32;
pub const RTCAM_AUTOMATICALLY_ACCEPT: RTC_ANSWER_MODE = 1i32;
pub const RTCAM_AUTOMATICALLY_REJECT: RTC_ANSWER_MODE = 2i32;
pub const RTCAM_NOT_SUPPORTED: RTC_ANSWER_MODE = 3i32;
pub type RTC_AUDIO_DEVICE = i32;
pub const RTCAD_SPEAKER: RTC_AUDIO_DEVICE = 0i32;
pub const RTCAD_MICROPHONE: RTC_AUDIO_DEVICE = 1i32;
pub type RTC_BUDDY_EVENT_TYPE = i32;
pub const RTCBET_BUDDY_ADD: RTC_BUDDY_EVENT_TYPE = 0i32;
pub const RTCBET_BUDDY_REMOVE: RTC_BUDDY_EVENT_TYPE = 1i32;
pub const RTCBET_BUDDY_UPDATE: RTC_BUDDY_EVENT_TYPE = 2i32;
pub const RTCBET_BUDDY_STATE_CHANGE: RTC_BUDDY_EVENT_TYPE = 3i32;
pub const RTCBET_BUDDY_ROAMED: RTC_BUDDY_EVENT_TYPE = 4i32;
pub const RTCBET_BUDDY_SUBSCRIBED: RTC_BUDDY_EVENT_TYPE = 5i32;
pub type RTC_BUDDY_SUBSCRIPTION_TYPE = i32;
pub const RTCBT_SUBSCRIBED: RTC_BUDDY_SUBSCRIPTION_TYPE = 0i32;
pub const RTCBT_ALWAYS_OFFLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = 1i32;
pub const RTCBT_ALWAYS_ONLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = 2i32;
pub const RTCBT_POLL: RTC_BUDDY_SUBSCRIPTION_TYPE = 3i32;
pub type RTC_CLIENT_EVENT_TYPE = i32;
pub const RTCCET_VOLUME_CHANGE: RTC_CLIENT_EVENT_TYPE = 0i32;
pub const RTCCET_DEVICE_CHANGE: RTC_CLIENT_EVENT_TYPE = 1i32;
pub const RTCCET_NETWORK_QUALITY_CHANGE: RTC_CLIENT_EVENT_TYPE = 2i32;
pub const RTCCET_ASYNC_CLEANUP_DONE: RTC_CLIENT_EVENT_TYPE = 3i32;
pub type RTC_DTMF = i32;
pub const RTC_DTMF_0: RTC_DTMF = 0i32;
pub const RTC_DTMF_1: RTC_DTMF = 1i32;
pub const RTC_DTMF_2: RTC_DTMF = 2i32;
pub const RTC_DTMF_3: RTC_DTMF = 3i32;
pub const RTC_DTMF_4: RTC_DTMF = 4i32;
pub const RTC_DTMF_5: RTC_DTMF = 5i32;
pub const RTC_DTMF_6: RTC_DTMF = 6i32;
pub const RTC_DTMF_7: RTC_DTMF = 7i32;
pub const RTC_DTMF_8: RTC_DTMF = 8i32;
pub const RTC_DTMF_9: RTC_DTMF = 9i32;
pub const RTC_DTMF_STAR: RTC_DTMF = 10i32;
pub const RTC_DTMF_POUND: RTC_DTMF = 11i32;
pub const RTC_DTMF_A: RTC_DTMF = 12i32;
pub const RTC_DTMF_B: RTC_DTMF = 13i32;
pub const RTC_DTMF_C: RTC_DTMF = 14i32;
pub const RTC_DTMF_D: RTC_DTMF = 15i32;
pub const RTC_DTMF_FLASH: RTC_DTMF = 16i32;
pub type RTC_EVENT = i32;
pub const RTCE_CLIENT: RTC_EVENT = 0i32;
pub const RTCE_REGISTRATION_STATE_CHANGE: RTC_EVENT = 1i32;
pub const RTCE_SESSION_STATE_CHANGE: RTC_EVENT = 2i32;
pub const RTCE_SESSION_OPERATION_COMPLETE: RTC_EVENT = 3i32;
pub const RTCE_PARTICIPANT_STATE_CHANGE: RTC_EVENT = 4i32;
pub const RTCE_MEDIA: RTC_EVENT = 5i32;
pub const RTCE_INTENSITY: RTC_EVENT = 6i32;
pub const RTCE_MESSAGING: RTC_EVENT = 7i32;
pub const RTCE_BUDDY: RTC_EVENT = 8i32;
pub const RTCE_WATCHER: RTC_EVENT = 9i32;
pub const RTCE_PROFILE: RTC_EVENT = 10i32;
pub const RTCE_USERSEARCH: RTC_EVENT = 11i32;
pub const RTCE_INFO: RTC_EVENT = 12i32;
pub const RTCE_GROUP: RTC_EVENT = 13i32;
pub const RTCE_MEDIA_REQUEST: RTC_EVENT = 14i32;
pub const RTCE_ROAMING: RTC_EVENT = 15i32;
pub const RTCE_PRESENCE_PROPERTY: RTC_EVENT = 16i32;
pub const RTCE_PRESENCE_DATA: RTC_EVENT = 17i32;
pub const RTCE_PRESENCE_STATUS: RTC_EVENT = 18i32;
pub const RTCE_SESSION_REFER_STATUS: RTC_EVENT = 19i32;
pub const RTCE_SESSION_REFERRED: RTC_EVENT = 20i32;
pub const RTCE_REINVITE: RTC_EVENT = 21i32;
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: ::windows_core_sys::HRESULT = -2131885961i32;
pub const RTC_E_BASIC_AUTH_SET_TLS: ::windows_core_sys::HRESULT = -2131886017i32;
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: ::windows_core_sys::HRESULT = -2131886042i32;
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: ::windows_core_sys::HRESULT = -2131886041i32;
pub const RTC_E_CLIENT_NOT_INITIALIZED: ::windows_core_sys::HRESULT = -2131886043i32;
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: ::windows_core_sys::HRESULT = -2131886061i32;
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: ::windows_core_sys::HRESULT = -2131886059i32;
pub const RTC_E_DUPLICATE_BUDDY: ::windows_core_sys::HRESULT = -2131886006i32;
pub const RTC_E_DUPLICATE_GROUP: ::windows_core_sys::HRESULT = -2131885998i32;
pub const RTC_E_DUPLICATE_REALM: ::windows_core_sys::HRESULT = -2131886013i32;
pub const RTC_E_DUPLICATE_WATCHER: ::windows_core_sys::HRESULT = -2131886005i32;
pub const RTC_E_INVALID_ACL_LIST: ::windows_core_sys::HRESULT = -2131886000i32;
pub const RTC_E_INVALID_ADDRESS_LOCAL: ::windows_core_sys::HRESULT = -2131886060i32;
pub const RTC_E_INVALID_BUDDY_LIST: ::windows_core_sys::HRESULT = -2131886001i32;
pub const RTC_E_INVALID_LISTEN_SOCKET: ::windows_core_sys::HRESULT = -2131885957i32;
pub const RTC_E_INVALID_OBJECT_STATE: ::windows_core_sys::HRESULT = -2131885983i32;
pub const RTC_E_INVALID_PORTRANGE: ::windows_core_sys::HRESULT = -2131885988i32;
pub const RTC_E_INVALID_PREFERENCE_LIST: ::windows_core_sys::HRESULT = -2131885991i32;
pub const RTC_E_INVALID_PROFILE: ::windows_core_sys::HRESULT = -2131886034i32;
pub const RTC_E_INVALID_PROXY_ADDRESS: ::windows_core_sys::HRESULT = -2131886058i32;
pub const RTC_E_INVALID_REGISTRATION_STATE: ::windows_core_sys::HRESULT = -2131885971i32;
pub const RTC_E_INVALID_SESSION_STATE: ::windows_core_sys::HRESULT = -2131886038i32;
pub const RTC_E_INVALID_SESSION_TYPE: ::windows_core_sys::HRESULT = -2131886039i32;
pub const RTC_E_INVALID_SIP_URL: ::windows_core_sys::HRESULT = -2131886062i32;
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: ::windows_core_sys::HRESULT = -2131885958i32;
pub const RTC_E_LOCAL_PHONE_NEEDED: ::windows_core_sys::HRESULT = -2131886036i32;
pub const RTC_E_MALFORMED_XML: ::windows_core_sys::HRESULT = -2131886004i32;
pub const RTC_E_MAX_PENDING_OPERATIONS: ::windows_core_sys::HRESULT = -2131885990i32;
pub const RTC_E_MAX_REDIRECTS: ::windows_core_sys::HRESULT = -2131885960i32;
pub const RTC_E_MEDIA_AEC: ::windows_core_sys::HRESULT = -2131886044i32;
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: ::windows_core_sys::HRESULT = -2131886047i32;
pub const RTC_E_MEDIA_CONTROLLER_STATE: ::windows_core_sys::HRESULT = -2131886049i32;
pub const RTC_E_MEDIA_DISABLED: ::windows_core_sys::HRESULT = -2131885970i32;
pub const RTC_E_MEDIA_ENABLED: ::windows_core_sys::HRESULT = -2131885969i32;
pub const RTC_E_MEDIA_NEED_TERMINAL: ::windows_core_sys::HRESULT = -2131886048i32;
pub const RTC_E_MEDIA_SESSION_IN_HOLD: ::windows_core_sys::HRESULT = -2131885962i32;
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: ::windows_core_sys::HRESULT = -2131885963i32;
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: ::windows_core_sys::HRESULT = -2131886046i32;
pub const RTC_E_NOT_ALLOWED: ::windows_core_sys::HRESULT = -2131885950i32;
pub const RTC_E_NOT_EXIST: ::windows_core_sys::HRESULT = -2131885992i32;
pub const RTC_E_NOT_PRESENCE_PROFILE: ::windows_core_sys::HRESULT = -2131885974i32;
pub const RTC_E_NO_BUDDY: ::windows_core_sys::HRESULT = -2131885996i32;
pub const RTC_E_NO_DEVICE: ::windows_core_sys::HRESULT = -2131886035i32;
pub const RTC_E_NO_GROUP: ::windows_core_sys::HRESULT = -2131885999i32;
pub const RTC_E_NO_PROFILE: ::windows_core_sys::HRESULT = -2131886037i32;
pub const RTC_E_NO_REALM: ::windows_core_sys::HRESULT = -2131885994i32;
pub const RTC_E_NO_TRANSPORT: ::windows_core_sys::HRESULT = -2131885993i32;
pub const RTC_E_NO_WATCHER: ::windows_core_sys::HRESULT = -2131885995i32;
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: ::windows_core_sys::HRESULT = -2131886018i32;
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: ::windows_core_sys::HRESULT = -2131755001i32;
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: ::windows_core_sys::HRESULT = -2131754997i32;
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: ::windows_core_sys::HRESULT = -2131755003i32;
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: ::windows_core_sys::HRESULT = -2131754998i32;
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: ::windows_core_sys::HRESULT = -2131755002i32;
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: ::windows_core_sys::HRESULT = -2131755000i32;
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: ::windows_core_sys::HRESULT = -2131754999i32;
pub const RTC_E_PLATFORM_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2131885952i32;
pub const RTC_E_POLICY_NOT_ALLOW: ::windows_core_sys::HRESULT = -2131886012i32;
pub const RTC_E_PORT_MANAGER_ALREADY_SET: ::windows_core_sys::HRESULT = -2131885956i32;
pub const RTC_E_PORT_MAPPING_FAILED: ::windows_core_sys::HRESULT = -2131886010i32;
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: ::windows_core_sys::HRESULT = -2131886011i32;
pub const RTC_E_PRESENCE_ENABLED: ::windows_core_sys::HRESULT = -2131885982i32;
pub const RTC_E_PRESENCE_NOT_ENABLED: ::windows_core_sys::HRESULT = -2131886040i32;
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: ::windows_core_sys::HRESULT = -2131886024i32;
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: ::windows_core_sys::HRESULT = -2131886025i32;
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: ::windows_core_sys::HRESULT = -2131886023i32;
pub const RTC_E_PROFILE_INVALID_SESSION: ::windows_core_sys::HRESULT = -2131886021i32;
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: ::windows_core_sys::HRESULT = -2131886020i32;
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: ::windows_core_sys::HRESULT = -2131886019i32;
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: ::windows_core_sys::HRESULT = -2131886022i32;
pub const RTC_E_PROFILE_NO_KEY: ::windows_core_sys::HRESULT = -2131886032i32;
pub const RTC_E_PROFILE_NO_NAME: ::windows_core_sys::HRESULT = -2131886031i32;
pub const RTC_E_PROFILE_NO_PROVISION: ::windows_core_sys::HRESULT = -2131886033i32;
pub const RTC_E_PROFILE_NO_SERVER: ::windows_core_sys::HRESULT = -2131886028i32;
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: ::windows_core_sys::HRESULT = -2131886027i32;
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: ::windows_core_sys::HRESULT = -2131886026i32;
pub const RTC_E_PROFILE_NO_USER: ::windows_core_sys::HRESULT = -2131886030i32;
pub const RTC_E_PROFILE_NO_USER_URI: ::windows_core_sys::HRESULT = -2131886029i32;
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: ::windows_core_sys::HRESULT = -2131886014i32;
pub const RTC_E_REDIRECT_PROCESSING_FAILED: ::windows_core_sys::HRESULT = -2131885959i32;
pub const RTC_E_REFER_NOT_ACCEPTED: ::windows_core_sys::HRESULT = -2131885968i32;
pub const RTC_E_REFER_NOT_ALLOWED: ::windows_core_sys::HRESULT = -2131885967i32;
pub const RTC_E_REFER_NOT_EXIST: ::windows_core_sys::HRESULT = -2131885966i32;
pub const RTC_E_REGISTRATION_DEACTIVATED: ::windows_core_sys::HRESULT = -2131885949i32;
pub const RTC_E_REGISTRATION_REJECTED: ::windows_core_sys::HRESULT = -2131885948i32;
pub const RTC_E_REGISTRATION_UNREGISTERED: ::windows_core_sys::HRESULT = -2131885947i32;
pub const RTC_E_ROAMING_ENABLED: ::windows_core_sys::HRESULT = -2131885981i32;
pub const RTC_E_ROAMING_FAILED: ::windows_core_sys::HRESULT = -2131886002i32;
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: ::windows_core_sys::HRESULT = -2131886003i32;
pub const RTC_E_SDP_CONNECTION_ADDR: ::windows_core_sys::HRESULT = -2131886070i32;
pub const RTC_E_SDP_FAILED_TO_BUILD: ::windows_core_sys::HRESULT = -2131886067i32;
pub const RTC_E_SDP_MULTICAST: ::windows_core_sys::HRESULT = -2131886071i32;
pub const RTC_E_SDP_NOT_PRESENT: ::windows_core_sys::HRESULT = -2131886074i32;
pub const RTC_E_SDP_NO_MEDIA: ::windows_core_sys::HRESULT = -2131886069i32;
pub const RTC_E_SDP_PARSE_FAILED: ::windows_core_sys::HRESULT = -2131886073i32;
pub const RTC_E_SDP_UPDATE_FAILED: ::windows_core_sys::HRESULT = -2131886072i32;
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: ::windows_core_sys::HRESULT = -2131885955i32;
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: ::windows_core_sys::HRESULT = -2131886009i32;
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: ::windows_core_sys::HRESULT = -2131886008i32;
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: ::windows_core_sys::HRESULT = -2131886007i32;
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: ::windows_core_sys::HRESULT = -2131885986i32;
pub const RTC_E_SIP_AUTH_FAILED: ::windows_core_sys::HRESULT = -2131886063i32;
pub const RTC_E_SIP_AUTH_HEADER_SENT: ::windows_core_sys::HRESULT = -2131886065i32;
pub const RTC_E_SIP_AUTH_TIME_SKEW: ::windows_core_sys::HRESULT = -2131885972i32;
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2131886064i32;
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: ::windows_core_sys::HRESULT = -2131885987i32;
pub const RTC_E_SIP_CALL_DISCONNECTED: ::windows_core_sys::HRESULT = -2131886055i32;
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: ::windows_core_sys::HRESULT = -2131886080i32;
pub const RTC_E_SIP_DNS_FAIL: ::windows_core_sys::HRESULT = -2131885978i32;
pub const RTC_E_SIP_HEADER_NOT_PRESENT: ::windows_core_sys::HRESULT = -2131886075i32;
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: ::windows_core_sys::HRESULT = -2131886016i32;
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: ::windows_core_sys::HRESULT = -2131885965i32;
pub const RTC_E_SIP_INVALID_CERTIFICATE: ::windows_core_sys::HRESULT = -2131885979i32;
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: ::windows_core_sys::HRESULT = -2131885973i32;
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: ::windows_core_sys::HRESULT = -2131886066i32;
pub const RTC_E_SIP_NEED_MORE_DATA: ::windows_core_sys::HRESULT = -2131886056i32;
pub const RTC_E_SIP_NO_STREAM: ::windows_core_sys::HRESULT = -2131886077i32;
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: ::windows_core_sys::HRESULT = -2131885984i32;
pub const RTC_E_SIP_PARSE_FAILED: ::windows_core_sys::HRESULT = -2131886076i32;
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: ::windows_core_sys::HRESULT = -2131885985i32;
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: ::windows_core_sys::HRESULT = -2131885951i32;
pub const RTC_E_SIP_REFER_OPERATION_PENDING: ::windows_core_sys::HRESULT = -2131885953i32;
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: ::windows_core_sys::HRESULT = -2131886054i32;
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: ::windows_core_sys::HRESULT = -2131886051i32;
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: ::windows_core_sys::HRESULT = -2131886052i32;
pub const RTC_E_SIP_STACK_SHUTDOWN: ::windows_core_sys::HRESULT = -2131886050i32;
pub const RTC_E_SIP_STREAM_NOT_PRESENT: ::windows_core_sys::HRESULT = -2131886078i32;
pub const RTC_E_SIP_STREAM_PRESENT: ::windows_core_sys::HRESULT = -2131886079i32;
pub const RTC_E_SIP_TCP_FAIL: ::windows_core_sys::HRESULT = -2131885977i32;
pub const RTC_E_SIP_TIMEOUT: ::windows_core_sys::HRESULT = -2131886068i32;
pub const RTC_E_SIP_TLS_FAIL: ::windows_core_sys::HRESULT = -2131885975i32;
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: ::windows_core_sys::HRESULT = -2131885980i32;
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2131886057i32;
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: ::windows_core_sys::HRESULT = -2131886053i32;
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: ::windows_core_sys::HRESULT = -2131885964i32;
pub const RTC_E_START_STREAM: ::windows_core_sys::HRESULT = -2131886045i32;
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: ::windows_core_sys::HRESULT = -2131820060i32;
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: ::windows_core_sys::HRESULT = -2131820059i32;
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: ::windows_core_sys::HRESULT = -2131820124i32;
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: ::windows_core_sys::HRESULT = -2131820144i32;
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: ::windows_core_sys::HRESULT = -2131820058i32;
pub const RTC_E_STATUS_CLIENT_CONFLICT: ::windows_core_sys::HRESULT = -2131820135i32;
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: ::windows_core_sys::HRESULT = -2131820141i32;
pub const RTC_E_STATUS_CLIENT_GONE: ::windows_core_sys::HRESULT = -2131820134i32;
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: ::windows_core_sys::HRESULT = -2131820133i32;
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: ::windows_core_sys::HRESULT = -2131820062i32;
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: ::windows_core_sys::HRESULT = -2131820139i32;
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: ::windows_core_sys::HRESULT = -2131820138i32;
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: ::windows_core_sys::HRESULT = -2131820140i32;
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: ::windows_core_sys::HRESULT = -2131820142i32;
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: ::windows_core_sys::HRESULT = -2131820137i32;
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: ::windows_core_sys::HRESULT = -2131820131i32;
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: ::windows_core_sys::HRESULT = -2131820136i32;
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: ::windows_core_sys::HRESULT = -2131820130i32;
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: ::windows_core_sys::HRESULT = -2131820064i32;
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: ::windows_core_sys::HRESULT = -2131820061i32;
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: ::windows_core_sys::HRESULT = -2131820063i32;
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: ::windows_core_sys::HRESULT = -2131820143i32;
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: ::windows_core_sys::HRESULT = -2131820129i32;
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: ::windows_core_sys::HRESULT = -2131819944i32;
pub const RTC_E_STATUS_GLOBAL_DECLINE: ::windows_core_sys::HRESULT = -2131819941i32;
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: ::windows_core_sys::HRESULT = -2131819940i32;
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: ::windows_core_sys::HRESULT = -2131819938i32;
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: ::windows_core_sys::HRESULT = 15663285i32;
pub const RTC_E_STATUS_INFO_QUEUED: ::windows_core_sys::HRESULT = 15663286i32;
pub const RTC_E_STATUS_INFO_RINGING: ::windows_core_sys::HRESULT = 15663284i32;
pub const RTC_E_STATUS_INFO_TRYING: ::windows_core_sys::HRESULT = 15663204i32;
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: ::windows_core_sys::HRESULT = -2131820056i32;
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: ::windows_core_sys::HRESULT = -2131820164i32;
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: ::windows_core_sys::HRESULT = -2131820243i32;
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: ::windows_core_sys::HRESULT = -2131820242i32;
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: ::windows_core_sys::HRESULT = -2131820244i32;
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: ::windows_core_sys::HRESULT = -2131820241i32;
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: ::windows_core_sys::HRESULT = -2131820239i32;
pub const RTC_E_STATUS_REQUEST_TERMINATED: ::windows_core_sys::HRESULT = -2131820057i32;
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: ::windows_core_sys::HRESULT = -2131820042i32;
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: ::windows_core_sys::HRESULT = -2131820044i32;
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: ::windows_core_sys::HRESULT = -2131820043i32;
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: ::windows_core_sys::HRESULT = -2131820040i32;
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: ::windows_core_sys::HRESULT = -2131820041i32;
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2131820039i32;
pub const RTC_E_STATUS_SESSION_PROGRESS: ::windows_core_sys::HRESULT = 15663287i32;
pub const RTC_E_STATUS_SUCCESS: ::windows_core_sys::HRESULT = 15663304i32;
pub const RTC_E_TOO_MANY_GROUPS: ::windows_core_sys::HRESULT = -2131885997i32;
pub const RTC_E_TOO_MANY_RETRIES: ::windows_core_sys::HRESULT = -2131885989i32;
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: ::windows_core_sys::HRESULT = -2131885976i32;
pub const RTC_E_UDP_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2131885954i32;
pub type RTC_GROUP_EVENT_TYPE = i32;
pub const RTCGET_GROUP_ADD: RTC_GROUP_EVENT_TYPE = 0i32;
pub const RTCGET_GROUP_REMOVE: RTC_GROUP_EVENT_TYPE = 1i32;
pub const RTCGET_GROUP_UPDATE: RTC_GROUP_EVENT_TYPE = 2i32;
pub const RTCGET_GROUP_BUDDY_ADD: RTC_GROUP_EVENT_TYPE = 3i32;
pub const RTCGET_GROUP_BUDDY_REMOVE: RTC_GROUP_EVENT_TYPE = 4i32;
pub const RTCGET_GROUP_ROAMED: RTC_GROUP_EVENT_TYPE = 5i32;
pub type RTC_LISTEN_MODE = i32;
pub const RTCLM_NONE: RTC_LISTEN_MODE = 0i32;
pub const RTCLM_DYNAMIC: RTC_LISTEN_MODE = 1i32;
pub const RTCLM_BOTH: RTC_LISTEN_MODE = 2i32;
pub type RTC_MEDIA_EVENT_REASON = i32;
pub const RTCMER_NORMAL: RTC_MEDIA_EVENT_REASON = 0i32;
pub const RTCMER_HOLD: RTC_MEDIA_EVENT_REASON = 1i32;
pub const RTCMER_TIMEOUT: RTC_MEDIA_EVENT_REASON = 2i32;
pub const RTCMER_BAD_DEVICE: RTC_MEDIA_EVENT_REASON = 3i32;
pub const RTCMER_NO_PORT: RTC_MEDIA_EVENT_REASON = 4i32;
pub const RTCMER_PORT_MAPPING_FAILED: RTC_MEDIA_EVENT_REASON = 5i32;
pub const RTCMER_REMOTE_REQUEST: RTC_MEDIA_EVENT_REASON = 6i32;
pub type RTC_MEDIA_EVENT_TYPE = i32;
pub const RTCMET_STOPPED: RTC_MEDIA_EVENT_TYPE = 0i32;
pub const RTCMET_STARTED: RTC_MEDIA_EVENT_TYPE = 1i32;
pub const RTCMET_FAILED: RTC_MEDIA_EVENT_TYPE = 2i32;
pub type RTC_MESSAGING_EVENT_TYPE = i32;
pub const RTCMSET_MESSAGE: RTC_MESSAGING_EVENT_TYPE = 0i32;
pub const RTCMSET_STATUS: RTC_MESSAGING_EVENT_TYPE = 1i32;
pub type RTC_MESSAGING_USER_STATUS = i32;
pub const RTCMUS_IDLE: RTC_MESSAGING_USER_STATUS = 0i32;
pub const RTCMUS_TYPING: RTC_MESSAGING_USER_STATUS = 1i32;
pub type RTC_OFFER_WATCHER_MODE = i32;
pub const RTCOWM_OFFER_WATCHER_EVENT: RTC_OFFER_WATCHER_MODE = 0i32;
pub const RTCOWM_AUTOMATICALLY_ADD_WATCHER: RTC_OFFER_WATCHER_MODE = 1i32;
pub type RTC_PARTICIPANT_STATE = i32;
pub const RTCPS_IDLE: RTC_PARTICIPANT_STATE = 0i32;
pub const RTCPS_PENDING: RTC_PARTICIPANT_STATE = 1i32;
pub const RTCPS_INCOMING: RTC_PARTICIPANT_STATE = 2i32;
pub const RTCPS_ANSWERING: RTC_PARTICIPANT_STATE = 3i32;
pub const RTCPS_INPROGRESS: RTC_PARTICIPANT_STATE = 4i32;
pub const RTCPS_ALERTING: RTC_PARTICIPANT_STATE = 5i32;
pub const RTCPS_CONNECTED: RTC_PARTICIPANT_STATE = 6i32;
pub const RTCPS_DISCONNECTING: RTC_PARTICIPANT_STATE = 7i32;
pub const RTCPS_DISCONNECTED: RTC_PARTICIPANT_STATE = 8i32;
pub type RTC_PORT_TYPE = i32;
pub const RTCPT_AUDIO_RTP: RTC_PORT_TYPE = 0i32;
pub const RTCPT_AUDIO_RTCP: RTC_PORT_TYPE = 1i32;
pub const RTCPT_VIDEO_RTP: RTC_PORT_TYPE = 2i32;
pub const RTCPT_VIDEO_RTCP: RTC_PORT_TYPE = 3i32;
pub const RTCPT_SIP: RTC_PORT_TYPE = 4i32;
pub type RTC_PRESENCE_PROPERTY = i32;
pub const RTCPP_PHONENUMBER: RTC_PRESENCE_PROPERTY = 0i32;
pub const RTCPP_DISPLAYNAME: RTC_PRESENCE_PROPERTY = 1i32;
pub const RTCPP_EMAIL: RTC_PRESENCE_PROPERTY = 2i32;
pub const RTCPP_DEVICE_NAME: RTC_PRESENCE_PROPERTY = 3i32;
pub const RTCPP_MULTIPLE: RTC_PRESENCE_PROPERTY = 4i32;
pub type RTC_PRESENCE_STATUS = i32;
pub const RTCXS_PRESENCE_OFFLINE: RTC_PRESENCE_STATUS = 0i32;
pub const RTCXS_PRESENCE_ONLINE: RTC_PRESENCE_STATUS = 1i32;
pub const RTCXS_PRESENCE_AWAY: RTC_PRESENCE_STATUS = 2i32;
pub const RTCXS_PRESENCE_IDLE: RTC_PRESENCE_STATUS = 3i32;
pub const RTCXS_PRESENCE_BUSY: RTC_PRESENCE_STATUS = 4i32;
pub const RTCXS_PRESENCE_BE_RIGHT_BACK: RTC_PRESENCE_STATUS = 5i32;
pub const RTCXS_PRESENCE_ON_THE_PHONE: RTC_PRESENCE_STATUS = 6i32;
pub const RTCXS_PRESENCE_OUT_TO_LUNCH: RTC_PRESENCE_STATUS = 7i32;
pub type RTC_PRIVACY_MODE = i32;
pub const RTCPM_BLOCK_LIST_EXCLUDED: RTC_PRIVACY_MODE = 0i32;
pub const RTCPM_ALLOW_LIST_ONLY: RTC_PRIVACY_MODE = 1i32;
pub type RTC_PROFILE_EVENT_TYPE = i32;
pub const RTCPFET_PROFILE_GET: RTC_PROFILE_EVENT_TYPE = 0i32;
pub const RTCPFET_PROFILE_UPDATE: RTC_PROFILE_EVENT_TYPE = 1i32;
pub type RTC_PROVIDER_URI = i32;
pub const RTCPU_URIHOMEPAGE: RTC_PROVIDER_URI = 0i32;
pub const RTCPU_URIHELPDESK: RTC_PROVIDER_URI = 1i32;
pub const RTCPU_URIPERSONALACCOUNT: RTC_PROVIDER_URI = 2i32;
pub const RTCPU_URIDISPLAYDURINGCALL: RTC_PROVIDER_URI = 3i32;
pub const RTCPU_URIDISPLAYDURINGIDLE: RTC_PROVIDER_URI = 4i32;
pub type RTC_REGISTRATION_STATE = i32;
pub const RTCRS_NOT_REGISTERED: RTC_REGISTRATION_STATE = 0i32;
pub const RTCRS_REGISTERING: RTC_REGISTRATION_STATE = 1i32;
pub const RTCRS_REGISTERED: RTC_REGISTRATION_STATE = 2i32;
pub const RTCRS_REJECTED: RTC_REGISTRATION_STATE = 3i32;
pub const RTCRS_UNREGISTERING: RTC_REGISTRATION_STATE = 4i32;
pub const RTCRS_ERROR: RTC_REGISTRATION_STATE = 5i32;
pub const RTCRS_LOGGED_OFF: RTC_REGISTRATION_STATE = 6i32;
pub const RTCRS_LOCAL_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = 7i32;
pub const RTCRS_REMOTE_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = 8i32;
pub type RTC_REINVITE_STATE = i32;
pub const RTCRIN_INCOMING: RTC_REINVITE_STATE = 0i32;
pub const RTCRIN_SUCCEEDED: RTC_REINVITE_STATE = 1i32;
pub const RTCRIN_FAIL: RTC_REINVITE_STATE = 2i32;
pub type RTC_RING_TYPE = i32;
pub const RTCRT_PHONE: RTC_RING_TYPE = 0i32;
pub const RTCRT_MESSAGE: RTC_RING_TYPE = 1i32;
pub const RTCRT_RINGBACK: RTC_RING_TYPE = 2i32;
pub type RTC_ROAMING_EVENT_TYPE = i32;
pub const RTCRET_BUDDY_ROAMING: RTC_ROAMING_EVENT_TYPE = 0i32;
pub const RTCRET_WATCHER_ROAMING: RTC_ROAMING_EVENT_TYPE = 1i32;
pub const RTCRET_PRESENCE_ROAMING: RTC_ROAMING_EVENT_TYPE = 2i32;
pub const RTCRET_PROFILE_ROAMING: RTC_ROAMING_EVENT_TYPE = 3i32;
pub const RTCRET_WPENDING_ROAMING: RTC_ROAMING_EVENT_TYPE = 4i32;
pub type RTC_SECURITY_LEVEL = i32;
pub const RTCSECL_UNSUPPORTED: RTC_SECURITY_LEVEL = 1i32;
pub const RTCSECL_SUPPORTED: RTC_SECURITY_LEVEL = 2i32;
pub const RTCSECL_REQUIRED: RTC_SECURITY_LEVEL = 3i32;
pub type RTC_SECURITY_TYPE = i32;
pub const RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = 0i32;
pub const RTCSECT_T120_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = 1i32;
pub type RTC_SESSION_REFER_STATUS = i32;
pub const RTCSRS_REFERRING: RTC_SESSION_REFER_STATUS = 0i32;
pub const RTCSRS_ACCEPTED: RTC_SESSION_REFER_STATUS = 1i32;
pub const RTCSRS_ERROR: RTC_SESSION_REFER_STATUS = 2i32;
pub const RTCSRS_REJECTED: RTC_SESSION_REFER_STATUS = 3i32;
pub const RTCSRS_DROPPED: RTC_SESSION_REFER_STATUS = 4i32;
pub const RTCSRS_DONE: RTC_SESSION_REFER_STATUS = 5i32;
pub type RTC_SESSION_STATE = i32;
pub const RTCSS_IDLE: RTC_SESSION_STATE = 0i32;
pub const RTCSS_INCOMING: RTC_SESSION_STATE = 1i32;
pub const RTCSS_ANSWERING: RTC_SESSION_STATE = 2i32;
pub const RTCSS_INPROGRESS: RTC_SESSION_STATE = 3i32;
pub const RTCSS_CONNECTED: RTC_SESSION_STATE = 4i32;
pub const RTCSS_DISCONNECTED: RTC_SESSION_STATE = 5i32;
pub const RTCSS_HOLD: RTC_SESSION_STATE = 6i32;
pub const RTCSS_REFER: RTC_SESSION_STATE = 7i32;
pub type RTC_SESSION_TYPE = i32;
pub const RTCST_PC_TO_PC: RTC_SESSION_TYPE = 0i32;
pub const RTCST_PC_TO_PHONE: RTC_SESSION_TYPE = 1i32;
pub const RTCST_PHONE_TO_PHONE: RTC_SESSION_TYPE = 2i32;
pub const RTCST_IM: RTC_SESSION_TYPE = 3i32;
pub const RTCST_MULTIPARTY_IM: RTC_SESSION_TYPE = 4i32;
pub const RTCST_APPLICATION: RTC_SESSION_TYPE = 5i32;
pub const RTC_S_ROAMING_NOT_SUPPORTED: ::windows_core_sys::HRESULT = 15597633i32;
pub type RTC_T120_APPLET = i32;
pub const RTCTA_WHITEBOARD: RTC_T120_APPLET = 0i32;
pub const RTCTA_APPSHARING: RTC_T120_APPLET = 1i32;
pub type RTC_TERMINATE_REASON = i32;
pub const RTCTR_NORMAL: RTC_TERMINATE_REASON = 0i32;
pub const RTCTR_DND: RTC_TERMINATE_REASON = 1i32;
pub const RTCTR_BUSY: RTC_TERMINATE_REASON = 2i32;
pub const RTCTR_REJECT: RTC_TERMINATE_REASON = 3i32;
pub const RTCTR_TIMEOUT: RTC_TERMINATE_REASON = 4i32;
pub const RTCTR_SHUTDOWN: RTC_TERMINATE_REASON = 5i32;
pub const RTCTR_INSUFFICIENT_SECURITY_LEVEL: RTC_TERMINATE_REASON = 6i32;
pub const RTCTR_NOT_SUPPORTED: RTC_TERMINATE_REASON = 7i32;
pub type RTC_USER_SEARCH_COLUMN = i32;
pub const RTCUSC_URI: RTC_USER_SEARCH_COLUMN = 0i32;
pub const RTCUSC_DISPLAYNAME: RTC_USER_SEARCH_COLUMN = 1i32;
pub const RTCUSC_TITLE: RTC_USER_SEARCH_COLUMN = 2i32;
pub const RTCUSC_OFFICE: RTC_USER_SEARCH_COLUMN = 3i32;
pub const RTCUSC_PHONE: RTC_USER_SEARCH_COLUMN = 4i32;
pub const RTCUSC_COMPANY: RTC_USER_SEARCH_COLUMN = 5i32;
pub const RTCUSC_CITY: RTC_USER_SEARCH_COLUMN = 6i32;
pub const RTCUSC_STATE: RTC_USER_SEARCH_COLUMN = 7i32;
pub const RTCUSC_COUNTRY: RTC_USER_SEARCH_COLUMN = 8i32;
pub const RTCUSC_EMAIL: RTC_USER_SEARCH_COLUMN = 9i32;
pub type RTC_USER_SEARCH_PREFERENCE = i32;
pub const RTCUSP_MAX_MATCHES: RTC_USER_SEARCH_PREFERENCE = 0i32;
pub const RTCUSP_TIME_LIMIT: RTC_USER_SEARCH_PREFERENCE = 1i32;
pub type RTC_VIDEO_DEVICE = i32;
pub const RTCVD_RECEIVE: RTC_VIDEO_DEVICE = 0i32;
pub const RTCVD_PREVIEW: RTC_VIDEO_DEVICE = 1i32;
pub type RTC_WATCHER_EVENT_TYPE = i32;
pub const RTCWET_WATCHER_ADD: RTC_WATCHER_EVENT_TYPE = 0i32;
pub const RTCWET_WATCHER_REMOVE: RTC_WATCHER_EVENT_TYPE = 1i32;
pub const RTCWET_WATCHER_UPDATE: RTC_WATCHER_EVENT_TYPE = 2i32;
pub const RTCWET_WATCHER_OFFERING: RTC_WATCHER_EVENT_TYPE = 3i32;
pub const RTCWET_WATCHER_ROAMED: RTC_WATCHER_EVENT_TYPE = 4i32;
pub type RTC_WATCHER_MATCH_MODE = i32;
pub const RTCWMM_EXACT_MATCH: RTC_WATCHER_MATCH_MODE = 0i32;
pub const RTCWMM_BEST_ACE_MATCH: RTC_WATCHER_MATCH_MODE = 1i32;
pub type RTC_WATCHER_STATE = i32;
pub const RTCWS_UNKNOWN: RTC_WATCHER_STATE = 0i32;
pub const RTCWS_OFFERING: RTC_WATCHER_STATE = 1i32;
pub const RTCWS_ALLOWED: RTC_WATCHER_STATE = 2i32;
pub const RTCWS_BLOCKED: RTC_WATCHER_STATE = 3i32;
pub const RTCWS_DENIED: RTC_WATCHER_STATE = 4i32;
pub const RTCWS_PROMPT: RTC_WATCHER_STATE = 5i32;
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "win32-networking-sys")]
pub struct TRANSPORT_SETTING {
    pub SettingId: ::win32_networking_sys::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "win32-networking-sys")]
impl ::core::marker::Copy for TRANSPORT_SETTING {}
#[cfg(feature = "win32-networking-sys")]
impl ::core::clone::Clone for TRANSPORT_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
