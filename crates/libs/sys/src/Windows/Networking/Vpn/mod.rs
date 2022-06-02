#[repr(C)]
pub struct IVpnAppId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnAppIdType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: VpnAppIdType) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnAppIdFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, r#type: VpnAppIdType, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannel {
    pub base__: ::windows_sys::core::IInspectable,
    pub AssociateTransport: unsafe extern "system" fn(this: *mut *mut Self, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, routescope: *mut ::core::ffi::c_void, namespacescope: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub RequestCredentials: unsafe extern "system" fn(this: *mut *mut Self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    RequestCredentials: usize,
    pub RequestVpnPacketBuffer: unsafe extern "system" fn(this: *mut *mut Self, r#type: VpnDataPathType, vpnpacketbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LogDiagnosticMessage: unsafe extern "system" fn(this: *mut *mut Self, message: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActivityChange: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActivityChange: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivityChange: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivityChange: usize,
    pub SetPlugInContext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlugInContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemHealth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestCustomPrompt: unsafe extern "system" fn(this: *mut *mut Self, customprompt: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestCustomPrompt: usize,
    pub SetErrorMessage: unsafe extern "system" fn(this: *mut *mut Self, message: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAllowedSslTlsVersions: unsafe extern "system" fn(this: *mut *mut Self, tunneltransport: *mut ::core::ffi::c_void, usetls12: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannel2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithMainTransport: unsafe extern "system" fn(this: *mut *mut Self, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assigneddomainname: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithMainTransport: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartExistingTransports: unsafe extern "system" fn(this: *mut *mut Self, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assigneddomainname: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartExistingTransports: usize,
    #[cfg(feature = "Foundation")]
    pub ActivityStateChange: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActivityStateChange: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivityStateChange: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivityStateChange: usize,
    pub GetVpnSendPacketBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVpnReceivePacketBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestCustomPromptAsync: unsafe extern "system" fn(this: *mut *mut Self, custompromptelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestCustomPromptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub RequestCredentialsWithCertificateAsync: unsafe extern "system" fn(this: *mut *mut Self, credtype: VpnCredentialType, credoptions: u32, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))]
    RequestCredentialsWithCertificateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCredentialsWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, credtype: VpnCredentialType, credoptions: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCredentialsWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCredentialsSimpleAsync: unsafe extern "system" fn(this: *mut *mut Self, credtype: VpnCredentialType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCredentialsSimpleAsync: usize,
    pub TerminateConnection: unsafe extern "system" fn(this: *mut *mut Self, message: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithTrafficFilter: unsafe extern "system" fn(this: *mut *mut Self, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assignednamespace: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void, assignedtrafficfilters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithTrafficFilter: usize,
}
#[repr(C)]
pub struct IVpnChannel4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddAndAssociateTransport: unsafe extern "system" fn(this: *mut *mut Self, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithMultipleTransports: unsafe extern "system" fn(this: *mut *mut Self, assignedclientipv4addresses: *mut ::core::ffi::c_void, assignedclientipv6addresses: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assignednamespace: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool, transports: *mut ::core::ffi::c_void, assignedtrafficfilters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithMultipleTransports: usize,
    pub ReplaceAndAssociateTransport: unsafe extern "system" fn(this: *mut *mut Self, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartReconnectingTransport: unsafe extern "system" fn(this: *mut *mut Self, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub GetSlotTypeForTransportContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut super::Sockets::ControlChannelTriggerStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    GetSlotTypeForTransportContext: usize,
    pub CurrentRequestTransportContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannel5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppendVpnReceivePacketBuffer: unsafe extern "system" fn(this: *mut *mut Self, decapsulatedpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppendVpnSendPacketBuffer: unsafe extern "system" fn(this: *mut *mut Self, encapsulatedpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlushVpnReceivePacketBuffers: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FlushVpnSendPacketBuffers: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannel6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivateForeground: unsafe extern "system" fn(this: *mut *mut Self, packagerelativeappid: ::windows_sys::core::HSTRING, sharedcontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivateForeground: usize,
}
#[repr(C)]
pub struct IVpnChannelActivityEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnChannelActivityEventType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannelActivityStateChangedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnChannelActivityEventType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannelConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServerServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerHostNameList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerHostNameList: usize,
    pub CustomField: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnChannelConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerUris: usize,
}
#[repr(C)]
pub struct IVpnChannelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProcessEventAsync: unsafe extern "system" fn(this: *mut *mut Self, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCredential {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub PasskeyCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasskeyCredential: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub CertificateCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    CertificateCredential: usize,
    pub AdditionalPin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub OldPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OldPasswordCredential: usize,
}
#[repr(C)]
pub struct IVpnCustomCheckBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetInitialCheckState: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InitialCheckState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Checked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomComboBox {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetOptionsText: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetOptionsText: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionsText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionsText: usize,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomEditBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDefaultText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DefaultText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNoEcho: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub NoEcho: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomErrorBox {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVpnCustomPrompt {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCompulsory: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Compulsory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBordered: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Bordered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomPromptBooleanInput {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InitialValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomPromptElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCompulsory: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Compulsory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEmphasized: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Emphasized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomPromptOptionSelector {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomPromptText {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomPromptTextInput {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIsTextHidden: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsTextHidden: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnCustomTextBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnDomainNameAssignment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DomainNameList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DomainNameList: usize,
    #[cfg(feature = "Foundation")]
    pub SetProxyAutoConfigurationUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProxyAutoConfigurationUri: usize,
    #[cfg(feature = "Foundation")]
    pub ProxyAutoConfigurationUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProxyAutoConfigurationUri: usize,
}
#[repr(C)]
pub struct IVpnDomainNameInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDomainName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDomainNameType: unsafe extern "system" fn(this: *mut *mut Self, value: VpnDomainNameType) -> ::windows_sys::core::HRESULT,
    pub DomainNameType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnDomainNameType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsServers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyServers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyServers: usize,
}
#[repr(C)]
pub struct IVpnDomainNameInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyUris: usize,
}
#[repr(C)]
pub struct IVpnDomainNameInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateVpnDomainNameInfo: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: *mut ::core::ffi::c_void, proxyserverlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateVpnDomainNameInfo: usize,
}
#[repr(C)]
pub struct IVpnForegroundActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SharedContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SharedContext: usize,
    pub ActivationOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnForegroundActivationOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self, result: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Complete: usize,
}
#[repr(C)]
pub struct IVpnInterfaceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAddressInfo: unsafe extern "system" fn(this: *mut *mut Self, id_array_size: *mut u32, id: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnInterfaceIdFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateVpnInterfaceId: unsafe extern "system" fn(this: *mut *mut Self, address_array_size: u32, address: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnManagementAgent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddProfileFromXmlAsync: unsafe extern "system" fn(this: *mut *mut Self, xml: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddProfileFromXmlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddProfileFromObjectAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddProfileFromObjectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateProfileFromXmlAsync: unsafe extern "system" fn(this: *mut *mut Self, xml: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateProfileFromXmlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateProfileFromObjectAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateProfileFromObjectAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfilesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfilesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectProfileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectProfileWithPasswordCredentialAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, passwordcredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectProfileWithPasswordCredentialAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectProfileAsync: usize,
}
#[repr(C)]
pub struct IVpnNamespaceAssignment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetNamespaceList: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetNamespaceList: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NamespaceList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NamespaceList: usize,
    #[cfg(feature = "Foundation")]
    pub SetProxyAutoConfigUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProxyAutoConfigUri: usize,
    #[cfg(feature = "Foundation")]
    pub ProxyAutoConfigUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProxyAutoConfigUri: usize,
}
#[repr(C)]
pub struct IVpnNamespaceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetNamespace: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDnsServers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsServers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetWebProxyServers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetWebProxyServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyServers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyServers: usize,
}
#[repr(C)]
pub struct IVpnNamespaceInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateVpnNamespaceInfo: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, dnsserverlist: *mut ::core::ffi::c_void, proxyserverlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateVpnNamespaceInfo: usize,
}
#[repr(C)]
pub struct IVpnNativeProfile {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Servers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Servers: usize,
    pub RoutingPolicyType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnRoutingPolicyType) -> ::windows_sys::core::HRESULT,
    pub SetRoutingPolicyType: unsafe extern "system" fn(this: *mut *mut Self, value: VpnRoutingPolicyType) -> ::windows_sys::core::HRESULT,
    pub NativeProtocolType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnNativeProtocolType) -> ::windows_sys::core::HRESULT,
    pub SetNativeProtocolType: unsafe extern "system" fn(this: *mut *mut Self, value: VpnNativeProtocolType) -> ::windows_sys::core::HRESULT,
    pub UserAuthenticationMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnAuthenticationMethod) -> ::windows_sys::core::HRESULT,
    pub SetUserAuthenticationMethod: unsafe extern "system" fn(this: *mut *mut Self, value: VpnAuthenticationMethod) -> ::windows_sys::core::HRESULT,
    pub TunnelAuthenticationMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnAuthenticationMethod) -> ::windows_sys::core::HRESULT,
    pub SetTunnelAuthenticationMethod: unsafe extern "system" fn(this: *mut *mut Self, value: VpnAuthenticationMethod) -> ::windows_sys::core::HRESULT,
    pub EapConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEapConfiguration: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnNativeProfile2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequireVpnClientAppUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequireVpnClientAppUI: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnManagementConnectionStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPacketBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: VpnPacketBufferStatus) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnPacketBufferStatus) -> ::windows_sys::core::HRESULT,
    pub SetTransportAffinity: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub TransportAffinity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPacketBuffer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPacketBuffer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTransportContext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransportContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPacketBufferFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateVpnPacketBuffer: unsafe extern "system" fn(this: *mut *mut Self, parentbuffer: *mut ::core::ffi::c_void, offset: u32, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPacketBufferList {
    pub base__: ::windows_sys::core::IInspectable,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddAtBegin: unsafe extern "system" fn(this: *mut *mut Self, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAtBegin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: VpnPacketBufferStatus) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnPacketBufferStatus) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPacketBufferList2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddLeadingPacket: unsafe extern "system" fn(this: *mut *mut Self, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveLeadingPacket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddTrailingPacket: unsafe extern "system" fn(this: *mut *mut Self, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveTrailingPacket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPickedCredential {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub PasskeyCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasskeyCredential: usize,
    pub AdditionalPin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub OldPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OldPasswordCredential: usize,
}
#[repr(C)]
pub struct IVpnPlugIn {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetKeepAlivePayload: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void, keepalivepacket: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Encapsulate: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void, packets: *mut ::core::ffi::c_void, encapulatedpackets: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Decapsulate: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void, encapbuffer: *mut ::core::ffi::c_void, decapsulatedpackets: *mut ::core::ffi::c_void, controlpacketstosend: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPlugInProfile {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerUris: usize,
    pub CustomConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCustomConfiguration: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VpnPluginPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetVpnPluginPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnPlugInProfile2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequireVpnClientAppUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequireVpnClientAppUI: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnManagementConnectionStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnProfile {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppTriggers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppTriggers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Routes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Routes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DomainNameInfoList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DomainNameInfoList: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TrafficFilters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrafficFilters: usize,
    pub RememberCredentials: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRememberCredentials: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlwaysOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysOn: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnRoute {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrefixSize: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub PrefixSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnRouteAssignment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv4InclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv4InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv6InclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv6InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv4InclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv4InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv6InclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv6InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv4ExclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv4ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv6ExclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv6ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv4ExclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv4ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv6ExclusionRoutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv6ExclusionRoutes: usize,
    pub SetExcludeLocalSubnets: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ExcludeLocalSubnets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnRouteFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateVpnRoute: unsafe extern "system" fn(this: *mut *mut Self, address: *mut ::core::ffi::c_void, prefixsize: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnSystemHealth {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub StatementOfHealth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StatementOfHealth: usize,
}
#[repr(C)]
pub struct IVpnTrafficFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAppId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppClaims: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppClaims: usize,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnIPProtocol) -> ::windows_sys::core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut *mut Self, value: VpnIPProtocol) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalPortRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalPortRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemotePortRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemotePortRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalAddressRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalAddressRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteAddressRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteAddressRanges: usize,
    pub RoutingPolicyType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VpnRoutingPolicyType) -> ::windows_sys::core::HRESULT,
    pub SetRoutingPolicyType: unsafe extern "system" fn(this: *mut *mut Self, value: VpnRoutingPolicyType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnTrafficFilterAssignment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub TrafficFilterList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrafficFilterList: usize,
    pub AllowOutbound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowOutbound: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowInbound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowInbound: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVpnTrafficFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, appid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type VpnAppId = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnAppIdType(pub i32);
impl VpnAppIdType {
    pub const PackageFamilyName: Self = Self(0i32);
    pub const FullyQualifiedBinaryName: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
}
impl ::core::marker::Copy for VpnAppIdType {}
impl ::core::clone::Clone for VpnAppIdType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnAuthenticationMethod(pub i32);
impl VpnAuthenticationMethod {
    pub const Mschapv2: Self = Self(0i32);
    pub const Eap: Self = Self(1i32);
    pub const Certificate: Self = Self(2i32);
    pub const PresharedKey: Self = Self(3i32);
}
impl ::core::marker::Copy for VpnAuthenticationMethod {}
impl ::core::clone::Clone for VpnAuthenticationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnChannel = *mut ::core::ffi::c_void;
pub type VpnChannelActivityEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnChannelActivityEventType(pub i32);
impl VpnChannelActivityEventType {
    pub const Idle: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnChannelActivityEventType {}
impl ::core::clone::Clone for VpnChannelActivityEventType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnChannelActivityStateChangedArgs = *mut ::core::ffi::c_void;
pub type VpnChannelConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnChannelRequestCredentialsOptions(pub u32);
impl VpnChannelRequestCredentialsOptions {
    pub const None: Self = Self(0u32);
    pub const Retrying: Self = Self(1u32);
    pub const UseForSingleSignIn: Self = Self(2u32);
}
impl ::core::marker::Copy for VpnChannelRequestCredentialsOptions {}
impl ::core::clone::Clone for VpnChannelRequestCredentialsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnCredential = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCredentialType(pub i32);
impl VpnCredentialType {
    pub const UsernamePassword: Self = Self(0i32);
    pub const UsernameOtpPin: Self = Self(1i32);
    pub const UsernamePasswordAndPin: Self = Self(2i32);
    pub const UsernamePasswordChange: Self = Self(3i32);
    pub const SmartCard: Self = Self(4i32);
    pub const ProtectedCertificate: Self = Self(5i32);
    pub const UnProtectedCertificate: Self = Self(6i32);
}
impl ::core::marker::Copy for VpnCredentialType {}
impl ::core::clone::Clone for VpnCredentialType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnCustomCheckBox = *mut ::core::ffi::c_void;
pub type VpnCustomComboBox = *mut ::core::ffi::c_void;
pub type VpnCustomEditBox = *mut ::core::ffi::c_void;
pub type VpnCustomErrorBox = *mut ::core::ffi::c_void;
pub type VpnCustomPromptBooleanInput = *mut ::core::ffi::c_void;
pub type VpnCustomPromptOptionSelector = *mut ::core::ffi::c_void;
pub type VpnCustomPromptText = *mut ::core::ffi::c_void;
pub type VpnCustomPromptTextInput = *mut ::core::ffi::c_void;
pub type VpnCustomTextBox = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnDataPathType(pub i32);
impl VpnDataPathType {
    pub const Send: Self = Self(0i32);
    pub const Receive: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnDataPathType {}
impl ::core::clone::Clone for VpnDataPathType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnDomainNameAssignment = *mut ::core::ffi::c_void;
pub type VpnDomainNameInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnDomainNameType(pub i32);
impl VpnDomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
    pub const Reserved: Self = Self(65535i32);
}
impl ::core::marker::Copy for VpnDomainNameType {}
impl ::core::clone::Clone for VpnDomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnForegroundActivatedEventArgs = *mut ::core::ffi::c_void;
pub type VpnForegroundActivationOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnIPProtocol(pub i32);
impl VpnIPProtocol {
    pub const None: Self = Self(0i32);
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
    pub const Icmp: Self = Self(1i32);
    pub const Ipv6Icmp: Self = Self(58i32);
    pub const Igmp: Self = Self(2i32);
    pub const Pgm: Self = Self(113i32);
}
impl ::core::marker::Copy for VpnIPProtocol {}
impl ::core::clone::Clone for VpnIPProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnInterfaceId = *mut ::core::ffi::c_void;
pub type VpnManagementAgent = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnManagementConnectionStatus(pub i32);
impl VpnManagementConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Disconnecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Connecting: Self = Self(3i32);
}
impl ::core::marker::Copy for VpnManagementConnectionStatus {}
impl ::core::clone::Clone for VpnManagementConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnManagementErrorStatus(pub i32);
impl VpnManagementErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Other: Self = Self(1i32);
    pub const InvalidXmlSyntax: Self = Self(2i32);
    pub const ProfileNameTooLong: Self = Self(3i32);
    pub const ProfileInvalidAppId: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
    pub const CannotFindProfile: Self = Self(6i32);
    pub const AlreadyDisconnecting: Self = Self(7i32);
    pub const AlreadyConnected: Self = Self(8i32);
    pub const GeneralAuthenticationFailure: Self = Self(9i32);
    pub const EapFailure: Self = Self(10i32);
    pub const SmartCardFailure: Self = Self(11i32);
    pub const CertificateFailure: Self = Self(12i32);
    pub const ServerConfiguration: Self = Self(13i32);
    pub const NoConnection: Self = Self(14i32);
    pub const ServerConnection: Self = Self(15i32);
    pub const UserNamePassword: Self = Self(16i32);
    pub const DnsNotResolvable: Self = Self(17i32);
    pub const InvalidIP: Self = Self(18i32);
}
impl ::core::marker::Copy for VpnManagementErrorStatus {}
impl ::core::clone::Clone for VpnManagementErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnNamespaceAssignment = *mut ::core::ffi::c_void;
pub type VpnNamespaceInfo = *mut ::core::ffi::c_void;
pub type VpnNativeProfile = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnNativeProtocolType(pub i32);
impl VpnNativeProtocolType {
    pub const Pptp: Self = Self(0i32);
    pub const L2tp: Self = Self(1i32);
    pub const IpsecIkev2: Self = Self(2i32);
}
impl ::core::marker::Copy for VpnNativeProtocolType {}
impl ::core::clone::Clone for VpnNativeProtocolType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnPacketBuffer = *mut ::core::ffi::c_void;
pub type VpnPacketBufferList = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnPacketBufferStatus(pub i32);
impl VpnPacketBufferStatus {
    pub const Ok: Self = Self(0i32);
    pub const InvalidBufferSize: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnPacketBufferStatus {}
impl ::core::clone::Clone for VpnPacketBufferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnPickedCredential = *mut ::core::ffi::c_void;
pub type VpnPlugInProfile = *mut ::core::ffi::c_void;
pub type VpnRoute = *mut ::core::ffi::c_void;
pub type VpnRouteAssignment = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnRoutingPolicyType(pub i32);
impl VpnRoutingPolicyType {
    pub const SplitRouting: Self = Self(0i32);
    pub const ForceAllTrafficOverVpn: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnRoutingPolicyType {}
impl ::core::clone::Clone for VpnRoutingPolicyType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VpnSystemHealth = *mut ::core::ffi::c_void;
pub type VpnTrafficFilter = *mut ::core::ffi::c_void;
pub type VpnTrafficFilterAssignment = *mut ::core::ffi::c_void;
