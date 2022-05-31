#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "win32-system-sys")]
    pub fn AcceptEx(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    pub fn EnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
    pub fn EnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
    pub fn FreeAddrInfoEx(paddrinfoex: *const addrinfoexA);
    pub fn FreeAddrInfoExW(paddrinfoex: *const addrinfoexW);
    pub fn FreeAddrInfoW(paddrinfo: *const addrinfoW);
    pub fn GetAcceptExSockaddrs(lpoutputbuffer: *const ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32);
    #[cfg(feature = "win32-system-sys")]
    pub fn GetAddrInfoExA(pname: ::windows_core_sys::PCSTR, pservicename: ::windows_core_sys::PCSTR, dwnamespace: u32, lpnspid: *const ::windows_core_sys::GUID, hints: *const addrinfoexA, ppresult: *mut *mut addrinfoexA, timeout: *const timeval, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: *mut ::win32_foundation_sys::HANDLE) -> i32;
    pub fn GetAddrInfoExCancel(lphandle: *const ::win32_foundation_sys::HANDLE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetAddrInfoExOverlappedResult(lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetAddrInfoExW(pname: ::windows_core_sys::PCWSTR, pservicename: ::windows_core_sys::PCWSTR, dwnamespace: u32, lpnspid: *const ::windows_core_sys::GUID, hints: *const addrinfoexW, ppresult: *mut *mut addrinfoexW, timeout: *const timeval, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle: *mut ::win32_foundation_sys::HANDLE) -> i32;
    pub fn GetAddrInfoW(pnodename: ::windows_core_sys::PCWSTR, pservicename: ::windows_core_sys::PCWSTR, phints: *const addrinfoW, ppresult: *mut *mut addrinfoW) -> i32;
    pub fn GetAddressByNameA(dwnamespace: u32, lpservicetype: *const ::windows_core_sys::GUID, lpservicename: ::windows_core_sys::PCSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpcsaddrbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: ::windows_core_sys::PSTR, lpdwaliasbufferlength: *mut u32) -> i32;
    pub fn GetAddressByNameW(dwnamespace: u32, lpservicetype: *const ::windows_core_sys::GUID, lpservicename: ::windows_core_sys::PCWSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpcsaddrbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: ::windows_core_sys::PWSTR, lpdwaliasbufferlength: *mut u32) -> i32;
    pub fn GetHostNameW(name: ::windows_core_sys::PWSTR, namelen: i32) -> i32;
    pub fn GetNameByTypeA(lpservicetype: *const ::windows_core_sys::GUID, lpservicename: ::windows_core_sys::PSTR, dwnamelength: u32) -> i32;
    pub fn GetNameByTypeW(lpservicetype: *const ::windows_core_sys::GUID, lpservicename: ::windows_core_sys::PWSTR, dwnamelength: u32) -> i32;
    pub fn GetNameInfoW(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: ::windows_core_sys::PWSTR, nodebuffersize: u32, pservicebuffer: ::windows_core_sys::PWSTR, servicebuffersize: u32, flags: i32) -> i32;
    pub fn GetServiceA(dwnamespace: u32, lpguid: *const ::windows_core_sys::GUID, lpservicename: ::windows_core_sys::PCSTR, dwproperties: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO) -> i32;
    pub fn GetServiceW(dwnamespace: u32, lpguid: *const ::windows_core_sys::GUID, lpservicename: ::windows_core_sys::PCWSTR, dwproperties: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO) -> i32;
    pub fn GetTypeByNameA(lpservicename: ::windows_core_sys::PCSTR, lpservicetype: *mut ::windows_core_sys::GUID) -> i32;
    pub fn GetTypeByNameW(lpservicename: ::windows_core_sys::PCWSTR, lpservicetype: *mut ::windows_core_sys::GUID) -> i32;
    pub fn InetNtopW(family: i32, paddr: *const ::core::ffi::c_void, pstringbuf: ::windows_core_sys::PWSTR, stringbufsize: usize) -> ::windows_core_sys::PWSTR;
    pub fn InetPtonW(family: i32, pszaddrstring: ::windows_core_sys::PCWSTR, paddrbuf: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn ProcessSocketNotifications(completionport: ::win32_foundation_sys::HANDLE, registrationcount: u32, registrationinfos: *mut SOCK_NOTIFY_REGISTRATION, timeoutms: u32, completioncount: u32, completionportentries: *mut ::win32_system_sys::IO::OVERLAPPED_ENTRY, receivedentrycount: *mut u32) -> u32;
    pub fn RtlEthernetAddressToStringA(addr: *const DL_EUI48, s: ::windows_core_sys::PSTR) -> ::windows_core_sys::PSTR;
    pub fn RtlEthernetAddressToStringW(addr: *const DL_EUI48, s: ::windows_core_sys::PWSTR) -> ::windows_core_sys::PWSTR;
    pub fn RtlEthernetStringToAddressA(s: ::windows_core_sys::PCSTR, terminator: *mut ::windows_core_sys::PSTR, addr: *mut DL_EUI48) -> i32;
    pub fn RtlEthernetStringToAddressW(s: ::windows_core_sys::PCWSTR, terminator: *mut ::windows_core_sys::PWSTR, addr: *mut DL_EUI48) -> i32;
    pub fn RtlIpv4AddressToStringA(addr: *const IN_ADDR, s: ::windows_core_sys::PSTR) -> ::windows_core_sys::PSTR;
    pub fn RtlIpv4AddressToStringExA(address: *const IN_ADDR, port: u16, addressstring: ::windows_core_sys::PSTR, addressstringlength: *mut u32) -> i32;
    pub fn RtlIpv4AddressToStringExW(address: *const IN_ADDR, port: u16, addressstring: ::windows_core_sys::PWSTR, addressstringlength: *mut u32) -> i32;
    pub fn RtlIpv4AddressToStringW(addr: *const IN_ADDR, s: ::windows_core_sys::PWSTR) -> ::windows_core_sys::PWSTR;
    pub fn RtlIpv4StringToAddressA(s: ::windows_core_sys::PCSTR, strict: ::win32_foundation_sys::BOOLEAN, terminator: *mut ::windows_core_sys::PSTR, addr: *mut IN_ADDR) -> i32;
    pub fn RtlIpv4StringToAddressExA(addressstring: ::windows_core_sys::PCSTR, strict: ::win32_foundation_sys::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
    pub fn RtlIpv4StringToAddressExW(addressstring: ::windows_core_sys::PCWSTR, strict: ::win32_foundation_sys::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
    pub fn RtlIpv4StringToAddressW(s: ::windows_core_sys::PCWSTR, strict: ::win32_foundation_sys::BOOLEAN, terminator: *mut ::windows_core_sys::PWSTR, addr: *mut IN_ADDR) -> i32;
    pub fn RtlIpv6AddressToStringA(addr: *const IN6_ADDR, s: ::windows_core_sys::PSTR) -> ::windows_core_sys::PSTR;
    pub fn RtlIpv6AddressToStringExA(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: ::windows_core_sys::PSTR, addressstringlength: *mut u32) -> i32;
    pub fn RtlIpv6AddressToStringExW(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: ::windows_core_sys::PWSTR, addressstringlength: *mut u32) -> i32;
    pub fn RtlIpv6AddressToStringW(addr: *const IN6_ADDR, s: ::windows_core_sys::PWSTR) -> ::windows_core_sys::PWSTR;
    pub fn RtlIpv6StringToAddressA(s: ::windows_core_sys::PCSTR, terminator: *mut ::windows_core_sys::PSTR, addr: *mut IN6_ADDR) -> i32;
    pub fn RtlIpv6StringToAddressExA(addressstring: ::windows_core_sys::PCSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
    pub fn RtlIpv6StringToAddressExW(addressstring: ::windows_core_sys::PCWSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
    pub fn RtlIpv6StringToAddressW(s: ::windows_core_sys::PCWSTR, terminator: *mut ::windows_core_sys::PWSTR, addr: *mut IN6_ADDR) -> i32;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn SetAddrInfoExA(pname: ::windows_core_sys::PCSTR, pservicename: ::windows_core_sys::PCSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const ::win32_system_sys::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows_core_sys::GUID, timeout: *const timeval, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: *mut ::win32_foundation_sys::HANDLE) -> i32;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn SetAddrInfoExW(pname: ::windows_core_sys::PCWSTR, pservicename: ::windows_core_sys::PCWSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const ::win32_system_sys::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows_core_sys::GUID, timeout: *const timeval, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: *mut ::win32_foundation_sys::HANDLE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetServiceA(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOA, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpdwstatusflags: *mut u32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetServiceW(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOW, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpdwstatusflags: *mut u32) -> i32;
    pub fn SetSocketMediaStreamingMode(value: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn TransmitFile(hsocket: SOCKET, hfile: ::win32_foundation_sys::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn WPUCompleteOverlappedRequest(s: SOCKET, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32;
    pub fn WSAAccept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: LPCONDITIONPROC, dwcallbackdata: usize) -> SOCKET;
    pub fn WSAAddressToStringA(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpszaddressstring: ::windows_core_sys::PSTR, lpdwaddressstringlength: *mut u32) -> i32;
    pub fn WSAAddressToStringW(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: ::windows_core_sys::PWSTR, lpdwaddressstringlength: *mut u32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAAdvertiseProvider(puuidproviderid: *const ::windows_core_sys::GUID, pnspv2routine: *const NSPV2_ROUTINE) -> i32;
    pub fn WSAAsyncGetHostByAddr(hwnd: ::win32_foundation_sys::HWND, wmsg: u32, addr: ::windows_core_sys::PCSTR, len: i32, r#type: i32, buf: ::windows_core_sys::PSTR, buflen: i32) -> ::win32_foundation_sys::HANDLE;
    pub fn WSAAsyncGetHostByName(hwnd: ::win32_foundation_sys::HWND, wmsg: u32, name: ::windows_core_sys::PCSTR, buf: ::windows_core_sys::PSTR, buflen: i32) -> ::win32_foundation_sys::HANDLE;
    pub fn WSAAsyncGetProtoByName(hwnd: ::win32_foundation_sys::HWND, wmsg: u32, name: ::windows_core_sys::PCSTR, buf: ::windows_core_sys::PSTR, buflen: i32) -> ::win32_foundation_sys::HANDLE;
    pub fn WSAAsyncGetProtoByNumber(hwnd: ::win32_foundation_sys::HWND, wmsg: u32, number: i32, buf: ::windows_core_sys::PSTR, buflen: i32) -> ::win32_foundation_sys::HANDLE;
    pub fn WSAAsyncGetServByName(hwnd: ::win32_foundation_sys::HWND, wmsg: u32, name: ::windows_core_sys::PCSTR, proto: ::windows_core_sys::PCSTR, buf: ::windows_core_sys::PSTR, buflen: i32) -> ::win32_foundation_sys::HANDLE;
    pub fn WSAAsyncGetServByPort(hwnd: ::win32_foundation_sys::HWND, wmsg: u32, port: i32, proto: ::windows_core_sys::PCSTR, buf: ::windows_core_sys::PSTR, buflen: i32) -> ::win32_foundation_sys::HANDLE;
    pub fn WSAAsyncSelect(s: SOCKET, hwnd: ::win32_foundation_sys::HWND, wmsg: u32, levent: i32) -> i32;
    pub fn WSACancelAsyncRequest(hasynctaskhandle: ::win32_foundation_sys::HANDLE) -> i32;
    pub fn WSACancelBlockingCall() -> i32;
    pub fn WSACleanup() -> i32;
    pub fn WSACloseEvent(hevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn WSAConnect(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const QOS, lpgqos: *const QOS) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAConnectByList(s: SOCKET, socketaddress: *const SOCKET_ADDRESS_LIST, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAConnectByNameA(s: SOCKET, nodename: ::windows_core_sys::PCSTR, servicename: ::windows_core_sys::PCSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAConnectByNameW(s: SOCKET, nodename: ::windows_core_sys::PCWSTR, servicename: ::windows_core_sys::PCWSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    pub fn WSACreateEvent() -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSADeleteSocketPeerTargetName(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32, overlapped: *const ::win32_system_sys::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSADuplicateSocketA(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOA) -> i32;
    pub fn WSADuplicateSocketW(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW) -> i32;
    pub fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOA) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXA) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
    pub fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
    pub fn WSAEnumNetworkEvents(s: SOCKET, heventobject: ::win32_foundation_sys::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS) -> i32;
    pub fn WSAEnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOA, lpdwbufferlength: *mut u32) -> i32;
    pub fn WSAEnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32) -> i32;
    pub fn WSAEventSelect(s: SOCKET, heventobject: ::win32_foundation_sys::HANDLE, lnetworkevents: i32) -> i32;
    pub fn WSAGetLastError() -> WSA_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAGetOverlappedResult(s: SOCKET, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: ::win32_foundation_sys::BOOL, lpdwflags: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn WSAGetQOSByName(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut QOS) -> ::win32_foundation_sys::BOOL;
    pub fn WSAGetServiceClassInfoA(lpproviderid: *const ::windows_core_sys::GUID, lpserviceclassid: *const ::windows_core_sys::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOA) -> i32;
    pub fn WSAGetServiceClassInfoW(lpproviderid: *const ::windows_core_sys::GUID, lpserviceclassid: *const ::windows_core_sys::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOW) -> i32;
    pub fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: *const ::windows_core_sys::GUID, lpszserviceclassname: ::windows_core_sys::PSTR, lpdwbufferlength: *mut u32) -> i32;
    pub fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: *const ::windows_core_sys::GUID, lpszserviceclassname: ::windows_core_sys::PWSTR, lpdwbufferlength: *mut u32) -> i32;
    pub fn WSAHtonl(s: SOCKET, hostlong: u32, lpnetlong: *mut u32) -> i32;
    pub fn WSAHtons(s: SOCKET, hostshort: u16, lpnetshort: *mut u16) -> i32;
    pub fn WSAImpersonateSocketPeer(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32) -> i32;
    pub fn WSAInstallServiceClassA(lpserviceclassinfo: *const WSASERVICECLASSINFOA) -> i32;
    pub fn WSAInstallServiceClassW(lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAIoctl(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSAIsBlocking() -> ::win32_foundation_sys::BOOL;
    pub fn WSAJoinLeaf(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const QOS, lpgqos: *const QOS, dwflags: u32) -> SOCKET;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSALookupServiceBeginA(lpqsrestrictions: *const WSAQUERYSETA, dwcontrolflags: u32, lphlookup: *mut ::win32_foundation_sys::HANDLE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSALookupServiceBeginW(lpqsrestrictions: *const WSAQUERYSETW, dwcontrolflags: u32, lphlookup: *mut ::win32_foundation_sys::HANDLE) -> i32;
    pub fn WSALookupServiceEnd(hlookup: ::win32_foundation_sys::HANDLE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSALookupServiceNextA(hlookup: ::win32_foundation_sys::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETA) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSALookupServiceNextW(hlookup: ::win32_foundation_sys::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSANSPIoctl(hlookup: ::win32_foundation_sys::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const WSACOMPLETION) -> i32;
    pub fn WSANtohl(s: SOCKET, netlong: u32, lphostlong: *mut u32) -> i32;
    pub fn WSANtohs(s: SOCKET, netshort: u16, lphostshort: *mut u16) -> i32;
    pub fn WSAPoll(fdarray: *mut WSAPOLLFD, fds: u32, timeout: i32) -> i32;
    pub fn WSAProviderCompleteAsyncCall(hasynccall: ::win32_foundation_sys::HANDLE, iretcode: i32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAProviderConfigChange(lpnotificationhandle: *mut ::win32_foundation_sys::HANDLE, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSAQuerySocketSecurity(socket: SOCKET, securityquerytemplate: *const SOCKET_SECURITY_QUERY_TEMPLATE, securityquerytemplatelen: u32, securityqueryinfo: *mut SOCKET_SECURITY_QUERY_INFO, securityqueryinfolen: *mut u32, overlapped: *const ::win32_system_sys::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSARecv(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSARecvDisconnect(s: SOCKET, lpinbounddisconnectdata: *const WSABUF) -> i32;
    pub fn WSARecvEx(s: SOCKET, buf: ::windows_core_sys::PSTR, len: i32, flags: *mut i32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSARecvFrom(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSARemoveServiceClass(lpserviceclassid: *const ::windows_core_sys::GUID) -> i32;
    pub fn WSAResetEvent(hevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn WSARevertImpersonation() -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASend(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSASendDisconnect(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASendMsg(handle: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASendTo(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSASetBlockingHook(lpblockfunc: ::win32_foundation_sys::FARPROC) -> ::win32_foundation_sys::FARPROC;
    pub fn WSASetEvent(hevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn WSASetLastError(ierror: i32);
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASetServiceA(lpqsreginfo: *const WSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASetServiceW(lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASetSocketPeerTargetName(socket: SOCKET, peertargetname: *const SOCKET_PEER_TARGET_NAME, peertargetnamelen: u32, overlapped: *const ::win32_system_sys::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSASetSocketSecurity(socket: SOCKET, securitysettings: *const SOCKET_SECURITY_SETTINGS, securitysettingslen: u32, overlapped: *const ::win32_system_sys::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, g: u32, dwflags: u32) -> SOCKET;
    pub fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32) -> SOCKET;
    pub fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSAData) -> i32;
    pub fn WSAStringToAddressA(addressstring: ::windows_core_sys::PCSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
    pub fn WSAStringToAddressW(addressstring: ::windows_core_sys::PCWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
    pub fn WSAUnadvertiseProvider(puuidproviderid: *const ::windows_core_sys::GUID) -> i32;
    pub fn WSAUnhookBlockingHook() -> i32;
    pub fn WSAWaitForMultipleEvents(cevents: u32, lphevents: *const ::win32_foundation_sys::HANDLE, fwaitall: ::win32_foundation_sys::BOOL, dwtimeout: u32, falertable: ::win32_foundation_sys::BOOL) -> u32;
    pub fn WSCDeinstallProvider(lpproviderid: *const ::windows_core_sys::GUID, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCDeinstallProvider32(lpproviderid: *const ::windows_core_sys::GUID, lperrno: *mut i32) -> i32;
    pub fn WSCEnableNSProvider(lpproviderid: *const ::windows_core_sys::GUID, fenable: ::win32_foundation_sys::BOOL) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCEnableNSProvider32(lpproviderid: *const ::windows_core_sys::GUID, fenable: ::win32_foundation_sys::BOOL) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCEnumNameSpaceProviders32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[cfg(feature = "win32-system-sys")]
    pub fn WSCEnumNameSpaceProvidersEx32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
    pub fn WSCEnumProtocols(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCEnumProtocols32(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
    pub fn WSCGetApplicationCategory(path: ::windows_core_sys::PCWSTR, pathlength: u32, extra: ::windows_core_sys::PCWSTR, extralength: u32, ppermittedlspcategories: *mut u32, lperrno: *mut i32) -> i32;
    pub fn WSCGetProviderInfo(lpproviderid: *const ::windows_core_sys::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCGetProviderInfo32(lpproviderid: *const ::windows_core_sys::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
    pub fn WSCGetProviderPath(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCGetProviderPath32(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
    pub fn WSCInstallNameSpace(lpszidentifier: ::windows_core_sys::PCWSTR, lpszpathname: ::windows_core_sys::PCWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_core_sys::GUID) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCInstallNameSpace32(lpszidentifier: ::windows_core_sys::PCWSTR, lpszpathname: ::windows_core_sys::PCWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_core_sys::GUID) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn WSCInstallNameSpaceEx(lpszidentifier: ::windows_core_sys::PCWSTR, lpszpathname: ::windows_core_sys::PCWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_core_sys::GUID, lpproviderspecific: *const ::win32_system_sys::Com::BLOB) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    #[cfg(feature = "win32-system-sys")]
    pub fn WSCInstallNameSpaceEx32(lpszidentifier: ::windows_core_sys::PCWSTR, lpszpathname: ::windows_core_sys::PCWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_core_sys::GUID, lpproviderspecific: *const ::win32_system_sys::Com::BLOB) -> i32;
    pub fn WSCInstallProvider(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCInstallProvider64_32(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCInstallProviderAndChains64_32(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpszproviderdllpath32: ::windows_core_sys::PCWSTR, lpszlspname: ::windows_core_sys::PCWSTR, dwserviceflags: u32, lpprotocolinfolist: *mut WSAPROTOCOL_INFOW, dwnumberofentries: u32, lpdwcatalogentryid: *mut u32, lperrno: *mut i32) -> i32;
    pub fn WSCSetApplicationCategory(path: ::windows_core_sys::PCWSTR, pathlength: u32, extra: ::windows_core_sys::PCWSTR, extralength: u32, permittedlspcategories: u32, pprevpermlspcat: *mut u32, lperrno: *mut i32) -> i32;
    pub fn WSCSetProviderInfo(lpproviderid: *const ::windows_core_sys::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCSetProviderInfo32(lpproviderid: *const ::windows_core_sys::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
    pub fn WSCUnInstallNameSpace(lpproviderid: *const ::windows_core_sys::GUID) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCUnInstallNameSpace32(lpproviderid: *const ::windows_core_sys::GUID) -> i32;
    pub fn WSCUpdateProvider(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCUpdateProvider32(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    pub fn WSCWriteNameSpaceOrder(lpproviderid: *mut ::windows_core_sys::GUID, dwnumberofentries: u32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCWriteNameSpaceOrder32(lpproviderid: *mut ::windows_core_sys::GUID, dwnumberofentries: u32) -> i32;
    pub fn WSCWriteProviderOrder(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn WSCWriteProviderOrder32(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
    pub fn __WSAFDIsSet(fd: SOCKET, param1: *mut fd_set) -> i32;
    pub fn accept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32) -> SOCKET;
    pub fn bind(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    pub fn closesocket(s: SOCKET) -> i32;
    pub fn connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    pub fn freeaddrinfo(paddrinfo: *const ADDRINFOA);
    pub fn getaddrinfo(pnodename: ::windows_core_sys::PCSTR, pservicename: ::windows_core_sys::PCSTR, phints: *const ADDRINFOA, ppresult: *mut *mut ADDRINFOA) -> i32;
    pub fn gethostbyaddr(addr: ::windows_core_sys::PCSTR, len: i32, r#type: i32) -> *mut hostent;
    pub fn gethostbyname(name: ::windows_core_sys::PCSTR) -> *mut hostent;
    pub fn gethostname(name: ::windows_core_sys::PSTR, namelen: i32) -> i32;
    pub fn getnameinfo(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: ::windows_core_sys::PSTR, nodebuffersize: u32, pservicebuffer: ::windows_core_sys::PSTR, servicebuffersize: u32, flags: i32) -> i32;
    pub fn getpeername(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
    pub fn getprotobyname(name: ::windows_core_sys::PCSTR) -> *mut protoent;
    pub fn getprotobynumber(number: i32) -> *mut protoent;
    pub fn getservbyname(name: ::windows_core_sys::PCSTR, proto: ::windows_core_sys::PCSTR) -> *mut servent;
    pub fn getservbyport(port: i32, proto: ::windows_core_sys::PCSTR) -> *mut servent;
    pub fn getsockname(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
    pub fn getsockopt(s: SOCKET, level: i32, optname: i32, optval: ::windows_core_sys::PSTR, optlen: *mut i32) -> i32;
    pub fn htonl(hostlong: u32) -> u32;
    pub fn htons(hostshort: u16) -> u16;
    pub fn inet_addr(cp: ::windows_core_sys::PCSTR) -> u32;
    pub fn inet_ntoa(r#in: IN_ADDR) -> ::windows_core_sys::PSTR;
    pub fn inet_ntop(family: i32, paddr: *const ::core::ffi::c_void, pstringbuf: ::windows_core_sys::PSTR, stringbufsize: usize) -> ::windows_core_sys::PSTR;
    pub fn inet_pton(family: i32, pszaddrstring: ::windows_core_sys::PCSTR, paddrbuf: *mut ::core::ffi::c_void) -> i32;
    pub fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u32) -> i32;
    pub fn listen(s: SOCKET, backlog: i32) -> i32;
    pub fn ntohl(netlong: u32) -> u32;
    pub fn ntohs(netshort: u16) -> u16;
    pub fn recv(s: SOCKET, buf: ::windows_core_sys::PSTR, len: i32, flags: SEND_RECV_FLAGS) -> i32;
    pub fn recvfrom(s: SOCKET, buf: ::windows_core_sys::PSTR, len: i32, flags: i32, from: *mut SOCKADDR, fromlen: *mut i32) -> i32;
    pub fn select(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval) -> i32;
    pub fn send(s: SOCKET, buf: ::windows_core_sys::PCSTR, len: i32, flags: SEND_RECV_FLAGS) -> i32;
    pub fn sendto(s: SOCKET, buf: ::windows_core_sys::PCSTR, len: i32, flags: i32, to: *const SOCKADDR, tolen: i32) -> i32;
    pub fn setsockopt(s: SOCKET, level: i32, optname: i32, optval: ::windows_core_sys::PCSTR, optlen: i32) -> i32;
    pub fn shutdown(s: SOCKET, how: i32) -> i32;
    pub fn socket(af: i32, r#type: i32, protocol: i32) -> SOCKET;
}
pub const AAL5_MODE_MESSAGE: u32 = 1u32;
pub const AAL5_MODE_STREAMING: u32 = 2u32;
#[repr(C)]
pub struct AAL5_PARAMETERS {
    pub ForwardMaxCPCSSDUSize: u32,
    pub BackwardMaxCPCSSDUSize: u32,
    pub Mode: u8,
    pub SSCSType: u8,
}
impl ::core::marker::Copy for AAL5_PARAMETERS {}
impl ::core::clone::Clone for AAL5_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AAL5_SSCS_FRAME_RELAY: u32 = 4u32;
pub const AAL5_SSCS_NULL: u32 = 0u32;
pub const AAL5_SSCS_SSCOP_ASSURED: u32 = 1u32;
pub const AAL5_SSCS_SSCOP_NON_ASSURED: u32 = 2u32;
#[repr(C)]
pub struct AALUSER_PARAMETERS {
    pub UserDefined: u32,
}
impl ::core::marker::Copy for AALUSER_PARAMETERS {}
impl ::core::clone::Clone for AALUSER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AAL_PARAMETERS_IE {
    pub AALType: AAL_TYPE,
    pub AALSpecificParameters: AAL_PARAMETERS_IE_0,
}
impl ::core::marker::Copy for AAL_PARAMETERS_IE {}
impl ::core::clone::Clone for AAL_PARAMETERS_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union AAL_PARAMETERS_IE_0 {
    pub AAL5Parameters: AAL5_PARAMETERS,
    pub AALUserParameters: AALUSER_PARAMETERS,
}
impl ::core::marker::Copy for AAL_PARAMETERS_IE_0 {}
impl ::core::clone::Clone for AAL_PARAMETERS_IE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AAL_TYPE = i32;
pub const AALTYPE_5: AAL_TYPE = 5i32;
pub const AALTYPE_USER: AAL_TYPE = 16i32;
pub type ADDRESS_FAMILY = u32;
pub const AF_INET: ADDRESS_FAMILY = 2u32;
pub const AF_INET6: ADDRESS_FAMILY = 23u32;
pub const AF_UNSPEC: ADDRESS_FAMILY = 0u32;
#[repr(C)]
pub struct ADDRINFOA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut ADDRINFOA,
}
impl ::core::marker::Copy for ADDRINFOA {}
impl ::core::clone::Clone for ADDRINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ADDRINFOEX_VERSION_2: u32 = 2u32;
pub const ADDRINFOEX_VERSION_3: u32 = 3u32;
pub const ADDRINFOEX_VERSION_4: u32 = 4u32;
pub const ADDRINFOEX_VERSION_5: u32 = 5u32;
pub const ADDRINFOEX_VERSION_6: u32 = 6u32;
#[repr(C)]
pub struct AFPROTOCOLS {
    pub iAddressFamily: i32,
    pub iProtocol: i32,
}
impl ::core::marker::Copy for AFPROTOCOLS {}
impl ::core::clone::Clone for AFPROTOCOLS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AF_12844: u16 = 25u16;
pub const AF_APPLETALK: u16 = 16u16;
pub const AF_ATM: u16 = 22u16;
pub const AF_BAN: u16 = 21u16;
pub const AF_CCITT: u16 = 10u16;
pub const AF_CHAOS: u16 = 5u16;
pub const AF_CLUSTER: u16 = 24u16;
pub const AF_DATAKIT: u16 = 9u16;
pub const AF_DECnet: u16 = 12u16;
pub const AF_DLI: u16 = 13u16;
pub const AF_ECMA: u16 = 8u16;
pub const AF_FIREFOX: u16 = 19u16;
pub const AF_HYLINK: u16 = 15u16;
pub const AF_HYPERV: u16 = 34u16;
pub const AF_ICLFXBM: u16 = 31u16;
pub const AF_IMPLINK: u16 = 3u16;
pub const AF_IPX: u16 = 6u16;
pub const AF_IRDA: u16 = 26u16;
pub const AF_ISO: u16 = 7u16;
pub const AF_LAT: u16 = 14u16;
pub const AF_LINK: u16 = 33u16;
pub const AF_MAX: u16 = 29u16;
pub const AF_NETBIOS: u16 = 17u16;
pub const AF_NETDES: u16 = 28u16;
pub const AF_NS: u16 = 6u16;
pub const AF_OSI: u16 = 7u16;
pub const AF_PUP: u16 = 4u16;
pub const AF_SNA: u16 = 11u16;
pub const AF_TCNMESSAGE: u16 = 30u16;
pub const AF_TCNPROCESS: u16 = 29u16;
pub const AF_UNIX: u16 = 1u16;
pub const AF_UNKNOWN1: u16 = 20u16;
pub const AF_VOICEVIEW: u16 = 18u16;
pub const AI_ADDRCONFIG: u32 = 1024u32;
pub const AI_ALL: u32 = 256u32;
pub const AI_BYPASS_DNS_CACHE: u32 = 64u32;
pub const AI_CANONNAME: u32 = 2u32;
pub const AI_DISABLE_IDN_ENCODING: u32 = 524288u32;
pub const AI_DNS_ONLY: u32 = 16u32;
pub const AI_DNS_RESPONSE_HOSTFILE: u32 = 2u32;
pub const AI_DNS_RESPONSE_SECURE: u32 = 1u32;
pub const AI_DNS_SERVER_TYPE_DOH: u32 = 2u32;
pub const AI_DNS_SERVER_TYPE_UDP: u32 = 1u32;
pub const AI_DNS_SERVER_UDP_FALLBACK: u32 = 1u32;
pub const AI_EXCLUSIVE_CUSTOM_SERVERS: u32 = 2097152u32;
pub const AI_EXTENDED: u32 = 2147483648u32;
pub const AI_FILESERVER: u32 = 262144u32;
pub const AI_FORCE_CLEAR_TEXT: u32 = 32u32;
pub const AI_FQDN: u32 = 131072u32;
pub const AI_NON_AUTHORITATIVE: u32 = 16384u32;
pub const AI_NUMERICHOST: u32 = 4u32;
pub const AI_NUMERICSERV: u32 = 8u32;
pub const AI_PASSIVE: u32 = 1u32;
pub const AI_REQUIRE_SECURE: u32 = 536870912u32;
pub const AI_RESOLUTION_HANDLE: u32 = 1073741824u32;
pub const AI_RETURN_PREFERRED_NAMES: u32 = 65536u32;
pub const AI_RETURN_RESPONSE_FLAGS: u32 = 268435456u32;
pub const AI_RETURN_TTL: u32 = 128u32;
pub const AI_SECURE: u32 = 32768u32;
pub const AI_SECURE_WITH_FALLBACK: u32 = 1048576u32;
pub const AI_V4MAPPED: u32 = 2048u32;
pub type ARP_HARDWARE_TYPE = i32;
pub const ARP_HW_ENET: ARP_HARDWARE_TYPE = 1i32;
pub const ARP_HW_802: ARP_HARDWARE_TYPE = 6i32;
#[repr(C)]
pub struct ARP_HEADER {
    pub HardwareAddressSpace: u16,
    pub ProtocolAddressSpace: u16,
    pub HardwareAddressLength: u8,
    pub ProtocolAddressLength: u8,
    pub Opcode: u16,
    pub SenderHardwareAddress: [u8; 1],
}
impl ::core::marker::Copy for ARP_HEADER {}
impl ::core::clone::Clone for ARP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ARP_OPCODE = i32;
pub const ARP_REQUEST: ARP_OPCODE = 1i32;
pub const ARP_RESPONSE: ARP_OPCODE = 2i32;
pub const ASSOCIATE_NAMERES_CONTEXT: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1503890279, data2: 54526, data3: 18145, data4: [186, 60, 135, 234, 116, 202, 48, 73] };
#[repr(C)]
pub struct ASSOCIATE_NAMERES_CONTEXT_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub Handle: u64,
}
impl ::core::marker::Copy for ASSOCIATE_NAMERES_CONTEXT_INPUT {}
impl ::core::clone::Clone for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ATMPROTO_AAL1: u32 = 1u32;
pub const ATMPROTO_AAL2: u32 = 2u32;
pub const ATMPROTO_AAL34: u32 = 3u32;
pub const ATMPROTO_AAL5: u32 = 5u32;
pub const ATMPROTO_AALUSER: u32 = 0u32;
#[repr(C)]
pub struct ATM_ADDRESS {
    pub AddressType: u32,
    pub NumofDigits: u32,
    pub Addr: [u8; 20],
}
impl ::core::marker::Copy for ATM_ADDRESS {}
impl ::core::clone::Clone for ATM_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ATM_ADDR_SIZE: u32 = 20u32;
pub const ATM_AESA: u32 = 2u32;
#[repr(C)]
pub struct ATM_BHLI {
    pub HighLayerInfoType: u32,
    pub HighLayerInfoLength: u32,
    pub HighLayerInfo: [u8; 8],
}
impl ::core::marker::Copy for ATM_BHLI {}
impl ::core::clone::Clone for ATM_BHLI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_BLLI {
    pub Layer2Protocol: u32,
    pub Layer2UserSpecifiedProtocol: u32,
    pub Layer3Protocol: u32,
    pub Layer3UserSpecifiedProtocol: u32,
    pub Layer3IPI: u32,
    pub SnapID: [u8; 5],
}
impl ::core::marker::Copy for ATM_BLLI {}
impl ::core::clone::Clone for ATM_BLLI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_BLLI_IE {
    pub Layer2Protocol: u32,
    pub Layer2Mode: u8,
    pub Layer2WindowSize: u8,
    pub Layer2UserSpecifiedProtocol: u32,
    pub Layer3Protocol: u32,
    pub Layer3Mode: u8,
    pub Layer3DefaultPacketSize: u8,
    pub Layer3PacketWindowSize: u8,
    pub Layer3UserSpecifiedProtocol: u32,
    pub Layer3IPI: u32,
    pub SnapID: [u8; 5],
}
impl ::core::marker::Copy for ATM_BLLI_IE {}
impl ::core::clone::Clone for ATM_BLLI_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_BROADBAND_BEARER_CAPABILITY_IE {
    pub BearerClass: u8,
    pub TrafficType: u8,
    pub TimingRequirements: u8,
    pub ClippingSusceptability: u8,
    pub UserPlaneConnectionConfig: u8,
}
impl ::core::marker::Copy for ATM_BROADBAND_BEARER_CAPABILITY_IE {}
impl ::core::clone::Clone for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_CALLING_PARTY_NUMBER_IE {
    pub ATM_Number: ATM_ADDRESS,
    pub Presentation_Indication: u8,
    pub Screening_Indicator: u8,
}
impl ::core::marker::Copy for ATM_CALLING_PARTY_NUMBER_IE {}
impl ::core::clone::Clone for ATM_CALLING_PARTY_NUMBER_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_CAUSE_IE {
    pub Location: u8,
    pub Cause: u8,
    pub DiagnosticsLength: u8,
    pub Diagnostics: [u8; 4],
}
impl ::core::marker::Copy for ATM_CAUSE_IE {}
impl ::core::clone::Clone for ATM_CAUSE_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_CONNECTION_ID {
    pub DeviceNumber: u32,
    pub VPI: u32,
    pub VCI: u32,
}
impl ::core::marker::Copy for ATM_CONNECTION_ID {}
impl ::core::clone::Clone for ATM_CONNECTION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ATM_E164: u32 = 1u32;
pub const ATM_NSAP: u32 = 2u32;
#[repr(C, packed(4))]
pub struct ATM_PVC_PARAMS {
    pub PvcConnectionId: ATM_CONNECTION_ID,
    pub PvcQos: QOS,
}
impl ::core::marker::Copy for ATM_PVC_PARAMS {}
impl ::core::clone::Clone for ATM_PVC_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_QOS_CLASS_IE {
    pub QOSClassForward: u8,
    pub QOSClassBackward: u8,
}
impl ::core::marker::Copy for ATM_QOS_CLASS_IE {}
impl ::core::clone::Clone for ATM_QOS_CLASS_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_TD {
    pub PeakCellRate_CLP0: u32,
    pub PeakCellRate_CLP01: u32,
    pub SustainableCellRate_CLP0: u32,
    pub SustainableCellRate_CLP01: u32,
    pub MaxBurstSize_CLP0: u32,
    pub MaxBurstSize_CLP01: u32,
    pub Tagging: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for ATM_TD {}
impl ::core::clone::Clone for ATM_TD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_TRAFFIC_DESCRIPTOR_IE {
    pub Forward: ATM_TD,
    pub Backward: ATM_TD,
    pub BestEffort: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for ATM_TRAFFIC_DESCRIPTOR_IE {}
impl ::core::clone::Clone for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATM_TRANSIT_NETWORK_SELECTION_IE {
    pub TypeOfNetworkId: u8,
    pub NetworkIdPlan: u8,
    pub NetworkIdLength: u8,
    pub NetworkId: [u8; 1],
}
impl ::core::marker::Copy for ATM_TRANSIT_NETWORK_SELECTION_IE {}
impl ::core::clone::Clone for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const BASE_PROTOCOL: u32 = 1u32;
pub const BCOB_A: u32 = 1u32;
pub const BCOB_C: u32 = 3u32;
pub const BCOB_X: u32 = 16u32;
pub const BHLI_HighLayerProfile: u32 = 2u32;
pub const BHLI_ISO: u32 = 0u32;
pub const BHLI_UserSpecific: u32 = 1u32;
pub const BHLI_VendorSpecificAppId: u32 = 3u32;
pub const BIGENDIAN: u32 = 0u32;
pub const BITS_PER_BYTE: u32 = 8u32;
pub const BLLI_L2_ELAPB: u32 = 8u32;
pub const BLLI_L2_HDLC_ABM: u32 = 11u32;
pub const BLLI_L2_HDLC_ARM: u32 = 9u32;
pub const BLLI_L2_HDLC_NRM: u32 = 10u32;
pub const BLLI_L2_ISO_1745: u32 = 1u32;
pub const BLLI_L2_ISO_7776: u32 = 17u32;
pub const BLLI_L2_LLC: u32 = 12u32;
pub const BLLI_L2_MODE_EXT: u32 = 128u32;
pub const BLLI_L2_MODE_NORMAL: u32 = 64u32;
pub const BLLI_L2_Q921: u32 = 2u32;
pub const BLLI_L2_Q922: u32 = 14u32;
pub const BLLI_L2_USER_SPECIFIED: u32 = 16u32;
pub const BLLI_L2_X25L: u32 = 6u32;
pub const BLLI_L2_X25M: u32 = 7u32;
pub const BLLI_L2_X75: u32 = 13u32;
pub const BLLI_L3_IPI_IP: u32 = 204u32;
pub const BLLI_L3_IPI_SNAP: u32 = 128u32;
pub const BLLI_L3_ISO_8208: u32 = 7u32;
pub const BLLI_L3_ISO_TR9577: u32 = 11u32;
pub const BLLI_L3_MODE_EXT: u32 = 128u32;
pub const BLLI_L3_MODE_NORMAL: u32 = 64u32;
pub const BLLI_L3_PACKET_1024: u32 = 10u32;
pub const BLLI_L3_PACKET_128: u32 = 7u32;
pub const BLLI_L3_PACKET_16: u32 = 4u32;
pub const BLLI_L3_PACKET_2048: u32 = 11u32;
pub const BLLI_L3_PACKET_256: u32 = 8u32;
pub const BLLI_L3_PACKET_32: u32 = 5u32;
pub const BLLI_L3_PACKET_4096: u32 = 12u32;
pub const BLLI_L3_PACKET_512: u32 = 9u32;
pub const BLLI_L3_PACKET_64: u32 = 6u32;
pub const BLLI_L3_SIO_8473: u32 = 9u32;
pub const BLLI_L3_T70: u32 = 10u32;
pub const BLLI_L3_USER_SPECIFIED: u32 = 16u32;
pub const BLLI_L3_X223: u32 = 8u32;
pub const BLLI_L3_X25: u32 = 6u32;
pub const BYTE_ORDER: u32 = 1234u32;
pub const CAUSE_AAL_PARAMETERS_UNSUPPORTED: u32 = 93u32;
pub const CAUSE_ACCESS_INFORMAION_DISCARDED: u32 = 43u32;
pub const CAUSE_BEARER_CAPABILITY_UNAUTHORIZED: u32 = 57u32;
pub const CAUSE_BEARER_CAPABILITY_UNAVAILABLE: u32 = 58u32;
pub const CAUSE_BEARER_CAPABILITY_UNIMPLEMENTED: u32 = 65u32;
pub const CAUSE_CALL_REJECTED: u32 = 21u32;
pub const CAUSE_CHANNEL_NONEXISTENT: u32 = 82u32;
pub const CAUSE_COND_PERMANENT: u32 = 1u32;
pub const CAUSE_COND_TRANSIENT: u32 = 2u32;
pub const CAUSE_COND_UNKNOWN: u32 = 0u32;
pub const CAUSE_DESTINATION_OUT_OF_ORDER: u32 = 27u32;
pub const CAUSE_INCOMPATIBLE_DESTINATION: u32 = 88u32;
pub const CAUSE_INCORRECT_MESSAGE_LENGTH: u32 = 104u32;
pub const CAUSE_INVALID_CALL_REFERENCE: u32 = 81u32;
pub const CAUSE_INVALID_ENDPOINT_REFERENCE: u32 = 89u32;
pub const CAUSE_INVALID_IE_CONTENTS: u32 = 100u32;
pub const CAUSE_INVALID_NUMBER_FORMAT: u32 = 28u32;
pub const CAUSE_INVALID_STATE_FOR_MESSAGE: u32 = 101u32;
pub const CAUSE_INVALID_TRANSIT_NETWORK_SELECTION: u32 = 91u32;
pub const CAUSE_LOC_BEYOND_INTERWORKING: u32 = 10u32;
pub const CAUSE_LOC_INTERNATIONAL_NETWORK: u32 = 7u32;
pub const CAUSE_LOC_PRIVATE_LOCAL: u32 = 1u32;
pub const CAUSE_LOC_PRIVATE_REMOTE: u32 = 5u32;
pub const CAUSE_LOC_PUBLIC_LOCAL: u32 = 2u32;
pub const CAUSE_LOC_PUBLIC_REMOTE: u32 = 4u32;
pub const CAUSE_LOC_TRANSIT_NETWORK: u32 = 3u32;
pub const CAUSE_LOC_USER: u32 = 0u32;
pub const CAUSE_MANDATORY_IE_MISSING: u32 = 96u32;
pub const CAUSE_NA_ABNORMAL: u32 = 4u32;
pub const CAUSE_NA_NORMAL: u32 = 0u32;
pub const CAUSE_NETWORK_OUT_OF_ORDER: u32 = 38u32;
pub const CAUSE_NORMAL_CALL_CLEARING: u32 = 16u32;
pub const CAUSE_NORMAL_UNSPECIFIED: u32 = 31u32;
pub const CAUSE_NO_ROUTE_TO_DESTINATION: u32 = 3u32;
pub const CAUSE_NO_ROUTE_TO_TRANSIT_NETWORK: u32 = 2u32;
pub const CAUSE_NO_USER_RESPONDING: u32 = 18u32;
pub const CAUSE_NO_VPI_VCI_AVAILABLE: u32 = 45u32;
pub const CAUSE_NUMBER_CHANGED: u32 = 22u32;
pub const CAUSE_OPTION_UNAVAILABLE: u32 = 63u32;
pub const CAUSE_PROTOCOL_ERROR: u32 = 111u32;
pub const CAUSE_PU_PROVIDER: u32 = 0u32;
pub const CAUSE_PU_USER: u32 = 8u32;
pub const CAUSE_QOS_UNAVAILABLE: u32 = 49u32;
pub const CAUSE_REASON_IE_INSUFFICIENT: u32 = 8u32;
pub const CAUSE_REASON_IE_MISSING: u32 = 4u32;
pub const CAUSE_REASON_USER: u32 = 0u32;
pub const CAUSE_RECOVERY_ON_TIMEOUT: u32 = 102u32;
pub const CAUSE_RESOURCE_UNAVAILABLE: u32 = 47u32;
pub const CAUSE_STATUS_ENQUIRY_RESPONSE: u32 = 30u32;
pub const CAUSE_TEMPORARY_FAILURE: u32 = 41u32;
pub const CAUSE_TOO_MANY_PENDING_ADD_PARTY: u32 = 92u32;
pub const CAUSE_UNALLOCATED_NUMBER: u32 = 1u32;
pub const CAUSE_UNIMPLEMENTED_IE: u32 = 99u32;
pub const CAUSE_UNIMPLEMENTED_MESSAGE_TYPE: u32 = 97u32;
pub const CAUSE_UNSUPPORTED_TRAFFIC_PARAMETERS: u32 = 73u32;
pub const CAUSE_USER_BUSY: u32 = 17u32;
pub const CAUSE_USER_CELL_RATE_UNAVAILABLE: u32 = 51u32;
pub const CAUSE_USER_REJECTS_CLIR: u32 = 23u32;
pub const CAUSE_VPI_VCI_UNACCEPTABLE: u32 = 10u32;
pub const CAUSE_VPI_VCI_UNAVAILABLE: u32 = 35u32;
pub const CF_ACCEPT: u32 = 0u32;
pub const CF_DEFER: u32 = 2u32;
pub const CF_REJECT: u32 = 1u32;
pub const CLIP_NOT: u32 = 0u32;
pub const CLIP_SUS: u32 = 32u32;
pub type CONTROL_CHANNEL_TRIGGER_STATUS = i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_INVALID: CONTROL_CHANNEL_TRIGGER_STATUS = 0i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = 1i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = 2i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = 3i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = 4i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED: CONTROL_CHANNEL_TRIGGER_STATUS = 5i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE: CONTROL_CHANNEL_TRIGGER_STATUS = 6i32;
#[repr(C)]
pub struct CSADDR_INFO {
    pub LocalAddr: SOCKET_ADDRESS,
    pub RemoteAddr: SOCKET_ADDRESS,
    pub iSocketType: i32,
    pub iProtocol: i32,
}
impl ::core::marker::Copy for CSADDR_INFO {}
impl ::core::clone::Clone for CSADDR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DE_REUSE_SOCKET: u32 = 2u32;
pub const DL_ADDRESS_LENGTH_MAXIMUM: u32 = 32u32;
#[repr(C)]
pub union DL_EI48 {
    pub Byte: [u8; 3],
}
impl ::core::marker::Copy for DL_EI48 {}
impl ::core::clone::Clone for DL_EI48 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DL_EI64 {
    pub Byte: [u8; 5],
}
impl ::core::marker::Copy for DL_EI64 {}
impl ::core::clone::Clone for DL_EI64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DL_EUI48 {
    pub Byte: [u8; 6],
    pub Anonymous: DL_EUI48_0,
}
impl ::core::marker::Copy for DL_EUI48 {}
impl ::core::clone::Clone for DL_EUI48 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DL_EUI48_0 {
    pub Oui: DL_OUI,
    pub Ei48: DL_EI48,
}
impl ::core::marker::Copy for DL_EUI48_0 {}
impl ::core::clone::Clone for DL_EUI48_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DL_EUI64 {
    pub Byte: [u8; 8],
    pub Value: u64,
    pub Anonymous: DL_EUI64_0,
}
impl ::core::marker::Copy for DL_EUI64 {}
impl ::core::clone::Clone for DL_EUI64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DL_EUI64_0 {
    pub Oui: DL_OUI,
    pub Anonymous: DL_EUI64_0_0,
}
impl ::core::marker::Copy for DL_EUI64_0 {}
impl ::core::clone::Clone for DL_EUI64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DL_EUI64_0_0 {
    pub Ei64: DL_EI64,
    pub Anonymous: DL_EUI64_0_0_0,
}
impl ::core::marker::Copy for DL_EUI64_0_0 {}
impl ::core::clone::Clone for DL_EUI64_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DL_EUI64_0_0_0 {
    pub Type: u8,
    pub Tse: u8,
    pub Ei48: DL_EI48,
}
impl ::core::marker::Copy for DL_EUI64_0_0_0 {}
impl ::core::clone::Clone for DL_EUI64_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DL_HEADER_LENGTH_MAXIMUM: u32 = 64u32;
#[repr(C)]
pub union DL_OUI {
    pub Byte: [u8; 3],
    pub Anonymous: DL_OUI_0,
}
impl ::core::marker::Copy for DL_OUI {}
impl ::core::clone::Clone for DL_OUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DL_OUI_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DL_OUI_0 {}
impl ::core::clone::Clone for DL_OUI_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DL_TEREDO_ADDRESS {
    pub Reserved: [u8; 6],
    pub Anonymous: DL_TEREDO_ADDRESS_0,
}
impl ::core::marker::Copy for DL_TEREDO_ADDRESS {}
impl ::core::clone::Clone for DL_TEREDO_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub union DL_TEREDO_ADDRESS_0 {
    pub Eui64: DL_EUI64,
    pub Anonymous: DL_TEREDO_ADDRESS_0_0,
}
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_0 {}
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct DL_TEREDO_ADDRESS_0_0 {
    pub Flags: u16,
    pub MappedPort: u16,
    pub MappedAddress: IN_ADDR,
}
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_0_0 {}
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DL_TEREDO_ADDRESS_PRV {
    pub Reserved: [u8; 6],
    pub Anonymous: DL_TEREDO_ADDRESS_PRV_0,
}
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_PRV {}
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_PRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub union DL_TEREDO_ADDRESS_PRV_0 {
    pub Eui64: DL_EUI64,
    pub Anonymous: DL_TEREDO_ADDRESS_PRV_0_0,
}
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_PRV_0 {}
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_PRV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct DL_TEREDO_ADDRESS_PRV_0_0 {
    pub Flags: u16,
    pub MappedPort: u16,
    pub MappedAddress: IN_ADDR,
    pub LocalAddress: IN_ADDR,
    pub InterfaceIndex: u32,
    pub LocalPort: u16,
    pub DlDestination: DL_EUI48,
}
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_PRV_0_0 {}
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_PRV_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct DL_TUNNEL_ADDRESS {
    pub CompartmentId: ::win32_system_sys::Kernel::COMPARTMENT_ID,
    pub ScopeId: SCOPE_ID,
    pub IpAddress: [u8; 1],
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for DL_TUNNEL_ADDRESS {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for DL_TUNNEL_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ETHERNET_HEADER {
    pub Destination: DL_EUI48,
    pub Source: DL_EUI48,
    pub Anonymous: ETHERNET_HEADER_0,
}
impl ::core::marker::Copy for ETHERNET_HEADER {}
impl ::core::clone::Clone for ETHERNET_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union ETHERNET_HEADER_0 {
    pub Type: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for ETHERNET_HEADER_0 {}
impl ::core::clone::Clone for ETHERNET_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ETHERNET_TYPE_802_1AD: u32 = 34984u32;
pub const ETHERNET_TYPE_802_1Q: u32 = 33024u32;
pub const ETHERNET_TYPE_ARP: u32 = 2054u32;
pub const ETHERNET_TYPE_IPV4: u32 = 2048u32;
pub const ETHERNET_TYPE_IPV6: u32 = 34525u32;
pub const ETHERNET_TYPE_MINIMUM: u32 = 1536u32;
pub const ETH_LENGTH_OF_HEADER: u32 = 14u32;
pub const ETH_LENGTH_OF_SNAP_HEADER: u32 = 8u32;
pub const ETH_LENGTH_OF_VLAN_HEADER: u32 = 4u32;
pub const EXT_LEN_UNIT: u32 = 8u32;
pub type FALLBACK_INDEX = i32;
pub const FallbackIndexTcpFastopen: FALLBACK_INDEX = 0i32;
pub const FallbackIndexMax: FALLBACK_INDEX = 1i32;
pub const FD_ACCEPT: u32 = 8u32;
pub const FD_ACCEPT_BIT: u32 = 3u32;
pub const FD_ADDRESS_LIST_CHANGE_BIT: u32 = 9u32;
pub const FD_CLOSE: u32 = 32u32;
pub const FD_CLOSE_BIT: u32 = 5u32;
pub const FD_CONNECT: u32 = 16u32;
pub const FD_CONNECT_BIT: u32 = 4u32;
pub const FD_GROUP_QOS_BIT: u32 = 7u32;
pub const FD_MAX_EVENTS: u32 = 10u32;
pub const FD_OOB: u32 = 4u32;
pub const FD_OOB_BIT: u32 = 2u32;
pub const FD_QOS_BIT: u32 = 6u32;
pub const FD_READ: u32 = 1u32;
pub const FD_READ_BIT: u32 = 0u32;
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: u32 = 8u32;
pub const FD_SETSIZE: u32 = 64u32;
pub const FD_WRITE: u32 = 2u32;
pub const FD_WRITE_BIT: u32 = 1u32;
pub const FIOASYNC: i32 = -2147195267i32;
pub const FIONBIO: i32 = -2147195266i32;
pub const FIONREAD: i32 = 1074030207i32;
#[repr(C)]
pub struct FLOWSPEC {
    pub TokenRate: u32,
    pub TokenBucketSize: u32,
    pub PeakBandwidth: u32,
    pub Latency: u32,
    pub DelayVariation: u32,
    pub ServiceType: u32,
    pub MaxSduSize: u32,
    pub MinimumPolicedSize: u32,
}
impl ::core::marker::Copy for FLOWSPEC {}
impl ::core::clone::Clone for FLOWSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FROM_PROTOCOL_INFO: i32 = -1i32;
pub const GAI_STRERROR_BUFFER_SIZE: u32 = 1024u32;
#[repr(C)]
pub struct GROUP_FILTER {
    pub gf_interface: u32,
    pub gf_group: SOCKADDR_STORAGE,
    pub gf_fmode: MULTICAST_MODE_TYPE,
    pub gf_numsrc: u32,
    pub gf_slist: [SOCKADDR_STORAGE; 1],
}
impl ::core::marker::Copy for GROUP_FILTER {}
impl ::core::clone::Clone for GROUP_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GROUP_REQ {
    pub gr_interface: u32,
    pub gr_group: SOCKADDR_STORAGE,
}
impl ::core::marker::Copy for GROUP_REQ {}
impl ::core::clone::Clone for GROUP_REQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GROUP_SOURCE_REQ {
    pub gsr_interface: u32,
    pub gsr_group: SOCKADDR_STORAGE,
    pub gsr_source: SOCKADDR_STORAGE,
}
impl ::core::marker::Copy for GROUP_SOURCE_REQ {}
impl ::core::clone::Clone for GROUP_SOURCE_REQ {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HWSAEVENT = isize;
pub const IAS_ATTRIB_INT: u32 = 1u32;
pub const IAS_ATTRIB_NO_ATTRIB: u32 = 0u32;
pub const IAS_ATTRIB_NO_CLASS: u32 = 16u32;
pub const IAS_ATTRIB_OCTETSEQ: u32 = 2u32;
pub const IAS_ATTRIB_STR: u32 = 3u32;
pub const IAS_MAX_ATTRIBNAME: u32 = 256u32;
pub const IAS_MAX_CLASSNAME: u32 = 64u32;
pub const IAS_MAX_OCTET_STRING: u32 = 1024u32;
pub const IAS_MAX_USER_STRING: u32 = 256u32;
pub type ICMP4_TIME_EXCEED_CODE = i32;
pub const ICMP4_TIME_EXCEED_TRANSIT: ICMP4_TIME_EXCEED_CODE = 0i32;
pub const ICMP4_TIME_EXCEED_REASSEMBLY: ICMP4_TIME_EXCEED_CODE = 1i32;
pub type ICMP4_UNREACH_CODE = i32;
pub const ICMP4_UNREACH_NET: ICMP4_UNREACH_CODE = 0i32;
pub const ICMP4_UNREACH_HOST: ICMP4_UNREACH_CODE = 1i32;
pub const ICMP4_UNREACH_PROTOCOL: ICMP4_UNREACH_CODE = 2i32;
pub const ICMP4_UNREACH_PORT: ICMP4_UNREACH_CODE = 3i32;
pub const ICMP4_UNREACH_FRAG_NEEDED: ICMP4_UNREACH_CODE = 4i32;
pub const ICMP4_UNREACH_SOURCEROUTE_FAILED: ICMP4_UNREACH_CODE = 5i32;
pub const ICMP4_UNREACH_NET_UNKNOWN: ICMP4_UNREACH_CODE = 6i32;
pub const ICMP4_UNREACH_HOST_UNKNOWN: ICMP4_UNREACH_CODE = 7i32;
pub const ICMP4_UNREACH_ISOLATED: ICMP4_UNREACH_CODE = 8i32;
pub const ICMP4_UNREACH_NET_ADMIN: ICMP4_UNREACH_CODE = 9i32;
pub const ICMP4_UNREACH_HOST_ADMIN: ICMP4_UNREACH_CODE = 10i32;
pub const ICMP4_UNREACH_NET_TOS: ICMP4_UNREACH_CODE = 11i32;
pub const ICMP4_UNREACH_HOST_TOS: ICMP4_UNREACH_CODE = 12i32;
pub const ICMP4_UNREACH_ADMIN: ICMP4_UNREACH_CODE = 13i32;
pub const ICMP6_DST_UNREACH_ADDR: u32 = 3u32;
pub const ICMP6_DST_UNREACH_ADMIN: u32 = 1u32;
pub const ICMP6_DST_UNREACH_BEYONDSCOPE: u32 = 2u32;
pub const ICMP6_DST_UNREACH_NOPORT: u32 = 4u32;
pub const ICMP6_DST_UNREACH_NOROUTE: u32 = 0u32;
pub const ICMP6_PARAMPROB_HEADER: u32 = 0u32;
pub const ICMP6_PARAMPROB_NEXTHEADER: u32 = 1u32;
pub const ICMP6_PARAMPROB_OPTION: u32 = 2u32;
pub const ICMP6_TIME_EXCEED_REASSEMBLY: u32 = 1u32;
pub const ICMP6_TIME_EXCEED_TRANSIT: u32 = 0u32;
#[repr(C)]
pub struct ICMPV4_ADDRESS_MASK_MESSAGE {
    pub Header: ICMP_MESSAGE,
    pub AddressMask: u32,
}
impl ::core::marker::Copy for ICMPV4_ADDRESS_MASK_MESSAGE {}
impl ::core::clone::Clone for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ICMPV4_INVALID_PREFERENCE_LEVEL: u32 = 2147483648u32;
#[repr(C)]
pub struct ICMPV4_ROUTER_ADVERT_ENTRY {
    pub RouterAdvertAddr: IN_ADDR,
    pub PreferenceLevel: i32,
}
impl ::core::marker::Copy for ICMPV4_ROUTER_ADVERT_ENTRY {}
impl ::core::clone::Clone for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICMPV4_ROUTER_ADVERT_HEADER {
    pub RaHeader: ICMP_MESSAGE,
}
impl ::core::marker::Copy for ICMPV4_ROUTER_ADVERT_HEADER {}
impl ::core::clone::Clone for ICMPV4_ROUTER_ADVERT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICMPV4_ROUTER_SOLICIT {
    pub RsHeader: ICMP_MESSAGE,
}
impl ::core::marker::Copy for ICMPV4_ROUTER_SOLICIT {}
impl ::core::clone::Clone for ICMPV4_ROUTER_SOLICIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICMPV4_TIMESTAMP_MESSAGE {
    pub Header: ICMP_MESSAGE,
    pub OriginateTimestamp: u32,
    pub ReceiveTimestamp: u32,
    pub TransmitTimestamp: u32,
}
impl ::core::marker::Copy for ICMPV4_TIMESTAMP_MESSAGE {}
impl ::core::clone::Clone for ICMPV4_TIMESTAMP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ICMPV6_ECHO_REQUEST_FLAG_REVERSE: u32 = 1u32;
#[repr(C)]
pub struct ICMP_ERROR_INFO {
    pub srcaddress: SOCKADDR_INET,
    pub protocol: IPPROTO,
    pub r#type: u8,
    pub code: u8,
}
impl ::core::marker::Copy for ICMP_ERROR_INFO {}
impl ::core::clone::Clone for ICMP_ERROR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICMP_HEADER {
    pub Type: u8,
    pub Code: u8,
    pub Checksum: u16,
}
impl ::core::marker::Copy for ICMP_HEADER {}
impl ::core::clone::Clone for ICMP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICMP_MESSAGE {
    pub Header: ICMP_HEADER,
    pub Data: ICMP_MESSAGE_0,
}
impl ::core::marker::Copy for ICMP_MESSAGE {}
impl ::core::clone::Clone for ICMP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union ICMP_MESSAGE_0 {
    pub Data32: [u32; 1],
    pub Data16: [u16; 2],
    pub Data8: [u8; 4],
}
impl ::core::marker::Copy for ICMP_MESSAGE_0 {}
impl ::core::clone::Clone for ICMP_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IFF_BROADCAST: u32 = 2u32;
pub const IFF_LOOPBACK: u32 = 4u32;
pub const IFF_MULTICAST: u32 = 16u32;
pub const IFF_POINTTOPOINT: u32 = 8u32;
pub const IFF_UP: u32 = 1u32;
#[repr(C)]
pub struct IGMPV3_QUERY_HEADER {
    pub Type: u8,
    pub Anonymous1: IGMPV3_QUERY_HEADER_0,
    pub Checksum: u16,
    pub MulticastAddress: IN_ADDR,
    pub _bitfield: u8,
    pub Anonymous2: IGMPV3_QUERY_HEADER_1,
    pub SourceCount: u16,
}
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER {}
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IGMPV3_QUERY_HEADER_0 {
    pub MaxRespCode: u8,
    pub Anonymous: IGMPV3_QUERY_HEADER_0_0,
}
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_0 {}
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGMPV3_QUERY_HEADER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_0_0 {}
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IGMPV3_QUERY_HEADER_1 {
    pub QueriersQueryInterfaceCode: u8,
    pub Anonymous: IGMPV3_QUERY_HEADER_1_0,
}
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_1 {}
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGMPV3_QUERY_HEADER_1_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_1_0 {}
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGMPV3_REPORT_HEADER {
    pub Type: u8,
    pub Reserved: u8,
    pub Checksum: u16,
    pub Reserved2: u16,
    pub RecordCount: u16,
}
impl ::core::marker::Copy for IGMPV3_REPORT_HEADER {}
impl ::core::clone::Clone for IGMPV3_REPORT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGMPV3_REPORT_RECORD_HEADER {
    pub Type: u8,
    pub AuxillaryDataLength: u8,
    pub SourceCount: u16,
    pub MulticastAddress: IN_ADDR,
}
impl ::core::marker::Copy for IGMPV3_REPORT_RECORD_HEADER {}
impl ::core::clone::Clone for IGMPV3_REPORT_RECORD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGMP_HEADER {
    pub Anonymous1: IGMP_HEADER_0,
    pub Anonymous2: IGMP_HEADER_1,
    pub Checksum: u16,
    pub MulticastAddress: IN_ADDR,
}
impl ::core::marker::Copy for IGMP_HEADER {}
impl ::core::clone::Clone for IGMP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IGMP_HEADER_0 {
    pub Anonymous: IGMP_HEADER_0_0,
    pub VersionType: u8,
}
impl ::core::marker::Copy for IGMP_HEADER_0 {}
impl ::core::clone::Clone for IGMP_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGMP_HEADER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IGMP_HEADER_0_0 {}
impl ::core::clone::Clone for IGMP_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IGMP_HEADER_1 {
    pub Reserved: u8,
    pub MaxRespTime: u8,
    pub Code: u8,
}
impl ::core::marker::Copy for IGMP_HEADER_1 {}
impl ::core::clone::Clone for IGMP_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IGMP_LEAVE_GROUP_TYPE: u32 = 23u32;
pub type IGMP_MAX_RESP_CODE_TYPE = i32;
pub const IGMP_MAX_RESP_CODE_TYPE_NORMAL: IGMP_MAX_RESP_CODE_TYPE = 0i32;
pub const IGMP_MAX_RESP_CODE_TYPE_FLOAT: IGMP_MAX_RESP_CODE_TYPE = 1i32;
pub const IGMP_QUERY_TYPE: u32 = 17u32;
pub const IGMP_VERSION1_REPORT_TYPE: u32 = 18u32;
pub const IGMP_VERSION2_REPORT_TYPE: u32 = 22u32;
pub const IGMP_VERSION3_REPORT_TYPE: u32 = 34u32;
pub const IMPLINK_HIGHEXPER: u32 = 158u32;
pub const IMPLINK_IP: u32 = 155u32;
pub const IMPLINK_LOWEXPER: u32 = 156u32;
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: u32 = 16u32;
pub const IN4ADDR_LOOPBACK: u32 = 16777343u32;
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: u32 = 8u32;
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: u32 = 4u32;
pub const IN6ADDR_6TO4PREFIX_LENGTH: u32 = 16u32;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: u32 = 64u32;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: u32 = 8u32;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: u32 = 104u32;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: u32 = 32u32;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: u32 = 96u32;
#[repr(C)]
pub struct IN6_ADDR {
    pub u: IN6_ADDR_0,
}
impl ::core::marker::Copy for IN6_ADDR {}
impl ::core::clone::Clone for IN6_ADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IN6_ADDR_0 {
    pub Byte: [u8; 16],
    pub Word: [u16; 8],
}
impl ::core::marker::Copy for IN6_ADDR_0 {}
impl ::core::clone::Clone for IN6_ADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IN6_EMBEDDEDV4_BITS_IN_BYTE: u32 = 8u32;
pub const IN6_EMBEDDEDV4_UOCTET_POSITION: u32 = 8u32;
#[repr(C)]
pub struct IN6_PKTINFO {
    pub ipi6_addr: IN6_ADDR,
    pub ipi6_ifindex: u32,
}
impl ::core::marker::Copy for IN6_PKTINFO {}
impl ::core::clone::Clone for IN6_PKTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const INADDR_LOOPBACK: u32 = 2130706433u32;
pub const INADDR_NONE: u32 = 4294967295u32;
pub const INCL_WINSOCK_API_PROTOTYPES: u32 = 1u32;
pub const INCL_WINSOCK_API_TYPEDEFS: u32 = 0u32;
pub const INET6_ADDRSTRLEN: u32 = 65u32;
pub const INET_ADDRSTRLEN: u32 = 22u32;
#[repr(C)]
pub struct INET_PORT_RANGE {
    pub StartPort: u16,
    pub NumberOfPorts: u16,
}
impl ::core::marker::Copy for INET_PORT_RANGE {}
impl ::core::clone::Clone for INET_PORT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INET_PORT_RESERVATION_INFORMATION {
    pub OwningPid: u32,
}
impl ::core::marker::Copy for INET_PORT_RESERVATION_INFORMATION {}
impl ::core::clone::Clone for INET_PORT_RESERVATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INET_PORT_RESERVATION_INSTANCE {
    pub Reservation: INET_PORT_RANGE,
    pub Token: INET_PORT_RESERVATION_TOKEN,
}
impl ::core::marker::Copy for INET_PORT_RESERVATION_INSTANCE {}
impl ::core::clone::Clone for INET_PORT_RESERVATION_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INET_PORT_RESERVATION_TOKEN {
    pub Token: u64,
}
impl ::core::marker::Copy for INET_PORT_RESERVATION_TOKEN {}
impl ::core::clone::Clone for INET_PORT_RESERVATION_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INTERFACE_INFO {
    pub iiFlags: u32,
    pub iiAddress: sockaddr_gen,
    pub iiBroadcastAddress: sockaddr_gen,
    pub iiNetmask: sockaddr_gen,
}
impl ::core::marker::Copy for INTERFACE_INFO {}
impl ::core::clone::Clone for INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INTERFACE_INFO_EX {
    pub iiFlags: u32,
    pub iiAddress: SOCKET_ADDRESS,
    pub iiBroadcastAddress: SOCKET_ADDRESS,
    pub iiNetmask: SOCKET_ADDRESS,
}
impl ::core::marker::Copy for INTERFACE_INFO_EX {}
impl ::core::clone::Clone for INTERFACE_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const INVALID_SOCKET: SOCKET = -1i32 as _;
#[repr(C)]
pub struct IN_ADDR {
    pub S_un: IN_ADDR_0,
}
impl ::core::marker::Copy for IN_ADDR {}
impl ::core::clone::Clone for IN_ADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IN_ADDR_0 {
    pub S_un_b: IN_ADDR_0_0,
    pub S_un_w: IN_ADDR_0_1,
    pub S_addr: u32,
}
impl ::core::marker::Copy for IN_ADDR_0 {}
impl ::core::clone::Clone for IN_ADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IN_ADDR_0_0 {
    pub s_b1: u8,
    pub s_b2: u8,
    pub s_b3: u8,
    pub s_b4: u8,
}
impl ::core::marker::Copy for IN_ADDR_0_0 {}
impl ::core::clone::Clone for IN_ADDR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IN_ADDR_0_1 {
    pub s_w1: u16,
    pub s_w2: u16,
}
impl ::core::marker::Copy for IN_ADDR_0_1 {}
impl ::core::clone::Clone for IN_ADDR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IN_CLASSA_HOST: u32 = 16777215u32;
pub const IN_CLASSA_MAX: u32 = 128u32;
pub const IN_CLASSA_NET: u32 = 4278190080u32;
pub const IN_CLASSA_NSHIFT: u32 = 24u32;
pub const IN_CLASSB_HOST: u32 = 65535u32;
pub const IN_CLASSB_MAX: u32 = 65536u32;
pub const IN_CLASSB_NET: u32 = 4294901760u32;
pub const IN_CLASSB_NSHIFT: u32 = 16u32;
pub const IN_CLASSC_HOST: u32 = 255u32;
pub const IN_CLASSC_NET: u32 = 4294967040u32;
pub const IN_CLASSC_NSHIFT: u32 = 8u32;
pub const IN_CLASSD_HOST: u32 = 268435455u32;
pub const IN_CLASSD_NET: u32 = 4026531840u32;
pub const IN_CLASSD_NSHIFT: u32 = 28u32;
#[repr(C)]
pub struct IN_PKTINFO {
    pub ipi_addr: IN_ADDR,
    pub ipi_ifindex: u32,
}
impl ::core::marker::Copy for IN_PKTINFO {}
impl ::core::clone::Clone for IN_PKTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IN_PKTINFO_EX {
    pub pkt_info: IN_PKTINFO,
    pub scope_id: SCOPE_ID,
}
impl ::core::marker::Copy for IN_PKTINFO_EX {}
impl ::core::clone::Clone for IN_PKTINFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IN_RECVERR {
    pub protocol: IPPROTO,
    pub info: u32,
    pub r#type: u8,
    pub code: u8,
}
impl ::core::marker::Copy for IN_RECVERR {}
impl ::core::clone::Clone for IN_RECVERR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IOCPARM_MASK: u32 = 127u32;
pub const IOC_IN: u32 = 2147483648u32;
pub const IOC_INOUT: u32 = 3221225472u32;
pub const IOC_OUT: u32 = 1073741824u32;
pub const IOC_PROTOCOL: u32 = 268435456u32;
pub const IOC_UNIX: u32 = 0u32;
pub const IOC_VENDOR: u32 = 402653184u32;
pub const IOC_VOID: u32 = 536870912u32;
pub const IOC_WS2: u32 = 134217728u32;
pub const IP4_OFF_MASK: u32 = 65311u32;
pub const IP6F_MORE_FRAG: u32 = 256u32;
pub const IP6F_OFF_MASK: u32 = 63743u32;
pub const IP6F_RESERVED_MASK: u32 = 1536u32;
pub const IP6OPT_MUTABLE: u32 = 32u32;
pub const IP6OPT_TYPE_DISCARD: u32 = 64u32;
pub const IP6OPT_TYPE_FORCEICMP: u32 = 128u32;
pub const IP6OPT_TYPE_ICMP: u32 = 192u32;
pub const IP6OPT_TYPE_SKIP: u32 = 0u32;
pub const IP6T_SO_ORIGINAL_DST: u32 = 12303u32;
pub const IPPORT_BIFFUDP: u32 = 512u32;
pub const IPPORT_CHARGEN: u32 = 19u32;
pub const IPPORT_CMDSERVER: u32 = 514u32;
pub const IPPORT_DAYTIME: u32 = 13u32;
pub const IPPORT_DISCARD: u32 = 9u32;
pub const IPPORT_DYNAMIC_MAX: u32 = 65535u32;
pub const IPPORT_DYNAMIC_MIN: u32 = 49152u32;
pub const IPPORT_ECHO: u32 = 7u32;
pub const IPPORT_EFSSERVER: u32 = 520u32;
pub const IPPORT_EPMAP: u32 = 135u32;
pub const IPPORT_EXECSERVER: u32 = 512u32;
pub const IPPORT_FINGER: u32 = 79u32;
pub const IPPORT_FTP: u32 = 21u32;
pub const IPPORT_FTP_DATA: u32 = 20u32;
pub const IPPORT_HTTPS: u32 = 443u32;
pub const IPPORT_IMAP: u32 = 143u32;
pub const IPPORT_IMAP3: u32 = 220u32;
pub const IPPORT_LDAP: u32 = 389u32;
pub const IPPORT_LOGINSERVER: u32 = 513u32;
pub const IPPORT_MICROSOFT_DS: u32 = 445u32;
pub const IPPORT_MSP: u32 = 18u32;
pub const IPPORT_MTP: u32 = 57u32;
pub const IPPORT_NAMESERVER: u32 = 42u32;
pub const IPPORT_NETBIOS_DGM: u32 = 138u32;
pub const IPPORT_NETBIOS_NS: u32 = 137u32;
pub const IPPORT_NETBIOS_SSN: u32 = 139u32;
pub const IPPORT_NETSTAT: u32 = 15u32;
pub const IPPORT_NTP: u32 = 123u32;
pub const IPPORT_POP3: u32 = 110u32;
pub const IPPORT_QOTD: u32 = 17u32;
pub const IPPORT_REGISTERED_MAX: u32 = 49151u32;
pub const IPPORT_REGISTERED_MIN: u32 = 1024u32;
pub const IPPORT_RESERVED: u32 = 1024u32;
pub const IPPORT_RJE: u32 = 77u32;
pub const IPPORT_ROUTESERVER: u32 = 520u32;
pub const IPPORT_SMTP: u32 = 25u32;
pub const IPPORT_SNMP: u32 = 161u32;
pub const IPPORT_SNMP_TRAP: u32 = 162u32;
pub const IPPORT_SUPDUP: u32 = 95u32;
pub const IPPORT_SYSTAT: u32 = 11u32;
pub const IPPORT_TCPMUX: u32 = 1u32;
pub const IPPORT_TELNET: u32 = 23u32;
pub const IPPORT_TFTP: u32 = 69u32;
pub const IPPORT_TIMESERVER: u32 = 37u32;
pub const IPPORT_TTYLINK: u32 = 87u32;
pub const IPPORT_WHOIS: u32 = 43u32;
pub const IPPORT_WHOSERVER: u32 = 513u32;
pub type IPPROTO = i32;
pub const IPPROTO_HOPOPTS: IPPROTO = 0i32;
pub const IPPROTO_ICMP: IPPROTO = 1i32;
pub const IPPROTO_IGMP: IPPROTO = 2i32;
pub const IPPROTO_GGP: IPPROTO = 3i32;
pub const IPPROTO_IPV4: IPPROTO = 4i32;
pub const IPPROTO_ST: IPPROTO = 5i32;
pub const IPPROTO_TCP: IPPROTO = 6i32;
pub const IPPROTO_CBT: IPPROTO = 7i32;
pub const IPPROTO_EGP: IPPROTO = 8i32;
pub const IPPROTO_IGP: IPPROTO = 9i32;
pub const IPPROTO_PUP: IPPROTO = 12i32;
pub const IPPROTO_UDP: IPPROTO = 17i32;
pub const IPPROTO_IDP: IPPROTO = 22i32;
pub const IPPROTO_RDP: IPPROTO = 27i32;
pub const IPPROTO_IPV6: IPPROTO = 41i32;
pub const IPPROTO_ROUTING: IPPROTO = 43i32;
pub const IPPROTO_FRAGMENT: IPPROTO = 44i32;
pub const IPPROTO_ESP: IPPROTO = 50i32;
pub const IPPROTO_AH: IPPROTO = 51i32;
pub const IPPROTO_ICMPV6: IPPROTO = 58i32;
pub const IPPROTO_NONE: IPPROTO = 59i32;
pub const IPPROTO_DSTOPTS: IPPROTO = 60i32;
pub const IPPROTO_ND: IPPROTO = 77i32;
pub const IPPROTO_ICLFXBM: IPPROTO = 78i32;
pub const IPPROTO_PIM: IPPROTO = 103i32;
pub const IPPROTO_PGM: IPPROTO = 113i32;
pub const IPPROTO_L2TP: IPPROTO = 115i32;
pub const IPPROTO_SCTP: IPPROTO = 132i32;
pub const IPPROTO_RAW: IPPROTO = 255i32;
pub const IPPROTO_MAX: IPPROTO = 256i32;
pub const IPPROTO_RESERVED_RAW: IPPROTO = 257i32;
pub const IPPROTO_RESERVED_IPSEC: IPPROTO = 258i32;
pub const IPPROTO_RESERVED_IPSECOFFLOAD: IPPROTO = 259i32;
pub const IPPROTO_RESERVED_WNV: IPPROTO = 260i32;
pub const IPPROTO_RESERVED_MAX: IPPROTO = 261i32;
pub const IPPROTO_IP: u32 = 0u32;
pub const IPPROTO_RM: u32 = 113u32;
#[repr(C, packed(1))]
pub struct IPTLS_METADATA {
    pub SequenceNumber: u64,
}
impl ::core::marker::Copy for IPTLS_METADATA {}
impl ::core::clone::Clone for IPTLS_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_HEADER {
    pub Anonymous1: IPV4_HEADER_0,
    pub Anonymous2: IPV4_HEADER_1,
    pub TotalLength: u16,
    pub Identification: u16,
    pub Anonymous3: IPV4_HEADER_2,
    pub TimeToLive: u8,
    pub Protocol: u8,
    pub HeaderChecksum: u16,
    pub SourceAddress: IN_ADDR,
    pub DestinationAddress: IN_ADDR,
}
impl ::core::marker::Copy for IPV4_HEADER {}
impl ::core::clone::Clone for IPV4_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV4_HEADER_0 {
    pub VersionAndHeaderLength: u8,
    pub Anonymous: IPV4_HEADER_0_0,
}
impl ::core::marker::Copy for IPV4_HEADER_0 {}
impl ::core::clone::Clone for IPV4_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_HEADER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV4_HEADER_0_0 {}
impl ::core::clone::Clone for IPV4_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV4_HEADER_1 {
    pub TypeOfServiceAndEcnField: u8,
    pub Anonymous: IPV4_HEADER_1_0,
}
impl ::core::marker::Copy for IPV4_HEADER_1 {}
impl ::core::clone::Clone for IPV4_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_HEADER_1_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV4_HEADER_1_0 {}
impl ::core::clone::Clone for IPV4_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV4_HEADER_2 {
    pub FlagsAndOffset: u16,
    pub Anonymous: IPV4_HEADER_2_0,
}
impl ::core::marker::Copy for IPV4_HEADER_2 {}
impl ::core::clone::Clone for IPV4_HEADER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_HEADER_2_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for IPV4_HEADER_2_0 {}
impl ::core::clone::Clone for IPV4_HEADER_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV4_MAX_MINIMUM_MTU: u32 = 576u32;
pub const IPV4_MINIMUM_MTU: u32 = 576u32;
pub const IPV4_MIN_MINIMUM_MTU: u32 = 352u32;
#[repr(C)]
pub struct IPV4_OPTION_HEADER {
    pub Anonymous: IPV4_OPTION_HEADER_0,
    pub OptionLength: u8,
}
impl ::core::marker::Copy for IPV4_OPTION_HEADER {}
impl ::core::clone::Clone for IPV4_OPTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV4_OPTION_HEADER_0 {
    pub OptionType: u8,
    pub Anonymous: IPV4_OPTION_HEADER_0_0,
}
impl ::core::marker::Copy for IPV4_OPTION_HEADER_0 {}
impl ::core::clone::Clone for IPV4_OPTION_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_OPTION_HEADER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV4_OPTION_HEADER_0_0 {}
impl ::core::clone::Clone for IPV4_OPTION_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IPV4_OPTION_TYPE = i32;
pub const IP_OPT_EOL: IPV4_OPTION_TYPE = 0i32;
pub const IP_OPT_NOP: IPV4_OPTION_TYPE = 1i32;
pub const IP_OPT_SECURITY: IPV4_OPTION_TYPE = 130i32;
pub const IP_OPT_LSRR: IPV4_OPTION_TYPE = 131i32;
pub const IP_OPT_TS: IPV4_OPTION_TYPE = 68i32;
pub const IP_OPT_RR: IPV4_OPTION_TYPE = 7i32;
pub const IP_OPT_SSRR: IPV4_OPTION_TYPE = 137i32;
pub const IP_OPT_SID: IPV4_OPTION_TYPE = 136i32;
pub const IP_OPT_ROUTER_ALERT: IPV4_OPTION_TYPE = 148i32;
pub const IP_OPT_MULTIDEST: IPV4_OPTION_TYPE = 149i32;
#[repr(C)]
pub struct IPV4_ROUTING_HEADER {
    pub OptionHeader: IPV4_OPTION_HEADER,
    pub Pointer: u8,
}
impl ::core::marker::Copy for IPV4_ROUTING_HEADER {}
impl ::core::clone::Clone for IPV4_ROUTING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_TIMESTAMP_OPTION {
    pub OptionHeader: IPV4_OPTION_HEADER,
    pub Pointer: u8,
    pub Anonymous: IPV4_TIMESTAMP_OPTION_0,
}
impl ::core::marker::Copy for IPV4_TIMESTAMP_OPTION {}
impl ::core::clone::Clone for IPV4_TIMESTAMP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV4_TIMESTAMP_OPTION_0 {
    pub FlagsOverflow: u8,
    pub Anonymous: IPV4_TIMESTAMP_OPTION_0_0,
}
impl ::core::marker::Copy for IPV4_TIMESTAMP_OPTION_0 {}
impl ::core::clone::Clone for IPV4_TIMESTAMP_OPTION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV4_TIMESTAMP_OPTION_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV4_TIMESTAMP_OPTION_0_0 {}
impl ::core::clone::Clone for IPV4_TIMESTAMP_OPTION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV4_VERSION: u32 = 4u32;
pub const IPV6_ADD_IFLIST: u32 = 29u32;
pub const IPV6_ADD_MEMBERSHIP: u32 = 12u32;
pub const IPV6_CHECKSUM: u32 = 26u32;
pub const IPV6_DEL_IFLIST: u32 = 30u32;
pub const IPV6_DONTFRAG: u32 = 14u32;
pub const IPV6_DROP_MEMBERSHIP: u32 = 13u32;
pub const IPV6_ECN: u32 = 50u32;
pub const IPV6_ECN_MASK: u32 = 12288u32;
pub const IPV6_ECN_SHIFT: u32 = 12u32;
#[repr(C)]
pub struct IPV6_EXTENSION_HEADER {
    pub NextHeader: u8,
    pub Length: u8,
}
impl ::core::marker::Copy for IPV6_EXTENSION_HEADER {}
impl ::core::clone::Clone for IPV6_EXTENSION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV6_FLOW_LABEL_MASK: u32 = 4294905600u32;
#[repr(C)]
pub struct IPV6_FRAGMENT_HEADER {
    pub NextHeader: u8,
    pub Reserved: u8,
    pub Anonymous: IPV6_FRAGMENT_HEADER_0,
    pub Id: u32,
}
impl ::core::marker::Copy for IPV6_FRAGMENT_HEADER {}
impl ::core::clone::Clone for IPV6_FRAGMENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV6_FRAGMENT_HEADER_0 {
    pub Anonymous: IPV6_FRAGMENT_HEADER_0_0,
    pub OffsetAndFlags: u16,
}
impl ::core::marker::Copy for IPV6_FRAGMENT_HEADER_0 {}
impl ::core::clone::Clone for IPV6_FRAGMENT_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_FRAGMENT_HEADER_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for IPV6_FRAGMENT_HEADER_0_0 {}
impl ::core::clone::Clone for IPV6_FRAGMENT_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV6_FULL_TRAFFIC_CLASS_MASK: u32 = 61455u32;
pub const IPV6_GET_IFLIST: u32 = 33u32;
pub const IPV6_HDRINCL: u32 = 2u32;
#[repr(C)]
pub struct IPV6_HEADER {
    pub Anonymous: IPV6_HEADER_0,
    pub PayloadLength: u16,
    pub NextHeader: u8,
    pub HopLimit: u8,
    pub SourceAddress: IN6_ADDR,
    pub DestinationAddress: IN6_ADDR,
}
impl ::core::marker::Copy for IPV6_HEADER {}
impl ::core::clone::Clone for IPV6_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IPV6_HEADER_0 {
    pub VersionClassFlow: u32,
    pub Anonymous: IPV6_HEADER_0_0,
}
impl ::core::marker::Copy for IPV6_HEADER_0 {}
impl ::core::clone::Clone for IPV6_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_HEADER_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IPV6_HEADER_0_0 {}
impl ::core::clone::Clone for IPV6_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV6_HOPLIMIT: u32 = 21u32;
pub const IPV6_HOPOPTS: u32 = 1u32;
pub const IPV6_IFLIST: u32 = 28u32;
pub const IPV6_JOIN_GROUP: u32 = 12u32;
pub const IPV6_LEAVE_GROUP: u32 = 13u32;
pub const IPV6_MINIMUM_MTU: u32 = 1280u32;
#[repr(C)]
pub struct IPV6_MREQ {
    pub ipv6mr_multiaddr: IN6_ADDR,
    pub ipv6mr_interface: u32,
}
impl ::core::marker::Copy for IPV6_MREQ {}
impl ::core::clone::Clone for IPV6_MREQ {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV6_MTU: u32 = 72u32;
pub const IPV6_MTU_DISCOVER: u32 = 71u32;
pub const IPV6_MULTICAST_HOPS: u32 = 10u32;
pub const IPV6_MULTICAST_IF: u32 = 9u32;
pub const IPV6_MULTICAST_LOOP: u32 = 11u32;
#[repr(C)]
pub union IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    pub Anonymous: IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0,
    pub Value: u32,
}
impl ::core::marker::Copy for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {}
impl ::core::clone::Clone for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl ::core::marker::Copy for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {}
impl ::core::clone::Clone for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV6_NRT_INTERFACE: u32 = 74u32;
#[repr(C)]
pub struct IPV6_OPTION_HEADER {
    pub Type: u8,
    pub DataLength: u8,
}
impl ::core::marker::Copy for IPV6_OPTION_HEADER {}
impl ::core::clone::Clone for IPV6_OPTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_OPTION_JUMBOGRAM {
    pub Header: IPV6_OPTION_HEADER,
    pub JumbogramLength: [u8; 4],
}
impl ::core::marker::Copy for IPV6_OPTION_JUMBOGRAM {}
impl ::core::clone::Clone for IPV6_OPTION_JUMBOGRAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_OPTION_ROUTER_ALERT {
    pub Header: IPV6_OPTION_HEADER,
    pub Value: [u8; 2],
}
impl ::core::marker::Copy for IPV6_OPTION_ROUTER_ALERT {}
impl ::core::clone::Clone for IPV6_OPTION_ROUTER_ALERT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IPV6_OPTION_TYPE = i32;
pub const IP6OPT_PAD1: IPV6_OPTION_TYPE = 0i32;
pub const IP6OPT_PADN: IPV6_OPTION_TYPE = 1i32;
pub const IP6OPT_TUNNEL_LIMIT: IPV6_OPTION_TYPE = 4i32;
pub const IP6OPT_ROUTER_ALERT: IPV6_OPTION_TYPE = 5i32;
pub const IP6OPT_JUMBO: IPV6_OPTION_TYPE = 194i32;
pub const IP6OPT_NSAP_ADDR: IPV6_OPTION_TYPE = 195i32;
pub const IPV6_PKTINFO: u32 = 19u32;
pub const IPV6_PKTINFO_EX: u32 = 51u32;
pub const IPV6_PROTECTION_LEVEL: u32 = 23u32;
pub const IPV6_RECVDSTADDR: u32 = 25u32;
pub const IPV6_RECVECN: u32 = 50u32;
pub const IPV6_RECVERR: u32 = 75u32;
pub const IPV6_RECVIF: u32 = 24u32;
pub const IPV6_RECVRTHDR: u32 = 38u32;
pub const IPV6_RECVTCLASS: u32 = 40u32;
#[repr(C)]
pub union IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    pub Anonymous: IPV6_ROUTER_ADVERTISEMENT_FLAGS_0,
    pub Value: u8,
}
impl ::core::marker::Copy for IPV6_ROUTER_ADVERTISEMENT_FLAGS {}
impl ::core::clone::Clone for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {}
impl ::core::clone::Clone for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IPV6_ROUTING_HEADER {
    pub NextHeader: u8,
    pub Length: u8,
    pub RoutingType: u8,
    pub SegmentsLeft: u8,
    pub Reserved: [u8; 4],
}
impl ::core::marker::Copy for IPV6_ROUTING_HEADER {}
impl ::core::clone::Clone for IPV6_ROUTING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPV6_RTHDR: u32 = 32u32;
pub const IPV6_TCLASS: u32 = 39u32;
pub const IPV6_TRAFFIC_CLASS_MASK: u32 = 49167u32;
pub const IPV6_UNICAST_HOPS: u32 = 4u32;
pub const IPV6_UNICAST_IF: u32 = 31u32;
pub const IPV6_USER_MTU: u32 = 76u32;
pub const IPV6_V6ONLY: u32 = 27u32;
pub const IPV6_VERSION: u32 = 96u32;
pub const IPV6_WFP_REDIRECT_CONTEXT: u32 = 70u32;
pub const IPV6_WFP_REDIRECT_RECORDS: u32 = 60u32;
pub const IPX_ADDRESS: u32 = 16391u32;
#[repr(C)]
pub struct IPX_ADDRESS_DATA {
    pub adapternum: i32,
    pub netnum: [u8; 4],
    pub nodenum: [u8; 6],
    pub wan: ::win32_foundation_sys::BOOLEAN,
    pub status: ::win32_foundation_sys::BOOLEAN,
    pub maxpkt: i32,
    pub linkspeed: u32,
}
impl ::core::marker::Copy for IPX_ADDRESS_DATA {}
impl ::core::clone::Clone for IPX_ADDRESS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPX_ADDRESS_NOTIFY: u32 = 16396u32;
pub const IPX_DSTYPE: u32 = 16386u32;
pub const IPX_EXTENDED_ADDRESS: u32 = 16388u32;
pub const IPX_FILTERPTYPE: u32 = 16385u32;
pub const IPX_GETNETINFO: u32 = 16392u32;
pub const IPX_GETNETINFO_NORIP: u32 = 16393u32;
pub const IPX_IMMEDIATESPXACK: u32 = 16400u32;
pub const IPX_MAXSIZE: u32 = 16390u32;
pub const IPX_MAX_ADAPTER_NUM: u32 = 16397u32;
#[repr(C)]
pub struct IPX_NETNUM_DATA {
    pub netnum: [u8; 4],
    pub hopcount: u16,
    pub netdelay: u16,
    pub cardnum: i32,
    pub router: [u8; 6],
}
impl ::core::marker::Copy for IPX_NETNUM_DATA {}
impl ::core::clone::Clone for IPX_NETNUM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPX_PTYPE: u32 = 16384u32;
pub const IPX_RECEIVE_BROADCAST: u32 = 16399u32;
pub const IPX_RECVHDR: u32 = 16389u32;
pub const IPX_RERIPNETNUMBER: u32 = 16398u32;
#[repr(C)]
pub struct IPX_SPXCONNSTATUS_DATA {
    pub ConnectionState: u8,
    pub WatchDogActive: u8,
    pub LocalConnectionId: u16,
    pub RemoteConnectionId: u16,
    pub LocalSequenceNumber: u16,
    pub LocalAckNumber: u16,
    pub LocalAllocNumber: u16,
    pub RemoteAckNumber: u16,
    pub RemoteAllocNumber: u16,
    pub LocalSocket: u16,
    pub ImmediateAddress: [u8; 6],
    pub RemoteNetwork: [u8; 4],
    pub RemoteNode: [u8; 6],
    pub RemoteSocket: u16,
    pub RetransmissionCount: u16,
    pub EstimatedRoundTripDelay: u16,
    pub RetransmittedPackets: u16,
    pub SuppressedPacket: u16,
}
impl ::core::marker::Copy for IPX_SPXCONNSTATUS_DATA {}
impl ::core::clone::Clone for IPX_SPXCONNSTATUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IPX_SPXGETCONNECTIONSTATUS: u32 = 16395u32;
pub const IPX_STOPFILTERPTYPE: u32 = 16387u32;
pub const IP_ADD_IFLIST: u32 = 29u32;
pub const IP_ADD_MEMBERSHIP: u32 = 12u32;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 15u32;
pub const IP_BLOCK_SOURCE: u32 = 17u32;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1u32;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1u32;
pub const IP_DEL_IFLIST: u32 = 30u32;
pub const IP_DONTFRAGMENT: u32 = 14u32;
pub const IP_DROP_MEMBERSHIP: u32 = 13u32;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 16u32;
pub const IP_ECN: u32 = 50u32;
pub const IP_GET_IFLIST: u32 = 33u32;
pub const IP_HDRINCL: u32 = 2u32;
pub const IP_HOPLIMIT: u32 = 21u32;
pub const IP_IFLIST: u32 = 28u32;
pub const IP_MAX_MEMBERSHIPS: u32 = 20u32;
#[repr(C)]
pub struct IP_MREQ {
    pub imr_multiaddr: IN_ADDR,
    pub imr_interface: IN_ADDR,
}
impl ::core::marker::Copy for IP_MREQ {}
impl ::core::clone::Clone for IP_MREQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IP_MREQ_SOURCE {
    pub imr_multiaddr: IN_ADDR,
    pub imr_sourceaddr: IN_ADDR,
    pub imr_interface: IN_ADDR,
}
impl ::core::marker::Copy for IP_MREQ_SOURCE {}
impl ::core::clone::Clone for IP_MREQ_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IP_MSFILTER {
    pub imsf_multiaddr: IN_ADDR,
    pub imsf_interface: IN_ADDR,
    pub imsf_fmode: MULTICAST_MODE_TYPE,
    pub imsf_numsrc: u32,
    pub imsf_slist: [IN_ADDR; 1],
}
impl ::core::marker::Copy for IP_MSFILTER {}
impl ::core::clone::Clone for IP_MSFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IP_MTU: u32 = 73u32;
pub const IP_MTU_DISCOVER: u32 = 71u32;
pub const IP_MULTICAST_IF: u32 = 9u32;
pub const IP_MULTICAST_LOOP: u32 = 11u32;
pub const IP_MULTICAST_TTL: u32 = 10u32;
pub const IP_NRT_INTERFACE: u32 = 74u32;
pub const IP_OPTIONS: u32 = 1u32;
pub type IP_OPTION_TIMESTAMP_FLAGS = i32;
pub const IP_OPTION_TIMESTAMP_ONLY: IP_OPTION_TIMESTAMP_FLAGS = 0i32;
pub const IP_OPTION_TIMESTAMP_ADDRESS: IP_OPTION_TIMESTAMP_FLAGS = 1i32;
pub const IP_OPTION_TIMESTAMP_SPECIFIC_ADDRESS: IP_OPTION_TIMESTAMP_FLAGS = 3i32;
pub const IP_ORIGINAL_ARRIVAL_IF: u32 = 47u32;
pub const IP_PKTINFO: u32 = 19u32;
pub const IP_PKTINFO_EX: u32 = 51u32;
pub const IP_PROTECTION_LEVEL: u32 = 23u32;
pub const IP_RECEIVE_BROADCAST: u32 = 22u32;
pub const IP_RECVDSTADDR: u32 = 25u32;
pub const IP_RECVECN: u32 = 50u32;
pub const IP_RECVERR: u32 = 75u32;
pub const IP_RECVIF: u32 = 24u32;
pub const IP_RECVRTHDR: u32 = 38u32;
pub const IP_RECVTCLASS: u32 = 40u32;
pub const IP_RECVTOS: u32 = 40u32;
pub const IP_RECVTTL: u32 = 21u32;
pub const IP_RTHDR: u32 = 32u32;
pub const IP_TCLASS: u32 = 39u32;
pub const IP_TOS: u32 = 3u32;
pub const IP_TTL: u32 = 4u32;
pub const IP_UNBLOCK_SOURCE: u32 = 18u32;
pub const IP_UNICAST_IF: u32 = 31u32;
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1i32;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1i32;
pub const IP_UNSPECIFIED_USER_MTU: u32 = 4294967295u32;
pub const IP_USER_MTU: u32 = 76u32;
pub const IP_VER_MASK: u32 = 240u32;
pub const IP_WFP_REDIRECT_CONTEXT: u32 = 70u32;
pub const IP_WFP_REDIRECT_RECORDS: u32 = 60u32;
pub const IRDA_PROTO_SOCK_STREAM: u32 = 1u32;
pub const IRLMP_9WIRE_MODE: u32 = 22u32;
pub const IRLMP_DISCOVERY_MODE: u32 = 25u32;
pub const IRLMP_ENUMDEVICES: u32 = 16u32;
pub const IRLMP_EXCLUSIVE_MODE: u32 = 20u32;
pub const IRLMP_IAS_QUERY: u32 = 18u32;
pub const IRLMP_IAS_SET: u32 = 17u32;
pub const IRLMP_IRLPT_MODE: u32 = 21u32;
pub const IRLMP_PARAMETERS: u32 = 24u32;
pub const IRLMP_SEND_PDU_LEN: u32 = 19u32;
pub const IRLMP_SHARP_MODE: u32 = 32u32;
pub const IRLMP_TINYTP_MODE: u32 = 23u32;
pub const ISOPROTO_CLNP: u32 = 31u32;
pub const ISOPROTO_CLTP: u32 = 30u32;
pub const ISOPROTO_ESIS: u32 = 34u32;
pub const ISOPROTO_INACT_NL: u32 = 33u32;
pub const ISOPROTO_INTRAISIS: u32 = 35u32;
pub const ISOPROTO_TP: u32 = 29u32;
pub const ISOPROTO_TP0: u32 = 25u32;
pub const ISOPROTO_TP1: u32 = 26u32;
pub const ISOPROTO_TP2: u32 = 27u32;
pub const ISOPROTO_TP3: u32 = 28u32;
pub const ISOPROTO_TP4: u32 = 29u32;
pub const ISOPROTO_X25: u32 = 32u32;
pub const ISO_EXP_DATA_NUSE: u32 = 1u32;
pub const ISO_EXP_DATA_USE: u32 = 0u32;
pub const ISO_HIERARCHICAL: u32 = 0u32;
pub const ISO_MAX_ADDR_LENGTH: u32 = 64u32;
pub const ISO_NON_HIERARCHICAL: u32 = 1u32;
pub const JL_BOTH: u32 = 4u32;
pub const JL_RECEIVER_ONLY: u32 = 2u32;
pub const JL_SENDER_ONLY: u32 = 1u32;
pub const LAYERED_PROTOCOL: u32 = 0u32;
pub const LITTLEENDIAN: u32 = 1u32;
pub const LM_BAUD_115200: u32 = 115200u32;
pub const LM_BAUD_1152K: u32 = 1152000u32;
pub const LM_BAUD_1200: u32 = 1200u32;
pub const LM_BAUD_16M: u32 = 16000000u32;
pub const LM_BAUD_19200: u32 = 19200u32;
pub const LM_BAUD_2400: u32 = 2400u32;
pub const LM_BAUD_38400: u32 = 38400u32;
pub const LM_BAUD_4M: u32 = 4000000u32;
pub const LM_BAUD_57600: u32 = 57600u32;
pub const LM_BAUD_576K: u32 = 576000u32;
pub const LM_BAUD_9600: u32 = 9600u32;
pub const LM_HB1_Computer: i32 = 4i32;
pub const LM_HB1_Fax: i32 = 32i32;
pub const LM_HB1_LANAccess: i32 = 64i32;
pub const LM_HB1_Modem: i32 = 16i32;
pub const LM_HB1_PDA_Palmtop: i32 = 2i32;
pub const LM_HB1_PnP: i32 = 1i32;
pub const LM_HB1_Printer: i32 = 8i32;
pub const LM_HB2_FileServer: i32 = 2i32;
pub const LM_HB2_Telephony: i32 = 1i32;
pub const LM_HB_Extension: i32 = 128i32;
#[repr(C)]
pub struct LM_IRPARMS {
    pub nTXDataBytes: u32,
    pub nRXDataBytes: u32,
    pub nBaudRate: u32,
    pub thresholdTime: u32,
    pub discTime: u32,
    pub nMSLinkTurn: u16,
    pub nTXPackets: u8,
    pub nRXPackets: u8,
}
impl ::core::marker::Copy for LM_IRPARMS {}
impl ::core::clone::Clone for LM_IRPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LOG2_BITS_PER_BYTE: u32 = 3u32;
pub type LPBLOCKINGCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcontext: usize) -> ::win32_foundation_sys::BOOL>;
pub type LPCONDITIONPROC = ::core::option::Option<unsafe extern "system" fn(lpcallerid: *mut WSABUF, lpcallerdata: *mut WSABUF, lpsqos: *mut QOS, lpgqos: *mut QOS, lpcalleeid: *mut WSABUF, lpcalleedata: *mut WSABUF, g: *mut u32, dwcallbackdata: usize) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_ACCEPTEX = ::core::option::Option<unsafe extern "system" fn(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_CONNECTEX = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpsendbuffer: *const ::core::ffi::c_void, dwsenddatalength: u32, lpdwbytessent: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_DISCONNECTEX = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, dwflags: u32, dwreserved: u32) -> ::win32_foundation_sys::BOOL>;
pub type LPFN_GETACCEPTEXSOCKADDRS = ::core::option::Option<unsafe extern "system" fn(lpoutputbuffer: *const ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32)>;
pub type LPFN_NSPAPI = ::core::option::Option<unsafe extern "system" fn() -> u32>;
pub type LPFN_RIOCLOSECOMPLETIONQUEUE = ::core::option::Option<unsafe extern "system" fn(cq: *const RIO_CQ_t)>;
pub type LPFN_RIOCREATECOMPLETIONQUEUE = ::core::option::Option<unsafe extern "system" fn(queuesize: u32, notificationcompletion: *const RIO_NOTIFICATION_COMPLETION) -> *mut RIO_CQ_t>;
pub type LPFN_RIOCREATEREQUESTQUEUE = ::core::option::Option<unsafe extern "system" fn(socket: SOCKET, maxoutstandingreceive: u32, maxreceivedatabuffers: u32, maxoutstandingsend: u32, maxsenddatabuffers: u32, receivecq: *const RIO_CQ_t, sendcq: *const RIO_CQ_t, socketcontext: *const ::core::ffi::c_void) -> *mut RIO_RQ_t>;
pub type LPFN_RIODEQUEUECOMPLETION = ::core::option::Option<unsafe extern "system" fn(cq: *const RIO_CQ_t, array: *mut RIORESULT, arraysize: u32) -> u32>;
pub type LPFN_RIODEREGISTERBUFFER = ::core::option::Option<unsafe extern "system" fn(bufferid: *const RIO_BUFFERID_t)>;
pub type LPFN_RIONOTIFY = ::core::option::Option<unsafe extern "system" fn(cq: *const RIO_CQ_t) -> i32>;
pub type LPFN_RIORECEIVE = ::core::option::Option<unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL>;
pub type LPFN_RIORECEIVEEX = ::core::option::Option<unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, plocaladdress: *const RIO_BUF, premoteaddress: *const RIO_BUF, pcontrolcontext: *const RIO_BUF, pflags: *const RIO_BUF, flags: u32, requestcontext: *const ::core::ffi::c_void) -> i32>;
pub type LPFN_RIOREGISTERBUFFER = ::core::option::Option<unsafe extern "system" fn(databuffer: ::windows_core_sys::PCSTR, datalength: u32) -> *mut RIO_BUFFERID_t>;
pub type LPFN_RIORESIZECOMPLETIONQUEUE = ::core::option::Option<unsafe extern "system" fn(cq: *const RIO_CQ_t, queuesize: u32) -> ::win32_foundation_sys::BOOL>;
pub type LPFN_RIORESIZEREQUESTQUEUE = ::core::option::Option<unsafe extern "system" fn(rq: *const RIO_RQ_t, maxoutstandingreceive: u32, maxoutstandingsend: u32) -> ::win32_foundation_sys::BOOL>;
pub type LPFN_RIOSEND = ::core::option::Option<unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL>;
pub type LPFN_RIOSENDEX = ::core::option::Option<unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, plocaladdress: *const RIO_BUF, premoteaddress: *const RIO_BUF, pcontrolcontext: *const RIO_BUF, pflags: *const RIO_BUF, flags: u32, requestcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_TRANSMITFILE = ::core::option::Option<unsafe extern "system" fn(hsocket: SOCKET, hfile: ::win32_foundation_sys::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_TRANSMITPACKETS = ::core::option::Option<unsafe extern "system" fn(hsocket: SOCKET, lppacketarray: *const TRANSMIT_PACKETS_ELEMENT, nelementcount: u32, nsendsize: u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, dwflags: u32) -> ::win32_foundation_sys::BOOL>;
pub type LPFN_WSAPOLL = ::core::option::Option<unsafe extern "system" fn(fdarray: *mut WSAPOLLFD, nfds: u32, timeout: i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_WSARECVMSG = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpmsg: *mut WSAMSG, lpdwnumberofbytesrecvd: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPFN_WSASENDMSG = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED)>;
pub type LPNSPCLEANUP = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID) -> i32>;
pub type LPNSPGETSERVICECLASSINFO = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpdwbufsize: *const u32, lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32>;
pub type LPNSPINSTALLSERVICECLASS = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPIOCTL = ::core::option::Option<unsafe extern "system" fn(hlookup: ::win32_foundation_sys::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const WSACOMPLETION, lpthreadid: *const WSATHREADID) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPLOOKUPSERVICEBEGIN = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpqsrestrictions: *const WSAQUERYSETW, lpserviceclassinfo: *const WSASERVICECLASSINFOW, dwcontrolflags: u32, lphlookup: *mut ::win32_foundation_sys::HANDLE) -> i32>;
pub type LPNSPLOOKUPSERVICEEND = ::core::option::Option<unsafe extern "system" fn(hlookup: ::win32_foundation_sys::HANDLE) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPLOOKUPSERVICENEXT = ::core::option::Option<unsafe extern "system" fn(hlookup: ::win32_foundation_sys::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32>;
pub type LPNSPREMOVESERVICECLASS = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpserviceclassid: *const ::windows_core_sys::GUID) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPSETSERVICE = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpserviceclassinfo: *const WSASERVICECLASSINFOW, lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32>;
#[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
pub type LPNSPSTARTUP = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpnsproutines: *mut NSP_ROUTINE) -> i32>;
pub type LPNSPV2CLEANUP = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, pvclientsessionarg: *const ::core::ffi::c_void) -> i32>;
pub type LPNSPV2CLIENTSESSIONRUNDOWN = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, pvclientsessionarg: *const ::core::ffi::c_void)>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPV2LOOKUPSERVICEBEGIN = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpqsrestrictions: *const WSAQUERYSET2W, dwcontrolflags: u32, lpvclientsessionarg: *const ::core::ffi::c_void, lphlookup: *mut ::win32_foundation_sys::HANDLE) -> i32>;
pub type LPNSPV2LOOKUPSERVICEEND = ::core::option::Option<unsafe extern "system" fn(hlookup: ::win32_foundation_sys::HANDLE) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPV2LOOKUPSERVICENEXTEX = ::core::option::Option<unsafe extern "system" fn(hasynccall: ::win32_foundation_sys::HANDLE, hlookup: ::win32_foundation_sys::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *const u32, lpqsresults: *mut WSAQUERYSET2W)>;
#[cfg(feature = "win32-system-sys")]
pub type LPNSPV2SETSERVICEEX = ::core::option::Option<unsafe extern "system" fn(hasynccall: ::win32_foundation_sys::HANDLE, lpproviderid: *const ::windows_core_sys::GUID, lpqsreginfo: *const WSAQUERYSET2W, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32, lpvclientsessionarg: *const ::core::ffi::c_void)>;
pub type LPNSPV2STARTUP = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, ppvclientsessionarg: *mut *mut ::core::ffi::c_void) -> i32>;
pub type LPSERVICE_CALLBACK_PROC = ::core::option::Option<unsafe extern "system" fn(lparam: ::win32_foundation_sys::LPARAM, hasynctaskhandle: ::win32_foundation_sys::HANDLE)>;
pub type LPWPUCLOSEEVENT = ::core::option::Option<unsafe extern "system" fn(hevent: ::win32_foundation_sys::HANDLE, lperrno: *mut i32) -> ::win32_foundation_sys::BOOL>;
pub type LPWPUCLOSESOCKETHANDLE = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lperrno: *mut i32) -> i32>;
pub type LPWPUCLOSETHREAD = ::core::option::Option<unsafe extern "system" fn(lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWPUCOMPLETEOVERLAPPEDREQUEST = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32>;
pub type LPWPUCREATEEVENT = ::core::option::Option<unsafe extern "system" fn(lperrno: *mut i32) -> ::win32_foundation_sys::HANDLE>;
pub type LPWPUCREATESOCKETHANDLE = ::core::option::Option<unsafe extern "system" fn(dwcatalogentryid: u32, dwcontext: usize, lperrno: *mut i32) -> SOCKET>;
pub type LPWPUFDISSET = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, fdset: *const fd_set) -> i32>;
pub type LPWPUGETPROVIDERPATH = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32>;
pub type LPWPUMODIFYIFSHANDLE = ::core::option::Option<unsafe extern "system" fn(dwcatalogentryid: u32, proposedhandle: SOCKET, lperrno: *mut i32) -> SOCKET>;
pub type LPWPUOPENCURRENTTHREAD = ::core::option::Option<unsafe extern "system" fn(lpthreadid: *mut WSATHREADID, lperrno: *mut i32) -> i32>;
pub type LPWPUPOSTMESSAGE = ::core::option::Option<unsafe extern "system" fn(hwnd: ::win32_foundation_sys::HWND, msg: u32, wparam: ::win32_foundation_sys::WPARAM, lparam: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
pub type LPWPUQUERYBLOCKINGCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcatalogentryid: u32, lplpfncallback: *mut LPBLOCKINGCALLBACK, lpdwcontext: *mut usize, lperrno: *mut i32) -> i32>;
pub type LPWPUQUERYSOCKETHANDLECONTEXT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpcontext: *mut usize, lperrno: *mut i32) -> i32>;
pub type LPWPUQUEUEAPC = ::core::option::Option<unsafe extern "system" fn(lpthreadid: *const WSATHREADID, lpfnuserapc: LPWSAUSERAPC, dwcontext: usize, lperrno: *mut i32) -> i32>;
pub type LPWPURESETEVENT = ::core::option::Option<unsafe extern "system" fn(hevent: ::win32_foundation_sys::HANDLE, lperrno: *mut i32) -> ::win32_foundation_sys::BOOL>;
pub type LPWPUSETEVENT = ::core::option::Option<unsafe extern "system" fn(hevent: ::win32_foundation_sys::HANDLE, lperrno: *mut i32) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerror: u32, cbtransferred: u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, dwflags: u32)>;
pub type LPWSAUSERAPC = ::core::option::Option<unsafe extern "system" fn(dwcontext: usize)>;
pub type LPWSCDEINSTALLPROVIDER = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lperrno: *mut i32) -> i32>;
pub type LPWSCENABLENSPROVIDER = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, fenable: ::win32_foundation_sys::BOOL) -> i32>;
pub type LPWSCENUMPROTOCOLS = ::core::option::Option<unsafe extern "system" fn(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32>;
pub type LPWSCGETPROVIDERPATH = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32>;
pub type LPWSCINSTALLNAMESPACE = ::core::option::Option<unsafe extern "system" fn(lpszidentifier: ::windows_core_sys::PCWSTR, lpszpathname: ::windows_core_sys::PCWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_core_sys::GUID) -> i32>;
pub type LPWSCINSTALLPROVIDER = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32>;
pub type LPWSCUNINSTALLNAMESPACE = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID) -> i32>;
pub type LPWSCUPDATEPROVIDER = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *const ::windows_core_sys::GUID, lpszproviderdllpath: ::windows_core_sys::PCWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32>;
pub type LPWSCWRITENAMESPACEORDER = ::core::option::Option<unsafe extern "system" fn(lpproviderid: *mut ::windows_core_sys::GUID, dwnumberofentries: u32) -> i32>;
pub type LPWSCWRITEPROVIDERORDER = ::core::option::Option<unsafe extern "system" fn(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32>;
pub type LPWSPACCEPT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: LPCONDITIONPROC, dwcallbackdata: usize, lperrno: *mut i32) -> SOCKET>;
pub type LPWSPADDRESSTOSTRING = ::core::option::Option<unsafe extern "system" fn(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: ::windows_core_sys::PWSTR, lpdwaddressstringlength: *mut u32, lperrno: *mut i32) -> i32>;
pub type LPWSPASYNCSELECT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, hwnd: ::win32_foundation_sys::HWND, wmsg: u32, levent: i32, lperrno: *mut i32) -> i32>;
pub type LPWSPBIND = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lperrno: *mut i32) -> i32>;
pub type LPWSPCANCELBLOCKINGCALL = ::core::option::Option<unsafe extern "system" fn(lperrno: *mut i32) -> i32>;
pub type LPWSPCLEANUP = ::core::option::Option<unsafe extern "system" fn(lperrno: *mut i32) -> i32>;
pub type LPWSPCLOSESOCKET = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lperrno: *mut i32) -> i32>;
pub type LPWSPCONNECT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const QOS, lpgqos: *const QOS, lperrno: *mut i32) -> i32>;
pub type LPWSPDUPLICATESOCKET = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW, lperrno: *mut i32) -> i32>;
pub type LPWSPENUMNETWORKEVENTS = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, heventobject: ::win32_foundation_sys::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS, lperrno: *mut i32) -> i32>;
pub type LPWSPEVENTSELECT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, heventobject: ::win32_foundation_sys::HANDLE, lnetworkevents: i32, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPGETOVERLAPPEDRESULT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpoverlapped: *const ::win32_system_sys::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: ::win32_foundation_sys::BOOL, lpdwflags: *mut u32, lperrno: *mut i32) -> ::win32_foundation_sys::BOOL>;
pub type LPWSPGETPEERNAME = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32, lperrno: *mut i32) -> i32>;
pub type LPWSPGETQOSBYNAME = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut QOS, lperrno: *mut i32) -> ::win32_foundation_sys::BOOL>;
pub type LPWSPGETSOCKNAME = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32, lperrno: *mut i32) -> i32>;
pub type LPWSPGETSOCKOPT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, level: i32, optname: i32, optval: ::windows_core_sys::PSTR, optlen: *mut i32, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPIOCTL = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32>;
pub type LPWSPJOINLEAF = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const QOS, lpgqos: *const QOS, dwflags: u32, lperrno: *mut i32) -> SOCKET>;
pub type LPWSPLISTEN = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, backlog: i32, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPRECV = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *const i32) -> i32>;
pub type LPWSPRECVDISCONNECT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpinbounddisconnectdata: *const WSABUF, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPRECVFROM = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32>;
pub type LPWSPSELECT = ::core::option::Option<unsafe extern "system" fn(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPSEND = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32>;
pub type LPWSPSENDDISCONNECT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF, lperrno: *mut i32) -> i32>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPSENDTO = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut ::win32_system_sys::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32>;
pub type LPWSPSETSOCKOPT = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, level: i32, optname: i32, optval: ::windows_core_sys::PCSTR, optlen: i32, lperrno: *mut i32) -> i32>;
pub type LPWSPSHUTDOWN = ::core::option::Option<unsafe extern "system" fn(s: SOCKET, how: i32, lperrno: *mut i32) -> i32>;
pub type LPWSPSOCKET = ::core::option::Option<unsafe extern "system" fn(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32, lperrno: *mut i32) -> SOCKET>;
#[cfg(feature = "win32-system-sys")]
pub type LPWSPSTARTUP = ::core::option::Option<unsafe extern "system" fn(wversionrequested: u16, lpwspdata: *const WSPData, lpprotocolinfo: *const WSAPROTOCOL_INFOW, upcalltable: WSPUPCALLTABLE, lpproctable: *mut WSPPROC_TABLE) -> i32>;
pub type LPWSPSTRINGTOADDRESS = ::core::option::Option<unsafe extern "system" fn(addressstring: ::windows_core_sys::PCWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32, lperrno: *mut i32) -> i32>;
pub const LSP_CRYPTO_COMPRESS: u32 = 64u32;
pub const LSP_FIREWALL: u32 = 8u32;
pub const LSP_INBOUND_MODIFY: u32 = 16u32;
pub const LSP_INSPECTOR: u32 = 1u32;
pub const LSP_LOCAL_CACHE: u32 = 128u32;
pub const LSP_OUTBOUND_MODIFY: u32 = 32u32;
pub const LSP_PROXY: u32 = 4u32;
pub const LSP_REDIRECTOR: u32 = 2u32;
pub const LSP_SYSTEM: u32 = 2147483648u32;
pub const LUP_ADDRCONFIG: u32 = 1048576u32;
pub const LUP_API_ANSI: u32 = 16777216u32;
pub const LUP_CONTAINERS: u32 = 2u32;
pub const LUP_DEEP: u32 = 1u32;
pub const LUP_DISABLE_IDN_ENCODING: u32 = 8388608u32;
pub const LUP_DNS_ONLY: u32 = 131072u32;
pub const LUP_DUAL_ADDR: u32 = 2097152u32;
pub const LUP_EXCLUSIVE_CUSTOM_SERVERS: u32 = 134217728u32;
pub const LUP_EXTENDED_QUERYSET: u32 = 33554432u32;
pub const LUP_FILESERVER: u32 = 4194304u32;
pub const LUP_FLUSHCACHE: u32 = 4096u32;
pub const LUP_FLUSHPREVIOUS: u32 = 8192u32;
pub const LUP_FORCE_CLEAR_TEXT: u32 = 1073741824u32;
pub const LUP_NEAREST: u32 = 8u32;
pub const LUP_NOCONTAINERS: u32 = 4u32;
pub const LUP_NON_AUTHORITATIVE: u32 = 16384u32;
pub const LUP_REQUIRE_SECURE: u32 = 268435456u32;
pub const LUP_RESOLUTION_HANDLE: u32 = 2147483648u32;
pub const LUP_RES_SERVICE: u32 = 32768u32;
pub const LUP_RETURN_ADDR: u32 = 256u32;
pub const LUP_RETURN_ALIASES: u32 = 1024u32;
pub const LUP_RETURN_ALL: u32 = 4080u32;
pub const LUP_RETURN_BLOB: u32 = 512u32;
pub const LUP_RETURN_COMMENT: u32 = 128u32;
pub const LUP_RETURN_NAME: u32 = 16u32;
pub const LUP_RETURN_PREFERRED_NAMES: u32 = 65536u32;
pub const LUP_RETURN_QUERY_STRING: u32 = 2048u32;
pub const LUP_RETURN_RESPONSE_FLAGS: u32 = 262144u32;
pub const LUP_RETURN_TTL: u32 = 536870912u32;
pub const LUP_RETURN_TYPE: u32 = 32u32;
pub const LUP_RETURN_VERSION: u32 = 64u32;
pub const LUP_SECURE: u32 = 32768u32;
pub const LUP_SECURE_WITH_FALLBACK: u32 = 67108864u32;
pub const LmCharSetASCII: u32 = 0u32;
pub const LmCharSetISO_8859_1: u32 = 1u32;
pub const LmCharSetISO_8859_2: u32 = 2u32;
pub const LmCharSetISO_8859_3: u32 = 3u32;
pub const LmCharSetISO_8859_4: u32 = 4u32;
pub const LmCharSetISO_8859_5: u32 = 5u32;
pub const LmCharSetISO_8859_6: u32 = 6u32;
pub const LmCharSetISO_8859_7: u32 = 7u32;
pub const LmCharSetISO_8859_8: u32 = 8u32;
pub const LmCharSetISO_8859_9: u32 = 9u32;
pub const LmCharSetUNICODE: u32 = 255u32;
pub const MAXGETHOSTSTRUCT: u32 = 1024u32;
pub const MAX_IPV4_HLEN: u32 = 60u32;
pub const MAX_IPV4_PACKET: u32 = 65535u32;
pub const MAX_IPV6_PAYLOAD: u32 = 65535u32;
pub const MAX_MCAST_TTL: u32 = 255u32;
pub const MAX_PROTOCOL_CHAIN: u32 = 7u32;
pub const MAX_WINDOW_INCREMENT_PERCENTAGE: u32 = 25u32;
pub const MCAST_BLOCK_SOURCE: u32 = 43u32;
pub const MCAST_JOIN_GROUP: u32 = 41u32;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 45u32;
pub const MCAST_LEAVE_GROUP: u32 = 42u32;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 46u32;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44u32;
#[repr(C)]
pub struct MLDV2_QUERY_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub Anonymous1: MLDV2_QUERY_HEADER_0,
    pub Reserved: u16,
    pub MulticastAddress: IN6_ADDR,
    pub _bitfield: u8,
    pub Anonymous2: MLDV2_QUERY_HEADER_1,
    pub SourceCount: u16,
}
impl ::core::marker::Copy for MLDV2_QUERY_HEADER {}
impl ::core::clone::Clone for MLDV2_QUERY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MLDV2_QUERY_HEADER_0 {
    pub MaxRespCode: u16,
    pub Anonymous: MLDV2_QUERY_HEADER_0_0,
}
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_0 {}
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MLDV2_QUERY_HEADER_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_0_0 {}
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MLDV2_QUERY_HEADER_1 {
    pub QueriersQueryInterfaceCode: u8,
    pub Anonymous: MLDV2_QUERY_HEADER_1_0,
}
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_1 {}
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MLDV2_QUERY_HEADER_1_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_1_0 {}
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MLDV2_REPORT_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub Reserved: u16,
    pub RecordCount: u16,
}
impl ::core::marker::Copy for MLDV2_REPORT_HEADER {}
impl ::core::clone::Clone for MLDV2_REPORT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MLDV2_REPORT_RECORD_HEADER {
    pub Type: u8,
    pub AuxillaryDataLength: u8,
    pub SourceCount: u16,
    pub MulticastAddress: IN6_ADDR,
}
impl ::core::marker::Copy for MLDV2_REPORT_RECORD_HEADER {}
impl ::core::clone::Clone for MLDV2_REPORT_RECORD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MLD_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub MaxRespTime: u16,
    pub Reserved: u16,
    pub MulticastAddress: IN6_ADDR,
}
impl ::core::marker::Copy for MLD_HEADER {}
impl ::core::clone::Clone for MLD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MLD_MAX_RESP_CODE_TYPE = i32;
pub const MLD_MAX_RESP_CODE_TYPE_NORMAL: MLD_MAX_RESP_CODE_TYPE = 0i32;
pub const MLD_MAX_RESP_CODE_TYPE_FLOAT: MLD_MAX_RESP_CODE_TYPE = 1i32;
pub const MSG_BCAST: u32 = 1024u32;
pub const MSG_CTRUNC: u32 = 512u32;
pub const MSG_ERRQUEUE: u32 = 4096u32;
pub const MSG_INTERRUPT: u32 = 16u32;
pub const MSG_MAXIOVLEN: u32 = 16u32;
pub const MSG_MCAST: u32 = 2048u32;
pub const MSG_PARTIAL: u32 = 32768u32;
pub const MSG_TRUNC: u32 = 256u32;
pub type MULTICAST_MODE_TYPE = i32;
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = 0i32;
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = 1i32;
#[repr(C)]
pub struct NAPI_DOMAIN_DESCRIPTION_BLOB {
    pub AuthLevel: u32,
    pub cchDomainName: u32,
    pub OffsetNextDomainDescription: u32,
    pub OffsetThisDomainName: u32,
}
impl ::core::marker::Copy for NAPI_DOMAIN_DESCRIPTION_BLOB {}
impl ::core::clone::Clone for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NAPI_PROVIDER_INSTALLATION_BLOB {
    pub dwVersion: u32,
    pub dwProviderType: u32,
    pub fSupportsWildCard: u32,
    pub cDomains: u32,
    pub OffsetFirstDomain: u32,
}
impl ::core::marker::Copy for NAPI_PROVIDER_INSTALLATION_BLOB {}
impl ::core::clone::Clone for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NAPI_PROVIDER_LEVEL = i32;
pub const ProviderLevel_None: NAPI_PROVIDER_LEVEL = 0i32;
pub const ProviderLevel_Secondary: NAPI_PROVIDER_LEVEL = 1i32;
pub const ProviderLevel_Primary: NAPI_PROVIDER_LEVEL = 2i32;
pub type NAPI_PROVIDER_TYPE = i32;
pub const ProviderType_Application: NAPI_PROVIDER_TYPE = 1i32;
pub const ProviderType_Service: NAPI_PROVIDER_TYPE = 2i32;
pub const ND_NA_FLAG_OVERRIDE: u32 = 536870912u32;
pub const ND_NA_FLAG_ROUTER: u32 = 2147483648u32;
pub const ND_NA_FLAG_SOLICITED: u32 = 1073741824u32;
pub type ND_OPTION_TYPE = i32;
pub const ND_OPT_SOURCE_LINKADDR: ND_OPTION_TYPE = 1i32;
pub const ND_OPT_TARGET_LINKADDR: ND_OPTION_TYPE = 2i32;
pub const ND_OPT_PREFIX_INFORMATION: ND_OPTION_TYPE = 3i32;
pub const ND_OPT_REDIRECTED_HEADER: ND_OPTION_TYPE = 4i32;
pub const ND_OPT_MTU: ND_OPTION_TYPE = 5i32;
pub const ND_OPT_NBMA_SHORTCUT_LIMIT: ND_OPTION_TYPE = 6i32;
pub const ND_OPT_ADVERTISEMENT_INTERVAL: ND_OPTION_TYPE = 7i32;
pub const ND_OPT_HOME_AGENT_INFORMATION: ND_OPTION_TYPE = 8i32;
pub const ND_OPT_SOURCE_ADDR_LIST: ND_OPTION_TYPE = 9i32;
pub const ND_OPT_TARGET_ADDR_LIST: ND_OPTION_TYPE = 10i32;
pub const ND_OPT_ROUTE_INFO: ND_OPTION_TYPE = 24i32;
pub const ND_OPT_RDNSS: ND_OPTION_TYPE = 25i32;
pub const ND_OPT_DNSSL: ND_OPTION_TYPE = 31i32;
pub const ND_OPT_DNSSL_MIN_LEN: u32 = 16u32;
pub const ND_OPT_PI_FLAG_AUTO: u32 = 64u32;
pub const ND_OPT_PI_FLAG_ONLINK: u32 = 128u32;
pub const ND_OPT_PI_FLAG_ROUTE: u32 = 1u32;
pub const ND_OPT_PI_FLAG_ROUTER_ADDR: u32 = 32u32;
pub const ND_OPT_PI_FLAG_SITE_PREFIX: u32 = 16u32;
pub const ND_OPT_RDNSS_MIN_LEN: u32 = 24u32;
pub const ND_OPT_RI_FLAG_PREFERENCE: u32 = 24u32;
pub const ND_RA_FLAG_HOME_AGENT: u32 = 32u32;
pub const ND_RA_FLAG_MANAGED: u32 = 128u32;
pub const ND_RA_FLAG_OTHER: u32 = 64u32;
pub const ND_RA_FLAG_PREFERENCE: u32 = 24u32;
pub const NETBIOS_GROUP_NAME: u32 = 1u32;
pub const NETBIOS_NAME_LENGTH: u32 = 16u32;
pub const NETBIOS_TYPE_QUICK_GROUP: u32 = 3u32;
pub const NETBIOS_TYPE_QUICK_UNIQUE: u32 = 2u32;
pub const NETBIOS_UNIQUE_NAME: u32 = 0u32;
#[repr(C)]
pub struct NETRESOURCE2A {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwUsage: u32,
    pub dwDisplayType: u32,
    pub lpLocalName: ::windows_core_sys::PSTR,
    pub lpRemoteName: ::windows_core_sys::PSTR,
    pub lpComment: ::windows_core_sys::PSTR,
    pub ns_info: NS_INFOA,
    pub ServiceType: ::windows_core_sys::GUID,
    pub dwProtocols: u32,
    pub lpiProtocols: *mut i32,
}
impl ::core::marker::Copy for NETRESOURCE2A {}
impl ::core::clone::Clone for NETRESOURCE2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NETRESOURCE2W {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwUsage: u32,
    pub dwDisplayType: u32,
    pub lpLocalName: ::windows_core_sys::PWSTR,
    pub lpRemoteName: ::windows_core_sys::PWSTR,
    pub lpComment: ::windows_core_sys::PWSTR,
    pub ns_info: NS_INFOA,
    pub ServiceType: ::windows_core_sys::GUID,
    pub dwProtocols: u32,
    pub lpiProtocols: *mut i32,
}
impl ::core::marker::Copy for NETRESOURCE2W {}
impl ::core::clone::Clone for NETRESOURCE2W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NI_DGRAM: u32 = 16u32;
pub const NI_MAXHOST: u32 = 1025u32;
pub const NI_MAXSERV: u32 = 32u32;
pub const NI_NAMEREQD: u32 = 4u32;
pub const NI_NOFQDN: u32 = 1u32;
pub const NI_NUMERICHOST: u32 = 2u32;
pub const NI_NUMERICSERV: u32 = 8u32;
pub const NLA_ALLUSERS_NETWORK: u32 = 1u32;
#[repr(C)]
pub struct NLA_BLOB {
    pub header: NLA_BLOB_1,
    pub data: NLA_BLOB_0,
}
impl ::core::marker::Copy for NLA_BLOB {}
impl ::core::clone::Clone for NLA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union NLA_BLOB_0 {
    pub rawData: [::win32_foundation_sys::CHAR; 1],
    pub interfaceData: NLA_BLOB_0_2,
    pub locationData: NLA_BLOB_0_3,
    pub connectivity: NLA_BLOB_0_1,
    pub ICS: NLA_BLOB_0_0,
}
impl ::core::marker::Copy for NLA_BLOB_0 {}
impl ::core::clone::Clone for NLA_BLOB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLA_BLOB_0_0 {
    pub remote: NLA_BLOB_0_0_0,
}
impl ::core::marker::Copy for NLA_BLOB_0_0 {}
impl ::core::clone::Clone for NLA_BLOB_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLA_BLOB_0_0_0 {
    pub speed: u32,
    pub r#type: u32,
    pub state: u32,
    pub machineName: [u16; 256],
    pub sharedAdapterName: [u16; 256],
}
impl ::core::marker::Copy for NLA_BLOB_0_0_0 {}
impl ::core::clone::Clone for NLA_BLOB_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLA_BLOB_0_1 {
    pub r#type: NLA_CONNECTIVITY_TYPE,
    pub internet: NLA_INTERNET,
}
impl ::core::marker::Copy for NLA_BLOB_0_1 {}
impl ::core::clone::Clone for NLA_BLOB_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLA_BLOB_0_2 {
    pub dwType: u32,
    pub dwSpeed: u32,
    pub adapterName: [::win32_foundation_sys::CHAR; 1],
}
impl ::core::marker::Copy for NLA_BLOB_0_2 {}
impl ::core::clone::Clone for NLA_BLOB_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLA_BLOB_0_3 {
    pub information: [::win32_foundation_sys::CHAR; 1],
}
impl ::core::marker::Copy for NLA_BLOB_0_3 {}
impl ::core::clone::Clone for NLA_BLOB_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NLA_BLOB_1 {
    pub r#type: NLA_BLOB_DATA_TYPE,
    pub dwSize: u32,
    pub nextOffset: u32,
}
impl ::core::marker::Copy for NLA_BLOB_1 {}
impl ::core::clone::Clone for NLA_BLOB_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NLA_BLOB_DATA_TYPE = i32;
pub const NLA_RAW_DATA: NLA_BLOB_DATA_TYPE = 0i32;
pub const NLA_INTERFACE: NLA_BLOB_DATA_TYPE = 1i32;
pub const NLA_802_1X_LOCATION: NLA_BLOB_DATA_TYPE = 2i32;
pub const NLA_CONNECTIVITY: NLA_BLOB_DATA_TYPE = 3i32;
pub const NLA_ICS: NLA_BLOB_DATA_TYPE = 4i32;
pub type NLA_CONNECTIVITY_TYPE = i32;
pub const NLA_NETWORK_AD_HOC: NLA_CONNECTIVITY_TYPE = 0i32;
pub const NLA_NETWORK_MANAGED: NLA_CONNECTIVITY_TYPE = 1i32;
pub const NLA_NETWORK_UNMANAGED: NLA_CONNECTIVITY_TYPE = 2i32;
pub const NLA_NETWORK_UNKNOWN: NLA_CONNECTIVITY_TYPE = 3i32;
pub const NLA_FRIENDLY_NAME: u32 = 2u32;
pub type NLA_INTERNET = i32;
pub const NLA_INTERNET_UNKNOWN: NLA_INTERNET = 0i32;
pub const NLA_INTERNET_NO: NLA_INTERNET = 1i32;
pub const NLA_INTERNET_YES: NLA_INTERNET = 2i32;
pub type NL_ADDRESS_TYPE = i32;
pub const NlatUnspecified: NL_ADDRESS_TYPE = 0i32;
pub const NlatUnicast: NL_ADDRESS_TYPE = 1i32;
pub const NlatAnycast: NL_ADDRESS_TYPE = 2i32;
pub const NlatMulticast: NL_ADDRESS_TYPE = 3i32;
pub const NlatBroadcast: NL_ADDRESS_TYPE = 4i32;
pub const NlatInvalid: NL_ADDRESS_TYPE = 5i32;
pub type NL_BANDWIDTH_FLAG = i32;
pub const NlbwDisabled: NL_BANDWIDTH_FLAG = 0i32;
pub const NlbwEnabled: NL_BANDWIDTH_FLAG = 1i32;
pub const NlbwUnchanged: NL_BANDWIDTH_FLAG = -1i32;
#[repr(C)]
pub struct NL_BANDWIDTH_INFORMATION {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for NL_BANDWIDTH_INFORMATION {}
impl ::core::clone::Clone for NL_BANDWIDTH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NL_DAD_STATE = i32;
pub const NldsInvalid: NL_DAD_STATE = 0i32;
pub const NldsTentative: NL_DAD_STATE = 1i32;
pub const NldsDuplicate: NL_DAD_STATE = 2i32;
pub const NldsDeprecated: NL_DAD_STATE = 3i32;
pub const NldsPreferred: NL_DAD_STATE = 4i32;
pub const IpDadStateInvalid: NL_DAD_STATE = 0i32;
pub const IpDadStateTentative: NL_DAD_STATE = 1i32;
pub const IpDadStateDuplicate: NL_DAD_STATE = 2i32;
pub const IpDadStateDeprecated: NL_DAD_STATE = 3i32;
pub const IpDadStatePreferred: NL_DAD_STATE = 4i32;
pub type NL_INTERFACE_NETWORK_CATEGORY_STATE = i32;
pub const NlincCategoryUnknown: NL_INTERFACE_NETWORK_CATEGORY_STATE = 0i32;
pub const NlincPublic: NL_INTERFACE_NETWORK_CATEGORY_STATE = 1i32;
pub const NlincPrivate: NL_INTERFACE_NETWORK_CATEGORY_STATE = 2i32;
pub const NlincDomainAuthenticated: NL_INTERFACE_NETWORK_CATEGORY_STATE = 3i32;
pub const NlincCategoryStateMax: NL_INTERFACE_NETWORK_CATEGORY_STATE = 4i32;
#[repr(C)]
pub struct NL_INTERFACE_OFFLOAD_ROD {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NL_INTERFACE_OFFLOAD_ROD {}
impl ::core::clone::Clone for NL_INTERFACE_OFFLOAD_ROD {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NL_LINK_LOCAL_ADDRESS_BEHAVIOR = i32;
pub const LinkLocalAlwaysOff: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = 0i32;
pub const LinkLocalDelayed: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = 1i32;
pub const LinkLocalAlwaysOn: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = 2i32;
pub const LinkLocalUnchanged: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = -1i32;
pub type NL_NEIGHBOR_STATE = i32;
pub const NlnsUnreachable: NL_NEIGHBOR_STATE = 0i32;
pub const NlnsIncomplete: NL_NEIGHBOR_STATE = 1i32;
pub const NlnsProbe: NL_NEIGHBOR_STATE = 2i32;
pub const NlnsDelay: NL_NEIGHBOR_STATE = 3i32;
pub const NlnsStale: NL_NEIGHBOR_STATE = 4i32;
pub const NlnsReachable: NL_NEIGHBOR_STATE = 5i32;
pub const NlnsPermanent: NL_NEIGHBOR_STATE = 6i32;
pub const NlnsMaximum: NL_NEIGHBOR_STATE = 7i32;
pub type NL_NETWORK_CATEGORY = i32;
pub const NetworkCategoryPublic: NL_NETWORK_CATEGORY = 0i32;
pub const NetworkCategoryPrivate: NL_NETWORK_CATEGORY = 1i32;
pub const NetworkCategoryDomainAuthenticated: NL_NETWORK_CATEGORY = 2i32;
pub const NetworkCategoryUnchanged: NL_NETWORK_CATEGORY = -1i32;
pub const NetworkCategoryUnknown: NL_NETWORK_CATEGORY = -1i32;
pub type NL_NETWORK_CONNECTIVITY_COST_HINT = i32;
pub const NetworkConnectivityCostHintUnknown: NL_NETWORK_CONNECTIVITY_COST_HINT = 0i32;
pub const NetworkConnectivityCostHintUnrestricted: NL_NETWORK_CONNECTIVITY_COST_HINT = 1i32;
pub const NetworkConnectivityCostHintFixed: NL_NETWORK_CONNECTIVITY_COST_HINT = 2i32;
pub const NetworkConnectivityCostHintVariable: NL_NETWORK_CONNECTIVITY_COST_HINT = 3i32;
#[repr(C)]
pub struct NL_NETWORK_CONNECTIVITY_HINT {
    pub ConnectivityLevel: NL_NETWORK_CONNECTIVITY_LEVEL_HINT,
    pub ConnectivityCost: NL_NETWORK_CONNECTIVITY_COST_HINT,
    pub ApproachingDataLimit: ::win32_foundation_sys::BOOLEAN,
    pub OverDataLimit: ::win32_foundation_sys::BOOLEAN,
    pub Roaming: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for NL_NETWORK_CONNECTIVITY_HINT {}
impl ::core::clone::Clone for NL_NETWORK_CONNECTIVITY_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NL_NETWORK_CONNECTIVITY_LEVEL_HINT = i32;
pub const NetworkConnectivityLevelHintUnknown: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 0i32;
pub const NetworkConnectivityLevelHintNone: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 1i32;
pub const NetworkConnectivityLevelHintLocalAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 2i32;
pub const NetworkConnectivityLevelHintInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 3i32;
pub const NetworkConnectivityLevelHintConstrainedInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 4i32;
pub const NetworkConnectivityLevelHintHidden: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = 5i32;
#[repr(C)]
pub struct NL_PATH_BANDWIDTH_ROD {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for NL_PATH_BANDWIDTH_ROD {}
impl ::core::clone::Clone for NL_PATH_BANDWIDTH_ROD {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NL_PREFIX_ORIGIN = i32;
pub const IpPrefixOriginOther: NL_PREFIX_ORIGIN = 0i32;
pub const IpPrefixOriginManual: NL_PREFIX_ORIGIN = 1i32;
pub const IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = 2i32;
pub const IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = 3i32;
pub const IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN = 4i32;
pub const IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = 16i32;
pub type NL_ROUTER_DISCOVERY_BEHAVIOR = i32;
pub const RouterDiscoveryDisabled: NL_ROUTER_DISCOVERY_BEHAVIOR = 0i32;
pub const RouterDiscoveryEnabled: NL_ROUTER_DISCOVERY_BEHAVIOR = 1i32;
pub const RouterDiscoveryDhcp: NL_ROUTER_DISCOVERY_BEHAVIOR = 2i32;
pub const RouterDiscoveryUnchanged: NL_ROUTER_DISCOVERY_BEHAVIOR = -1i32;
pub type NL_ROUTE_ORIGIN = i32;
pub const NlroManual: NL_ROUTE_ORIGIN = 0i32;
pub const NlroWellKnown: NL_ROUTE_ORIGIN = 1i32;
pub const NlroDHCP: NL_ROUTE_ORIGIN = 2i32;
pub const NlroRouterAdvertisement: NL_ROUTE_ORIGIN = 3i32;
pub const Nlro6to4: NL_ROUTE_ORIGIN = 4i32;
pub type NL_ROUTE_PROTOCOL = i32;
pub const RouteProtocolOther: NL_ROUTE_PROTOCOL = 1i32;
pub const RouteProtocolLocal: NL_ROUTE_PROTOCOL = 2i32;
pub const RouteProtocolNetMgmt: NL_ROUTE_PROTOCOL = 3i32;
pub const RouteProtocolIcmp: NL_ROUTE_PROTOCOL = 4i32;
pub const RouteProtocolEgp: NL_ROUTE_PROTOCOL = 5i32;
pub const RouteProtocolGgp: NL_ROUTE_PROTOCOL = 6i32;
pub const RouteProtocolHello: NL_ROUTE_PROTOCOL = 7i32;
pub const RouteProtocolRip: NL_ROUTE_PROTOCOL = 8i32;
pub const RouteProtocolIsIs: NL_ROUTE_PROTOCOL = 9i32;
pub const RouteProtocolEsIs: NL_ROUTE_PROTOCOL = 10i32;
pub const RouteProtocolCisco: NL_ROUTE_PROTOCOL = 11i32;
pub const RouteProtocolBbn: NL_ROUTE_PROTOCOL = 12i32;
pub const RouteProtocolOspf: NL_ROUTE_PROTOCOL = 13i32;
pub const RouteProtocolBgp: NL_ROUTE_PROTOCOL = 14i32;
pub const RouteProtocolIdpr: NL_ROUTE_PROTOCOL = 15i32;
pub const RouteProtocolEigrp: NL_ROUTE_PROTOCOL = 16i32;
pub const RouteProtocolDvmrp: NL_ROUTE_PROTOCOL = 17i32;
pub const RouteProtocolRpl: NL_ROUTE_PROTOCOL = 18i32;
pub const RouteProtocolDhcp: NL_ROUTE_PROTOCOL = 19i32;
pub const MIB_IPPROTO_OTHER: NL_ROUTE_PROTOCOL = 1i32;
pub const PROTO_IP_OTHER: NL_ROUTE_PROTOCOL = 1i32;
pub const MIB_IPPROTO_LOCAL: NL_ROUTE_PROTOCOL = 2i32;
pub const PROTO_IP_LOCAL: NL_ROUTE_PROTOCOL = 2i32;
pub const MIB_IPPROTO_NETMGMT: NL_ROUTE_PROTOCOL = 3i32;
pub const PROTO_IP_NETMGMT: NL_ROUTE_PROTOCOL = 3i32;
pub const MIB_IPPROTO_ICMP: NL_ROUTE_PROTOCOL = 4i32;
pub const PROTO_IP_ICMP: NL_ROUTE_PROTOCOL = 4i32;
pub const MIB_IPPROTO_EGP: NL_ROUTE_PROTOCOL = 5i32;
pub const PROTO_IP_EGP: NL_ROUTE_PROTOCOL = 5i32;
pub const MIB_IPPROTO_GGP: NL_ROUTE_PROTOCOL = 6i32;
pub const PROTO_IP_GGP: NL_ROUTE_PROTOCOL = 6i32;
pub const MIB_IPPROTO_HELLO: NL_ROUTE_PROTOCOL = 7i32;
pub const PROTO_IP_HELLO: NL_ROUTE_PROTOCOL = 7i32;
pub const MIB_IPPROTO_RIP: NL_ROUTE_PROTOCOL = 8i32;
pub const PROTO_IP_RIP: NL_ROUTE_PROTOCOL = 8i32;
pub const MIB_IPPROTO_IS_IS: NL_ROUTE_PROTOCOL = 9i32;
pub const PROTO_IP_IS_IS: NL_ROUTE_PROTOCOL = 9i32;
pub const MIB_IPPROTO_ES_IS: NL_ROUTE_PROTOCOL = 10i32;
pub const PROTO_IP_ES_IS: NL_ROUTE_PROTOCOL = 10i32;
pub const MIB_IPPROTO_CISCO: NL_ROUTE_PROTOCOL = 11i32;
pub const PROTO_IP_CISCO: NL_ROUTE_PROTOCOL = 11i32;
pub const MIB_IPPROTO_BBN: NL_ROUTE_PROTOCOL = 12i32;
pub const PROTO_IP_BBN: NL_ROUTE_PROTOCOL = 12i32;
pub const MIB_IPPROTO_OSPF: NL_ROUTE_PROTOCOL = 13i32;
pub const PROTO_IP_OSPF: NL_ROUTE_PROTOCOL = 13i32;
pub const MIB_IPPROTO_BGP: NL_ROUTE_PROTOCOL = 14i32;
pub const PROTO_IP_BGP: NL_ROUTE_PROTOCOL = 14i32;
pub const MIB_IPPROTO_IDPR: NL_ROUTE_PROTOCOL = 15i32;
pub const PROTO_IP_IDPR: NL_ROUTE_PROTOCOL = 15i32;
pub const MIB_IPPROTO_EIGRP: NL_ROUTE_PROTOCOL = 16i32;
pub const PROTO_IP_EIGRP: NL_ROUTE_PROTOCOL = 16i32;
pub const MIB_IPPROTO_DVMRP: NL_ROUTE_PROTOCOL = 17i32;
pub const PROTO_IP_DVMRP: NL_ROUTE_PROTOCOL = 17i32;
pub const MIB_IPPROTO_RPL: NL_ROUTE_PROTOCOL = 18i32;
pub const PROTO_IP_RPL: NL_ROUTE_PROTOCOL = 18i32;
pub const MIB_IPPROTO_DHCP: NL_ROUTE_PROTOCOL = 19i32;
pub const PROTO_IP_DHCP: NL_ROUTE_PROTOCOL = 19i32;
pub const MIB_IPPROTO_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = 10002i32;
pub const PROTO_IP_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = 10002i32;
pub const MIB_IPPROTO_NT_STATIC: NL_ROUTE_PROTOCOL = 10006i32;
pub const PROTO_IP_NT_STATIC: NL_ROUTE_PROTOCOL = 10006i32;
pub const MIB_IPPROTO_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = 10007i32;
pub const PROTO_IP_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = 10007i32;
pub type NL_SUFFIX_ORIGIN = i32;
pub const NlsoOther: NL_SUFFIX_ORIGIN = 0i32;
pub const NlsoManual: NL_SUFFIX_ORIGIN = 1i32;
pub const NlsoWellKnown: NL_SUFFIX_ORIGIN = 2i32;
pub const NlsoDhcp: NL_SUFFIX_ORIGIN = 3i32;
pub const NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = 4i32;
pub const NlsoRandom: NL_SUFFIX_ORIGIN = 5i32;
pub const IpSuffixOriginOther: NL_SUFFIX_ORIGIN = 0i32;
pub const IpSuffixOriginManual: NL_SUFFIX_ORIGIN = 1i32;
pub const IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = 2i32;
pub const IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = 3i32;
pub const IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = 4i32;
pub const IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = 5i32;
pub const IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = 16i32;
#[repr(C)]
pub struct NPI_MODULEID {
    pub Length: u16,
    pub Type: NPI_MODULEID_TYPE,
    pub Anonymous: NPI_MODULEID_0,
}
impl ::core::marker::Copy for NPI_MODULEID {}
impl ::core::clone::Clone for NPI_MODULEID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union NPI_MODULEID_0 {
    pub Guid: ::windows_core_sys::GUID,
    pub IfLuid: ::win32_foundation_sys::LUID,
}
impl ::core::marker::Copy for NPI_MODULEID_0 {}
impl ::core::clone::Clone for NPI_MODULEID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NPI_MODULEID_TYPE = i32;
pub const MIT_GUID: NPI_MODULEID_TYPE = 1i32;
pub const MIT_IF_LUID: NPI_MODULEID_TYPE = 2i32;
pub const NSPROTO_IPX: u32 = 1000u32;
pub const NSPROTO_SPX: u32 = 1256u32;
pub const NSPROTO_SPXII: u32 = 1257u32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct NSPV2_ROUTINE {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub NSPv2Startup: LPNSPV2STARTUP,
    pub NSPv2Cleanup: LPNSPV2CLEANUP,
    pub NSPv2LookupServiceBegin: LPNSPV2LOOKUPSERVICEBEGIN,
    pub NSPv2LookupServiceNextEx: LPNSPV2LOOKUPSERVICENEXTEX,
    pub NSPv2LookupServiceEnd: LPNSPV2LOOKUPSERVICEEND,
    pub NSPv2SetServiceEx: LPNSPV2SETSERVICEEX,
    pub NSPv2ClientSessionRundown: LPNSPV2CLIENTSESSIONRUNDOWN,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for NSPV2_ROUTINE {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for NSPV2_ROUTINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
pub struct NSP_ROUTINE {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub NSPCleanup: LPNSPCLEANUP,
    pub NSPLookupServiceBegin: LPNSPLOOKUPSERVICEBEGIN,
    pub NSPLookupServiceNext: LPNSPLOOKUPSERVICENEXT,
    pub NSPLookupServiceEnd: LPNSPLOOKUPSERVICEEND,
    pub NSPSetService: LPNSPSETSERVICE,
    pub NSPInstallServiceClass: LPNSPINSTALLSERVICECLASS,
    pub NSPRemoveServiceClass: LPNSPREMOVESERVICECLASS,
    pub NSPGetServiceClassInfo: LPNSPGETSERVICECLASSINFO,
    pub NSPIoctl: LPNSPIOCTL,
}
#[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
impl ::core::marker::Copy for NSP_ROUTINE {}
#[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
impl ::core::clone::Clone for NSP_ROUTINE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NSTYPE_DYNAMIC: u32 = 2u32;
pub const NSTYPE_ENUMERABLE: u32 = 4u32;
pub const NSTYPE_HIERARCHICAL: u32 = 1u32;
pub const NSTYPE_WORKGROUP: u32 = 8u32;
pub const NS_ALL: u32 = 0u32;
pub const NS_DEFAULT: u32 = 0u32;
pub const NS_DHCP: u32 = 6u32;
pub const NS_DNS: u32 = 12u32;
pub const NS_EMAIL: u32 = 37u32;
#[repr(C)]
pub struct NS_INFOA {
    pub dwNameSpace: u32,
    pub dwNameSpaceFlags: u32,
    pub lpNameSpace: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for NS_INFOA {}
impl ::core::clone::Clone for NS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NS_INFOW {
    pub dwNameSpace: u32,
    pub dwNameSpaceFlags: u32,
    pub lpNameSpace: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for NS_INFOW {}
impl ::core::clone::Clone for NS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NS_LOCALNAME: u32 = 19u32;
pub const NS_MS: u32 = 30u32;
pub const NS_NBP: u32 = 20u32;
pub const NS_NDS: u32 = 2u32;
pub const NS_NETBT: u32 = 13u32;
pub const NS_NETDES: u32 = 60u32;
pub const NS_NIS: u32 = 41u32;
pub const NS_NISPLUS: u32 = 42u32;
pub const NS_NLA: u32 = 15u32;
pub const NS_NTDS: u32 = 32u32;
pub const NS_PEER_BROWSE: u32 = 3u32;
pub const NS_SAP: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct NS_SERVICE_INFOA {
    pub dwNameSpace: u32,
    pub ServiceInfo: SERVICE_INFOA,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for NS_SERVICE_INFOA {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for NS_SERVICE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct NS_SERVICE_INFOW {
    pub dwNameSpace: u32,
    pub ServiceInfo: SERVICE_INFOW,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for NS_SERVICE_INFOW {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for NS_SERVICE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NS_SLP: u32 = 5u32;
pub const NS_STDA: u32 = 31u32;
pub const NS_TCPIP_HOSTS: u32 = 11u32;
pub const NS_TCPIP_LOCAL: u32 = 10u32;
pub const NS_VNS: u32 = 50u32;
pub const NS_WINS: u32 = 14u32;
pub const NS_WRQ: u32 = 50u32;
pub const NS_X500: u32 = 40u32;
pub const PFL_HIDDEN: u32 = 4u32;
pub const PFL_MATCHES_PROTOCOL_ZERO: u32 = 8u32;
pub const PFL_MULTIPLE_PROTO_ENTRIES: u32 = 1u32;
pub const PFL_NETWORKDIRECT_PROVIDER: u32 = 16u32;
pub const PFL_RECOMMENDED_PROTO_ENTRY: u32 = 2u32;
pub const PF_APPLETALK: u16 = 16u16;
pub const PF_ATM: u16 = 22u16;
pub const PF_BAN: u16 = 21u16;
pub const PF_CCITT: u16 = 10u16;
pub const PF_CHAOS: u16 = 5u16;
pub const PF_DATAKIT: u16 = 9u16;
pub const PF_DECnet: u16 = 12u16;
pub const PF_DLI: u16 = 13u16;
pub const PF_ECMA: u16 = 8u16;
pub const PF_FIREFOX: u16 = 19u16;
pub const PF_HYLINK: u16 = 15u16;
pub const PF_IMPLINK: u16 = 3u16;
pub const PF_IPX: u16 = 6u16;
pub const PF_IRDA: u16 = 26u16;
pub const PF_ISO: u16 = 7u16;
pub const PF_LAT: u16 = 14u16;
pub const PF_MAX: u16 = 29u16;
pub const PF_NS: u16 = 6u16;
pub const PF_OSI: u16 = 7u16;
pub const PF_PUP: u16 = 4u16;
pub const PF_SNA: u16 = 11u16;
pub const PF_UNIX: u16 = 1u16;
pub const PF_UNKNOWN1: u16 = 20u16;
pub const PF_VOICEVIEW: u16 = 18u16;
pub const PI_ALLOWED: u32 = 0u32;
pub const PI_NUMBER_NOT_AVAILABLE: u32 = 128u32;
pub const PI_RESTRICTED: u32 = 64u32;
pub type PMTUD_STATE = i32;
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = 0i32;
pub const IP_PMTUDISC_DO: PMTUD_STATE = 1i32;
pub const IP_PMTUDISC_DONT: PMTUD_STATE = 2i32;
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = 3i32;
pub const IP_PMTUDISC_MAX: PMTUD_STATE = 4i32;
pub const POLLERR: u16 = 1u16;
pub const POLLHUP: u16 = 2u16;
pub const POLLIN: u16 = 768u16;
pub const POLLNVAL: u16 = 4u16;
pub const POLLOUT: u16 = 16u16;
pub const POLLPRI: u16 = 1024u16;
pub const POLLRDBAND: u16 = 512u16;
pub const POLLRDNORM: u16 = 256u16;
pub const POLLWRBAND: u16 = 32u16;
pub const POLLWRNORM: u16 = 16u16;
#[repr(C)]
pub struct PRIORITY_STATUS {
    pub Sender: SOCKET_PRIORITY_HINT,
    pub Receiver: SOCKET_PRIORITY_HINT,
}
impl ::core::marker::Copy for PRIORITY_STATUS {}
impl ::core::clone::Clone for PRIORITY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROP_ADDRESSES: u32 = 256u32;
pub const PROP_ALL: u32 = 2147483648u32;
pub const PROP_COMMENT: u32 = 1u32;
pub const PROP_DISPLAY_HINT: u32 = 4u32;
pub const PROP_LOCALE: u32 = 2u32;
pub const PROP_MACHINE: u32 = 32u32;
pub const PROP_SD: u32 = 512u32;
pub const PROP_START_TIME: u32 = 16u32;
pub const PROP_VERSION: u32 = 8u32;
pub const PROTECTION_LEVEL_DEFAULT: u32 = 20u32;
pub const PROTECTION_LEVEL_EDGERESTRICTED: u32 = 20u32;
pub const PROTECTION_LEVEL_RESTRICTED: u32 = 30u32;
pub const PROTECTION_LEVEL_UNRESTRICTED: u32 = 10u32;
#[repr(C)]
pub struct PROTOCOL_INFOA {
    pub dwServiceFlags: u32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub dwMessageSize: u32,
    pub lpProtocol: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PROTOCOL_INFOA {}
impl ::core::clone::Clone for PROTOCOL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROTOCOL_INFOW {
    pub dwServiceFlags: u32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub dwMessageSize: u32,
    pub lpProtocol: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PROTOCOL_INFOW {}
impl ::core::clone::Clone for PROTOCOL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PVD_CONFIG: u32 = 12289u32;
#[repr(C)]
pub struct Q2931_IE {
    pub IEType: Q2931_IE_TYPE,
    pub IELength: u32,
    pub IE: [u8; 1],
}
impl ::core::marker::Copy for Q2931_IE {}
impl ::core::clone::Clone for Q2931_IE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Q2931_IE_TYPE = i32;
pub const IE_AALParameters: Q2931_IE_TYPE = 0i32;
pub const IE_TrafficDescriptor: Q2931_IE_TYPE = 1i32;
pub const IE_BroadbandBearerCapability: Q2931_IE_TYPE = 2i32;
pub const IE_BHLI: Q2931_IE_TYPE = 3i32;
pub const IE_BLLI: Q2931_IE_TYPE = 4i32;
pub const IE_CalledPartyNumber: Q2931_IE_TYPE = 5i32;
pub const IE_CalledPartySubaddress: Q2931_IE_TYPE = 6i32;
pub const IE_CallingPartyNumber: Q2931_IE_TYPE = 7i32;
pub const IE_CallingPartySubaddress: Q2931_IE_TYPE = 8i32;
pub const IE_Cause: Q2931_IE_TYPE = 9i32;
pub const IE_QOSClass: Q2931_IE_TYPE = 10i32;
pub const IE_TransitNetworkSelection: Q2931_IE_TYPE = 11i32;
#[repr(C)]
pub struct QOS {
    pub SendingFlowspec: FLOWSPEC,
    pub ReceivingFlowspec: FLOWSPEC,
    pub ProviderSpecific: WSABUF,
}
impl ::core::marker::Copy for QOS {}
impl ::core::clone::Clone for QOS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const QOS_CLASS0: u32 = 0u32;
pub const QOS_CLASS1: u32 = 1u32;
pub const QOS_CLASS2: u32 = 2u32;
pub const QOS_CLASS3: u32 = 3u32;
pub const QOS_CLASS4: u32 = 4u32;
#[repr(C)]
pub struct RCVALL_IF {
    pub Mode: RCVALL_VALUE,
    pub Interface: u32,
}
impl ::core::marker::Copy for RCVALL_IF {}
impl ::core::clone::Clone for RCVALL_IF {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RCVALL_VALUE = i32;
pub const RCVALL_OFF: RCVALL_VALUE = 0i32;
pub const RCVALL_ON: RCVALL_VALUE = 1i32;
pub const RCVALL_SOCKETLEVELONLY: RCVALL_VALUE = 2i32;
pub const RCVALL_IPLEVEL: RCVALL_VALUE = 3i32;
pub const REAL_TIME_NOTIFICATION_CAPABILITY: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1801027994, data2: 23726, data3: 18733, data4: [169, 1, 42, 60, 44, 80, 22, 79] };
pub const REAL_TIME_NOTIFICATION_CAPABILITY_EX: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1749277187, data2: 5450, data3: 17942, data4: [165, 8, 68, 55, 18, 149, 249, 107] };
#[repr(C)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for REAL_TIME_NOTIFICATION_SETTING_INPUT {}
impl ::core::clone::Clone for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: ::windows_core_sys::GUID,
    pub Unmark: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {}
impl ::core::clone::Clone for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    pub ChannelStatus: CONTROL_CHANNEL_TRIGGER_STATUS,
}
impl ::core::marker::Copy for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {}
impl ::core::clone::Clone for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RESOURCE_DISPLAY_TYPE = u32;
pub const RESOURCEDISPLAYTYPE_DOMAIN: RESOURCE_DISPLAY_TYPE = 1u32;
pub const RESOURCEDISPLAYTYPE_FILE: RESOURCE_DISPLAY_TYPE = 4u32;
pub const RESOURCEDISPLAYTYPE_GENERIC: RESOURCE_DISPLAY_TYPE = 0u32;
pub const RESOURCEDISPLAYTYPE_GROUP: RESOURCE_DISPLAY_TYPE = 5u32;
pub const RESOURCEDISPLAYTYPE_SERVER: RESOURCE_DISPLAY_TYPE = 2u32;
pub const RESOURCEDISPLAYTYPE_SHARE: RESOURCE_DISPLAY_TYPE = 3u32;
pub const RESOURCEDISPLAYTYPE_TREE: RESOURCE_DISPLAY_TYPE = 10u32;
pub const RESULT_IS_ADDED: u32 = 16u32;
pub const RESULT_IS_ALIAS: u32 = 1u32;
pub const RESULT_IS_CHANGED: u32 = 32u32;
pub const RESULT_IS_DELETED: u32 = 64u32;
pub const RES_FIND_MULTIPLE: u32 = 2u32;
pub const RES_FLUSH_CACHE: u32 = 2u32;
pub const RES_SERVICE: u32 = 4u32;
pub const RES_SOFT_SEARCH: u32 = 1u32;
pub const RES_UNUSED_1: u32 = 1u32;
#[repr(C)]
pub struct RIORESULT {
    pub Status: i32,
    pub BytesTransferred: u32,
    pub SocketContext: u64,
    pub RequestContext: u64,
}
impl ::core::marker::Copy for RIORESULT {}
impl ::core::clone::Clone for RIORESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RIO_BUF {
    pub BufferId: *mut RIO_BUFFERID_t,
    pub Offset: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for RIO_BUF {}
impl ::core::clone::Clone for RIO_BUF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RIO_BUFFERID_t(pub u8);
#[repr(C)]
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: u32,
}
impl ::core::marker::Copy for RIO_CMSG_BUFFER {}
impl ::core::clone::Clone for RIO_CMSG_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RIO_CORRUPT_CQ: u32 = 4294967295u32;
#[repr(C)]
pub struct RIO_CQ_t(pub u8);
#[repr(C)]
pub struct RIO_EXTENSION_FUNCTION_TABLE {
    pub cbSize: u32,
    pub RIOReceive: LPFN_RIORECEIVE,
    pub RIOReceiveEx: LPFN_RIORECEIVEEX,
    pub RIOSend: LPFN_RIOSEND,
    pub RIOSendEx: LPFN_RIOSENDEX,
    pub RIOCloseCompletionQueue: LPFN_RIOCLOSECOMPLETIONQUEUE,
    pub RIOCreateCompletionQueue: LPFN_RIOCREATECOMPLETIONQUEUE,
    pub RIOCreateRequestQueue: LPFN_RIOCREATEREQUESTQUEUE,
    pub RIODequeueCompletion: LPFN_RIODEQUEUECOMPLETION,
    pub RIODeregisterBuffer: LPFN_RIODEREGISTERBUFFER,
    pub RIONotify: LPFN_RIONOTIFY,
    pub RIORegisterBuffer: LPFN_RIOREGISTERBUFFER,
    pub RIOResizeCompletionQueue: LPFN_RIORESIZECOMPLETIONQUEUE,
    pub RIOResizeRequestQueue: LPFN_RIORESIZEREQUESTQUEUE,
}
impl ::core::marker::Copy for RIO_EXTENSION_FUNCTION_TABLE {}
impl ::core::clone::Clone for RIO_EXTENSION_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RIO_MAX_CQ_SIZE: u32 = 134217728u32;
pub const RIO_MSG_COMMIT_ONLY: u32 = 8u32;
pub const RIO_MSG_DEFER: u32 = 2u32;
pub const RIO_MSG_DONT_NOTIFY: u32 = 1u32;
pub const RIO_MSG_WAITALL: u32 = 4u32;
#[repr(C)]
pub struct RIO_NOTIFICATION_COMPLETION {
    pub Type: RIO_NOTIFICATION_COMPLETION_TYPE,
    pub Anonymous: RIO_NOTIFICATION_COMPLETION_0,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RIO_NOTIFICATION_COMPLETION_0 {
    pub Event: RIO_NOTIFICATION_COMPLETION_0_0,
    pub Iocp: RIO_NOTIFICATION_COMPLETION_0_1,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_0 {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RIO_NOTIFICATION_COMPLETION_0_0 {
    pub EventHandle: ::win32_foundation_sys::HANDLE,
    pub NotifyReset: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_0_0 {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RIO_NOTIFICATION_COMPLETION_0_1 {
    pub IocpHandle: ::win32_foundation_sys::HANDLE,
    pub CompletionKey: *mut ::core::ffi::c_void,
    pub Overlapped: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_0_1 {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RIO_NOTIFICATION_COMPLETION_TYPE = i32;
pub const RIO_EVENT_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = 1i32;
pub const RIO_IOCP_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = 2i32;
#[repr(C)]
pub struct RIO_RQ_t(pub u8);
pub const RM_ADD_RECEIVE_IF: u32 = 1008u32;
pub const RM_DEL_RECEIVE_IF: u32 = 1009u32;
#[repr(C)]
pub struct RM_FEC_INFO {
    pub FECBlockSize: u16,
    pub FECProActivePackets: u16,
    pub FECGroupSize: u8,
    pub fFECOnDemandParityEnabled: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for RM_FEC_INFO {}
impl ::core::clone::Clone for RM_FEC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RM_FLUSHCACHE: u32 = 1003u32;
pub const RM_HIGH_SPEED_INTRANET_OPT: u32 = 1014u32;
pub const RM_LATEJOIN: u32 = 1006u32;
pub const RM_OPTIONSBASE: u32 = 1000u32;
pub const RM_RATE_WINDOW_SIZE: u32 = 1001u32;
pub const RM_RECEIVER_STATISTICS: u32 = 1013u32;
#[repr(C)]
pub struct RM_RECEIVER_STATS {
    pub NumODataPacketsReceived: u64,
    pub NumRDataPacketsReceived: u64,
    pub NumDuplicateDataPackets: u64,
    pub DataBytesReceived: u64,
    pub TotalBytesReceived: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub AverageSequencesInWindow: u64,
    pub MinSequencesInWindow: u64,
    pub MaxSequencesInWindow: u64,
    pub FirstNakSequenceNumber: u64,
    pub NumPendingNaks: u64,
    pub NumOutstandingNaks: u64,
    pub NumDataPacketsBuffered: u64,
    pub TotalSelectiveNaksSent: u64,
    pub TotalParityNaksSent: u64,
}
impl ::core::marker::Copy for RM_RECEIVER_STATS {}
impl ::core::clone::Clone for RM_RECEIVER_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RM_SENDER_STATISTICS: u32 = 1005u32;
#[repr(C)]
pub struct RM_SENDER_STATS {
    pub DataBytesSent: u64,
    pub TotalBytesSent: u64,
    pub NaksReceived: u64,
    pub NaksReceivedTooLate: u64,
    pub NumOutstandingNaks: u64,
    pub NumNaksAfterRData: u64,
    pub RepairPacketsSent: u64,
    pub BufferSpaceAvailable: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TotalODataPacketsSent: u64,
}
impl ::core::marker::Copy for RM_SENDER_STATS {}
impl ::core::clone::Clone for RM_SENDER_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RM_SENDER_WINDOW_ADVANCE_METHOD: u32 = 1004u32;
#[repr(C)]
pub struct RM_SEND_WINDOW {
    pub RateKbitsPerSec: u32,
    pub WindowSizeInMSecs: u32,
    pub WindowSizeInBytes: u32,
}
impl ::core::marker::Copy for RM_SEND_WINDOW {}
impl ::core::clone::Clone for RM_SEND_WINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RM_SEND_WINDOW_ADV_RATE: u32 = 1010u32;
pub const RM_SET_MCAST_TTL: u32 = 1012u32;
pub const RM_SET_MESSAGE_BOUNDARY: u32 = 1002u32;
pub const RM_SET_SEND_IF: u32 = 1007u32;
pub const RM_USE_FEC: u32 = 1011u32;
#[repr(C)]
pub struct RSS_SCALABILITY_INFO {
    pub RssEnabled: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for RSS_SCALABILITY_INFO {}
impl ::core::clone::Clone for RSS_SCALABILITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SAP_FIELD_ABSENT: u32 = 4294967294u32;
pub const SAP_FIELD_ANY: u32 = 4294967295u32;
pub const SAP_FIELD_ANY_AESA_REST: u32 = 4294967291u32;
pub const SAP_FIELD_ANY_AESA_SEL: u32 = 4294967290u32;
#[repr(C)]
pub struct SCOPE_ID {
    pub Anonymous: SCOPE_ID_0,
}
impl ::core::marker::Copy for SCOPE_ID {}
impl ::core::clone::Clone for SCOPE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SCOPE_ID_0 {
    pub Anonymous: SCOPE_ID_0_0,
    pub Value: u32,
}
impl ::core::marker::Copy for SCOPE_ID_0 {}
impl ::core::clone::Clone for SCOPE_ID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SCOPE_ID_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SCOPE_ID_0_0 {}
impl ::core::clone::Clone for SCOPE_ID_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SCOPE_LEVEL = i32;
pub const ScopeLevelInterface: SCOPE_LEVEL = 1i32;
pub const ScopeLevelLink: SCOPE_LEVEL = 2i32;
pub const ScopeLevelSubnet: SCOPE_LEVEL = 3i32;
pub const ScopeLevelAdmin: SCOPE_LEVEL = 4i32;
pub const ScopeLevelSite: SCOPE_LEVEL = 5i32;
pub const ScopeLevelOrganization: SCOPE_LEVEL = 8i32;
pub const ScopeLevelGlobal: SCOPE_LEVEL = 14i32;
pub const ScopeLevelCount: SCOPE_LEVEL = 16i32;
pub const SD_BOTH: u32 = 2u32;
pub const SD_RECEIVE: u32 = 0u32;
pub const SD_SEND: u32 = 1u32;
pub const SECURITY_PROTOCOL_NONE: u32 = 0u32;
pub const SENDER_DEFAULT_LATE_JOINER_PERCENTAGE: u32 = 0u32;
pub const SENDER_DEFAULT_RATE_KBITS_PER_SEC: u32 = 56u32;
pub const SENDER_DEFAULT_WINDOW_ADV_PERCENTAGE: u32 = 15u32;
pub const SENDER_MAX_LATE_JOINER_PERCENTAGE: u32 = 75u32;
pub type SEND_RECV_FLAGS = i32;
pub const MSG_OOB: SEND_RECV_FLAGS = 1i32;
pub const MSG_PEEK: SEND_RECV_FLAGS = 2i32;
pub const MSG_DONTROUTE: SEND_RECV_FLAGS = 4i32;
pub const MSG_WAITALL: SEND_RECV_FLAGS = 8i32;
pub const MSG_PUSH_IMMEDIATE: SEND_RECV_FLAGS = 32i32;
#[repr(C)]
pub struct SERVICE_ADDRESS {
    pub dwAddressType: u32,
    pub dwAddressFlags: u32,
    pub dwAddressLength: u32,
    pub dwPrincipalLength: u32,
    pub lpAddress: *mut u8,
    pub lpPrincipal: *mut u8,
}
impl ::core::marker::Copy for SERVICE_ADDRESS {}
impl ::core::clone::Clone for SERVICE_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_ADDRESSES {
    pub dwAddressCount: u32,
    pub Addresses: [SERVICE_ADDRESS; 1],
}
impl ::core::marker::Copy for SERVICE_ADDRESSES {}
impl ::core::clone::Clone for SERVICE_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_ADDRESS_FLAG_RPC_CN: u32 = 1u32;
pub const SERVICE_ADDRESS_FLAG_RPC_DG: u32 = 2u32;
pub const SERVICE_ADDRESS_FLAG_RPC_NB: u32 = 4u32;
#[repr(C)]
pub struct SERVICE_ASYNC_INFO {
    pub lpServiceCallbackProc: LPSERVICE_CALLBACK_PROC,
    pub lParam: ::win32_foundation_sys::LPARAM,
    pub hAsyncTaskHandle: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for SERVICE_ASYNC_INFO {}
impl ::core::clone::Clone for SERVICE_ASYNC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_FLAG_DEFER: u32 = 1u32;
pub const SERVICE_FLAG_HARD: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct SERVICE_INFOA {
    pub lpServiceType: *mut ::windows_core_sys::GUID,
    pub lpServiceName: ::windows_core_sys::PSTR,
    pub lpComment: ::windows_core_sys::PSTR,
    pub lpLocale: ::windows_core_sys::PSTR,
    pub dwDisplayHint: RESOURCE_DISPLAY_TYPE,
    pub dwVersion: u32,
    pub dwTime: u32,
    pub lpMachineName: ::windows_core_sys::PSTR,
    pub lpServiceAddress: *mut SERVICE_ADDRESSES,
    pub ServiceSpecificInfo: ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for SERVICE_INFOA {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for SERVICE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct SERVICE_INFOW {
    pub lpServiceType: *mut ::windows_core_sys::GUID,
    pub lpServiceName: ::windows_core_sys::PWSTR,
    pub lpComment: ::windows_core_sys::PWSTR,
    pub lpLocale: ::windows_core_sys::PWSTR,
    pub dwDisplayHint: RESOURCE_DISPLAY_TYPE,
    pub dwVersion: u32,
    pub dwTime: u32,
    pub lpMachineName: ::windows_core_sys::PWSTR,
    pub lpServiceAddress: *mut SERVICE_ADDRESSES,
    pub ServiceSpecificInfo: ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for SERVICE_INFOW {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for SERVICE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_LOCAL: u32 = 4u32;
pub const SERVICE_MULTIPLE: u32 = 1u32;
pub const SERVICE_RESOURCE: u32 = 1u32;
pub const SERVICE_SERVICE: u32 = 2u32;
#[repr(C)]
pub struct SERVICE_TYPE_INFO {
    pub dwTypeNameOffset: u32,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE; 1],
}
impl ::core::marker::Copy for SERVICE_TYPE_INFO {}
impl ::core::clone::Clone for SERVICE_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TYPE_INFO_ABSA {
    pub lpTypeName: ::windows_core_sys::PSTR,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE_ABSA; 1],
}
impl ::core::marker::Copy for SERVICE_TYPE_INFO_ABSA {}
impl ::core::clone::Clone for SERVICE_TYPE_INFO_ABSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TYPE_INFO_ABSW {
    pub lpTypeName: ::windows_core_sys::PWSTR,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE_ABSW; 1],
}
impl ::core::marker::Copy for SERVICE_TYPE_INFO_ABSW {}
impl ::core::clone::Clone for SERVICE_TYPE_INFO_ABSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TYPE_VALUE {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub dwValueNameOffset: u32,
    pub dwValueOffset: u32,
}
impl ::core::marker::Copy for SERVICE_TYPE_VALUE {}
impl ::core::clone::Clone for SERVICE_TYPE_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TYPE_VALUE_ABSA {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValueName: ::windows_core_sys::PSTR,
    pub lpValue: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SERVICE_TYPE_VALUE_ABSA {}
impl ::core::clone::Clone for SERVICE_TYPE_VALUE_ABSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TYPE_VALUE_ABSW {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValueName: ::windows_core_sys::PWSTR,
    pub lpValue: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SERVICE_TYPE_VALUE_ABSW {}
impl ::core::clone::Clone for SERVICE_TYPE_VALUE_ABSW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_TYPE_VALUE_CONN: &str = "ConnectionOriented";
pub const SERVICE_TYPE_VALUE_CONNA: &str = "ConnectionOriented";
pub const SERVICE_TYPE_VALUE_CONNW: &str = "ConnectionOriented";
pub const SERVICE_TYPE_VALUE_IPXPORTA: &str = "IpxSocket";
pub const SERVICE_TYPE_VALUE_IPXPORTW: &str = "IpxSocket";
pub const SERVICE_TYPE_VALUE_OBJECTID: &str = "ObjectId";
pub const SERVICE_TYPE_VALUE_OBJECTIDA: &str = "ObjectId";
pub const SERVICE_TYPE_VALUE_OBJECTIDW: &str = "ObjectId";
pub const SERVICE_TYPE_VALUE_SAPID: &str = "SapId";
pub const SERVICE_TYPE_VALUE_SAPIDA: &str = "SapId";
pub const SERVICE_TYPE_VALUE_SAPIDW: &str = "SapId";
pub const SERVICE_TYPE_VALUE_TCPPORT: &str = "TcpPort";
pub const SERVICE_TYPE_VALUE_TCPPORTA: &str = "TcpPort";
pub const SERVICE_TYPE_VALUE_TCPPORTW: &str = "TcpPort";
pub const SERVICE_TYPE_VALUE_UDPPORT: &str = "UdpPort";
pub const SERVICE_TYPE_VALUE_UDPPORTA: &str = "UdpPort";
pub const SERVICE_TYPE_VALUE_UDPPORTW: &str = "UdpPort";
pub type SET_SERVICE_OPERATION = u32;
pub const SERVICE_REGISTER: SET_SERVICE_OPERATION = 1u32;
pub const SERVICE_DEREGISTER: SET_SERVICE_OPERATION = 2u32;
pub const SERVICE_FLUSH: SET_SERVICE_OPERATION = 3u32;
pub const SERVICE_ADD_TYPE: SET_SERVICE_OPERATION = 4u32;
pub const SERVICE_DELETE_TYPE: SET_SERVICE_OPERATION = 5u32;
pub const SET_SERVICE_PARTIAL_SUCCESS: u32 = 1u32;
pub const SG_CONSTRAINED_GROUP: u32 = 2u32;
pub const SG_UNCONSTRAINED_GROUP: u32 = 1u32;
pub const SIOCATMARK: i32 = 1074033415i32;
pub const SIOCGHIWAT: i32 = 1074033409i32;
pub const SIOCGLOWAT: i32 = 1074033411i32;
pub const SIOCSHIWAT: i32 = -2147192064i32;
pub const SIOCSLOWAT: i32 = -2147192062i32;
pub const SIO_ABSORB_RTRALERT: u32 = 2550136837u32;
pub const SIO_ACQUIRE_PORT_RESERVATION: u32 = 2550136932u32;
pub const SIO_ADDRESS_LIST_CHANGE: u32 = 671088663u32;
pub const SIO_ADDRESS_LIST_QUERY: u32 = 1207959574u32;
pub const SIO_ADDRESS_LIST_SORT: u32 = 3355443225u32;
pub const SIO_AF_UNIX_GETPEERPID: u32 = 1476395264u32;
pub const SIO_AF_UNIX_SETBINDPARENTPATH: u32 = 2550137089u32;
pub const SIO_AF_UNIX_SETCONNPARENTPATH: u32 = 2550137090u32;
pub const SIO_APPLY_TRANSPORT_SETTING: u32 = 2550136851u32;
pub const SIO_ASSOCIATE_HANDLE: u32 = 2281701377u32;
pub const SIO_ASSOCIATE_PORT_RESERVATION: u32 = 2550136934u32;
pub const SIO_ASSOCIATE_PVC: u32 = 2417360899u32;
pub const SIO_BASE_HANDLE: u32 = 1207959586u32;
pub const SIO_BSP_HANDLE: u32 = 1207959579u32;
pub const SIO_BSP_HANDLE_POLL: u32 = 1207959581u32;
pub const SIO_BSP_HANDLE_SELECT: u32 = 1207959580u32;
pub const SIO_CPU_AFFINITY: u32 = 2550136853u32;
pub const SIO_DELETE_PEER_TARGET_NAME: u32 = 2550137035u32;
pub const SIO_ENABLE_CIRCULAR_QUEUEING: u32 = 671088642u32;
pub const SIO_EXT_POLL: u32 = 3355443231u32;
pub const SIO_EXT_SELECT: u32 = 3355443230u32;
pub const SIO_EXT_SENDMSG: u32 = 3355443232u32;
pub const SIO_FIND_ROUTE: u32 = 1207959555u32;
pub const SIO_FLUSH: u32 = 671088644u32;
pub const SIO_GET_ATM_ADDRESS: u32 = 3491102722u32;
pub const SIO_GET_ATM_CONNECTION_ID: u32 = 1343619076u32;
pub const SIO_GET_BROADCAST_ADDRESS: u32 = 1207959557u32;
pub const SIO_GET_EXTENSION_FUNCTION_POINTER: u32 = 3355443206u32;
pub const SIO_GET_GROUP_QOS: u32 = 3355443208u32;
pub const SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER: u32 = 3355443236u32;
pub const SIO_GET_NUMBER_OF_ATM_DEVICES: u32 = 1343619073u32;
pub const SIO_GET_QOS: u32 = 3355443207u32;
pub const SIO_GET_TX_TIMESTAMP: u32 = 2550137066u32;
pub const SIO_INDEX_ADD_MCAST: u32 = 2550136842u32;
pub const SIO_INDEX_BIND: u32 = 2550136840u32;
pub const SIO_INDEX_DEL_MCAST: u32 = 2550136843u32;
pub const SIO_INDEX_MCASTIF: u32 = 2550136841u32;
pub const SIO_KEEPALIVE_VALS: u32 = 2550136836u32;
pub const SIO_LIMIT_BROADCASTS: u32 = 2550136839u32;
pub const SIO_LOOPBACK_FAST_PATH: u32 = 2550136848u32;
pub const SIO_MULTICAST_SCOPE: u32 = 2281701386u32;
pub const SIO_MULTIPOINT_LOOPBACK: u32 = 2281701385u32;
pub const SIO_NSP_NOTIFY_CHANGE: u32 = 2281701401u32;
pub const SIO_PRIORITY_HINT: u32 = 2550136856u32;
pub const SIO_QUERY_RSS_PROCESSOR_INFO: u32 = 1207959589u32;
pub const SIO_QUERY_RSS_SCALABILITY_INFO: u32 = 1476395218u32;
pub const SIO_QUERY_SECURITY: u32 = 3623878857u32;
pub const SIO_QUERY_TARGET_PNP_HANDLE: u32 = 1207959576u32;
pub const SIO_QUERY_TRANSPORT_SETTING: u32 = 2550136852u32;
pub const SIO_QUERY_WFP_ALE_ENDPOINT_HANDLE: u32 = 1476395213u32;
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_CONTEXT: u32 = 2550137053u32;
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_RECORDS: u32 = 2550137052u32;
pub const SIO_RCVALL: u32 = 2550136833u32;
pub const SIO_RCVALL_IF: u32 = 2550136846u32;
pub const SIO_RCVALL_IGMPMCAST: u32 = 2550136835u32;
pub const SIO_RCVALL_MCAST: u32 = 2550136834u32;
pub const SIO_RCVALL_MCAST_IF: u32 = 2550136845u32;
pub const SIO_RELEASE_PORT_RESERVATION: u32 = 2550136933u32;
pub const SIO_RESERVED_1: u32 = 2281701402u32;
pub const SIO_RESERVED_2: u32 = 2281701409u32;
pub const SIO_ROUTING_INTERFACE_CHANGE: u32 = 2281701397u32;
pub const SIO_ROUTING_INTERFACE_QUERY: u32 = 3355443220u32;
pub const SIO_SET_COMPATIBILITY_MODE: u32 = 2550137132u32;
pub const SIO_SET_GROUP_QOS: u32 = 2281701388u32;
pub const SIO_SET_PEER_TARGET_NAME: u32 = 2550137034u32;
pub const SIO_SET_PRIORITY_HINT: u32 = 2550136856u32;
pub const SIO_SET_QOS: u32 = 2281701387u32;
pub const SIO_SET_SECURITY: u32 = 2550137032u32;
pub const SIO_SET_WFP_CONNECTION_REDIRECT_RECORDS: u32 = 2550137054u32;
pub const SIO_SOCKET_CLOSE_NOTIFY: u32 = 2550136845u32;
pub const SIO_SOCKET_USAGE_NOTIFICATION: u32 = 2550137036u32;
pub const SIO_TCP_INFO: u32 = 3623878695u32;
pub const SIO_TCP_INITIAL_RTO: u32 = 2550136849u32;
pub const SIO_TCP_SET_ACK_FREQUENCY: u32 = 2550136855u32;
pub const SIO_TCP_SET_ICW: u32 = 2550136854u32;
pub const SIO_TIMESTAMPING: u32 = 2550137067u32;
pub const SIO_TRANSLATE_HANDLE: u32 = 3355443213u32;
pub const SIO_UCAST_IF: u32 = 2550136838u32;
pub const SIO_UDP_CONNRESET: u32 = 2550136844u32;
pub const SIO_UDP_NETRESET: u32 = 2550136847u32;
pub const SIZEOF_IP_OPT_ROUTERALERT: u32 = 4u32;
pub const SIZEOF_IP_OPT_ROUTING_HEADER: u32 = 3u32;
pub const SIZEOF_IP_OPT_SECURITY: u32 = 11u32;
pub const SIZEOF_IP_OPT_STREAMIDENTIFIER: u32 = 4u32;
pub const SIZEOF_IP_OPT_TIMESTAMP_HEADER: u32 = 4u32;
pub const SI_NETWORK: u32 = 3u32;
pub const SI_USER_FAILED: u32 = 2u32;
pub const SI_USER_NOT_SCREENED: u32 = 0u32;
pub const SI_USER_PASSED: u32 = 1u32;
pub const SNAP_CONTROL: u32 = 3u32;
pub const SNAP_DSAP: u32 = 170u32;
#[repr(C)]
pub struct SNAP_HEADER {
    pub Dsap: u8,
    pub Ssap: u8,
    pub Control: u8,
    pub Oui: [u8; 3],
    pub Type: u16,
}
impl ::core::marker::Copy for SNAP_HEADER {}
impl ::core::clone::Clone for SNAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SNAP_OUI: u32 = 0u32;
pub const SNAP_SSAP: u32 = 170u32;
#[repr(C)]
pub struct SOCKADDR {
    pub sa_family: u16,
    pub sa_data: [::win32_foundation_sys::CHAR; 14],
}
impl ::core::marker::Copy for SOCKADDR {}
impl ::core::clone::Clone for SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_DL {
    pub sdl_family: u16,
    pub sdl_data: [u8; 8],
    pub sdl_zero: [u8; 4],
}
impl ::core::marker::Copy for SOCKADDR_DL {}
impl ::core::clone::Clone for SOCKADDR_DL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_IN {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: IN_ADDR,
    pub sin_zero: [::win32_foundation_sys::CHAR; 8],
}
impl ::core::marker::Copy for SOCKADDR_IN {}
impl ::core::clone::Clone for SOCKADDR_IN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_IN6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
    pub Anonymous: SOCKADDR_IN6_0,
}
impl ::core::marker::Copy for SOCKADDR_IN6 {}
impl ::core::clone::Clone for SOCKADDR_IN6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SOCKADDR_IN6_0 {
    pub sin6_scope_id: u32,
    pub sin6_scope_struct: SCOPE_ID,
}
impl ::core::marker::Copy for SOCKADDR_IN6_0 {}
impl ::core::clone::Clone for SOCKADDR_IN6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_IN6_PAIR {
    pub SourceAddress: *mut SOCKADDR_IN6,
    pub DestinationAddress: *mut SOCKADDR_IN6,
}
impl ::core::marker::Copy for SOCKADDR_IN6_PAIR {}
impl ::core::clone::Clone for SOCKADDR_IN6_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_IN6_W2KSP1 {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for SOCKADDR_IN6_W2KSP1 {}
impl ::core::clone::Clone for SOCKADDR_IN6_W2KSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SOCKADDR_INET {
    pub Ipv4: SOCKADDR_IN,
    pub Ipv6: SOCKADDR_IN6,
    pub si_family: u16,
}
impl ::core::marker::Copy for SOCKADDR_INET {}
impl ::core::clone::Clone for SOCKADDR_INET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_IRDA {
    pub irdaAddressFamily: u16,
    pub irdaDeviceID: [u8; 4],
    pub irdaServiceName: [::win32_foundation_sys::CHAR; 25],
}
impl ::core::marker::Copy for SOCKADDR_IRDA {}
impl ::core::clone::Clone for SOCKADDR_IRDA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_STORAGE {
    pub ss_family: u16,
    pub __ss_pad1: [::win32_foundation_sys::CHAR; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [::win32_foundation_sys::CHAR; 112],
}
impl ::core::marker::Copy for SOCKADDR_STORAGE {}
impl ::core::clone::Clone for SOCKADDR_STORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKADDR_STORAGE_XP {
    pub ss_family: i16,
    pub __ss_pad1: [::win32_foundation_sys::CHAR; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [::win32_foundation_sys::CHAR; 112],
}
impl ::core::marker::Copy for SOCKADDR_STORAGE_XP {}
impl ::core::clone::Clone for SOCKADDR_STORAGE_XP {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SOCKET = usize;
#[repr(C)]
pub struct SOCKET_ADDRESS {
    pub lpSockaddr: *mut SOCKADDR,
    pub iSockaddrLength: i32,
}
impl ::core::marker::Copy for SOCKET_ADDRESS {}
impl ::core::clone::Clone for SOCKET_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKET_ADDRESS_LIST {
    pub iAddressCount: i32,
    pub Address: [SOCKET_ADDRESS; 1],
}
impl ::core::marker::Copy for SOCKET_ADDRESS_LIST {}
impl ::core::clone::Clone for SOCKET_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SOCKET_DEFAULT2_QM_POLICY: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2932010908, data2: 14925, data3: 19774, data4: [136, 66, 35, 153, 66, 227, 154, 71] };
pub const SOCKET_ERROR: i32 = -1i32;
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: u32 = 2u32;
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: u32 = 4u32;
pub const SOCKET_INFO_CONNECTION_SECURED: u32 = 1u32;
#[repr(C)]
pub struct SOCKET_PEER_TARGET_NAME {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTargetNameStringLen: u32,
    pub AllStrings: [u16; 1],
}
impl ::core::marker::Copy for SOCKET_PEER_TARGET_NAME {}
impl ::core::clone::Clone for SOCKET_PEER_TARGET_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SOCKET_PRIORITY_HINT = i32;
pub const SocketPriorityHintVeryLow: SOCKET_PRIORITY_HINT = 0i32;
pub const SocketPriorityHintLow: SOCKET_PRIORITY_HINT = 1i32;
pub const SocketPriorityHintNormal: SOCKET_PRIORITY_HINT = 2i32;
pub const SocketMaximumPriorityHintType: SOCKET_PRIORITY_HINT = 3i32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct SOCKET_PROCESSOR_AFFINITY {
    pub Processor: ::win32_system_sys::Kernel::PROCESSOR_NUMBER,
    pub NumaNodeId: u16,
    pub Reserved: u16,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for SOCKET_PROCESSOR_AFFINITY {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for SOCKET_PROCESSOR_AFFINITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: u32 = 1u32;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: u32 = 1u32;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: u32 = 2u32;
pub type SOCKET_SECURITY_PROTOCOL = i32;
pub const SOCKET_SECURITY_PROTOCOL_DEFAULT: SOCKET_SECURITY_PROTOCOL = 0i32;
pub const SOCKET_SECURITY_PROTOCOL_IPSEC: SOCKET_SECURITY_PROTOCOL = 1i32;
pub const SOCKET_SECURITY_PROTOCOL_IPSEC2: SOCKET_SECURITY_PROTOCOL = 2i32;
pub const SOCKET_SECURITY_PROTOCOL_INVALID: SOCKET_SECURITY_PROTOCOL = 3i32;
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_INFO {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_INFO {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
    pub MmSaId: u64,
    pub QmSaId: u64,
    pub NegotiationWinerr: u32,
    pub SaLookupContext: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_TEMPLATE {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
    pub Flags: u32,
    pub FieldMask: u32,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKET_SECURITY_SETTINGS {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
}
impl ::core::marker::Copy for SOCKET_SECURITY_SETTINGS {}
impl ::core::clone::Clone for SOCKET_SECURITY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SOCKET_SECURITY_SETTINGS_IPSEC {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
    pub IpsecFlags: u32,
    pub AuthipMMPolicyKey: ::windows_core_sys::GUID,
    pub AuthipQMPolicyKey: ::windows_core_sys::GUID,
    pub Reserved: ::windows_core_sys::GUID,
    pub Reserved2: u64,
    pub UserNameStringLen: u32,
    pub DomainNameStringLen: u32,
    pub PasswordStringLen: u32,
    pub AllStrings: [u16; 1],
}
impl ::core::marker::Copy for SOCKET_SECURITY_SETTINGS_IPSEC {}
impl ::core::clone::Clone for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SOCKET_SETTINGS_ALLOW_INSECURE: u32 = 2u32;
pub const SOCKET_SETTINGS_GUARANTEE_ENCRYPTION: u32 = 1u32;
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: u32 = 4u32;
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: u32 = 2u32;
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: u32 = 8u32;
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: u32 = 1u32;
pub type SOCKET_USAGE_TYPE = i32;
pub const SYSTEM_CRITICAL_SOCKET: SOCKET_USAGE_TYPE = 1i32;
pub const SOCK_DGRAM: u16 = 2u16;
pub const SOCK_NOTIFY_EVENT_ERR: u32 = 64u32;
pub const SOCK_NOTIFY_EVENT_HANGUP: u32 = 4u32;
pub const SOCK_NOTIFY_EVENT_IN: u32 = 1u32;
pub const SOCK_NOTIFY_EVENT_OUT: u32 = 2u32;
pub const SOCK_NOTIFY_EVENT_REMOVE: u32 = 128u32;
pub const SOCK_NOTIFY_OP_DISABLE: u32 = 2u32;
pub const SOCK_NOTIFY_OP_ENABLE: u32 = 1u32;
pub const SOCK_NOTIFY_OP_NONE: u32 = 0u32;
pub const SOCK_NOTIFY_OP_REMOVE: u32 = 4u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_HANGUP: u32 = 4u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_IN: u32 = 1u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_NONE: u32 = 0u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_OUT: u32 = 2u32;
#[repr(C)]
pub struct SOCK_NOTIFY_REGISTRATION {
    pub socket: SOCKET,
    pub completionKey: *mut ::core::ffi::c_void,
    pub eventFilter: u16,
    pub operation: u8,
    pub triggerFlags: u8,
    pub registrationResult: u32,
}
impl ::core::marker::Copy for SOCK_NOTIFY_REGISTRATION {}
impl ::core::clone::Clone for SOCK_NOTIFY_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SOCK_NOTIFY_TRIGGER_EDGE: u32 = 8u32;
pub const SOCK_NOTIFY_TRIGGER_LEVEL: u32 = 4u32;
pub const SOCK_NOTIFY_TRIGGER_ONESHOT: u32 = 1u32;
pub const SOCK_NOTIFY_TRIGGER_PERSISTENT: u32 = 2u32;
pub const SOCK_RAW: u16 = 3u16;
pub const SOCK_RDM: u16 = 4u16;
pub const SOCK_SEQPACKET: u16 = 5u16;
pub const SOCK_STREAM: u16 = 1u16;
pub const SOL_IRLMP: u32 = 255u32;
pub const SOL_SOCKET: u32 = 65535u32;
pub const SOMAXCONN: u32 = 5u32;
pub const SO_ACCEPTCONN: u32 = 2u32;
pub const SO_BROADCAST: u32 = 32u32;
pub const SO_BSP_STATE: u32 = 4105u32;
pub const SO_COMPARTMENT_ID: u32 = 12292u32;
pub const SO_CONDITIONAL_ACCEPT: u32 = 12290u32;
pub const SO_CONNDATA: u32 = 28672u32;
pub const SO_CONNDATALEN: u32 = 28676u32;
pub const SO_CONNECT_TIME: u32 = 28684u32;
pub const SO_CONNOPT: u32 = 28673u32;
pub const SO_CONNOPTLEN: u32 = 28677u32;
pub const SO_DEBUG: u32 = 1u32;
pub const SO_DISCDATA: u32 = 28674u32;
pub const SO_DISCDATALEN: u32 = 28678u32;
pub const SO_DISCOPT: u32 = 28675u32;
pub const SO_DISCOPTLEN: u32 = 28679u32;
pub const SO_DONTROUTE: u32 = 16u32;
pub const SO_ERROR: u32 = 4103u32;
pub const SO_GROUP_ID: u32 = 8193u32;
pub const SO_GROUP_PRIORITY: u32 = 8194u32;
pub const SO_KEEPALIVE: u32 = 8u32;
pub const SO_LINGER: u32 = 128u32;
pub const SO_MAXDG: u32 = 28681u32;
pub const SO_MAXPATHDG: u32 = 28682u32;
pub const SO_MAX_MSG_SIZE: u32 = 8195u32;
pub const SO_OOBINLINE: u32 = 256u32;
pub const SO_OPENTYPE: u32 = 28680u32;
pub const SO_ORIGINAL_DST: u32 = 12303u32;
pub const SO_PAUSE_ACCEPT: u32 = 12291u32;
pub const SO_PORT_SCALABILITY: u32 = 12294u32;
pub const SO_PROTOCOL_INFO: u32 = 8197u32;
pub const SO_PROTOCOL_INFOA: u32 = 8196u32;
pub const SO_PROTOCOL_INFOW: u32 = 8197u32;
pub const SO_RANDOMIZE_PORT: u32 = 12293u32;
pub const SO_RCVBUF: u32 = 4098u32;
pub const SO_RCVLOWAT: u32 = 4100u32;
pub const SO_RCVTIMEO: u32 = 4102u32;
pub const SO_REUSEADDR: u32 = 4u32;
pub const SO_REUSE_MULTICASTPORT: u32 = 12296u32;
pub const SO_REUSE_UNICASTPORT: u32 = 12295u32;
pub const SO_SNDBUF: u32 = 4097u32;
pub const SO_SNDLOWAT: u32 = 4099u32;
pub const SO_SNDTIMEO: u32 = 4101u32;
pub const SO_SYNCHRONOUS_ALERT: u32 = 16u32;
pub const SO_SYNCHRONOUS_NONALERT: u32 = 32u32;
pub const SO_TIMESTAMP: u32 = 12298u32;
pub const SO_TIMESTAMP_ID: u32 = 12299u32;
pub const SO_TYPE: u32 = 4104u32;
pub const SO_UPDATE_ACCEPT_CONTEXT: u32 = 28683u32;
pub const SO_UPDATE_CONNECT_CONTEXT: u32 = 28688u32;
pub const SO_USELOOPBACK: u32 = 64u32;
pub type TCPSTATE = i32;
pub const TCPSTATE_CLOSED: TCPSTATE = 0i32;
pub const TCPSTATE_LISTEN: TCPSTATE = 1i32;
pub const TCPSTATE_SYN_SENT: TCPSTATE = 2i32;
pub const TCPSTATE_SYN_RCVD: TCPSTATE = 3i32;
pub const TCPSTATE_ESTABLISHED: TCPSTATE = 4i32;
pub const TCPSTATE_FIN_WAIT_1: TCPSTATE = 5i32;
pub const TCPSTATE_FIN_WAIT_2: TCPSTATE = 6i32;
pub const TCPSTATE_CLOSE_WAIT: TCPSTATE = 7i32;
pub const TCPSTATE_CLOSING: TCPSTATE = 8i32;
pub const TCPSTATE_LAST_ACK: TCPSTATE = 9i32;
pub const TCPSTATE_TIME_WAIT: TCPSTATE = 10i32;
pub const TCPSTATE_MAX: TCPSTATE = 11i32;
#[repr(C)]
pub struct TCP_ACK_FREQUENCY_PARAMETERS {
    pub TcpDelayedAckFrequency: u8,
}
impl ::core::marker::Copy for TCP_ACK_FREQUENCY_PARAMETERS {}
impl ::core::clone::Clone for TCP_ACK_FREQUENCY_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TCP_ATMARK: u32 = 8u32;
pub const TCP_BSDURGENT: u32 = 28672u32;
pub const TCP_CONGESTION_ALGORITHM: u32 = 12u32;
pub const TCP_DELAY_FIN_ACK: u32 = 13u32;
pub const TCP_EXPEDITED_1122: u32 = 2u32;
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: u32 = 18u32;
pub const TCP_FASTOPEN: u32 = 15u32;
pub const TCP_ICMP_ERROR_INFO: u32 = 19u32;
pub type TCP_ICW_LEVEL = i32;
pub const TCP_ICW_LEVEL_DEFAULT: TCP_ICW_LEVEL = 0i32;
pub const TCP_ICW_LEVEL_HIGH: TCP_ICW_LEVEL = 1i32;
pub const TCP_ICW_LEVEL_VERY_HIGH: TCP_ICW_LEVEL = 2i32;
pub const TCP_ICW_LEVEL_AGGRESSIVE: TCP_ICW_LEVEL = 3i32;
pub const TCP_ICW_LEVEL_EXPERIMENTAL: TCP_ICW_LEVEL = 4i32;
pub const TCP_ICW_LEVEL_COMPAT: TCP_ICW_LEVEL = 254i32;
pub const TCP_ICW_LEVEL_MAX: TCP_ICW_LEVEL = 255i32;
#[repr(C)]
pub struct TCP_ICW_PARAMETERS {
    pub Level: TCP_ICW_LEVEL,
}
impl ::core::marker::Copy for TCP_ICW_PARAMETERS {}
impl ::core::clone::Clone for TCP_ICW_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TCP_INFO_v0 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: ::win32_foundation_sys::BOOLEAN,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
}
impl ::core::marker::Copy for TCP_INFO_v0 {}
impl ::core::clone::Clone for TCP_INFO_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TCP_INFO_v1 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: ::win32_foundation_sys::BOOLEAN,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: u64,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: u64,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: u64,
}
impl ::core::marker::Copy for TCP_INFO_v1 {}
impl ::core::clone::Clone for TCP_INFO_v1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: u32 = 0u32;
pub const TCP_INITIAL_RTO_DEFAULT_RTT: u32 = 0u32;
#[repr(C)]
pub struct TCP_INITIAL_RTO_PARAMETERS {
    pub Rtt: u16,
    pub MaxSynRetransmissions: u8,
}
impl ::core::marker::Copy for TCP_INITIAL_RTO_PARAMETERS {}
impl ::core::clone::Clone for TCP_INITIAL_RTO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TCP_KEEPALIVE: u32 = 3u32;
pub const TCP_KEEPCNT: u32 = 16u32;
pub const TCP_KEEPIDLE: u32 = 3u32;
pub const TCP_KEEPINTVL: u32 = 17u32;
pub const TCP_MAXRT: u32 = 5u32;
pub const TCP_MAXRTMS: u32 = 14u32;
pub const TCP_MAXSEG: u32 = 4u32;
pub const TCP_NODELAY: u32 = 1u32;
pub const TCP_NOSYNRETRIES: u32 = 9u32;
pub const TCP_NOURG: u32 = 7u32;
pub const TCP_OFFLOAD_NOT_PREFERRED: u32 = 1u32;
pub const TCP_OFFLOAD_NO_PREFERENCE: u32 = 0u32;
pub const TCP_OFFLOAD_PREFERENCE: u32 = 11u32;
pub const TCP_OFFLOAD_PREFERRED: u32 = 2u32;
pub const TCP_STDURG: u32 = 6u32;
pub const TCP_TIMESTAMPS: u32 = 10u32;
pub const TF_DISCONNECT: u32 = 1u32;
pub const TF_REUSE_SOCKET: u32 = 2u32;
pub const TF_USE_DEFAULT_WORKER: u32 = 0u32;
pub const TF_USE_KERNEL_APC: u32 = 32u32;
pub const TF_USE_SYSTEM_THREAD: u32 = 16u32;
pub const TF_WRITE_BEHIND: u32 = 4u32;
pub const TH_ACK: u32 = 16u32;
pub const TH_CWR: u32 = 128u32;
pub const TH_ECE: u32 = 64u32;
pub const TH_FIN: u32 = 1u32;
pub const TH_NETDEV: u32 = 1u32;
pub const TH_OPT_EOL: u32 = 0u32;
pub const TH_OPT_FASTOPEN: u32 = 34u32;
pub const TH_OPT_MSS: u32 = 2u32;
pub const TH_OPT_NOP: u32 = 1u32;
pub const TH_OPT_SACK: u32 = 5u32;
pub const TH_OPT_SACK_PERMITTED: u32 = 4u32;
pub const TH_OPT_TS: u32 = 8u32;
pub const TH_OPT_WS: u32 = 3u32;
pub const TH_PSH: u32 = 8u32;
pub const TH_RST: u32 = 4u32;
pub const TH_SYN: u32 = 2u32;
pub const TH_TAPI: u32 = 2u32;
pub const TH_URG: u32 = 32u32;
#[repr(C)]
pub struct TIMESTAMPING_CONFIG {
    pub Flags: u32,
    pub TxTimestampsBuffered: u16,
}
impl ::core::marker::Copy for TIMESTAMPING_CONFIG {}
impl ::core::clone::Clone for TIMESTAMPING_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TIMESTAMPING_FLAG_RX: u32 = 1u32;
pub const TIMESTAMPING_FLAG_TX: u32 = 2u32;
pub const TNS_PLAN_CARRIER_ID_CODE: u32 = 1u32;
pub const TNS_TYPE_NATIONAL: u32 = 64u32;
pub const TP_DISCONNECT: u32 = 1u32;
pub const TP_ELEMENT_EOP: u32 = 4u32;
pub const TP_ELEMENT_FILE: u32 = 2u32;
pub const TP_ELEMENT_MEMORY: u32 = 1u32;
pub const TP_REUSE_SOCKET: u32 = 2u32;
pub const TP_USE_DEFAULT_WORKER: u32 = 0u32;
pub const TP_USE_KERNEL_APC: u32 = 32u32;
pub const TP_USE_SYSTEM_THREAD: u32 = 16u32;
#[repr(C)]
pub struct TRANSMIT_FILE_BUFFERS {
    pub Head: *mut ::core::ffi::c_void,
    pub HeadLength: u32,
    pub Tail: *mut ::core::ffi::c_void,
    pub TailLength: u32,
}
impl ::core::marker::Copy for TRANSMIT_FILE_BUFFERS {}
impl ::core::clone::Clone for TRANSMIT_FILE_BUFFERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TRANSMIT_PACKETS_ELEMENT {
    pub dwElFlags: u32,
    pub cLength: u32,
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0,
}
impl ::core::marker::Copy for TRANSMIT_PACKETS_ELEMENT {}
impl ::core::clone::Clone for TRANSMIT_PACKETS_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union TRANSMIT_PACKETS_ELEMENT_0 {
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0_0,
    pub pBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TRANSMIT_PACKETS_ELEMENT_0 {}
impl ::core::clone::Clone for TRANSMIT_PACKETS_ELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TRANSMIT_PACKETS_ELEMENT_0_0 {
    pub nFileOffset: i64,
    pub hFile: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for TRANSMIT_PACKETS_ELEMENT_0_0 {}
impl ::core::clone::Clone for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TRANSPORT_SETTING_ID {
    pub Guid: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for TRANSPORT_SETTING_ID {}
impl ::core::clone::Clone for TRANSPORT_SETTING_ID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TR_END_TO_END: u32 = 1u32;
pub const TR_NOIND: u32 = 0u32;
pub const TR_NO_END_TO_END: u32 = 2u32;
pub const TT_CBR: u32 = 4u32;
pub const TT_NOIND: u32 = 0u32;
pub const TT_VBR: u32 = 8u32;
pub type TUNNEL_SUB_TYPE = i32;
pub const TUNNEL_SUB_TYPE_NONE: TUNNEL_SUB_TYPE = 0i32;
pub const TUNNEL_SUB_TYPE_CP: TUNNEL_SUB_TYPE = 1i32;
pub const TUNNEL_SUB_TYPE_IPTLS: TUNNEL_SUB_TYPE = 2i32;
pub const TUNNEL_SUB_TYPE_HA: TUNNEL_SUB_TYPE = 3i32;
pub const UDP_CHECKSUM_COVERAGE: u32 = 20u32;
pub const UDP_COALESCED_INFO: u32 = 3u32;
pub const UDP_NOCHECKSUM: u32 = 1u32;
pub const UDP_RECV_MAX_COALESCED_SIZE: u32 = 3u32;
pub const UDP_SEND_MSG_SIZE: u32 = 2u32;
pub const UNIX_PATH_MAX: u32 = 108u32;
pub const UP_P2MP: u32 = 1u32;
pub const UP_P2P: u32 = 0u32;
#[repr(C)]
pub struct VLAN_TAG {
    pub Anonymous: VLAN_TAG_0,
    pub Type: u16,
}
impl ::core::marker::Copy for VLAN_TAG {}
impl ::core::clone::Clone for VLAN_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VLAN_TAG_0 {
    pub Tag: u16,
    pub Anonymous: VLAN_TAG_0_0,
}
impl ::core::marker::Copy for VLAN_TAG_0 {}
impl ::core::clone::Clone for VLAN_TAG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VLAN_TAG_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for VLAN_TAG_0_0 {}
impl ::core::clone::Clone for VLAN_TAG_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VNSPROTO_IPC: u32 = 1u32;
pub const VNSPROTO_RELIABLE_IPC: u32 = 2u32;
pub const VNSPROTO_SPP: u32 = 3u32;
pub const WCE_AF_IRDA: u32 = 22u32;
#[repr(C)]
pub struct WCE_DEVICELIST {
    pub numDevice: u32,
    pub Device: [WCE_IRDA_DEVICE_INFO; 1],
}
impl ::core::marker::Copy for WCE_DEVICELIST {}
impl ::core::clone::Clone for WCE_DEVICELIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WCE_IRDA_DEVICE_INFO {
    pub irdaDeviceID: [u8; 4],
    pub irdaDeviceName: [::win32_foundation_sys::CHAR; 22],
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for WCE_IRDA_DEVICE_INFO {}
impl ::core::clone::Clone for WCE_IRDA_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WCE_PF_IRDA: u32 = 22u32;
pub const WINDOWS_AF_IRDA: u32 = 26u32;
#[repr(C)]
pub struct WINDOWS_DEVICELIST {
    pub numDevice: u32,
    pub Device: [WINDOWS_IRDA_DEVICE_INFO; 1],
}
impl ::core::marker::Copy for WINDOWS_DEVICELIST {}
impl ::core::clone::Clone for WINDOWS_DEVICELIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IAS_QUERY {
    pub irdaDeviceID: [u8; 4],
    pub irdaClassName: [::win32_foundation_sys::CHAR; 64],
    pub irdaAttribName: [::win32_foundation_sys::CHAR; 256],
    pub irdaAttribType: u32,
    pub irdaAttribute: WINDOWS_IAS_QUERY_0,
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WINDOWS_IAS_QUERY_0 {
    pub irdaAttribInt: i32,
    pub irdaAttribOctetSeq: WINDOWS_IAS_QUERY_0_0,
    pub irdaAttribUsrStr: WINDOWS_IAS_QUERY_0_1,
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IAS_QUERY_0_0 {
    pub Len: u32,
    pub OctetSeq: [u8; 1024],
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY_0_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IAS_QUERY_0_1 {
    pub Len: u32,
    pub CharSet: u32,
    pub UsrStr: [u8; 256],
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY_0_1 {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IAS_SET {
    pub irdaClassName: [::win32_foundation_sys::CHAR; 64],
    pub irdaAttribName: [::win32_foundation_sys::CHAR; 256],
    pub irdaAttribType: u32,
    pub irdaAttribute: WINDOWS_IAS_SET_0,
}
impl ::core::marker::Copy for WINDOWS_IAS_SET {}
impl ::core::clone::Clone for WINDOWS_IAS_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WINDOWS_IAS_SET_0 {
    pub irdaAttribInt: i32,
    pub irdaAttribOctetSeq: WINDOWS_IAS_SET_0_0,
    pub irdaAttribUsrStr: WINDOWS_IAS_SET_0_1,
}
impl ::core::marker::Copy for WINDOWS_IAS_SET_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_SET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IAS_SET_0_0 {
    pub Len: u16,
    pub OctetSeq: [u8; 1024],
}
impl ::core::marker::Copy for WINDOWS_IAS_SET_0_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_SET_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IAS_SET_0_1 {
    pub Len: u8,
    pub CharSet: u8,
    pub UsrStr: [u8; 256],
}
impl ::core::marker::Copy for WINDOWS_IAS_SET_0_1 {}
impl ::core::clone::Clone for WINDOWS_IAS_SET_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WINDOWS_IRDA_DEVICE_INFO {
    pub irdaDeviceID: [u8; 4],
    pub irdaDeviceName: [::win32_foundation_sys::CHAR; 22],
    pub irdaDeviceHints1: u8,
    pub irdaDeviceHints2: u8,
    pub irdaCharSet: u8,
}
impl ::core::marker::Copy for WINDOWS_IRDA_DEVICE_INFO {}
impl ::core::clone::Clone for WINDOWS_IRDA_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WINDOWS_PF_IRDA: u32 = 26u32;
#[repr(C)]
pub struct WSABUF {
    pub len: u32,
    pub buf: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for WSABUF {}
impl ::core::clone::Clone for WSABUF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSACOMPLETION {
    pub Type: WSACOMPLETIONTYPE,
    pub Parameters: WSACOMPLETION_0,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSACOMPLETION {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSACOMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub union WSACOMPLETION_0 {
    pub WindowMessage: WSACOMPLETION_0_3,
    pub Event: WSACOMPLETION_0_1,
    pub Apc: WSACOMPLETION_0_0,
    pub Port: WSACOMPLETION_0_2,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSACOMPLETION_0 {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSACOMPLETION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSACOMPLETION_0_0 {
    pub lpOverlapped: *mut ::win32_system_sys::IO::OVERLAPPED,
    pub lpfnCompletionProc: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSACOMPLETION_0_0 {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSACOMPLETION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSACOMPLETION_0_1 {
    pub lpOverlapped: *mut ::win32_system_sys::IO::OVERLAPPED,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSACOMPLETION_0_1 {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSACOMPLETION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSACOMPLETION_0_2 {
    pub lpOverlapped: *mut ::win32_system_sys::IO::OVERLAPPED,
    pub hPort: ::win32_foundation_sys::HANDLE,
    pub Key: usize,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSACOMPLETION_0_2 {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSACOMPLETION_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSACOMPLETION_0_3 {
    pub hWnd: ::win32_foundation_sys::HWND,
    pub uMsg: u32,
    pub context: ::win32_foundation_sys::WPARAM,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSACOMPLETION_0_3 {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSACOMPLETION_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WSACOMPLETIONTYPE = i32;
pub const NSP_NOTIFY_IMMEDIATELY: WSACOMPLETIONTYPE = 0i32;
pub const NSP_NOTIFY_HWND: WSACOMPLETIONTYPE = 1i32;
pub const NSP_NOTIFY_EVENT: WSACOMPLETIONTYPE = 2i32;
pub const NSP_NOTIFY_PORT: WSACOMPLETIONTYPE = 3i32;
pub const NSP_NOTIFY_APC: WSACOMPLETIONTYPE = 4i32;
pub const WSADESCRIPTION_LEN: u32 = 256u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct WSAData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: ::windows_core_sys::PSTR,
    pub szDescription: [::win32_foundation_sys::CHAR; 257],
    pub szSystemStatus: [::win32_foundation_sys::CHAR; 129],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for WSAData {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for WSAData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct WSAData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [::win32_foundation_sys::CHAR; 257],
    pub szSystemStatus: [::win32_foundation_sys::CHAR; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: ::windows_core_sys::PSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for WSAData {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for WSAData {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WSAECOMPARATOR = i32;
pub const COMP_EQUAL: WSAECOMPARATOR = 0i32;
pub const COMP_NOTLESS: WSAECOMPARATOR = 1i32;
pub type WSAESETSERVICEOP = i32;
pub const RNRSERVICE_REGISTER: WSAESETSERVICEOP = 0i32;
pub const RNRSERVICE_DEREGISTER: WSAESETSERVICEOP = 1i32;
pub const RNRSERVICE_DELETE: WSAESETSERVICEOP = 2i32;
#[repr(C)]
pub struct WSAMSG {
    pub name: *mut SOCKADDR,
    pub namelen: i32,
    pub lpBuffers: *mut WSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for WSAMSG {}
impl ::core::clone::Clone for WSAMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSANAMESPACE_INFOA {
    pub NSProviderId: ::windows_core_sys::GUID,
    pub dwNameSpace: u32,
    pub fActive: ::win32_foundation_sys::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for WSANAMESPACE_INFOA {}
impl ::core::clone::Clone for WSANAMESPACE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSANAMESPACE_INFOEXA {
    pub NSProviderId: ::windows_core_sys::GUID,
    pub dwNameSpace: u32,
    pub fActive: ::win32_foundation_sys::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: ::windows_core_sys::PSTR,
    pub ProviderSpecific: ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSANAMESPACE_INFOEXA {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSANAMESPACE_INFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSANAMESPACE_INFOEXW {
    pub NSProviderId: ::windows_core_sys::GUID,
    pub dwNameSpace: u32,
    pub fActive: ::win32_foundation_sys::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: ::windows_core_sys::PWSTR,
    pub ProviderSpecific: ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSANAMESPACE_INFOEXW {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSANAMESPACE_INFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSANAMESPACE_INFOW {
    pub NSProviderId: ::windows_core_sys::GUID,
    pub dwNameSpace: u32,
    pub fActive: ::win32_foundation_sys::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for WSANAMESPACE_INFOW {}
impl ::core::clone::Clone for WSANAMESPACE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSANETWORKEVENTS {
    pub lNetworkEvents: i32,
    pub iErrorCode: [i32; 10],
}
impl ::core::marker::Copy for WSANETWORKEVENTS {}
impl ::core::clone::Clone for WSANETWORKEVENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSANSCLASSINFOA {
    pub lpszName: ::windows_core_sys::PSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSANSCLASSINFOA {}
impl ::core::clone::Clone for WSANSCLASSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSANSCLASSINFOW {
    pub lpszName: ::windows_core_sys::PWSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSANSCLASSINFOW {}
impl ::core::clone::Clone for WSANSCLASSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAPOLLDATA {
    pub result: i32,
    pub fds: u32,
    pub timeout: i32,
    pub fdArray: [WSAPOLLFD; 1],
}
impl ::core::marker::Copy for WSAPOLLDATA {}
impl ::core::clone::Clone for WSAPOLLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAPOLLFD {
    pub fd: SOCKET,
    pub events: i16,
    pub revents: i16,
}
impl ::core::marker::Copy for WSAPOLLFD {}
impl ::core::clone::Clone for WSAPOLLFD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAPROTOCOLCHAIN {
    pub ChainLen: i32,
    pub ChainEntries: [u32; 7],
}
impl ::core::marker::Copy for WSAPROTOCOLCHAIN {}
impl ::core::clone::Clone for WSAPROTOCOLCHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAPROTOCOL_INFOA {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: ::windows_core_sys::GUID,
    pub dwCatalogEntryId: u32,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [::win32_foundation_sys::CHAR; 256],
}
impl ::core::marker::Copy for WSAPROTOCOL_INFOA {}
impl ::core::clone::Clone for WSAPROTOCOL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAPROTOCOL_INFOW {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: ::windows_core_sys::GUID,
    pub dwCatalogEntryId: u32,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [u16; 256],
}
impl ::core::marker::Copy for WSAPROTOCOL_INFOW {}
impl ::core::clone::Clone for WSAPROTOCOL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WSAPROTOCOL_LEN: u32 = 255u32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSAQUERYSET2A {
    pub dwSize: u32,
    pub lpszServiceInstanceName: ::windows_core_sys::PSTR,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: ::windows_core_sys::PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows_core_sys::GUID,
    pub lpszContext: ::windows_core_sys::PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: ::windows_core_sys::PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSAQUERYSET2A {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSAQUERYSET2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSAQUERYSET2W {
    pub dwSize: u32,
    pub lpszServiceInstanceName: ::windows_core_sys::PWSTR,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: ::windows_core_sys::PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows_core_sys::GUID,
    pub lpszContext: ::windows_core_sys::PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: ::windows_core_sys::PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSAQUERYSET2W {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSAQUERYSET2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSAQUERYSETA {
    pub dwSize: u32,
    pub lpszServiceInstanceName: ::windows_core_sys::PSTR,
    pub lpServiceClassId: *mut ::windows_core_sys::GUID,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: ::windows_core_sys::PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows_core_sys::GUID,
    pub lpszContext: ::windows_core_sys::PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: ::windows_core_sys::PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSAQUERYSETA {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSAQUERYSETA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSAQUERYSETW {
    pub dwSize: u32,
    pub lpszServiceInstanceName: ::windows_core_sys::PWSTR,
    pub lpServiceClassId: *mut ::windows_core_sys::GUID,
    pub lpVersion: *mut WSAVERSION,
    pub lpszComment: ::windows_core_sys::PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: *mut ::windows_core_sys::GUID,
    pub lpszContext: ::windows_core_sys::PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: *mut AFPROTOCOLS,
    pub lpszQueryString: ::windows_core_sys::PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: *mut CSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: *mut ::win32_system_sys::Com::BLOB,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSAQUERYSETW {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSAQUERYSETW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSASENDMSG {
    pub lpMsg: *mut WSAMSG,
    pub dwFlags: u32,
    pub lpNumberOfBytesSent: *mut u32,
    pub lpOverlapped: *mut ::win32_system_sys::IO::OVERLAPPED,
    pub lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSASENDMSG {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSASENDMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSASERVICECLASSINFOA {
    pub lpServiceClassId: *mut ::windows_core_sys::GUID,
    pub lpszServiceClassName: ::windows_core_sys::PSTR,
    pub dwCount: u32,
    pub lpClassInfos: *mut WSANSCLASSINFOA,
}
impl ::core::marker::Copy for WSASERVICECLASSINFOA {}
impl ::core::clone::Clone for WSASERVICECLASSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSASERVICECLASSINFOW {
    pub lpServiceClassId: *mut ::windows_core_sys::GUID,
    pub lpszServiceClassName: ::windows_core_sys::PWSTR,
    pub dwCount: u32,
    pub lpClassInfos: *mut WSANSCLASSINFOW,
}
impl ::core::marker::Copy for WSASERVICECLASSINFOW {}
impl ::core::clone::Clone for WSASERVICECLASSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WSASYS_STATUS_LEN: u32 = 128u32;
#[repr(C)]
pub struct WSATHREADID {
    pub ThreadHandle: ::win32_foundation_sys::HANDLE,
    pub Reserved: usize,
}
impl ::core::marker::Copy for WSATHREADID {}
impl ::core::clone::Clone for WSATHREADID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSAVERSION {
    pub dwVersion: u32,
    pub ecHow: WSAECOMPARATOR,
}
impl ::core::marker::Copy for WSAVERSION {}
impl ::core::clone::Clone for WSAVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WSA_COMPATIBILITY_BEHAVIOR_ID = i32;
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = 0i32;
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID = 1i32;
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID = 2i32;
#[repr(C)]
pub struct WSA_COMPATIBILITY_MODE {
    pub BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    pub TargetOsVersion: u32,
}
impl ::core::marker::Copy for WSA_COMPATIBILITY_MODE {}
impl ::core::clone::Clone for WSA_COMPATIBILITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WSA_ERROR = i32;
pub const WSA_IO_PENDING: WSA_ERROR = 997i32;
pub const WSA_IO_INCOMPLETE: WSA_ERROR = 996i32;
pub const WSA_INVALID_HANDLE: WSA_ERROR = 6i32;
pub const WSA_INVALID_PARAMETER: WSA_ERROR = 87i32;
pub const WSA_NOT_ENOUGH_MEMORY: WSA_ERROR = 8i32;
pub const WSA_OPERATION_ABORTED: WSA_ERROR = 995i32;
pub const WSABASEERR: WSA_ERROR = 10000i32;
pub const WSAEINTR: WSA_ERROR = 10004i32;
pub const WSAEBADF: WSA_ERROR = 10009i32;
pub const WSAEACCES: WSA_ERROR = 10013i32;
pub const WSAEFAULT: WSA_ERROR = 10014i32;
pub const WSAEINVAL: WSA_ERROR = 10022i32;
pub const WSAEMFILE: WSA_ERROR = 10024i32;
pub const WSAEWOULDBLOCK: WSA_ERROR = 10035i32;
pub const WSAEINPROGRESS: WSA_ERROR = 10036i32;
pub const WSAEALREADY: WSA_ERROR = 10037i32;
pub const WSAENOTSOCK: WSA_ERROR = 10038i32;
pub const WSAEDESTADDRREQ: WSA_ERROR = 10039i32;
pub const WSAEMSGSIZE: WSA_ERROR = 10040i32;
pub const WSAEPROTOTYPE: WSA_ERROR = 10041i32;
pub const WSAENOPROTOOPT: WSA_ERROR = 10042i32;
pub const WSAEPROTONOSUPPORT: WSA_ERROR = 10043i32;
pub const WSAESOCKTNOSUPPORT: WSA_ERROR = 10044i32;
pub const WSAEOPNOTSUPP: WSA_ERROR = 10045i32;
pub const WSAEPFNOSUPPORT: WSA_ERROR = 10046i32;
pub const WSAEAFNOSUPPORT: WSA_ERROR = 10047i32;
pub const WSAEADDRINUSE: WSA_ERROR = 10048i32;
pub const WSAEADDRNOTAVAIL: WSA_ERROR = 10049i32;
pub const WSAENETDOWN: WSA_ERROR = 10050i32;
pub const WSAENETUNREACH: WSA_ERROR = 10051i32;
pub const WSAENETRESET: WSA_ERROR = 10052i32;
pub const WSAECONNABORTED: WSA_ERROR = 10053i32;
pub const WSAECONNRESET: WSA_ERROR = 10054i32;
pub const WSAENOBUFS: WSA_ERROR = 10055i32;
pub const WSAEISCONN: WSA_ERROR = 10056i32;
pub const WSAENOTCONN: WSA_ERROR = 10057i32;
pub const WSAESHUTDOWN: WSA_ERROR = 10058i32;
pub const WSAETOOMANYREFS: WSA_ERROR = 10059i32;
pub const WSAETIMEDOUT: WSA_ERROR = 10060i32;
pub const WSAECONNREFUSED: WSA_ERROR = 10061i32;
pub const WSAELOOP: WSA_ERROR = 10062i32;
pub const WSAENAMETOOLONG: WSA_ERROR = 10063i32;
pub const WSAEHOSTDOWN: WSA_ERROR = 10064i32;
pub const WSAEHOSTUNREACH: WSA_ERROR = 10065i32;
pub const WSAENOTEMPTY: WSA_ERROR = 10066i32;
pub const WSAEPROCLIM: WSA_ERROR = 10067i32;
pub const WSAEUSERS: WSA_ERROR = 10068i32;
pub const WSAEDQUOT: WSA_ERROR = 10069i32;
pub const WSAESTALE: WSA_ERROR = 10070i32;
pub const WSAEREMOTE: WSA_ERROR = 10071i32;
pub const WSASYSNOTREADY: WSA_ERROR = 10091i32;
pub const WSAVERNOTSUPPORTED: WSA_ERROR = 10092i32;
pub const WSANOTINITIALISED: WSA_ERROR = 10093i32;
pub const WSAEDISCON: WSA_ERROR = 10101i32;
pub const WSAENOMORE: WSA_ERROR = 10102i32;
pub const WSAECANCELLED: WSA_ERROR = 10103i32;
pub const WSAEINVALIDPROCTABLE: WSA_ERROR = 10104i32;
pub const WSAEINVALIDPROVIDER: WSA_ERROR = 10105i32;
pub const WSAEPROVIDERFAILEDINIT: WSA_ERROR = 10106i32;
pub const WSASYSCALLFAILURE: WSA_ERROR = 10107i32;
pub const WSASERVICE_NOT_FOUND: WSA_ERROR = 10108i32;
pub const WSATYPE_NOT_FOUND: WSA_ERROR = 10109i32;
pub const WSA_E_NO_MORE: WSA_ERROR = 10110i32;
pub const WSA_E_CANCELLED: WSA_ERROR = 10111i32;
pub const WSAEREFUSED: WSA_ERROR = 10112i32;
pub const WSAHOST_NOT_FOUND: WSA_ERROR = 11001i32;
pub const WSATRY_AGAIN: WSA_ERROR = 11002i32;
pub const WSANO_RECOVERY: WSA_ERROR = 11003i32;
pub const WSANO_DATA: WSA_ERROR = 11004i32;
pub const WSA_QOS_RECEIVERS: WSA_ERROR = 11005i32;
pub const WSA_QOS_SENDERS: WSA_ERROR = 11006i32;
pub const WSA_QOS_NO_SENDERS: WSA_ERROR = 11007i32;
pub const WSA_QOS_NO_RECEIVERS: WSA_ERROR = 11008i32;
pub const WSA_QOS_REQUEST_CONFIRMED: WSA_ERROR = 11009i32;
pub const WSA_QOS_ADMISSION_FAILURE: WSA_ERROR = 11010i32;
pub const WSA_QOS_POLICY_FAILURE: WSA_ERROR = 11011i32;
pub const WSA_QOS_BAD_STYLE: WSA_ERROR = 11012i32;
pub const WSA_QOS_BAD_OBJECT: WSA_ERROR = 11013i32;
pub const WSA_QOS_TRAFFIC_CTRL_ERROR: WSA_ERROR = 11014i32;
pub const WSA_QOS_GENERIC_ERROR: WSA_ERROR = 11015i32;
pub const WSA_QOS_ESERVICETYPE: WSA_ERROR = 11016i32;
pub const WSA_QOS_EFLOWSPEC: WSA_ERROR = 11017i32;
pub const WSA_QOS_EPROVSPECBUF: WSA_ERROR = 11018i32;
pub const WSA_QOS_EFILTERSTYLE: WSA_ERROR = 11019i32;
pub const WSA_QOS_EFILTERTYPE: WSA_ERROR = 11020i32;
pub const WSA_QOS_EFILTERCOUNT: WSA_ERROR = 11021i32;
pub const WSA_QOS_EOBJLENGTH: WSA_ERROR = 11022i32;
pub const WSA_QOS_EFLOWCOUNT: WSA_ERROR = 11023i32;
pub const WSA_QOS_EUNKOWNPSOBJ: WSA_ERROR = 11024i32;
pub const WSA_QOS_EPOLICYOBJ: WSA_ERROR = 11025i32;
pub const WSA_QOS_EFLOWDESC: WSA_ERROR = 11026i32;
pub const WSA_QOS_EPSFLOWSPEC: WSA_ERROR = 11027i32;
pub const WSA_QOS_EPSFILTERSPEC: WSA_ERROR = 11028i32;
pub const WSA_QOS_ESDMODEOBJ: WSA_ERROR = 11029i32;
pub const WSA_QOS_ESHAPERATEOBJ: WSA_ERROR = 11030i32;
pub const WSA_QOS_RESERVED_PETYPE: WSA_ERROR = 11031i32;
pub const WSA_SECURE_HOST_NOT_FOUND: WSA_ERROR = 11032i32;
pub const WSA_IPSEC_NAME_POLICY_ERROR: WSA_ERROR = 11033i32;
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: u32 = 64u32;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: u32 = 4u32;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: u32 = 2u32;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: u32 = 16u32;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: u32 = 8u32;
pub const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 128u32;
pub const WSA_FLAG_OVERLAPPED: u32 = 1u32;
pub const WSA_FLAG_REGISTERED_IO: u32 = 256u32;
pub const WSA_INFINITE: u32 = 4294967295u32;
pub const WSA_MAXIMUM_WAIT_EVENTS: u32 = 64u32;
pub const WSA_WAIT_EVENT_0: u32 = 0u32;
pub const WSA_WAIT_FAILED: u32 = 4294967295u32;
pub const WSA_WAIT_IO_COMPLETION: u32 = 192u32;
#[repr(C)]
pub struct WSC_PROVIDER_AUDIT_INFO {
    pub RecordSize: u32,
    pub Reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSC_PROVIDER_AUDIT_INFO {}
impl ::core::clone::Clone for WSC_PROVIDER_AUDIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WSC_PROVIDER_INFO_TYPE = i32;
pub const ProviderInfoLspCategories: WSC_PROVIDER_INFO_TYPE = 0i32;
pub const ProviderInfoAudit: WSC_PROVIDER_INFO_TYPE = 1i32;
pub const WSK_SO_BASE: u32 = 16384u32;
pub const WSPDESCRIPTION_LEN: u32 = 255u32;
#[repr(C)]
pub struct WSPData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [u16; 256],
}
impl ::core::marker::Copy for WSPData {}
impl ::core::clone::Clone for WSPData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct WSPPROC_TABLE {
    pub lpWSPAccept: LPWSPACCEPT,
    pub lpWSPAddressToString: LPWSPADDRESSTOSTRING,
    pub lpWSPAsyncSelect: LPWSPASYNCSELECT,
    pub lpWSPBind: LPWSPBIND,
    pub lpWSPCancelBlockingCall: LPWSPCANCELBLOCKINGCALL,
    pub lpWSPCleanup: LPWSPCLEANUP,
    pub lpWSPCloseSocket: LPWSPCLOSESOCKET,
    pub lpWSPConnect: LPWSPCONNECT,
    pub lpWSPDuplicateSocket: LPWSPDUPLICATESOCKET,
    pub lpWSPEnumNetworkEvents: LPWSPENUMNETWORKEVENTS,
    pub lpWSPEventSelect: LPWSPEVENTSELECT,
    pub lpWSPGetOverlappedResult: LPWSPGETOVERLAPPEDRESULT,
    pub lpWSPGetPeerName: LPWSPGETPEERNAME,
    pub lpWSPGetSockName: LPWSPGETSOCKNAME,
    pub lpWSPGetSockOpt: LPWSPGETSOCKOPT,
    pub lpWSPGetQOSByName: LPWSPGETQOSBYNAME,
    pub lpWSPIoctl: LPWSPIOCTL,
    pub lpWSPJoinLeaf: LPWSPJOINLEAF,
    pub lpWSPListen: LPWSPLISTEN,
    pub lpWSPRecv: LPWSPRECV,
    pub lpWSPRecvDisconnect: LPWSPRECVDISCONNECT,
    pub lpWSPRecvFrom: LPWSPRECVFROM,
    pub lpWSPSelect: LPWSPSELECT,
    pub lpWSPSend: LPWSPSEND,
    pub lpWSPSendDisconnect: LPWSPSENDDISCONNECT,
    pub lpWSPSendTo: LPWSPSENDTO,
    pub lpWSPSetSockOpt: LPWSPSETSOCKOPT,
    pub lpWSPShutdown: LPWSPSHUTDOWN,
    pub lpWSPSocket: LPWSPSOCKET,
    pub lpWSPStringToAddress: LPWSPSTRINGTOADDRESS,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for WSPPROC_TABLE {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for WSPPROC_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSPUPCALLTABLE {
    pub lpWPUCloseEvent: LPWPUCLOSEEVENT,
    pub lpWPUCloseSocketHandle: LPWPUCLOSESOCKETHANDLE,
    pub lpWPUCreateEvent: LPWPUCREATEEVENT,
    pub lpWPUCreateSocketHandle: LPWPUCREATESOCKETHANDLE,
    pub lpWPUFDIsSet: LPWPUFDISSET,
    pub lpWPUGetProviderPath: LPWPUGETPROVIDERPATH,
    pub lpWPUModifyIFSHandle: LPWPUMODIFYIFSHANDLE,
    pub lpWPUPostMessage: LPWPUPOSTMESSAGE,
    pub lpWPUQueryBlockingCallback: LPWPUQUERYBLOCKINGCALLBACK,
    pub lpWPUQuerySocketHandleContext: LPWPUQUERYSOCKETHANDLECONTEXT,
    pub lpWPUQueueApc: LPWPUQUEUEAPC,
    pub lpWPUResetEvent: LPWPURESETEVENT,
    pub lpWPUSetEvent: LPWPUSETEVENT,
    pub lpWPUOpenCurrentThread: LPWPUOPENCURRENTTHREAD,
    pub lpWPUCloseThread: LPWPUCLOSETHREAD,
}
impl ::core::marker::Copy for WSPUPCALLTABLE {}
impl ::core::clone::Clone for WSPUPCALLTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WSS_OPERATION_IN_PROGRESS: i32 = 259i32;
pub const XP1_CONNECTIONLESS: u32 = 1u32;
pub const XP1_CONNECT_DATA: u32 = 128u32;
pub const XP1_DISCONNECT_DATA: u32 = 256u32;
pub const XP1_EXPEDITED_DATA: u32 = 64u32;
pub const XP1_GRACEFUL_CLOSE: u32 = 32u32;
pub const XP1_GUARANTEED_DELIVERY: u32 = 2u32;
pub const XP1_GUARANTEED_ORDER: u32 = 4u32;
pub const XP1_IFS_HANDLES: u32 = 131072u32;
pub const XP1_INTERRUPT: u32 = 16384u32;
pub const XP1_MESSAGE_ORIENTED: u32 = 8u32;
pub const XP1_MULTIPOINT_CONTROL_PLANE: u32 = 2048u32;
pub const XP1_MULTIPOINT_DATA_PLANE: u32 = 4096u32;
pub const XP1_PARTIAL_MESSAGE: u32 = 262144u32;
pub const XP1_PSEUDO_STREAM: u32 = 16u32;
pub const XP1_QOS_SUPPORTED: u32 = 8192u32;
pub const XP1_SAN_SUPPORT_SDP: u32 = 524288u32;
pub const XP1_SUPPORT_BROADCAST: u32 = 512u32;
pub const XP1_SUPPORT_MULTIPOINT: u32 = 1024u32;
pub const XP1_UNI_RECV: u32 = 65536u32;
pub const XP1_UNI_SEND: u32 = 32768u32;
pub const XP_BANDWIDTH_ALLOCATION: u32 = 2048u32;
pub const XP_CONNECTIONLESS: u32 = 1u32;
pub const XP_CONNECT_DATA: u32 = 128u32;
pub const XP_DISCONNECT_DATA: u32 = 256u32;
pub const XP_ENCRYPTS: u32 = 8192u32;
pub const XP_EXPEDITED_DATA: u32 = 64u32;
pub const XP_FRAGMENTATION: u32 = 4096u32;
pub const XP_GRACEFUL_CLOSE: u32 = 32u32;
pub const XP_GUARANTEED_DELIVERY: u32 = 2u32;
pub const XP_GUARANTEED_ORDER: u32 = 4u32;
pub const XP_MESSAGE_ORIENTED: u32 = 8u32;
pub const XP_PSEUDO_STREAM: u32 = 16u32;
pub const XP_SUPPORTS_BROADCAST: u32 = 512u32;
pub const XP_SUPPORTS_MULTICAST: u32 = 1024u32;
pub const _BIG_ENDIAN: u32 = 4321u32;
pub const _LITTLE_ENDIAN: u32 = 1234u32;
pub const _PDP_ENDIAN: u32 = 3412u32;
pub const _SS_MAXSIZE: u32 = 128u32;
#[repr(C)]
pub struct addrinfoW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut addrinfoW,
}
impl ::core::marker::Copy for addrinfoW {}
impl ::core::clone::Clone for addrinfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfo_dns_server {
    pub ai_servertype: u32,
    pub ai_flags: u64,
    pub ai_addrlen: u32,
    pub ai_addr: *mut SOCKADDR,
    pub Anonymous: addrinfo_dns_server_0,
}
impl ::core::marker::Copy for addrinfo_dns_server {}
impl ::core::clone::Clone for addrinfo_dns_server {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union addrinfo_dns_server_0 {
    pub ai_template: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for addrinfo_dns_server_0 {}
impl ::core::clone::Clone for addrinfo_dns_server_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoex2A {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoex2A,
    pub ai_version: i32,
    pub ai_fqdn: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for addrinfoex2A {}
impl ::core::clone::Clone for addrinfoex2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoex2W {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoex2W,
    pub ai_version: i32,
    pub ai_fqdn: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for addrinfoex2W {}
impl ::core::clone::Clone for addrinfoex2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoex3 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoex3,
    pub ai_version: i32,
    pub ai_fqdn: ::windows_core_sys::PWSTR,
    pub ai_interfaceindex: i32,
}
impl ::core::marker::Copy for addrinfoex3 {}
impl ::core::clone::Clone for addrinfoex3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoex4 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoex4,
    pub ai_version: i32,
    pub ai_fqdn: ::windows_core_sys::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for addrinfoex4 {}
impl ::core::clone::Clone for addrinfoex4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoex5 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoex5,
    pub ai_version: i32,
    pub ai_fqdn: ::windows_core_sys::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: ::win32_foundation_sys::HANDLE,
    pub ai_ttl: u32,
}
impl ::core::marker::Copy for addrinfoex5 {}
impl ::core::clone::Clone for addrinfoex5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoex6 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoex5,
    pub ai_version: i32,
    pub ai_fqdn: ::windows_core_sys::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: ::win32_foundation_sys::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: *mut addrinfo_dns_server,
    pub ai_responseflags: u64,
}
impl ::core::marker::Copy for addrinfoex6 {}
impl ::core::clone::Clone for addrinfoex6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoexA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoexA,
}
impl ::core::marker::Copy for addrinfoexA {}
impl ::core::clone::Clone for addrinfoexA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct addrinfoexW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: ::windows_core_sys::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut ::core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut ::windows_core_sys::GUID,
    pub ai_next: *mut addrinfoexW,
}
impl ::core::marker::Copy for addrinfoexW {}
impl ::core::clone::Clone for addrinfoexW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
impl ::core::marker::Copy for cmsghdr {}
impl ::core::clone::Clone for cmsghdr {
    fn clone(&self) -> Self {
        *self
    }
}
pub type eWINDOW_ADVANCE_METHOD = i32;
pub const E_WINDOW_ADVANCE_BY_TIME: eWINDOW_ADVANCE_METHOD = 1i32;
pub const E_WINDOW_USE_AS_DATA_CACHE: eWINDOW_ADVANCE_METHOD = 2i32;
#[repr(C)]
pub struct fd_set {
    pub fd_count: u32,
    pub fd_array: [SOCKET; 64],
}
impl ::core::marker::Copy for fd_set {}
impl ::core::clone::Clone for fd_set {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct hostent {
    pub h_name: ::windows_core_sys::PSTR,
    pub h_aliases: *mut *mut i8,
    pub h_addrtype: i16,
    pub h_length: i16,
    pub h_addr_list: *mut *mut i8,
}
impl ::core::marker::Copy for hostent {}
impl ::core::clone::Clone for hostent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct in6_pktinfo_ex {
    pub pkt_info: IN6_PKTINFO,
    pub scope_id: SCOPE_ID,
}
impl ::core::marker::Copy for in6_pktinfo_ex {}
impl ::core::clone::Clone for in6_pktinfo_ex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct linger {
    pub l_onoff: u16,
    pub l_linger: u16,
}
impl ::core::marker::Copy for linger {}
impl ::core::clone::Clone for linger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_neighbor_advert {
    pub nd_na_hdr: ICMP_MESSAGE,
    pub nd_na_target: IN6_ADDR,
}
impl ::core::marker::Copy for nd_neighbor_advert {}
impl ::core::clone::Clone for nd_neighbor_advert {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_neighbor_solicit {
    pub nd_ns_hdr: ICMP_MESSAGE,
    pub nd_ns_target: IN6_ADDR,
}
impl ::core::marker::Copy for nd_neighbor_solicit {}
impl ::core::clone::Clone for nd_neighbor_solicit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_dnssl {
    pub nd_opt_dnssl_type: u8,
    pub nd_opt_dnssl_len: u8,
    pub nd_opt_dnssl_reserved: u16,
    pub nd_opt_dnssl_lifetime: u32,
}
impl ::core::marker::Copy for nd_opt_dnssl {}
impl ::core::clone::Clone for nd_opt_dnssl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_hdr {
    pub nd_opt_type: u8,
    pub nd_opt_len: u8,
}
impl ::core::marker::Copy for nd_opt_hdr {}
impl ::core::clone::Clone for nd_opt_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_mtu {
    pub nd_opt_mtu_type: u8,
    pub nd_opt_mtu_len: u8,
    pub nd_opt_mtu_reserved: u16,
    pub nd_opt_mtu_mtu: u32,
}
impl ::core::marker::Copy for nd_opt_mtu {}
impl ::core::clone::Clone for nd_opt_mtu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_prefix_info {
    pub nd_opt_pi_type: u8,
    pub nd_opt_pi_len: u8,
    pub nd_opt_pi_prefix_len: u8,
    pub Anonymous1: nd_opt_prefix_info_0,
    pub nd_opt_pi_valid_time: u32,
    pub nd_opt_pi_preferred_time: u32,
    pub Anonymous2: nd_opt_prefix_info_1,
    pub nd_opt_pi_prefix: IN6_ADDR,
}
impl ::core::marker::Copy for nd_opt_prefix_info {}
impl ::core::clone::Clone for nd_opt_prefix_info {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union nd_opt_prefix_info_0 {
    pub nd_opt_pi_flags_reserved: u8,
    pub Flags: nd_opt_prefix_info_0_0,
}
impl ::core::marker::Copy for nd_opt_prefix_info_0 {}
impl ::core::clone::Clone for nd_opt_prefix_info_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_prefix_info_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for nd_opt_prefix_info_0_0 {}
impl ::core::clone::Clone for nd_opt_prefix_info_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union nd_opt_prefix_info_1 {
    pub nd_opt_pi_reserved2: u32,
    pub Anonymous: nd_opt_prefix_info_1_0,
}
impl ::core::marker::Copy for nd_opt_prefix_info_1 {}
impl ::core::clone::Clone for nd_opt_prefix_info_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_prefix_info_1_0 {
    pub nd_opt_pi_reserved3: [u8; 3],
    pub nd_opt_pi_site_prefix_len: u8,
}
impl ::core::marker::Copy for nd_opt_prefix_info_1_0 {}
impl ::core::clone::Clone for nd_opt_prefix_info_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_rd_hdr {
    pub nd_opt_rh_type: u8,
    pub nd_opt_rh_len: u8,
    pub nd_opt_rh_reserved1: u16,
    pub nd_opt_rh_reserved2: u32,
}
impl ::core::marker::Copy for nd_opt_rd_hdr {}
impl ::core::clone::Clone for nd_opt_rd_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_rdnss {
    pub nd_opt_rdnss_type: u8,
    pub nd_opt_rdnss_len: u8,
    pub nd_opt_rdnss_reserved: u16,
    pub nd_opt_rdnss_lifetime: u32,
}
impl ::core::marker::Copy for nd_opt_rdnss {}
impl ::core::clone::Clone for nd_opt_rdnss {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_route_info {
    pub nd_opt_ri_type: u8,
    pub nd_opt_ri_len: u8,
    pub nd_opt_ri_prefix_len: u8,
    pub Anonymous: nd_opt_route_info_0,
    pub nd_opt_ri_route_lifetime: u32,
    pub nd_opt_ri_prefix: IN6_ADDR,
}
impl ::core::marker::Copy for nd_opt_route_info {}
impl ::core::clone::Clone for nd_opt_route_info {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union nd_opt_route_info_0 {
    pub nd_opt_ri_flags_reserved: u8,
    pub Flags: nd_opt_route_info_0_0,
}
impl ::core::marker::Copy for nd_opt_route_info_0 {}
impl ::core::clone::Clone for nd_opt_route_info_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_opt_route_info_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for nd_opt_route_info_0_0 {}
impl ::core::clone::Clone for nd_opt_route_info_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_redirect {
    pub nd_rd_hdr: ICMP_MESSAGE,
    pub nd_rd_target: IN6_ADDR,
    pub nd_rd_dst: IN6_ADDR,
}
impl ::core::marker::Copy for nd_redirect {}
impl ::core::clone::Clone for nd_redirect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_router_advert {
    pub nd_ra_hdr: ICMP_MESSAGE,
    pub nd_ra_reachable: u32,
    pub nd_ra_retransmit: u32,
}
impl ::core::marker::Copy for nd_router_advert {}
impl ::core::clone::Clone for nd_router_advert {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct nd_router_solicit {
    pub nd_rs_hdr: ICMP_MESSAGE,
}
impl ::core::marker::Copy for nd_router_solicit {}
impl ::core::clone::Clone for nd_router_solicit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct netent {
    pub n_name: ::windows_core_sys::PSTR,
    pub n_aliases: *mut *mut i8,
    pub n_addrtype: i16,
    pub n_net: u32,
}
impl ::core::marker::Copy for netent {}
impl ::core::clone::Clone for netent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct protoent {
    pub p_name: ::windows_core_sys::PSTR,
    pub p_aliases: *mut *mut i8,
    pub p_proto: i16,
}
impl ::core::marker::Copy for protoent {}
impl ::core::clone::Clone for protoent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct servent {
    pub s_name: ::windows_core_sys::PSTR,
    pub s_aliases: *mut *mut i8,
    pub s_proto: ::windows_core_sys::PSTR,
    pub s_port: i16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for servent {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for servent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct servent {
    pub s_name: ::windows_core_sys::PSTR,
    pub s_aliases: *mut *mut i8,
    pub s_port: i16,
    pub s_proto: ::windows_core_sys::PSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for servent {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for servent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_atm {
    pub satm_family: u16,
    pub satm_number: ATM_ADDRESS,
    pub satm_blli: ATM_BLLI,
    pub satm_bhli: ATM_BHLI,
}
impl ::core::marker::Copy for sockaddr_atm {}
impl ::core::clone::Clone for sockaddr_atm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union sockaddr_gen {
    pub Address: SOCKADDR,
    pub AddressIn: SOCKADDR_IN,
    pub AddressIn6: sockaddr_in6_old,
}
impl ::core::marker::Copy for sockaddr_gen {}
impl ::core::clone::Clone for sockaddr_gen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_in6_old {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
}
impl ::core::marker::Copy for sockaddr_in6_old {}
impl ::core::clone::Clone for sockaddr_in6_old {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_ipx {
    pub sa_family: i16,
    pub sa_netnum: [::win32_foundation_sys::CHAR; 4],
    pub sa_nodenum: [::win32_foundation_sys::CHAR; 6],
    pub sa_socket: u16,
}
impl ::core::marker::Copy for sockaddr_ipx {}
impl ::core::clone::Clone for sockaddr_ipx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_nb {
    pub snb_family: i16,
    pub snb_type: u16,
    pub snb_name: [::win32_foundation_sys::CHAR; 16],
}
impl ::core::marker::Copy for sockaddr_nb {}
impl ::core::clone::Clone for sockaddr_nb {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_tp {
    pub tp_family: u16,
    pub tp_addr_type: u16,
    pub tp_taddr_len: u16,
    pub tp_tsel_len: u16,
    pub tp_addr: [u8; 64],
}
impl ::core::marker::Copy for sockaddr_tp {}
impl ::core::clone::Clone for sockaddr_tp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [::win32_foundation_sys::CHAR; 108],
}
impl ::core::marker::Copy for sockaddr_un {}
impl ::core::clone::Clone for sockaddr_un {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockaddr_vns {
    pub sin_family: u16,
    pub net_address: [u8; 4],
    pub subnet_addr: [u8; 2],
    pub port: [u8; 2],
    pub hops: u8,
    pub filler: [u8; 5],
}
impl ::core::marker::Copy for sockaddr_vns {}
impl ::core::clone::Clone for sockaddr_vns {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct sockproto {
    pub sp_family: u16,
    pub sp_protocol: u16,
}
impl ::core::marker::Copy for sockproto {}
impl ::core::clone::Clone for sockproto {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct tcp_hdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub _bitfield: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}
impl ::core::marker::Copy for tcp_hdr {}
impl ::core::clone::Clone for tcp_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_keepalive {
    pub onoff: u32,
    pub keepalivetime: u32,
    pub keepaliveinterval: u32,
}
impl ::core::marker::Copy for tcp_keepalive {}
impl ::core::clone::Clone for tcp_keepalive {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_opt_fastopen {
    pub Kind: u8,
    pub Length: u8,
    pub Cookie: [u8; 1],
}
impl ::core::marker::Copy for tcp_opt_fastopen {}
impl ::core::clone::Clone for tcp_opt_fastopen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct tcp_opt_mss {
    pub Kind: u8,
    pub Length: u8,
    pub Mss: u16,
}
impl ::core::marker::Copy for tcp_opt_mss {}
impl ::core::clone::Clone for tcp_opt_mss {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_opt_sack {
    pub Kind: u8,
    pub Length: u8,
    pub Block: [tcp_opt_sack_0; 1],
}
impl ::core::marker::Copy for tcp_opt_sack {}
impl ::core::clone::Clone for tcp_opt_sack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct tcp_opt_sack_0 {
    pub Left: u32,
    pub Right: u32,
}
impl ::core::marker::Copy for tcp_opt_sack_0 {}
impl ::core::clone::Clone for tcp_opt_sack_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_opt_sack_permitted {
    pub Kind: u8,
    pub Length: u8,
}
impl ::core::marker::Copy for tcp_opt_sack_permitted {}
impl ::core::clone::Clone for tcp_opt_sack_permitted {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct tcp_opt_ts {
    pub Kind: u8,
    pub Length: u8,
    pub Val: u32,
    pub EcR: u32,
}
impl ::core::marker::Copy for tcp_opt_ts {}
impl ::core::clone::Clone for tcp_opt_ts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_opt_unknown {
    pub Kind: u8,
    pub Length: u8,
}
impl ::core::marker::Copy for tcp_opt_unknown {}
impl ::core::clone::Clone for tcp_opt_unknown {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_opt_ws {
    pub Kind: u8,
    pub Length: u8,
    pub ShiftCnt: u8,
}
impl ::core::marker::Copy for tcp_opt_ws {}
impl ::core::clone::Clone for tcp_opt_ws {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
impl ::core::marker::Copy for timeval {}
impl ::core::clone::Clone for timeval {
    fn clone(&self) -> Self {
        *self
    }
}
