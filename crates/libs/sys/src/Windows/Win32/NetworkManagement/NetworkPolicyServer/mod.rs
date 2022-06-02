#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ACCOUNTINGPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_ACCOUNTING: ACCOUNTINGPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_ACCOUNTING_INTERIM: ACCOUNTINGPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_AUTHENTICATION: ACCOUNTINGPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_OPEN_NEW_FREQUENCY: ACCOUNTINGPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_OPEN_NEW_SIZE: ACCOUNTINGPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_FILE_DIRECTORY: ACCOUNTINGPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_IAS1_FORMAT: ACCOUNTINGPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_ENABLE_LOGGING: ACCOUNTINGPROPERTIES = 1033i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_DELETE_IF_FULL: ACCOUNTINGPROPERTIES = 1034i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_SQL_MAX_SESSIONS: ACCOUNTINGPROPERTIES = 1035i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_AUTHENTICATION_INTERIM: ACCOUNTINGPROPERTIES = 1036i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_FILE_IS_BACKUP: ACCOUNTINGPROPERTIES = 1037i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_DISCARD_REQUEST_ON_FAILURE: ACCOUNTINGPROPERTIES = 1038i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ATTRIBUTEFILTER = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_FILTER_NONE: ATTRIBUTEFILTER = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_FILTER_VPN_DIALUP: ATTRIBUTEFILTER = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_FILTER_IEEE_802_1x: ATTRIBUTEFILTER = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ATTRIBUTEID = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_UNDEFINED: ATTRIBUTEID = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_MIN_VALUE: ATTRIBUTEID = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_USER_NAME: ATTRIBUTEID = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_USER_PASSWORD: ATTRIBUTEID = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CHAP_PASSWORD: ATTRIBUTEID = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_IP_ADDRESS: ATTRIBUTEID = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_PORT: ATTRIBUTEID = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_SERVICE_TYPE: ATTRIBUTEID = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_PROTOCOL: ATTRIBUTEID = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IP_ADDRESS: ATTRIBUTEID = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IP_NETMASK: ATTRIBUTEID = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_ROUTING: ATTRIBUTEID = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FILTER_ID: ATTRIBUTEID = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_MTU: ATTRIBUTEID = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_COMPRESSION: ATTRIBUTEID = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_IP_HOST: ATTRIBUTEID = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_SERVICE: ATTRIBUTEID = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_TCP_PORT: ATTRIBUTEID = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_UNASSIGNED1: ATTRIBUTEID = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_REPLY_MESSAGE: ATTRIBUTEID = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLBACK_NUMBER: ATTRIBUTEID = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLBACK_ID: ATTRIBUTEID = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_UNASSIGNED2: ATTRIBUTEID = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_ROUTE: ATTRIBUTEID = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPX_NETWORK: ATTRIBUTEID = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_STATE: ATTRIBUTEID = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CLASS: ATTRIBUTEID = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_VENDOR_SPECIFIC: ATTRIBUTEID = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_SESSION_TIMEOUT: ATTRIBUTEID = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_IDLE_TIMEOUT: ATTRIBUTEID = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TERMINATION_ACTION: ATTRIBUTEID = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLED_STATION_ID: ATTRIBUTEID = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLING_STATION_ID: ATTRIBUTEID = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_IDENTIFIER: ATTRIBUTEID = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PROXY_STATE: ATTRIBUTEID = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_SERVICE: ATTRIBUTEID = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_NODE: ATTRIBUTEID = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_GROUP: ATTRIBUTEID = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_LINK: ATTRIBUTEID = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_NET: ATTRIBUTEID = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_ZONE: ATTRIBUTEID = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_STATUS_TYPE: ATTRIBUTEID = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_DELAY_TIME: ATTRIBUTEID = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_INPUT_OCTETS: ATTRIBUTEID = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_OUTPUT_OCTETS: ATTRIBUTEID = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_SESSION_ID: ATTRIBUTEID = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_AUTHENTIC: ATTRIBUTEID = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_SESSION_TIME: ATTRIBUTEID = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_INPUT_PACKETS: ATTRIBUTEID = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_OUTPUT_PACKETS: ATTRIBUTEID = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_TERMINATE_CAUSE: ATTRIBUTEID = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_MULTI_SSN_ID: ATTRIBUTEID = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_LINK_COUNT: ATTRIBUTEID = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CHAP_CHALLENGE: ATTRIBUTEID = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_PORT_TYPE: ATTRIBUTEID = 61u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PORT_LIMIT: ATTRIBUTEID = 62u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_PORT: ATTRIBUTEID = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_TYPE: ATTRIBUTEID = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_MEDIUM_TYPE: ATTRIBUTEID = 65u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_CLIENT_ENDPT: ATTRIBUTEID = 66u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_SERVER_ENDPT: ATTRIBUTEID = 67u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_TUNNEL_CONN: ATTRIBUTEID = 68u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_PASSWORD: ATTRIBUTEID = 69u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_PASSWORD: ATTRIBUTEID = 70u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_FEATURES: ATTRIBUTEID = 71u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_ZONE_ACCESS: ATTRIBUTEID = 72u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_SECURITY: ATTRIBUTEID = 73u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_SECURITY_DATA: ATTRIBUTEID = 74u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PASSWORD_RETRY: ATTRIBUTEID = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PROMPT: ATTRIBUTEID = 76u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CONNECT_INFO: ATTRIBUTEID = 77u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CONFIGURATION_TOKEN: ATTRIBUTEID = 78u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_EAP_MESSAGE: ATTRIBUTEID = 79u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_SIGNATURE: ATTRIBUTEID = 80u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_PVT_GROUP_ID: ATTRIBUTEID = 81u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_ASSIGNMENT_ID: ATTRIBUTEID = 82u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_PREFERENCE: ATTRIBUTEID = 83u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_CHALLENGE_RESPONSE: ATTRIBUTEID = 84u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_INTERIM_INTERVAL: ATTRIBUTEID = 85u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_IPv6_ADDRESS: ATTRIBUTEID = 95u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_INTERFACE_ID: ATTRIBUTEID = 96u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_PREFIX: ATTRIBUTEID = 97u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_IPv6_HOST: ATTRIBUTEID = 98u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_ROUTE: ATTRIBUTEID = 99u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_POOL: ATTRIBUTEID = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IP_ADDRESS: ATTRIBUTEID = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_CALLBACK_NUMBER: ATTRIBUTEID = 4097u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_CALLING_STATION_ID: ATTRIBUTEID = 4098u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_NP_CALLING_STATION_ID: ATTRIBUTEID = 4099u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_ROUTE: ATTRIBUTEID = 4100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_IGNORE_USER_DIALIN_PROPERTIES: ATTRIBUTEID = 4101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_TIME_OF_DAY: ATTRIBUTEID = 4102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_CALLED_STATION_ID: ATTRIBUTEID = 4103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_ALLOWED_PORT_TYPES: ATTRIBUTEID = 4104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_AUTHENTICATION_TYPE: ATTRIBUTEID = 4105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_ALLOWED_EAP_TYPE: ATTRIBUTEID = 4106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SHARED_SECRET: ATTRIBUTEID = 4107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_IP_ADDRESS: ATTRIBUTEID = 4108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_PACKET_HEADER: ATTRIBUTEID = 4109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_TOKEN_GROUPS: ATTRIBUTEID = 4110u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ALLOW_DIALIN: ATTRIBUTEID = 4111u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REQUEST_ID: ATTRIBUTEID = 4112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MANIPULATION_TARGET: ATTRIBUTEID = 4113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MANIPULATION_RULE: ATTRIBUTEID = 4114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ORIGINAL_USER_NAME: ATTRIBUTEID = 4115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_VENDOR_TYPE: ATTRIBUTEID = 4116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_UDP_PORT: ATTRIBUTEID = 4117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_CHALLENGE: ATTRIBUTEID = 4118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_RESPONSE: ATTRIBUTEID = 4119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_DOMAIN: ATTRIBUTEID = 4120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_ERROR: ATTRIBUTEID = 4121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_CPW1: ATTRIBUTEID = 4122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_CPW2: ATTRIBUTEID = 4123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_LM_ENC_PW: ATTRIBUTEID = 4124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_NT_ENC_PW: ATTRIBUTEID = 4125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_MPPE_KEYS: ATTRIBUTEID = 4126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_AUTHENTICATION_TYPE: ATTRIBUTEID = 4127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_NAME: ATTRIBUTEID = 4128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NT4_ACCOUNT_NAME: ATTRIBUTEID = 4129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_FULLY_QUALIFIED_USER_NAME: ATTRIBUTEID = 4130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NTGROUPS: ATTRIBUTEID = 4131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_FRIENDLY_NAME: ATTRIBUTEID = 4132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_AUTH_PROVIDER_TYPE: ATTRIBUTEID = 4133u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_ACCT_AUTH_TYPE: ATTRIBUTEID = 4134u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_ACCT_EAP_TYPE: ATTRIBUTEID = 4135u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PACKET_TYPE: ATTRIBUTEID = 4136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_AUTH_PROVIDER_NAME: ATTRIBUTEID = 4137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ACCT_PROVIDER_TYPE: ATTRIBUTEID = 4138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ACCT_PROVIDER_NAME: ATTRIBUTEID = 4139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_MPPE_SEND_KEY: ATTRIBUTEID = 4140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_MPPE_RECV_KEY: ATTRIBUTEID = 4141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REASON_CODE: ATTRIBUTEID = 4142u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_FILTER: ATTRIBUTEID = 4143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP2_RESPONSE: ATTRIBUTEID = 4144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP2_SUCCESS: ATTRIBUTEID = 4145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP2_CPW: ATTRIBUTEID = 4146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_VENDOR: ATTRIBUTEID = 4147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_VERSION: ATTRIBUTEID = 4148u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_NAME: ATTRIBUTEID = 4149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_PRIMARY_DNS_SERVER: ATTRIBUTEID = 4150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_SECONDARY_DNS_SERVER: ATTRIBUTEID = 4151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_PRIMARY_NBNS_SERVER: ATTRIBUTEID = 4152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_SECONDARY_NBNS_SERVER: ATTRIBUTEID = 4153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROXY_POLICY_NAME: ATTRIBUTEID = 4154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROVIDER_TYPE: ATTRIBUTEID = 4155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROVIDER_NAME: ATTRIBUTEID = 4156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REMOTE_SERVER_ADDRESS: ATTRIBUTEID = 4157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_GENERATE_CLASS_ATTRIBUTE: ATTRIBUTEID = 4158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_CLIENT_NAME: ATTRIBUTEID = 4159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_CLIENT_VERSION: ATTRIBUTEID = 4160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ALLOWED_CERTIFICATE_EKU: ATTRIBUTEID = 4161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EXTENSION_STATE: ATTRIBUTEID = 4162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_GENERATE_SESSION_TIMEOUT: ATTRIBUTEID = 4163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SESSION_TIMEOUT: ATTRIBUTEID = 4164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_IPFILTER: ATTRIBUTEID = 4165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_SESSION_TIMEOUT: ATTRIBUTEID = 4166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_USER_SECURITY_IDENTITY: ATTRIBUTEID = 4167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REMOTE_RADIUS_TO_WINDOWS_USER_MAPPING: ATTRIBUTEID = 4168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PASSPORT_USER_MAPPING_UPN_SUFFIX: ATTRIBUTEID = 4169u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_TUNNEL_TAG: ATTRIBUTEID = 4170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_PEAPUPFRONT_ENABLED: ATTRIBUTEID = 4171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CERTIFICATE_EKU: ATTRIBUTEID = 8097u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_CONFIG: ATTRIBUTEID = 8098u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PEAP_EMBEDDED_EAP_TYPEID: ATTRIBUTEID = 8099u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PEAP_FAST_ROAMED_SESSION: ATTRIBUTEID = 8100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_TYPEID: ATTRIBUTEID = 8101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_EAP_TLV: ATTRIBUTEID = 8102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REJECT_REASON_CODE: ATTRIBUTEID = 8103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROXY_EAP_CONFIG: ATTRIBUTEID = 8104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_SESSION: ATTRIBUTEID = 8105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_IS_REPLAY: ATTRIBUTEID = 8106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLEAR_TEXT_PASSWORD: ATTRIBUTEID = 8107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IDENTITY_TYPE: ATTRIBUTEID = 8108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_SERVICE_CLASS: ATTRIBUTEID = 8109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_USER_CLASS: ATTRIBUTEID = 8110u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_STATE: ATTRIBUTEID = 8111u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_OVERRIDE_RAP_AUTH: ATTRIBUTEID = 8112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PEAP_CHANNEL_UP: ATTRIBUTEID = 8113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NAME_MAPPED: ATTRIBUTEID = 8114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_POLICY_ENFORCED: ATTRIBUTEID = 8115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_NTGROUPS: ATTRIBUTEID = 8116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_USER_NTGROUPS: ATTRIBUTEID = 8117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_TOKEN_GROUPS: ATTRIBUTEID = 8118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_USER_TOKEN_GROUPS: ATTRIBUTEID = 8119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_GRACE_TIME: ATTRIBUTEID = 8120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_URL: ATTRIBUTEID = 8121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_FIXUP_SERVERS: ATTRIBUTEID = 8122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_NOT_QUARANTINE_CAPABLE: ATTRIBUTEID = 8123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SYSTEM_HEALTH_RESULT: ATTRIBUTEID = 8124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SYSTEM_HEALTH_VALIDATORS: ATTRIBUTEID = 8125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_NAME: ATTRIBUTEID = 8126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NT4_MACHINE_NAME: ATTRIBUTEID = 8127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SESSION_HANDLE: ATTRIBUTEID = 8128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_FULLY_QUALIFIED_MACHINE_NAME: ATTRIBUTEID = 8129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_FIXUP_SERVERS_CONFIGURATION: ATTRIBUTEID = 8130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_QUARANTINE_COMPATIBLE: ATTRIBUTEID = 8131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_NETWORK_ACCESS_SERVER_TYPE: ATTRIBUTEID = 8132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SESSION_ID: ATTRIBUTEID = 8133u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_AFW_QUARANTINE_ZONE: ATTRIBUTEID = 8134u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_AFW_PROTECTION_LEVEL: ATTRIBUTEID = 8135u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_UPDATE_NON_COMPLIANT: ATTRIBUTEID = 8136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REQUEST_START_TIME: ATTRIBUTEID = 8137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_MACHINE_NAME: ATTRIBUTEID = 8138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_IPv6_ADDRESS: ATTRIBUTEID = 8139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_INTERFACE_ID: ATTRIBUTEID = 8140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IPv6_PREFIX: ATTRIBUTEID = 8141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IPv6_ROUTE: ATTRIBUTEID = 8142u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_GRACE_TIME_CONFIGURATION: ATTRIBUTEID = 8143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IPv6_FILTER: ATTRIBUTEID = 8144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IPV4_REMEDIATION_SERVERS: ATTRIBUTEID = 8145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IPV6_REMEDIATION_SERVERS: ATTRIBUTEID = 8146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROXY_RETRY_COUNT: ATTRIBUTEID = 8147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_INVENTORY: ATTRIBUTEID = 8148u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ABSOLUTE_TIME: ATTRIBUTEID = 8149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_SOH: ATTRIBUTEID = 8150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_TYPES_CONFIGURED_IN_PROXYPOLICY: ATTRIBUTEID = 8151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_HCAP_LOCATION_GROUP_NAME: ATTRIBUTEID = 8152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_EXTENDED_QUARANTINE_STATE: ATTRIBUTEID = 8153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SOH_CARRIER_EAPTLV: ATTRIBUTEID = 8154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_HCAP_USER_GROUPS: ATTRIBUTEID = 8155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_MACHINE_HEALTHCHECK_ONLY: ATTRIBUTEID = 8156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_POLICY_EVALUATED_SHV: ATTRIBUTEID = 8157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_CORRELATION_ID: ATTRIBUTEID = 8158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_HCAP_USER_NAME: ATTRIBUTEID = 8159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NT4_HCAP_ACCOUNT_NAME: ATTRIBUTEID = 8160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_USER_TOKEN_SID: ATTRIBUTEID = 8161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_TOKEN_SID: ATTRIBUTEID = 8162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_VALIDATED: ATTRIBUTEID = 8163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_USER_IPv4_ADDRESS: ATTRIBUTEID = 8164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_USER_IPv6_ADDRESS: ATTRIBUTEID = 8165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_TSG_DEVICE_REDIRECTION: ATTRIBUTEID = 8166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ACCEPT_REASON_CODE: ATTRIBUTEID = 8167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_LOGGING_RESULT: ATTRIBUTEID = 8168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SERVER_IP_ADDRESS: ATTRIBUTEID = 8169u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SERVER_IPv6_ADDRESS: ATTRIBUTEID = 8170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_RADIUS_USERNAME_ENCODING_ASCII: ATTRIBUTEID = 8171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_ROUTING_DOMAIN_ID: ATTRIBUTEID = 8172u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CERTIFICATE_THUMBPRINT: ATTRIBUTEID = 8250u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_ENCRYPTION_TYPE: ATTRIBUTEID = 4294967206u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_ENCRYPTION_POLICY: ATTRIBUTEID = 4294967207u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_BAP_REQUIRED: ATTRIBUTEID = 4294967208u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_BAP_LINE_DOWN_TIME: ATTRIBUTEID = 4294967209u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_BAP_LINE_DOWN_LIMIT: ATTRIBUTEID = 4294967210u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ATTRIBUTEINFO = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const NAME: ATTRIBUTEINFO = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYNTAX: ATTRIBUTEINFO = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RESTRICTIONS: ATTRIBUTEINFO = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DESCRIPTION: ATTRIBUTEINFO = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const VENDORID: ATTRIBUTEINFO = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const LDAPNAME: ATTRIBUTEINFO = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const VENDORTYPE: ATTRIBUTEINFO = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ATTRIBUTEPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ID: ATTRIBUTEPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_VENDOR_ID: ATTRIBUTEPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_VENDOR_TYPE_ID: ATTRIBUTEPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_IS_ENUMERABLE: ATTRIBUTEPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ENUM_NAMES: ATTRIBUTEPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ENUM_VALUES: ATTRIBUTEPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_SYNTAX: ATTRIBUTEPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_MULTIPLE: ATTRIBUTEPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_LOG_ORDINAL: ATTRIBUTEPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROFILE: ATTRIBUTEPROPERTIES = 1033i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_CONDITION: ATTRIBUTEPROPERTIES = 1034i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_DISPLAY_NAME: ATTRIBUTEPROPERTIES = 1035i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_VALUE: ATTRIBUTEPROPERTIES = 1036i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROXY_PROFILE: ATTRIBUTEPROPERTIES = 1037i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROXY_CONDITION: ATTRIBUTEPROPERTIES = 1038i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_VPNDIALUP: ATTRIBUTEPROPERTIES = 1039i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_8021X: ATTRIBUTEPROPERTIES = 1040i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ENUM_FILTERS: ATTRIBUTEPROPERTIES = 1041i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ATTRIBUTERESTRICTIONS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MULTIVALUED: ATTRIBUTERESTRICTIONS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINPROFILE: ATTRIBUTERESTRICTIONS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINCONDITION: ATTRIBUTERESTRICTIONS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINPROXYPROFILE: ATTRIBUTERESTRICTIONS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINPROXYCONDITION: ATTRIBUTERESTRICTIONS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINVPNDIALUP: ATTRIBUTERESTRICTIONS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDIN8021X: ATTRIBUTERESTRICTIONS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type ATTRIBUTESYNTAX = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_BOOLEAN: ATTRIBUTESYNTAX = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_INTEGER: ATTRIBUTESYNTAX = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_ENUMERATOR: ATTRIBUTESYNTAX = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_INETADDR: ATTRIBUTESYNTAX = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_STRING: ATTRIBUTESYNTAX = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_OCTETSTRING: ATTRIBUTESYNTAX = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_UTCTIME: ATTRIBUTESYNTAX = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_PROVIDERSPECIFIC: ATTRIBUTESYNTAX = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_UNSIGNEDINTEGER: ATTRIBUTESYNTAX = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_INETADDR6: ATTRIBUTESYNTAX = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type AUTHENTICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_INVALID: AUTHENTICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_PAP: AUTHENTICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MD5CHAP: AUTHENTICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP: AUTHENTICATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP2: AUTHENTICATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_EAP: AUTHENTICATION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_ARAP: AUTHENTICATION_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_NONE: AUTHENTICATION_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_CUSTOM: AUTHENTICATION_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP_CPW: AUTHENTICATION_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP2_CPW: AUTHENTICATION_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_PEAP: AUTHENTICATION_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_AUTHORIZATION_VALUE_W: &str = "AuthorizationDLLs";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_ENFORCE_NP_FOR_PAP_CHALLENGE_RESPONSE_VALUE_W: &str = "EnforceNetworkPolicyForPAPBasedChallengeResponse";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_EXTENSIONS_VALUE_W: &str = "ExtensionDLLs";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_PARAMETERS_KEY_W: &str = "System\\CurrentControlSet\\Services\\AuthSrv\\Parameters";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type CLIENTPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_REQUIRE_SIGNATURE: CLIENTPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_UNUSED: CLIENTPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_SHARED_SECRET: CLIENTPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_NAS_MANUFACTURER: CLIENTPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_ADDRESS: CLIENTPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_QUARANTINE_COMPATIBLE: CLIENTPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_ENABLED: CLIENTPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_SECRET_TEMPLATE_GUID: CLIENTPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type CONDITIONPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CONDITION_TEXT: CONDITIONPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type DICTIONARYPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_DICTIONARY_ATTRIBUTES_COLLECTION: DICTIONARYPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_DICTIONARY_LOCATION: DICTIONARYPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IASCOMMONPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_RESERVED: IASCOMMONPROPERTIES = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_CLASS: IASCOMMONPROPERTIES = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_NAME: IASCOMMONPROPERTIES = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_DESCRIPTION: IASCOMMONPROPERTIES = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_ID: IASCOMMONPROPERTIES = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_DATASTORE_NAME: IASCOMMONPROPERTIES = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_TEMPLATE_GUID: IASCOMMONPROPERTIES = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_OPAQUE: IASCOMMONPROPERTIES = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_START: IASCOMMONPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IASCOMPONENTPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_COMPONENT_ID: IASCOMPONENTPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_COMPONENT_PROG_ID: IASCOMPONENTPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_COMPONENT_START: IASCOMPONENTPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IASDATASTORE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DATA_STORE_LOCAL: IASDATASTORE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DATA_STORE_DIRECTORY: IASDATASTORE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IASDOMAINTYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_NONE: IASDOMAINTYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_NT4: IASDOMAINTYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_NT5: IASDOMAINTYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_MIXED: IASDOMAINTYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IASOSTYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT4_WORKSTATION: IASOSTYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT5_WORKSTATION: IASOSTYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_WORKSTATION: IASOSTYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_1_WORKSTATION: IASOSTYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_2_WORKSTATION: IASOSTYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_3_WORKSTATION: IASOSTYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT10_0_WORKSTATION: IASOSTYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT4_SERVER: IASOSTYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT5_SERVER: IASOSTYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_SERVER: IASOSTYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_1_SERVER: IASOSTYPE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_2_SERVER: IASOSTYPE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_3_SERVER: IASOSTYPE = 12i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT10_0_SERVER: IASOSTYPE = 13i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IASPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_RADIUSSERVERGROUPS_COLLECTION: IASPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_POLICIES_COLLECTION: IASPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROFILES_COLLECTION: IASPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROTOCOLS_COLLECTION: IASPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_AUDITORS_COLLECTION: IASPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_REQUESTHANDLERS_COLLECTION: IASPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROXYPOLICIES_COLLECTION: IASPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROXYPROFILES_COLLECTION: IASPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_REMEDIATIONSERVERGROUPS_COLLECTION: IASPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_SHVTEMPLATES_COLLECTION: IASPROPERTIES = 1033i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IDENTITY_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_IDENTITY_NO_DEFAULT: IDENTITY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type IPFILTERPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IPFILTER_ATTRIBUTES_COLLECTION: IPFILTERPROPERTIES = 1024i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISdo {
    pub base__: super::super::System::Com::IDispatch,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, id: i32, pppropertyinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, id: i32, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutProperty: unsafe extern "system" fn(this: *mut *mut Self, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutProperty: usize,
    pub ResetProperty: unsafe extern "system" fn(this: *mut *mut Self, id: i32) -> ::windows_sys::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISdoCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, pitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reload: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNameUnique: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNameUnique: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, name: *const super::super::System::Com::VARIANT, pitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISdoDictionaryOld {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnumAttributes: unsafe extern "system" fn(this: *mut *mut Self, id: *mut super::super::System::Com::VARIANT, pvalues: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnumAttributes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttributeInfo: unsafe extern "system" fn(this: *mut *mut Self, id: ATTRIBUTEID, pinfoids: *const super::super::System::Com::VARIANT, pinfovalues: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttributeInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnumAttributeValues: unsafe extern "system" fn(this: *mut *mut Self, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Com::VARIANT, pvaluesdesc: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnumAttributeValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAttribute: unsafe extern "system" fn(this: *mut *mut Self, id: ATTRIBUTEID, ppattributeobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeID: unsafe extern "system" fn(this: *mut *mut Self, bstrattributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: *mut ATTRIBUTEID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeID: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISdoMachine {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Attach: unsafe extern "system" fn(this: *mut *mut Self, bstrcomputername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Attach: usize,
    pub GetDictionarySDO: unsafe extern "system" fn(this: *mut *mut Self, ppdictionarysdo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetServiceSDO: unsafe extern "system" fn(this: *mut *mut Self, edatastore: IASDATASTORE, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppservicesdo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetServiceSDO: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserSDO: unsafe extern "system" fn(this: *mut *mut Self, edatastore: IASDATASTORE, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppusersdo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserSDO: usize,
    pub GetOSType: unsafe extern "system" fn(this: *mut *mut Self, eostype: *mut IASOSTYPE) -> ::windows_sys::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut *mut Self, edomaintype: *mut IASDOMAINTYPE) -> ::windows_sys::core::HRESULT,
    pub IsDirectoryAvailable: unsafe extern "system" fn(this: *mut *mut Self, booldirectoryavailable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttachedComputer: unsafe extern "system" fn(this: *mut *mut Self, bstrcomputername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttachedComputer: usize,
    pub GetSDOSchema: unsafe extern "system" fn(this: *mut *mut Self, ppsdoschema: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISdoMachine2 {
    pub base__: ISdoMachine,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTemplatesSDO: unsafe extern "system" fn(this: *mut *mut Self, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplatessdo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTemplatesSDO: usize,
    pub EnableTemplates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConfigAgainstTemplates: unsafe extern "system" fn(this: *mut *mut Self, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppconfigroot: *mut *mut ::core::ffi::c_void, pptemplatesroot: *mut *mut ::core::ffi::c_void, bforcedsync: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConfigAgainstTemplates: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportRemoteTemplates: unsafe extern "system" fn(this: *mut *mut Self, plocaltemplatesroot: *mut ::core::ffi::c_void, bstrremotemachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportRemoteTemplates: usize,
    pub Reload: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISdoServiceControl {
    pub base__: super::super::System::Com::IDispatch,
    pub StartService: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopService: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetServiceStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ResetService: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITemplateSdo {
    pub base__: ISdo,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddToCollection: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcollection: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddToCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddToSdo: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psdotarget: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddToSdo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddToSdoAsProperty: unsafe extern "system" fn(this: *mut *mut Self, psdotarget: *mut ::core::ffi::c_void, id: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddToSdoAsProperty: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type NAMESPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NAMES_REALMS: NAMESPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type NAPPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NAP_POLICIES_COLLECTION: NAPPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHV_TEMPLATES_COLLECTION: NAPPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type NEW_LOG_FILE_FREQUENCY = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_UNLIMITED_SIZE: NEW_LOG_FILE_FREQUENCY = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_DAILY: NEW_LOG_FILE_FREQUENCY = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_WEEKLY: NEW_LOG_FILE_FREQUENCY = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_MONTHLY: NEW_LOG_FILE_FREQUENCY = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_WHEN_FILE_SIZE_REACHES: NEW_LOG_FILE_FREQUENCY = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type NTEVENTLOGPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_EVENTLOG_LOG_APPLICATION_EVENTS: NTEVENTLOGPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_EVENTLOG_LOG_MALFORMED: NTEVENTLOGPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_EVENTLOG_LOG_DEBUG: NTEVENTLOGPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type NTSAMPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NTSAM_ALLOW_LM_AUTHENTICATION: NTSAMPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type POLICYPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_CONSTRAINT: POLICYPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_MERIT: POLICYPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_UNUSED0: POLICYPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_UNUSED1: POLICYPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_PROFILE_NAME: POLICYPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_ACTION: POLICYPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_CONDITIONS_COLLECTION: POLICYPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_ENABLED: POLICYPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_SOURCETAG: POLICYPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_FREE_ATTRIBUTES = ::core::option::Option<unsafe extern "system" fn(pattrs: *mut RADIUS_ATTRIBUTE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_INIT = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_PROCESS = ::core::option::Option<unsafe extern "system" fn(pattrs: *const RADIUS_ATTRIBUTE, pfaction: *mut RADIUS_ACTION) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_PROCESS_2 = ::core::option::Option<unsafe extern "system" fn(pecb: *mut RADIUS_EXTENSION_CONTROL_BLOCK) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_PROCESS_EX = ::core::option::Option<unsafe extern "system" fn(pinattrs: *const RADIUS_ATTRIBUTE, poutattrs: *mut *mut RADIUS_ATTRIBUTE, pfaction: *mut RADIUS_ACTION) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_TERM = ::core::option::Option<unsafe extern "system" fn()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PROFILEPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROFILE_ATTRIBUTES_COLLECTION: PROFILEPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROFILE_IPFILTER_TEMPLATE_GUID: PROFILEPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PROTOCOLPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROTOCOL_REQUEST_HANDLER: PROTOCOLPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROTOCOL_START: PROTOCOLPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUSPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_ACCOUNTING_PORT: RADIUSPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_AUTHENTICATION_PORT: RADIUSPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_CLIENTS_COLLECTION: RADIUSPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_VENDORS_COLLECTION: RADIUSPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUSPROXYPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSPROXY_SERVERGROUPS: RADIUSPROXYPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUSSERVERGROUPPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVERGROUP_SERVERS_COLLECTION: RADIUSSERVERGROUPPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUSSERVERPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_AUTH_PORT: RADIUSSERVERPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_AUTH_SECRET: RADIUSSERVERPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ACCT_PORT: RADIUSSERVERPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ACCT_SECRET: RADIUSSERVERPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ADDRESS: RADIUSSERVERPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_FORWARD_ACCT_ONOFF: RADIUSSERVERPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_PRIORITY: RADIUSSERVERPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_WEIGHT: RADIUSSERVERPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_TIMEOUT: RADIUSSERVERPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_MAX_LOST: RADIUSSERVERPROPERTIES = 1033i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_BLACKOUT: RADIUSSERVERPROPERTIES = 1034i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_SEND_SIGNATURE: RADIUSSERVERPROPERTIES = 1035i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_AUTH_SECRET_TEMPLATE_GUID: RADIUSSERVERPROPERTIES = 1036i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ACCT_SECRET_TEMPLATE_GUID: RADIUSSERVERPROPERTIES = 1037i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_ACTION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const raContinue: RADIUS_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const raReject: RADIUS_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const raAccept: RADIUS_ACTION = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_ATTRIBUTE {
    pub dwAttrType: u32,
    pub fDataType: RADIUS_DATA_TYPE,
    pub cbDataLength: u32,
    pub Anonymous: RADIUS_ATTRIBUTE_0,
}
impl ::core::marker::Copy for RADIUS_ATTRIBUTE {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub union RADIUS_ATTRIBUTE_0 {
    pub dwValue: u32,
    pub lpValue: *const u8,
}
impl ::core::marker::Copy for RADIUS_ATTRIBUTE_0 {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_ATTRIBUTE_ARRAY {
    pub cbSize: u32,
    pub Add: isize,
    pub AttributeAt: isize,
    pub GetSize: isize,
    pub InsertAt: isize,
    pub RemoveAt: isize,
    pub SetAt: isize,
}
impl ::core::marker::Copy for RADIUS_ATTRIBUTE_ARRAY {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_ATTRIBUTE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratMinimum: RADIUS_ATTRIBUTE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratUserName: RADIUS_ATTRIBUTE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratUserPassword: RADIUS_ATTRIBUTE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCHAPPassword: RADIUS_ATTRIBUTE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASIPAddress: RADIUS_ATTRIBUTE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASPort: RADIUS_ATTRIBUTE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratServiceType: RADIUS_ATTRIBUTE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedProtocol: RADIUS_ATTRIBUTE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPAddress: RADIUS_ATTRIBUTE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPNetmask: RADIUS_ATTRIBUTE_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedRouting: RADIUS_ATTRIBUTE_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFilterId: RADIUS_ATTRIBUTE_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedMTU: RADIUS_ATTRIBUTE_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedCompression: RADIUS_ATTRIBUTE_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginIPHost: RADIUS_ATTRIBUTE_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginService: RADIUS_ATTRIBUTE_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginPort: RADIUS_ATTRIBUTE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratReplyMessage: RADIUS_ATTRIBUTE_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCallbackNumber: RADIUS_ATTRIBUTE_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCallbackId: RADIUS_ATTRIBUTE_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedRoute: RADIUS_ATTRIBUTE_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPXNetwork: RADIUS_ATTRIBUTE_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratState: RADIUS_ATTRIBUTE_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratClass: RADIUS_ATTRIBUTE_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratVendorSpecific: RADIUS_ATTRIBUTE_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSessionTimeout: RADIUS_ATTRIBUTE_TYPE = 27i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratIdleTimeout: RADIUS_ATTRIBUTE_TYPE = 28i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTerminationAction: RADIUS_ATTRIBUTE_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCalledStationId: RADIUS_ATTRIBUTE_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCallingStationId: RADIUS_ATTRIBUTE_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASIdentifier: RADIUS_ATTRIBUTE_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratProxyState: RADIUS_ATTRIBUTE_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginLATService: RADIUS_ATTRIBUTE_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginLATNode: RADIUS_ATTRIBUTE_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginLATGroup: RADIUS_ATTRIBUTE_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedAppleTalkLink: RADIUS_ATTRIBUTE_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedAppleTalkNetwork: RADIUS_ATTRIBUTE_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedAppleTalkZone: RADIUS_ATTRIBUTE_TYPE = 39i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctStatusType: RADIUS_ATTRIBUTE_TYPE = 40i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctDelayTime: RADIUS_ATTRIBUTE_TYPE = 41i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctInputOctets: RADIUS_ATTRIBUTE_TYPE = 42i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctOutputOctets: RADIUS_ATTRIBUTE_TYPE = 43i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctSessionId: RADIUS_ATTRIBUTE_TYPE = 44i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctAuthentic: RADIUS_ATTRIBUTE_TYPE = 45i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctSessionTime: RADIUS_ATTRIBUTE_TYPE = 46i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctInputPackets: RADIUS_ATTRIBUTE_TYPE = 47i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctOutputPackets: RADIUS_ATTRIBUTE_TYPE = 48i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctTerminationCause: RADIUS_ATTRIBUTE_TYPE = 49i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCHAPChallenge: RADIUS_ATTRIBUTE_TYPE = 60i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASPortType: RADIUS_ATTRIBUTE_TYPE = 61i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratPortLimit: RADIUS_ATTRIBUTE_TYPE = 62i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTunnelType: RADIUS_ATTRIBUTE_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratMediumType: RADIUS_ATTRIBUTE_TYPE = 65i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTunnelPassword: RADIUS_ATTRIBUTE_TYPE = 69i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTunnelPrivateGroupID: RADIUS_ATTRIBUTE_TYPE = 81i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASIPv6Address: RADIUS_ATTRIBUTE_TYPE = 95i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedInterfaceId: RADIUS_ATTRIBUTE_TYPE = 96i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPv6Prefix: RADIUS_ATTRIBUTE_TYPE = 97i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginIPv6Host: RADIUS_ATTRIBUTE_TYPE = 98i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPv6Route: RADIUS_ATTRIBUTE_TYPE = 99i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPv6Pool: RADIUS_ATTRIBUTE_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCode: RADIUS_ATTRIBUTE_TYPE = 262i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratIdentifier: RADIUS_ATTRIBUTE_TYPE = 263i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAuthenticator: RADIUS_ATTRIBUTE_TYPE = 264i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSrcIPAddress: RADIUS_ATTRIBUTE_TYPE = 265i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSrcPort: RADIUS_ATTRIBUTE_TYPE = 266i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratProvider: RADIUS_ATTRIBUTE_TYPE = 267i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratStrippedUserName: RADIUS_ATTRIBUTE_TYPE = 268i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFQUserName: RADIUS_ATTRIBUTE_TYPE = 269i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratPolicyName: RADIUS_ATTRIBUTE_TYPE = 270i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratUniqueId: RADIUS_ATTRIBUTE_TYPE = 271i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratExtensionState: RADIUS_ATTRIBUTE_TYPE = 272i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratEAPTLV: RADIUS_ATTRIBUTE_TYPE = 273i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratRejectReasonCode: RADIUS_ATTRIBUTE_TYPE = 274i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCRPPolicyName: RADIUS_ATTRIBUTE_TYPE = 275i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratProviderName: RADIUS_ATTRIBUTE_TYPE = 276i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratClearTextPassword: RADIUS_ATTRIBUTE_TYPE = 277i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSrcIPv6Address: RADIUS_ATTRIBUTE_TYPE = 278i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCertificateThumbprint: RADIUS_ATTRIBUTE_TYPE = 279i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_AUTHENTICATION_PROVIDER = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapUnknown: RADIUS_AUTHENTICATION_PROVIDER = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapUsersFile: RADIUS_AUTHENTICATION_PROVIDER = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapProxy: RADIUS_AUTHENTICATION_PROVIDER = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapWindowsNT: RADIUS_AUTHENTICATION_PROVIDER = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapMCIS: RADIUS_AUTHENTICATION_PROVIDER = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapODBC: RADIUS_AUTHENTICATION_PROVIDER = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapNone: RADIUS_AUTHENTICATION_PROVIDER = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_CODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcUnknown: RADIUS_CODE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessRequest: RADIUS_CODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessAccept: RADIUS_CODE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessReject: RADIUS_CODE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccountingRequest: RADIUS_CODE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccountingResponse: RADIUS_CODE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessChallenge: RADIUS_CODE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcDiscard: RADIUS_CODE = 256i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtUnknown: RADIUS_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtString: RADIUS_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtAddress: RADIUS_DATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtInteger: RADIUS_DATA_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtTime: RADIUS_DATA_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtIpv6Address: RADIUS_DATA_TYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub repPoint: RADIUS_EXTENSION_POINT,
    pub rcRequestType: RADIUS_CODE,
    pub rcResponseType: RADIUS_CODE,
    pub GetRequest: isize,
    pub GetResponse: isize,
    pub SetResponseType: isize,
}
impl ::core::marker::Copy for RADIUS_EXTENSION_CONTROL_BLOCK {}
impl ::core::clone::Clone for RADIUS_EXTENSION_CONTROL_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_FREE_ATTRIBUTES: &str = "RadiusExtensionFreeAttributes";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_INIT: &str = "RadiusExtensionInit";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_EXTENSION_POINT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const repAuthentication: RADIUS_EXTENSION_POINT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const repAuthorization: RADIUS_EXTENSION_POINT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_PROCESS: &str = "RadiusExtensionProcess";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_PROCESS2: &str = "RadiusExtensionProcess2";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_PROCESS_EX: &str = "RadiusExtensionProcessEx";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_TERM: &str = "RadiusExtensionTerm";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type RADIUS_REJECT_REASON_CODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcUndefined: RADIUS_REJECT_REASON_CODE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAccountUnknown: RADIUS_REJECT_REASON_CODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAccountDisabled: RADIUS_REJECT_REASON_CODE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAccountExpired: RADIUS_REJECT_REASON_CODE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAuthenticationFailure: RADIUS_REJECT_REASON_CODE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_VSA_FORMAT {
    pub VendorId: [u8; 4],
    pub VendorType: u8,
    pub VendorLength: u8,
    pub AttributeSpecific: [u8; 1],
}
impl ::core::marker::Copy for RADIUS_VSA_FORMAT {}
impl ::core::clone::Clone for RADIUS_VSA_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type REMEDIATIONSERVERGROUPPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVERGROUP_SERVERS_COLLECTION: REMEDIATIONSERVERGROUPPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type REMEDIATIONSERVERPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVER_ADDRESS: REMEDIATIONSERVERPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVER_FRIENDLY_NAME: REMEDIATIONSERVERPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type REMEDIATIONSERVERSPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVERS_SERVERGROUPS: REMEDIATIONSERVERSPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type SERVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_IAS: SERVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_RAS: SERVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_RAMGMTSVC: SERVICE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_MAX: SERVICE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type SHAREDSECRETPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHAREDSECRET_STRING: SHAREDSECRETPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type SHVTEMPLATEPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHV_COMBINATION_TYPE: SHVTEMPLATEPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHV_LIST: SHVTEMPLATEPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHVCONFIG_LIST: SHVTEMPLATEPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type SHV_COMBINATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ALL_PASS: SHV_COMBINATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ALL_FAIL: SHV_COMBINATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_PASS: SHV_COMBINATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_FAIL: SHV_COMBINATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_INFECTED: SHV_COMBINATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_TRANSITIONAL: SHV_COMBINATION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_UNKNOWN: SHV_COMBINATION_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_MAX: SHV_COMBINATION_TYPE = 7i32;
pub const SdoMachine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3911289575, data2: 40593, data3: 4561, data4: [191, 96, 0, 128, 199, 132, 107, 192] };
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type TEMPLATESPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_POLICIES_TEMPLATES: TEMPLATESPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROFILES_TEMPLATES: TEMPLATESPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROFILES_COLLECTION: TEMPLATESPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROXYPOLICIES_TEMPLATES: TEMPLATESPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROXYPROFILES_TEMPLATES: TEMPLATESPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROXYPROFILES_COLLECTION: TEMPLATESPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_REMEDIATIONSERVERGROUPS_TEMPLATES: TEMPLATESPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_SHVTEMPLATES_TEMPLATES: TEMPLATESPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_CLIENTS_TEMPLATES: TEMPLATESPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_RADIUSSERVERS_TEMPLATES: TEMPLATESPROPERTIES = 1033i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_SHAREDSECRETS_TEMPLATES: TEMPLATESPROPERTIES = 1034i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_IPFILTERS_TEMPLATES: TEMPLATESPROPERTIES = 1035i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type USERPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_CALLING_STATION_ID: USERPROPERTIES = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_CALLING_STATION_ID: USERPROPERTIES = 1025i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_CALLBACK_NUMBER: USERPROPERTIES = 1026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_ROUTE: USERPROPERTIES = 1027i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_IP_ADDRESS: USERPROPERTIES = 1028i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_CALLBACK_NUMBER: USERPROPERTIES = 1029i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_ROUTE: USERPROPERTIES = 1030i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IP_ADDRESS: USERPROPERTIES = 1031i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_ALLOW_DIALIN: USERPROPERTIES = 1032i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SERVICE_TYPE: USERPROPERTIES = 1033i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_IPV6_ROUTE: USERPROPERTIES = 1034i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IPV6_ROUTE: USERPROPERTIES = 1035i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_INTERFACE_ID: USERPROPERTIES = 1036i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_INTERFACE_ID: USERPROPERTIES = 1037i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_IPV6_PREFIX: USERPROPERTIES = 1038i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IPV6_PREFIX: USERPROPERTIES = 1039i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type VENDORPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NAS_VENDOR_ID: VENDORPROPERTIES = 1024i32;
