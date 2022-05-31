#[link(name = "windows")]
extern "system" {
    pub fn ActivateActCtx(hactctx: ::win32_foundation_sys::HANDLE, lpcookie: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn AddRefActCtx(hactctx: ::win32_foundation_sys::HANDLE);
    pub fn ApplyDeltaA(applyflags: i64, lpsourcename: ::windows_core_sys::PCSTR, lpdeltaname: ::windows_core_sys::PCSTR, lptargetname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: *const ::win32_foundation_sys::FILETIME, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyDeltaW(applyflags: i64, lpsourcename: ::windows_core_sys::PCWSTR, lpdeltaname: ::windows_core_sys::PCWSTR, lptargetname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileA(patchfilename: ::windows_core_sys::PCSTR, oldfilename: ::windows_core_sys::PCSTR, newfilename: ::windows_core_sys::PCSTR, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileByBuffers(patchfilemapped: *const u8, patchfilesize: u32, oldfilemapped: *const u8, oldfilesize: u32, newfilebuffer: *mut *mut u8, newfilebuffersize: u32, newfileactualsize: *mut u32, newfiletime: *mut ::win32_foundation_sys::FILETIME, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileByHandles(patchfilehandle: ::win32_foundation_sys::HANDLE, oldfilehandle: ::win32_foundation_sys::HANDLE, newfilehandle: ::win32_foundation_sys::HANDLE, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileByHandlesEx(patchfilehandle: ::win32_foundation_sys::HANDLE, oldfilehandle: ::win32_foundation_sys::HANDLE, newfilehandle: ::win32_foundation_sys::HANDLE, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileExA(patchfilename: ::windows_core_sys::PCSTR, oldfilename: ::windows_core_sys::PCSTR, newfilename: ::windows_core_sys::PCSTR, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileExW(patchfilename: ::windows_core_sys::PCWSTR, oldfilename: ::windows_core_sys::PCWSTR, newfilename: ::windows_core_sys::PCWSTR, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn ApplyPatchToFileW(patchfilename: ::windows_core_sys::PCWSTR, oldfilename: ::windows_core_sys::PCWSTR, newfilename: ::windows_core_sys::PCWSTR, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CreateActCtxA(pactctx: *const ACTCTXA) -> ::win32_foundation_sys::HANDLE;
    pub fn CreateActCtxW(pactctx: *const ACTCTXW) -> ::win32_foundation_sys::HANDLE;
    pub fn CreateDeltaA(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: ::windows_core_sys::PCSTR, lptargetname: ::windows_core_sys::PCSTR, lpsourceoptionsname: ::windows_core_sys::PCSTR, lptargetoptionsname: ::windows_core_sys::PCSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const ::win32_foundation_sys::FILETIME, hashalgid: u32, lpdeltaname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: *const ::win32_foundation_sys::FILETIME, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> ::win32_foundation_sys::BOOL;
    pub fn CreateDeltaW(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: ::windows_core_sys::PCWSTR, lptargetname: ::windows_core_sys::PCWSTR, lpsourceoptionsname: ::windows_core_sys::PCWSTR, lptargetoptionsname: ::windows_core_sys::PCWSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const ::win32_foundation_sys::FILETIME, hashalgid: u32, lpdeltaname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn CreatePatchFileA(oldfilename: ::windows_core_sys::PCSTR, newfilename: ::windows_core_sys::PCSTR, patchfilename: ::windows_core_sys::PCSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> ::win32_foundation_sys::BOOL;
    pub fn CreatePatchFileByHandles(oldfilehandle: ::win32_foundation_sys::HANDLE, newfilehandle: ::win32_foundation_sys::HANDLE, patchfilehandle: ::win32_foundation_sys::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> ::win32_foundation_sys::BOOL;
    pub fn CreatePatchFileByHandlesEx(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H, newfilehandle: ::win32_foundation_sys::HANDLE, patchfilehandle: ::win32_foundation_sys::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn CreatePatchFileExA(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A, newfilename: ::windows_core_sys::PCSTR, patchfilename: ::windows_core_sys::PCSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn CreatePatchFileExW(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W, newfilename: ::windows_core_sys::PCWSTR, patchfilename: ::windows_core_sys::PCWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn CreatePatchFileW(oldfilename: ::windows_core_sys::PCWSTR, newfilename: ::windows_core_sys::PCWSTR, patchfilename: ::windows_core_sys::PCWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> ::win32_foundation_sys::BOOL;
    pub fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> ::win32_foundation_sys::BOOL;
    pub fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> ::win32_foundation_sys::BOOL;
    pub fn ExtractPatchHeaderToFileA(patchfilename: ::windows_core_sys::PCSTR, patchheaderfilename: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn ExtractPatchHeaderToFileByHandles(patchfilehandle: ::win32_foundation_sys::HANDLE, patchheaderfilehandle: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn ExtractPatchHeaderToFileW(patchfilename: ::windows_core_sys::PCWSTR, patchheaderfilename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: *const ::windows_core_sys::GUID, ulsectionid: u32, lpguidtofind: *const ::windows_core_sys::GUID, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn FindActCtxSectionStringA(dwflags: u32, lpextensionguid: *const ::windows_core_sys::GUID, ulsectionid: u32, lpstringtofind: ::windows_core_sys::PCSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn FindActCtxSectionStringW(dwflags: u32, lpextensionguid: *const ::windows_core_sys::GUID, ulsectionid: u32, lpstringtofind: ::windows_core_sys::PCWSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> ::win32_foundation_sys::BOOL;
    pub fn GetCurrentActCtx(lphactctx: *mut ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn GetDeltaInfoA(lpdeltaname: ::windows_core_sys::PCSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn GetDeltaInfoW(lpdeltaname: ::windows_core_sys::PCWSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn GetDeltaSignatureA(filetypeset: i64, hashalgid: u32, lpsourcename: ::windows_core_sys::PCSTR, lphash: *mut DELTA_HASH) -> ::win32_foundation_sys::BOOL;
    pub fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> ::win32_foundation_sys::BOOL;
    pub fn GetDeltaSignatureW(filetypeset: i64, hashalgid: u32, lpsourcename: ::windows_core_sys::PCWSTR, lphash: *mut DELTA_HASH) -> ::win32_foundation_sys::BOOL;
    pub fn GetFilePatchSignatureA(filename: ::windows_core_sys::PCSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows_core_sys::PSTR) -> ::win32_foundation_sys::BOOL;
    pub fn GetFilePatchSignatureByBuffer(filebufferwritable: *mut u8, filesize: u32, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows_core_sys::PSTR) -> ::win32_foundation_sys::BOOL;
    pub fn GetFilePatchSignatureByHandle(filehandle: ::win32_foundation_sys::HANDLE, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows_core_sys::PSTR) -> ::win32_foundation_sys::BOOL;
    pub fn GetFilePatchSignatureW(filename: ::windows_core_sys::PCWSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn MsiAdvertiseProductA(szpackagepath: ::windows_core_sys::PCSTR, szscriptfilepath: ::windows_core_sys::PCSTR, sztransforms: ::windows_core_sys::PCSTR, lgidlanguage: u16) -> u32;
    pub fn MsiAdvertiseProductExA(szpackagepath: ::windows_core_sys::PCSTR, szscriptfilepath: ::windows_core_sys::PCSTR, sztransforms: ::windows_core_sys::PCSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    pub fn MsiAdvertiseProductExW(szpackagepath: ::windows_core_sys::PCWSTR, szscriptfilepath: ::windows_core_sys::PCWSTR, sztransforms: ::windows_core_sys::PCWSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    pub fn MsiAdvertiseProductW(szpackagepath: ::windows_core_sys::PCWSTR, szscriptfilepath: ::windows_core_sys::PCWSTR, sztransforms: ::windows_core_sys::PCWSTR, lgidlanguage: u16) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn MsiAdvertiseScriptA(szscriptfile: ::windows_core_sys::PCSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: ::win32_foundation_sys::BOOL) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn MsiAdvertiseScriptW(szscriptfile: ::windows_core_sys::PCWSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: ::win32_foundation_sys::BOOL) -> u32;
    pub fn MsiApplyMultiplePatchesA(szpatchpackages: ::windows_core_sys::PCSTR, szproductcode: ::windows_core_sys::PCSTR, szpropertieslist: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiApplyMultiplePatchesW(szpatchpackages: ::windows_core_sys::PCWSTR, szproductcode: ::windows_core_sys::PCWSTR, szpropertieslist: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiApplyPatchA(szpatchpackage: ::windows_core_sys::PCSTR, szinstallpackage: ::windows_core_sys::PCSTR, einstalltype: INSTALLTYPE, szcommandline: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiApplyPatchW(szpatchpackage: ::windows_core_sys::PCWSTR, szinstallpackage: ::windows_core_sys::PCWSTR, einstalltype: INSTALLTYPE, szcommandline: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiBeginTransactionA(szname: ::windows_core_sys::PCSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut ::win32_foundation_sys::HANDLE) -> u32;
    pub fn MsiBeginTransactionW(szname: ::windows_core_sys::PCWSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut ::win32_foundation_sys::HANDLE) -> u32;
    pub fn MsiCloseAllHandles() -> u32;
    pub fn MsiCloseHandle(hany: MSIHANDLE) -> u32;
    pub fn MsiCollectUserInfoA(szproduct: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiCollectUserInfoW(szproduct: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiConfigureFeatureA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiConfigureFeatureW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiConfigureProductA(szproduct: ::windows_core_sys::PCSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiConfigureProductExA(szproduct: ::windows_core_sys::PCSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiConfigureProductExW(szproduct: ::windows_core_sys::PCWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiConfigureProductW(szproduct: ::windows_core_sys::PCWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiCreateRecord(cparams: u32) -> MSIHANDLE;
    pub fn MsiCreateTransformSummaryInfoA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows_core_sys::PCSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    pub fn MsiCreateTransformSummaryInfoW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows_core_sys::PCWSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    pub fn MsiDatabaseApplyTransformA(hdatabase: MSIHANDLE, sztransformfile: ::windows_core_sys::PCSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    pub fn MsiDatabaseApplyTransformW(hdatabase: MSIHANDLE, sztransformfile: ::windows_core_sys::PCWSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    pub fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32;
    pub fn MsiDatabaseExportA(hdatabase: MSIHANDLE, sztablename: ::windows_core_sys::PCSTR, szfolderpath: ::windows_core_sys::PCSTR, szfilename: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiDatabaseExportW(hdatabase: MSIHANDLE, sztablename: ::windows_core_sys::PCWSTR, szfolderpath: ::windows_core_sys::PCWSTR, szfilename: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiDatabaseGenerateTransformA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows_core_sys::PCSTR, ireserved1: i32, ireserved2: i32) -> u32;
    pub fn MsiDatabaseGenerateTransformW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows_core_sys::PCWSTR, ireserved1: i32, ireserved2: i32) -> u32;
    pub fn MsiDatabaseGetPrimaryKeysA(hdatabase: MSIHANDLE, sztablename: ::windows_core_sys::PCSTR, phrecord: *mut MSIHANDLE) -> u32;
    pub fn MsiDatabaseGetPrimaryKeysW(hdatabase: MSIHANDLE, sztablename: ::windows_core_sys::PCWSTR, phrecord: *mut MSIHANDLE) -> u32;
    pub fn MsiDatabaseImportA(hdatabase: MSIHANDLE, szfolderpath: ::windows_core_sys::PCSTR, szfilename: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiDatabaseImportW(hdatabase: MSIHANDLE, szfolderpath: ::windows_core_sys::PCWSTR, szfilename: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiDatabaseIsTablePersistentA(hdatabase: MSIHANDLE, sztablename: ::windows_core_sys::PCSTR) -> MSICONDITION;
    pub fn MsiDatabaseIsTablePersistentW(hdatabase: MSIHANDLE, sztablename: ::windows_core_sys::PCWSTR) -> MSICONDITION;
    pub fn MsiDatabaseMergeA(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiDatabaseMergeW(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiDatabaseOpenViewA(hdatabase: MSIHANDLE, szquery: ::windows_core_sys::PCSTR, phview: *mut MSIHANDLE) -> u32;
    pub fn MsiDatabaseOpenViewW(hdatabase: MSIHANDLE, szquery: ::windows_core_sys::PCWSTR, phview: *mut MSIHANDLE) -> u32;
    pub fn MsiDetermineApplicablePatchesA(szproductpackagepath: ::windows_core_sys::PCSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    pub fn MsiDetermineApplicablePatchesW(szproductpackagepath: ::windows_core_sys::PCWSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    pub fn MsiDeterminePatchSequenceA(szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    pub fn MsiDeterminePatchSequenceW(szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    pub fn MsiDoActionA(hinstall: MSIHANDLE, szaction: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiDoActionW(hinstall: MSIHANDLE, szaction: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiEnableLogA(dwlogmode: INSTALLOGMODE, szlogfile: ::windows_core_sys::PCSTR, dwlogattributes: u32) -> u32;
    pub fn MsiEnableLogW(dwlogmode: INSTALLOGMODE, szlogfile: ::windows_core_sys::PCWSTR, dwlogattributes: u32) -> u32;
    pub fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32;
    pub fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32;
    pub fn MsiEnumClientsA(szcomponent: ::windows_core_sys::PCSTR, iproductindex: u32, lpproductbuf: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiEnumClientsExA(szcomponent: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::windows_core_sys::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows_core_sys::PSTR, pcchsid: *mut u32) -> u32;
    pub fn MsiEnumClientsExW(szcomponent: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::windows_core_sys::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows_core_sys::PWSTR, pcchsid: *mut u32) -> u32;
    pub fn MsiEnumClientsW(szcomponent: ::windows_core_sys::PCWSTR, iproductindex: u32, lpproductbuf: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiEnumComponentCostsA(hinstall: MSIHANDLE, szcomponent: ::windows_core_sys::PCSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows_core_sys::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    pub fn MsiEnumComponentCostsW(hinstall: MSIHANDLE, szcomponent: ::windows_core_sys::PCWSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows_core_sys::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    pub fn MsiEnumComponentQualifiersA(szcomponent: ::windows_core_sys::PCSTR, iindex: u32, lpqualifierbuf: ::windows_core_sys::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows_core_sys::PSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    pub fn MsiEnumComponentQualifiersW(szcomponent: ::windows_core_sys::PCWSTR, iindex: u32, lpqualifierbuf: ::windows_core_sys::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows_core_sys::PWSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    pub fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiEnumComponentsExA(szusersid: ::windows_core_sys::PCSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::windows_core_sys::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows_core_sys::PSTR, pcchsid: *mut u32) -> u32;
    pub fn MsiEnumComponentsExW(szusersid: ::windows_core_sys::PCWSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::windows_core_sys::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows_core_sys::PWSTR, pcchsid: *mut u32) -> u32;
    pub fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiEnumFeaturesA(szproduct: ::windows_core_sys::PCSTR, ifeatureindex: u32, lpfeaturebuf: ::windows_core_sys::PSTR, lpparentbuf: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiEnumFeaturesW(szproduct: ::windows_core_sys::PCWSTR, ifeatureindex: u32, lpfeaturebuf: ::windows_core_sys::PWSTR, lpparentbuf: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiEnumPatchesA(szproduct: ::windows_core_sys::PCSTR, ipatchindex: u32, lppatchbuf: ::windows_core_sys::PSTR, lptransformsbuf: ::windows_core_sys::PSTR, pcchtransformsbuf: *mut u32) -> u32;
    pub fn MsiEnumPatchesExA(szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::windows_core_sys::PSTR, sztargetproductcode: ::windows_core_sys::PSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: ::windows_core_sys::PSTR, pcchtargetusersid: *mut u32) -> u32;
    pub fn MsiEnumPatchesExW(szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::windows_core_sys::PWSTR, sztargetproductcode: ::windows_core_sys::PWSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: ::windows_core_sys::PWSTR, pcchtargetusersid: *mut u32) -> u32;
    pub fn MsiEnumPatchesW(szproduct: ::windows_core_sys::PCWSTR, ipatchindex: u32, lppatchbuf: ::windows_core_sys::PWSTR, lptransformsbuf: ::windows_core_sys::PWSTR, pcchtransformsbuf: *mut u32) -> u32;
    pub fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiEnumProductsExA(szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::windows_core_sys::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows_core_sys::PSTR, pcchsid: *mut u32) -> u32;
    pub fn MsiEnumProductsExW(szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::windows_core_sys::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows_core_sys::PWSTR, pcchsid: *mut u32) -> u32;
    pub fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiEnumRelatedProductsA(lpupgradecode: ::windows_core_sys::PCSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiEnumRelatedProductsW(lpupgradecode: ::windows_core_sys::PCWSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiEvaluateConditionA(hinstall: MSIHANDLE, szcondition: ::windows_core_sys::PCSTR) -> MSICONDITION;
    pub fn MsiEvaluateConditionW(hinstall: MSIHANDLE, szcondition: ::windows_core_sys::PCWSTR) -> MSICONDITION;
    pub fn MsiExtractPatchXMLDataA(szpatchpath: ::windows_core_sys::PCSTR, dwreserved: u32, szxmldata: ::windows_core_sys::PSTR, pcchxmldata: *mut u32) -> u32;
    pub fn MsiExtractPatchXMLDataW(szpatchpath: ::windows_core_sys::PCWSTR, dwreserved: u32, szxmldata: ::windows_core_sys::PWSTR, pcchxmldata: *mut u32) -> u32;
    pub fn MsiFormatRecordA(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: ::windows_core_sys::PSTR, pcchresultbuf: *mut u32) -> u32;
    pub fn MsiFormatRecordW(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: ::windows_core_sys::PWSTR, pcchresultbuf: *mut u32) -> u32;
    pub fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE;
    pub fn MsiGetComponentPathA(szproduct: ::windows_core_sys::PCSTR, szcomponent: ::windows_core_sys::PCSTR, lppathbuf: ::windows_core_sys::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    pub fn MsiGetComponentPathExA(szproductcode: ::windows_core_sys::PCSTR, szcomponentcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows_core_sys::PSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    pub fn MsiGetComponentPathExW(szproductcode: ::windows_core_sys::PCWSTR, szcomponentcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows_core_sys::PWSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    pub fn MsiGetComponentPathW(szproduct: ::windows_core_sys::PCWSTR, szcomponent: ::windows_core_sys::PCWSTR, lppathbuf: ::windows_core_sys::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    pub fn MsiGetComponentStateA(hinstall: MSIHANDLE, szcomponent: ::windows_core_sys::PCSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    pub fn MsiGetComponentStateW(hinstall: MSIHANDLE, szcomponent: ::windows_core_sys::PCWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    pub fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE;
    pub fn MsiGetFeatureCostA(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    pub fn MsiGetFeatureCostW(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCWSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    pub fn MsiGetFeatureInfoA(hproduct: MSIHANDLE, szfeature: ::windows_core_sys::PCSTR, lpattributes: *mut u32, lptitlebuf: ::windows_core_sys::PSTR, pcchtitlebuf: *mut u32, lphelpbuf: ::windows_core_sys::PSTR, pcchhelpbuf: *mut u32) -> u32;
    pub fn MsiGetFeatureInfoW(hproduct: MSIHANDLE, szfeature: ::windows_core_sys::PCWSTR, lpattributes: *mut u32, lptitlebuf: ::windows_core_sys::PWSTR, pcchtitlebuf: *mut u32, lphelpbuf: ::windows_core_sys::PWSTR, pcchhelpbuf: *mut u32) -> u32;
    pub fn MsiGetFeatureStateA(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    pub fn MsiGetFeatureStateW(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    pub fn MsiGetFeatureUsageA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    pub fn MsiGetFeatureUsageW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    pub fn MsiGetFeatureValidStatesA(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCSTR, lpinstallstates: *mut u32) -> u32;
    pub fn MsiGetFeatureValidStatesW(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCWSTR, lpinstallstates: *mut u32) -> u32;
    pub fn MsiGetFileHashA(szfilepath: ::windows_core_sys::PCSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    pub fn MsiGetFileHashW(szfilepath: ::windows_core_sys::PCWSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[cfg(feature = "win32-security-sys")]
    pub fn MsiGetFileSignatureInformationA(szsignedobjectpath: ::windows_core_sys::PCSTR, dwflags: u32, ppccertcontext: *mut *mut ::win32_security_sys::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-security-sys")]
    pub fn MsiGetFileSignatureInformationW(szsignedobjectpath: ::windows_core_sys::PCWSTR, dwflags: u32, ppccertcontext: *mut *mut ::win32_security_sys::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn MsiGetFileVersionA(szfilepath: ::windows_core_sys::PCSTR, lpversionbuf: ::windows_core_sys::PSTR, pcchversionbuf: *mut u32, lplangbuf: ::windows_core_sys::PSTR, pcchlangbuf: *mut u32) -> u32;
    pub fn MsiGetFileVersionW(szfilepath: ::windows_core_sys::PCWSTR, lpversionbuf: ::windows_core_sys::PWSTR, pcchversionbuf: *mut u32, lplangbuf: ::windows_core_sys::PWSTR, pcchlangbuf: *mut u32) -> u32;
    pub fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16;
    pub fn MsiGetLastErrorRecord() -> MSIHANDLE;
    pub fn MsiGetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE) -> ::win32_foundation_sys::BOOL;
    pub fn MsiGetPatchFileListA(szproductcode: ::windows_core_sys::PCSTR, szpatchpackages: ::windows_core_sys::PCSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    pub fn MsiGetPatchFileListW(szproductcode: ::windows_core_sys::PCWSTR, szpatchpackages: ::windows_core_sys::PCWSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    pub fn MsiGetPatchInfoA(szpatch: ::windows_core_sys::PCSTR, szattribute: ::windows_core_sys::PCSTR, lpvaluebuf: ::windows_core_sys::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetPatchInfoExA(szpatchcode: ::windows_core_sys::PCSTR, szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows_core_sys::PCSTR, lpvalue: ::windows_core_sys::PSTR, pcchvalue: *mut u32) -> u32;
    pub fn MsiGetPatchInfoExW(szpatchcode: ::windows_core_sys::PCWSTR, szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows_core_sys::PCWSTR, lpvalue: ::windows_core_sys::PWSTR, pcchvalue: *mut u32) -> u32;
    pub fn MsiGetPatchInfoW(szpatch: ::windows_core_sys::PCWSTR, szattribute: ::windows_core_sys::PCWSTR, lpvaluebuf: ::windows_core_sys::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetProductCodeA(szcomponent: ::windows_core_sys::PCSTR, lpbuf39: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiGetProductCodeW(szcomponent: ::windows_core_sys::PCWSTR, lpbuf39: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiGetProductInfoA(szproduct: ::windows_core_sys::PCSTR, szattribute: ::windows_core_sys::PCSTR, lpvaluebuf: ::windows_core_sys::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetProductInfoExA(szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows_core_sys::PCSTR, szvalue: ::windows_core_sys::PSTR, pcchvalue: *mut u32) -> u32;
    pub fn MsiGetProductInfoExW(szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows_core_sys::PCWSTR, szvalue: ::windows_core_sys::PWSTR, pcchvalue: *mut u32) -> u32;
    pub fn MsiGetProductInfoFromScriptA(szscriptfile: ::windows_core_sys::PCSTR, lpproductbuf39: ::windows_core_sys::PSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: ::windows_core_sys::PSTR, pcchnamebuf: *mut u32, lppackagebuf: ::windows_core_sys::PSTR, pcchpackagebuf: *mut u32) -> u32;
    pub fn MsiGetProductInfoFromScriptW(szscriptfile: ::windows_core_sys::PCWSTR, lpproductbuf39: ::windows_core_sys::PWSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: ::windows_core_sys::PWSTR, pcchnamebuf: *mut u32, lppackagebuf: ::windows_core_sys::PWSTR, pcchpackagebuf: *mut u32) -> u32;
    pub fn MsiGetProductInfoW(szproduct: ::windows_core_sys::PCWSTR, szattribute: ::windows_core_sys::PCWSTR, lpvaluebuf: ::windows_core_sys::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetProductPropertyA(hproduct: MSIHANDLE, szproperty: ::windows_core_sys::PCSTR, lpvaluebuf: ::windows_core_sys::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetProductPropertyW(hproduct: MSIHANDLE, szproperty: ::windows_core_sys::PCWSTR, lpvaluebuf: ::windows_core_sys::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetPropertyA(hinstall: MSIHANDLE, szname: ::windows_core_sys::PCSTR, szvaluebuf: ::windows_core_sys::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetPropertyW(hinstall: MSIHANDLE, szname: ::windows_core_sys::PCWSTR, szvaluebuf: ::windows_core_sys::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiGetShortcutTargetA(szshortcutpath: ::windows_core_sys::PCSTR, szproductcode: ::windows_core_sys::PSTR, szfeatureid: ::windows_core_sys::PSTR, szcomponentcode: ::windows_core_sys::PSTR) -> u32;
    pub fn MsiGetShortcutTargetW(szshortcutpath: ::windows_core_sys::PCWSTR, szproductcode: ::windows_core_sys::PWSTR, szfeatureid: ::windows_core_sys::PWSTR, szcomponentcode: ::windows_core_sys::PWSTR) -> u32;
    pub fn MsiGetSourcePathA(hinstall: MSIHANDLE, szfolder: ::windows_core_sys::PCSTR, szpathbuf: ::windows_core_sys::PSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiGetSourcePathW(hinstall: MSIHANDLE, szfolder: ::windows_core_sys::PCWSTR, szpathbuf: ::windows_core_sys::PWSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiGetSummaryInformationA(hdatabase: MSIHANDLE, szdatabasepath: ::windows_core_sys::PCSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    pub fn MsiGetSummaryInformationW(hdatabase: MSIHANDLE, szdatabasepath: ::windows_core_sys::PCWSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    pub fn MsiGetTargetPathA(hinstall: MSIHANDLE, szfolder: ::windows_core_sys::PCSTR, szpathbuf: ::windows_core_sys::PSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiGetTargetPathW(hinstall: MSIHANDLE, szfolder: ::windows_core_sys::PCWSTR, szpathbuf: ::windows_core_sys::PWSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiGetUserInfoA(szproduct: ::windows_core_sys::PCSTR, lpusernamebuf: ::windows_core_sys::PSTR, pcchusernamebuf: *mut u32, lporgnamebuf: ::windows_core_sys::PSTR, pcchorgnamebuf: *mut u32, lpserialbuf: ::windows_core_sys::PSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    pub fn MsiGetUserInfoW(szproduct: ::windows_core_sys::PCWSTR, lpusernamebuf: ::windows_core_sys::PWSTR, pcchusernamebuf: *mut u32, lporgnamebuf: ::windows_core_sys::PWSTR, pcchorgnamebuf: *mut u32, lpserialbuf: ::windows_core_sys::PWSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    pub fn MsiInstallMissingComponentA(szproduct: ::windows_core_sys::PCSTR, szcomponent: ::windows_core_sys::PCSTR, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiInstallMissingComponentW(szproduct: ::windows_core_sys::PCWSTR, szcomponent: ::windows_core_sys::PCWSTR, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiInstallMissingFileA(szproduct: ::windows_core_sys::PCSTR, szfile: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiInstallMissingFileW(szproduct: ::windows_core_sys::PCWSTR, szfile: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiInstallProductA(szpackagepath: ::windows_core_sys::PCSTR, szcommandline: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiInstallProductW(szpackagepath: ::windows_core_sys::PCWSTR, szcommandline: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiIsProductElevatedA(szproduct: ::windows_core_sys::PCSTR, pfelevated: *mut ::win32_foundation_sys::BOOL) -> u32;
    pub fn MsiIsProductElevatedW(szproduct: ::windows_core_sys::PCWSTR, pfelevated: *mut ::win32_foundation_sys::BOOL) -> u32;
    pub fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut ::win32_foundation_sys::HANDLE) -> u32;
    pub fn MsiLocateComponentA(szcomponent: ::windows_core_sys::PCSTR, lppathbuf: ::windows_core_sys::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    pub fn MsiLocateComponentW(szcomponent: ::windows_core_sys::PCWSTR, lppathbuf: ::windows_core_sys::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    pub fn MsiNotifySidChangeA(poldsid: ::windows_core_sys::PCSTR, pnewsid: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiNotifySidChangeW(poldsid: ::windows_core_sys::PCWSTR, pnewsid: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiOpenDatabaseA(szdatabasepath: ::windows_core_sys::PCSTR, szpersist: ::windows_core_sys::PCSTR, phdatabase: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenDatabaseW(szdatabasepath: ::windows_core_sys::PCWSTR, szpersist: ::windows_core_sys::PCWSTR, phdatabase: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenPackageA(szpackagepath: ::windows_core_sys::PCSTR, hproduct: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenPackageExA(szpackagepath: ::windows_core_sys::PCSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenPackageExW(szpackagepath: ::windows_core_sys::PCWSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenPackageW(szpackagepath: ::windows_core_sys::PCWSTR, hproduct: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenProductA(szproduct: ::windows_core_sys::PCSTR, hproduct: *mut MSIHANDLE) -> u32;
    pub fn MsiOpenProductW(szproduct: ::windows_core_sys::PCWSTR, hproduct: *mut MSIHANDLE) -> u32;
    pub fn MsiPreviewBillboardA(hpreview: MSIHANDLE, szcontrolname: ::windows_core_sys::PCSTR, szbillboard: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiPreviewBillboardW(hpreview: MSIHANDLE, szcontrolname: ::windows_core_sys::PCWSTR, szbillboard: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiPreviewDialogA(hpreview: MSIHANDLE, szdialogname: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiPreviewDialogW(hpreview: MSIHANDLE, szdialogname: ::windows_core_sys::PCWSTR) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn MsiProcessAdvertiseScriptA(szscriptfile: ::windows_core_sys::PCSTR, sziconfolder: ::windows_core_sys::PCSTR, hregdata: super::Registry::HKEY, fshortcuts: ::win32_foundation_sys::BOOL, fremoveitems: ::win32_foundation_sys::BOOL) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn MsiProcessAdvertiseScriptW(szscriptfile: ::windows_core_sys::PCWSTR, sziconfolder: ::windows_core_sys::PCWSTR, hregdata: super::Registry::HKEY, fshortcuts: ::win32_foundation_sys::BOOL, fremoveitems: ::win32_foundation_sys::BOOL) -> u32;
    pub fn MsiProcessMessage(hinstall: MSIHANDLE, emessagetype: INSTALLMESSAGE, hrecord: MSIHANDLE) -> i32;
    pub fn MsiProvideAssemblyA(szassemblyname: ::windows_core_sys::PCSTR, szappcontext: ::windows_core_sys::PCSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows_core_sys::PSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideAssemblyW(szassemblyname: ::windows_core_sys::PCWSTR, szappcontext: ::windows_core_sys::PCWSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows_core_sys::PWSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideComponentA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR, szcomponent: ::windows_core_sys::PCSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core_sys::PSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideComponentW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR, szcomponent: ::windows_core_sys::PCWSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core_sys::PWSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideQualifiedComponentA(szcategory: ::windows_core_sys::PCSTR, szqualifier: ::windows_core_sys::PCSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core_sys::PSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideQualifiedComponentExA(szcategory: ::windows_core_sys::PCSTR, szqualifier: ::windows_core_sys::PCSTR, dwinstallmode: INSTALLMODE, szproduct: ::windows_core_sys::PCSTR, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows_core_sys::PSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideQualifiedComponentExW(szcategory: ::windows_core_sys::PCWSTR, szqualifier: ::windows_core_sys::PCWSTR, dwinstallmode: INSTALLMODE, szproduct: ::windows_core_sys::PCWSTR, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows_core_sys::PWSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiProvideQualifiedComponentW(szcategory: ::windows_core_sys::PCWSTR, szqualifier: ::windows_core_sys::PCWSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core_sys::PWSTR, pcchpathbuf: *mut u32) -> u32;
    pub fn MsiQueryComponentStateA(szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: ::windows_core_sys::PCSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    pub fn MsiQueryComponentStateW(szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: ::windows_core_sys::PCWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    pub fn MsiQueryFeatureStateA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR) -> INSTALLSTATE;
    pub fn MsiQueryFeatureStateExA(szproductcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: ::windows_core_sys::PCSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    pub fn MsiQueryFeatureStateExW(szproductcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: ::windows_core_sys::PCWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    pub fn MsiQueryFeatureStateW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR) -> INSTALLSTATE;
    pub fn MsiQueryProductStateA(szproduct: ::windows_core_sys::PCSTR) -> INSTALLSTATE;
    pub fn MsiQueryProductStateW(szproduct: ::windows_core_sys::PCWSTR) -> INSTALLSTATE;
    pub fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32;
    pub fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32;
    pub fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32;
    pub fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32;
    pub fn MsiRecordGetStringA(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: ::windows_core_sys::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiRecordGetStringW(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: ::windows_core_sys::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> ::win32_foundation_sys::BOOL;
    pub fn MsiRecordReadStream(hrecord: MSIHANDLE, ifield: u32, szdatabuf: ::windows_core_sys::PSTR, pcbdatabuf: *mut u32) -> u32;
    pub fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32;
    pub fn MsiRecordSetStreamA(hrecord: MSIHANDLE, ifield: u32, szfilepath: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiRecordSetStreamW(hrecord: MSIHANDLE, ifield: u32, szfilepath: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiRecordSetStringA(hrecord: MSIHANDLE, ifield: u32, szvalue: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiRecordSetStringW(hrecord: MSIHANDLE, ifield: u32, szvalue: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiReinstallFeatureA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    pub fn MsiReinstallFeatureW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    pub fn MsiReinstallProductA(szproduct: ::windows_core_sys::PCSTR, szreinstallmode: REINSTALLMODE) -> u32;
    pub fn MsiReinstallProductW(szproduct: ::windows_core_sys::PCWSTR, szreinstallmode: REINSTALLMODE) -> u32;
    pub fn MsiRemovePatchesA(szpatchlist: ::windows_core_sys::PCSTR, szproductcode: ::windows_core_sys::PCSTR, euninstalltype: INSTALLTYPE, szpropertylist: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiRemovePatchesW(szpatchlist: ::windows_core_sys::PCWSTR, szproductcode: ::windows_core_sys::PCWSTR, euninstalltype: INSTALLTYPE, szpropertylist: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSequenceA(hinstall: MSIHANDLE, sztable: ::windows_core_sys::PCSTR, isequencemode: i32) -> u32;
    pub fn MsiSequenceW(hinstall: MSIHANDLE, sztable: ::windows_core_sys::PCWSTR, isequencemode: i32) -> u32;
    pub fn MsiSetComponentStateA(hinstall: MSIHANDLE, szcomponent: ::windows_core_sys::PCSTR, istate: INSTALLSTATE) -> u32;
    pub fn MsiSetComponentStateW(hinstall: MSIHANDLE, szcomponent: ::windows_core_sys::PCWSTR, istate: INSTALLSTATE) -> u32;
    pub fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERA;
    pub fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32;
    pub fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERW;
    pub fn MsiSetFeatureAttributesA(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCSTR, dwattributes: u32) -> u32;
    pub fn MsiSetFeatureAttributesW(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCWSTR, dwattributes: u32) -> u32;
    pub fn MsiSetFeatureStateA(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCSTR, istate: INSTALLSTATE) -> u32;
    pub fn MsiSetFeatureStateW(hinstall: MSIHANDLE, szfeature: ::windows_core_sys::PCWSTR, istate: INSTALLSTATE) -> u32;
    pub fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32;
    pub fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: *mut ::win32_foundation_sys::HWND) -> INSTALLUILEVEL;
    pub fn MsiSetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE, fstate: ::win32_foundation_sys::BOOL) -> u32;
    pub fn MsiSetPropertyA(hinstall: MSIHANDLE, szname: ::windows_core_sys::PCSTR, szvalue: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSetPropertyW(hinstall: MSIHANDLE, szname: ::windows_core_sys::PCWSTR, szvalue: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSetTargetPathA(hinstall: MSIHANDLE, szfolder: ::windows_core_sys::PCSTR, szfolderpath: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSetTargetPathW(hinstall: MSIHANDLE, szfolder: ::windows_core_sys::PCWSTR, szfolderpath: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: ::windows_core_sys::PCSTR, szdiskprompt: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: ::windows_core_sys::PCWSTR, szdiskprompt: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSourceListAddSourceA(szproduct: ::windows_core_sys::PCSTR, szusername: ::windows_core_sys::PCSTR, dwreserved: u32, szsource: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSourceListAddSourceExA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows_core_sys::PCSTR, dwindex: u32) -> u32;
    pub fn MsiSourceListAddSourceExW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows_core_sys::PCWSTR, dwindex: u32) -> u32;
    pub fn MsiSourceListAddSourceW(szproduct: ::windows_core_sys::PCWSTR, szusername: ::windows_core_sys::PCWSTR, dwreserved: u32, szsource: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSourceListClearAllA(szproduct: ::windows_core_sys::PCSTR, szusername: ::windows_core_sys::PCSTR, dwreserved: u32) -> u32;
    pub fn MsiSourceListClearAllExA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    pub fn MsiSourceListClearAllExW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    pub fn MsiSourceListClearAllW(szproduct: ::windows_core_sys::PCWSTR, szusername: ::windows_core_sys::PCWSTR, dwreserved: u32) -> u32;
    pub fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    pub fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    pub fn MsiSourceListClearSourceA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSourceListClearSourceW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: ::windows_core_sys::PSTR, pcchvolumelabel: *mut u32, szdiskprompt: ::windows_core_sys::PSTR, pcchdiskprompt: *mut u32) -> u32;
    pub fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: ::windows_core_sys::PWSTR, pcchvolumelabel: *mut u32, szdiskprompt: ::windows_core_sys::PWSTR, pcchdiskprompt: *mut u32) -> u32;
    pub fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows_core_sys::PSTR, pcchsource: *mut u32) -> u32;
    pub fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows_core_sys::PWSTR, pcchsource: *mut u32) -> u32;
    pub fn MsiSourceListForceResolutionA(szproduct: ::windows_core_sys::PCSTR, szusername: ::windows_core_sys::PCSTR, dwreserved: u32) -> u32;
    pub fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    pub fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    pub fn MsiSourceListForceResolutionW(szproduct: ::windows_core_sys::PCWSTR, szusername: ::windows_core_sys::PCWSTR, dwreserved: u32) -> u32;
    pub fn MsiSourceListGetInfoA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows_core_sys::PCSTR, szvalue: ::windows_core_sys::PSTR, pcchvalue: *mut u32) -> u32;
    pub fn MsiSourceListGetInfoW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows_core_sys::PCWSTR, szvalue: ::windows_core_sys::PWSTR, pcchvalue: *mut u32) -> u32;
    pub fn MsiSourceListSetInfoA(szproductcodeorpatchcode: ::windows_core_sys::PCSTR, szusersid: ::windows_core_sys::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows_core_sys::PCSTR, szvalue: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSourceListSetInfoW(szproductcodeorpatchcode: ::windows_core_sys::PCWSTR, szusersid: ::windows_core_sys::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows_core_sys::PCWSTR, szvalue: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiSummaryInfoGetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut ::win32_foundation_sys::FILETIME, szvaluebuf: ::windows_core_sys::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: MSIHANDLE, puipropertycount: *mut u32) -> u32;
    pub fn MsiSummaryInfoGetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut ::win32_foundation_sys::FILETIME, szvaluebuf: ::windows_core_sys::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32;
    pub fn MsiSummaryInfoSetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut ::win32_foundation_sys::FILETIME, szvalue: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiSummaryInfoSetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut ::win32_foundation_sys::FILETIME, szvalue: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiUseFeatureA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR) -> INSTALLSTATE;
    pub fn MsiUseFeatureExA(szproduct: ::windows_core_sys::PCSTR, szfeature: ::windows_core_sys::PCSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    pub fn MsiUseFeatureExW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    pub fn MsiUseFeatureW(szproduct: ::windows_core_sys::PCWSTR, szfeature: ::windows_core_sys::PCWSTR) -> INSTALLSTATE;
    pub fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32;
    pub fn MsiVerifyPackageA(szpackagepath: ::windows_core_sys::PCSTR) -> u32;
    pub fn MsiVerifyPackageW(szpackagepath: ::windows_core_sys::PCWSTR) -> u32;
    pub fn MsiViewClose(hview: MSIHANDLE) -> u32;
    pub fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32;
    pub fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32;
    pub fn MsiViewGetColumnInfo(hview: MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32;
    pub fn MsiViewGetErrorA(hview: MSIHANDLE, szcolumnnamebuffer: ::windows_core_sys::PSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    pub fn MsiViewGetErrorW(hview: MSIHANDLE, szcolumnnamebuffer: ::windows_core_sys::PWSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    pub fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32;
    pub fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, newfilecoffbase: u32, newfilecofftime: u32, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE) -> i32;
    pub fn QueryActCtxSettingsW(dwflags: u32, hactctx: ::win32_foundation_sys::HANDLE, settingsnamespace: ::windows_core_sys::PCWSTR, settingname: ::windows_core_sys::PCWSTR, pvbuffer: ::windows_core_sys::PWSTR, dwbuffer: usize, pdwwrittenorrequired: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn QueryActCtxW(dwflags: u32, hactctx: ::win32_foundation_sys::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn ReleaseActCtx(hactctx: ::win32_foundation_sys::HANDLE);
    pub fn SfcGetNextProtectedFile(rpchandle: ::win32_foundation_sys::HANDLE, protfiledata: *mut PROTECTED_FILE_DATA) -> ::win32_foundation_sys::BOOL;
    pub fn SfcIsFileProtected(rpchandle: ::win32_foundation_sys::HANDLE, protfilename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn SfcIsKeyProtected(keyhandle: super::Registry::HKEY, subkeyname: ::windows_core_sys::PCWSTR, keysam: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SfpVerifyFile(pszfilename: ::windows_core_sys::PCSTR, pszerror: ::windows_core_sys::PCSTR, dwerrsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TestApplyPatchToFileA(patchfilename: ::windows_core_sys::PCSTR, oldfilename: ::windows_core_sys::PCSTR, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TestApplyPatchToFileByBuffers(patchfilebuffer: *const u8, patchfilesize: u32, oldfilebuffer: *const u8, oldfilesize: u32, newfilesize: *mut u32, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TestApplyPatchToFileByHandles(patchfilehandle: ::win32_foundation_sys::HANDLE, oldfilehandle: ::win32_foundation_sys::HANDLE, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TestApplyPatchToFileW(patchfilename: ::windows_core_sys::PCWSTR, oldfilename: ::windows_core_sys::PCWSTR, applyoptionflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn ZombifyActCtx(hactctx: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
}
#[repr(C)]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows_core_sys::PCSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows_core_sys::PCSTR,
    pub lpResourceName: ::windows_core_sys::PCSTR,
    pub lpApplicationName: ::windows_core_sys::PCSTR,
    pub hModule: ::win32_foundation_sys::HINSTANCE,
}
impl ::core::marker::Copy for ACTCTXA {}
impl ::core::clone::Clone for ACTCTXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows_core_sys::PCWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows_core_sys::PCWSTR,
    pub lpResourceName: ::windows_core_sys::PCWSTR,
    pub lpApplicationName: ::windows_core_sys::PCWSTR,
    pub hModule: ::win32_foundation_sys::HINSTANCE,
}
impl ::core::marker::Copy for ACTCTXW {}
impl ::core::clone::Clone for ACTCTXW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ACTCTX_COMPATIBILITY_ELEMENT_TYPE = i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 0i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 1i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 2i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 3i32;
pub type ACTCTX_REQUESTED_RUN_LEVEL = i32;
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = 0i32;
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = 1i32;
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = 2i32;
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = 3i32;
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = 4i32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: ::win32_foundation_sys::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulEncodedAssemblyIdentityLength: u32,
    pub ulManifestPathType: u32,
    pub ulManifestPathLength: u32,
    pub liManifestLastWriteTime: i64,
    pub ulPolicyPathType: u32,
    pub ulPolicyPathLength: u32,
    pub liPolicyLastWriteTime: i64,
    pub ulMetadataSatelliteRosterIndex: u32,
    pub ulManifestVersionMajor: u32,
    pub ulManifestVersionMinor: u32,
    pub ulPolicyVersionMajor: u32,
    pub ulPolicyVersionMinor: u32,
    pub ulAssemblyDirectoryNameLength: u32,
    pub lpAssemblyEncodedAssemblyIdentity: ::windows_core_sys::PCWSTR,
    pub lpAssemblyManifestPath: ::windows_core_sys::PCWSTR,
    pub lpAssemblyPolicyPath: ::windows_core_sys::PCWSTR,
    pub lpAssemblyDirectoryName: ::windows_core_sys::PCWSTR,
    pub ulFileCount: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 1],
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub dwFlags: u32,
    pub ulFormatVersion: u32,
    pub ulAssemblyCount: u32,
    pub ulRootManifestPathType: u32,
    pub ulRootManifestPathChars: u32,
    pub ulRootConfigurationPathType: u32,
    pub ulRootConfigurationPathChars: u32,
    pub ulAppDirPathType: u32,
    pub ulAppDirPathChars: u32,
    pub lpRootManifestPath: ::windows_core_sys::PCWSTR,
    pub lpRootConfigurationPath: ::windows_core_sys::PCWSTR,
    pub lpAppDirPath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ADVERTISEFLAGS = i32;
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = 0i32;
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = 1i32;
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
pub type ASM_BIND_FLAGS = u32;
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = 1u32;
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = 2u32;
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = 4u32;
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = 8u32;
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = 16u32;
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = 32u32;
pub type ASM_CMP_FLAGS = i32;
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = 1i32;
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = 2i32;
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = 4i32;
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = 8i32;
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = 16i32;
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = 32i32;
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = 64i32;
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = 128i32;
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = 255i32;
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = 256i32;
pub type ASM_DISPLAY_FLAGS = i32;
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = 1i32;
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = 2i32;
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = 4i32;
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = 8i32;
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = 16i32;
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = 32i32;
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = 64i32;
pub type ASM_NAME = i32;
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = 0i32;
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = 1i32;
pub const ASM_NAME_HASH_VALUE: ASM_NAME = 2i32;
pub const ASM_NAME_NAME: ASM_NAME = 3i32;
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = 4i32;
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = 5i32;
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = 6i32;
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = 7i32;
pub const ASM_NAME_CULTURE: ASM_NAME = 8i32;
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = 9i32;
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = 10i32;
pub const ASM_NAME_HASH_ALGID: ASM_NAME = 11i32;
pub const ASM_NAME_ALIAS: ASM_NAME = 12i32;
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = 13i32;
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = 14i32;
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = 15i32;
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = 16i32;
pub const ASM_NAME_CUSTOM: ASM_NAME = 17i32;
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = 18i32;
pub const ASM_NAME_MVID: ASM_NAME = 19i32;
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = 20i32;
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[repr(C)]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: ::windows_core_sys::PCWSTR,
    pub lpFilePath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for ASSEMBLY_FILE_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: ::windows_core_sys::PWSTR,
    pub cchBuf: u32,
}
impl ::core::marker::Copy for ASSEMBLY_INFO {}
impl ::core::clone::Clone for ASSEMBLY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_EvalCom2: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1851660560, data2: 32851, data3: 18016, data4: [183, 149, 107, 97, 46, 41, 188, 88] };
pub const CLSID_MsmMerge2: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 4182345173, data2: 10745, data3: 18243, data4: [152, 5, 153, 188, 63, 53, 182, 120] };
#[repr(C)]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: ::windows_core_sys::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
impl ::core::marker::Copy for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::clone::Clone for COMPATIBILITY_CONTEXT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CREATE_ASM_NAME_OBJ_FLAGS = i32;
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = 1i32;
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = 2i32;
pub const DEFAULT_DISK_ID: u32 = 2u32;
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[repr(C)]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl ::core::marker::Copy for DELTA_HASH {}
impl ::core::clone::Clone for DELTA_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DELTA_HEADER_INFO {
    pub FileTypeSet: i64,
    pub FileType: i64,
    pub Flags: i64,
    pub TargetSize: usize,
    pub TargetFileTime: ::win32_foundation_sys::FILETIME,
    pub TargetHashAlgId: u32,
    pub TargetHash: DELTA_HASH,
}
impl ::core::marker::Copy for DELTA_HEADER_INFO {}
impl ::core::clone::Clone for DELTA_HEADER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for DELTA_INPUT {}
impl ::core::clone::Clone for DELTA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DELTA_INPUT_0 {
    pub lpcStart: *const ::core::ffi::c_void,
    pub lpStart: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DELTA_INPUT_0 {}
impl ::core::clone::Clone for DELTA_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[repr(C)]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut ::core::ffi::c_void,
    pub uSize: usize,
}
impl ::core::marker::Copy for DELTA_OUTPUT {}
impl ::core::clone::Clone for DELTA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[repr(C)]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: ::windows_core_sys::GUID,
    pub szIdentifier: ::windows_core_sys::PCWSTR,
    pub szNonCannonicalData: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for FUSION_INSTALL_REFERENCE {}
impl ::core::clone::Clone for FUSION_INSTALL_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2955910501, data2: 64375, data3: 20346, data4: [175, 165, 179, 145, 48, 159, 17, 201] };
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 784938083, data2: 45251, data3: 17889, data4: [131, 100, 50, 126, 150, 174, 168, 86] };
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2364391957, data2: 44107, data3: 18571, data4: [147, 192, 165, 10, 73, 203, 47, 184] };
pub const IACTIONNAME_ADMIN: &str = "ADMIN";
pub const IACTIONNAME_ADVERTISE: &str = "ADVERTISE";
pub const IACTIONNAME_COLLECTUSERINFO: &str = "CollectUserInfo";
pub const IACTIONNAME_FIRSTRUN: &str = "FirstRun";
pub const IACTIONNAME_INSTALL: &str = "INSTALL";
pub const IACTIONNAME_SEQUENCE: &str = "SEQUENCE";
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
pub type IASSEMBLYCACHE_UNINSTALL_DISPOSITION = u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 1u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 2u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 3u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 4u32;
pub type IAssemblyCache = *mut ::core::ffi::c_void;
pub type IAssemblyCacheItem = *mut ::core::ffi::c_void;
pub type IAssemblyName = *mut ::core::ffi::c_void;
pub type IEnumMsmDependency = *mut ::core::ffi::c_void;
pub type IEnumMsmError = *mut ::core::ffi::c_void;
pub type IEnumMsmString = *mut ::core::ffi::c_void;
pub type IMsmDependencies = *mut ::core::ffi::c_void;
pub type IMsmDependency = *mut ::core::ffi::c_void;
pub type IMsmError = *mut ::core::ffi::c_void;
pub type IMsmErrors = *mut ::core::ffi::c_void;
pub type IMsmGetFiles = *mut ::core::ffi::c_void;
pub type IMsmMerge = *mut ::core::ffi::c_void;
pub type IMsmStrings = *mut ::core::ffi::c_void;
pub const INFO_BASE: u32 = 3222229249u32;
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
pub const INFO_PCP_PATH: u32 = 3222229252u32;
pub const INFO_PROPERTY: u32 = 3222229255u32;
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
pub type INSTALLFEATUREATTRIBUTE = i32;
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = 1i32;
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = 2i32;
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = 4i32;
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = 8i32;
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = 16i32;
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = 32i32;
pub type INSTALLLEVEL = i32;
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = 0i32;
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = 1i32;
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = 65535i32;
pub type INSTALLLOGATTRIBUTES = i32;
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = 1i32;
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = 2i32;
pub type INSTALLMESSAGE = i32;
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = 0i32;
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = 16777216i32;
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = 33554432i32;
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = 50331648i32;
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = 67108864i32;
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = 83886080i32;
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = 100663296i32;
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = 117440512i32;
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = 134217728i32;
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = 150994944i32;
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = 167772160i32;
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = 184549376i32;
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = 201326592i32;
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = 218103808i32;
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = 234881024i32;
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = 251658240i32;
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = 419430400i32;
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = 436207616i32;
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = 452984832i32;
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
pub type INSTALLMODE = i32;
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = -4i32;
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = -3i32;
pub const INSTALLMODE_NODETECTION: INSTALLMODE = -2i32;
pub const INSTALLMODE_EXISTING: INSTALLMODE = -1i32;
pub const INSTALLMODE_DEFAULT: INSTALLMODE = 0i32;
pub type INSTALLOGMODE = i32;
pub const INSTALLLOGMODE_FATALEXIT: INSTALLOGMODE = 1i32;
pub const INSTALLLOGMODE_ERROR: INSTALLOGMODE = 2i32;
pub const INSTALLLOGMODE_WARNING: INSTALLOGMODE = 4i32;
pub const INSTALLLOGMODE_USER: INSTALLOGMODE = 8i32;
pub const INSTALLLOGMODE_INFO: INSTALLOGMODE = 16i32;
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLOGMODE = 64i32;
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLOGMODE = 128i32;
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLOGMODE = 256i32;
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLOGMODE = 512i32;
pub const INSTALLLOGMODE_COMMONDATA: INSTALLOGMODE = 2048i32;
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLOGMODE = 1024i32;
pub const INSTALLLOGMODE_VERBOSE: INSTALLOGMODE = 4096i32;
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLOGMODE = 8192i32;
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLOGMODE = 16384i32;
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLOGMODE = 32768i32;
pub const INSTALLLOGMODE_PROGRESS: INSTALLOGMODE = 1024i32;
pub const INSTALLLOGMODE_INITIALIZE: INSTALLOGMODE = 4096i32;
pub const INSTALLLOGMODE_TERMINATE: INSTALLOGMODE = 8192i32;
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLOGMODE = 16384i32;
pub const INSTALLLOGMODE_FILESINUSE: INSTALLOGMODE = 32i32;
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLOGMODE = 33554432i32;
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLOGMODE = 67108864i32;
pub const INSTALLLOGMODE_INSTALLEND: INSTALLOGMODE = 134217728i32;
pub const INSTALLPROPERTY_ASSIGNMENTTYPE: &str = "AssignmentType";
pub const INSTALLPROPERTY_AUTHORIZED_LUA_APP: &str = "AuthorizedLUAApp";
pub const INSTALLPROPERTY_DISKPROMPT: &str = "DiskPrompt";
pub const INSTALLPROPERTY_DISPLAYNAME: &str = "DisplayName";
pub const INSTALLPROPERTY_HELPLINK: &str = "HelpLink";
pub const INSTALLPROPERTY_HELPTELEPHONE: &str = "HelpTelephone";
pub const INSTALLPROPERTY_INSTALLDATE: &str = "InstallDate";
pub const INSTALLPROPERTY_INSTALLEDLANGUAGE: &str = "InstalledLanguage";
pub const INSTALLPROPERTY_INSTALLEDPRODUCTNAME: &str = "InstalledProductName";
pub const INSTALLPROPERTY_INSTALLLOCATION: &str = "InstallLocation";
pub const INSTALLPROPERTY_INSTALLSOURCE: &str = "InstallSource";
pub const INSTALLPROPERTY_INSTANCETYPE: &str = "InstanceType";
pub const INSTALLPROPERTY_LANGUAGE: &str = "Language";
pub const INSTALLPROPERTY_LASTUSEDSOURCE: &str = "LastUsedSource";
pub const INSTALLPROPERTY_LASTUSEDTYPE: &str = "LastUsedType";
pub const INSTALLPROPERTY_LOCALPACKAGE: &str = "LocalPackage";
pub const INSTALLPROPERTY_LUAENABLED: &str = "LUAEnabled";
pub const INSTALLPROPERTY_MEDIAPACKAGEPATH: &str = "MediaPackagePath";
pub const INSTALLPROPERTY_MOREINFOURL: &str = "MoreInfoURL";
pub const INSTALLPROPERTY_PACKAGECODE: &str = "PackageCode";
pub const INSTALLPROPERTY_PACKAGENAME: &str = "PackageName";
pub const INSTALLPROPERTY_PATCHSTATE: &str = "State";
pub const INSTALLPROPERTY_PATCHTYPE: &str = "PatchType";
pub const INSTALLPROPERTY_PRODUCTICON: &str = "ProductIcon";
pub const INSTALLPROPERTY_PRODUCTID: &str = "ProductID";
pub const INSTALLPROPERTY_PRODUCTNAME: &str = "ProductName";
pub const INSTALLPROPERTY_PRODUCTSTATE: &str = "State";
pub const INSTALLPROPERTY_PUBLISHER: &str = "Publisher";
pub const INSTALLPROPERTY_REGCOMPANY: &str = "RegCompany";
pub const INSTALLPROPERTY_REGOWNER: &str = "RegOwner";
pub const INSTALLPROPERTY_TRANSFORMS: &str = "Transforms";
pub const INSTALLPROPERTY_UNINSTALLABLE: &str = "Uninstallable";
pub const INSTALLPROPERTY_URLINFOABOUT: &str = "URLInfoAbout";
pub const INSTALLPROPERTY_URLUPDATEINFO: &str = "URLUpdateInfo";
pub const INSTALLPROPERTY_VERSION: &str = "Version";
pub const INSTALLPROPERTY_VERSIONMAJOR: &str = "VersionMajor";
pub const INSTALLPROPERTY_VERSIONMINOR: &str = "VersionMinor";
pub const INSTALLPROPERTY_VERSIONSTRING: &str = "VersionString";
pub type INSTALLSTATE = i32;
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = -7i32;
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = -6i32;
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = -5i32;
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = -4i32;
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = -3i32;
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = -2i32;
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = -1i32;
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = 0i32;
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = 1i32;
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = 1i32;
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = 2i32;
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = 3i32;
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = 4i32;
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = 5i32;
pub type INSTALLTYPE = i32;
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = 0i32;
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = 1i32;
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = 2i32;
pub type INSTALLUILEVEL = i32;
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = 0i32;
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = 1i32;
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = 2i32;
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = 3i32;
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = 4i32;
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = 5i32;
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = 128i32;
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = 64i32;
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = 32i32;
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = 256i32;
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = 512i32;
pub type INSTALLUI_HANDLERA = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows_core_sys::PCSTR) -> i32>;
pub type INSTALLUI_HANDLERW = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows_core_sys::PCWSTR) -> i32>;
pub type IPMApplicationInfo = *mut ::core::ffi::c_void;
pub type IPMApplicationInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMBackgroundServiceAgentInfo = *mut ::core::ffi::c_void;
pub type IPMBackgroundServiceAgentInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMBackgroundWorkerInfo = *mut ::core::ffi::c_void;
pub type IPMBackgroundWorkerInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMDeploymentManager = *mut ::core::ffi::c_void;
pub type IPMEnumerationManager = *mut ::core::ffi::c_void;
pub type IPMExtensionCachedFileUpdaterInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionContractInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileExtensionInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileOpenPickerInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileSavePickerInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMExtensionProtocolInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionShareTargetInfo = *mut ::core::ffi::c_void;
pub type IPMLiveTileJobInfo = *mut ::core::ffi::c_void;
pub type IPMLiveTileJobInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTaskInfo = *mut ::core::ffi::c_void;
pub type IPMTaskInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTileInfo = *mut ::core::ffi::c_void;
pub type IPMTileInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTilePropertyEnumerator = *mut ::core::ffi::c_void;
pub type IPMTilePropertyInfo = *mut ::core::ffi::c_void;
pub const IPROPNAME_ACTION: &str = "ACTION";
pub const IPROPNAME_ADMINTOOLS_FOLDER: &str = "AdminToolsFolder";
pub const IPROPNAME_ADMINUSER: &str = "AdminUser";
pub const IPROPNAME_ADMIN_PROPERTIES: &str = "AdminProperties";
pub const IPROPNAME_AFTERREBOOT: &str = "AFTERREBOOT";
pub const IPROPNAME_ALLOWEDPROPERTIES: &str = "SecureCustomProperties";
pub const IPROPNAME_ALLUSERS: &str = "ALLUSERS";
pub const IPROPNAME_APPDATA_FOLDER: &str = "AppDataFolder";
pub const IPROPNAME_ARM: &str = "Arm";
pub const IPROPNAME_ARM64: &str = "Arm64";
pub const IPROPNAME_ARPAUTHORIZEDCDFPREFIX: &str = "ARPAUTHORIZEDCDFPREFIX";
pub const IPROPNAME_ARPCOMMENTS: &str = "ARPCOMMENTS";
pub const IPROPNAME_ARPCONTACT: &str = "ARPCONTACT";
pub const IPROPNAME_ARPHELPLINK: &str = "ARPHELPLINK";
pub const IPROPNAME_ARPHELPTELEPHONE: &str = "ARPHELPTELEPHONE";
pub const IPROPNAME_ARPINSTALLLOCATION: &str = "ARPINSTALLLOCATION";
pub const IPROPNAME_ARPNOMODIFY: &str = "ARPNOMODIFY";
pub const IPROPNAME_ARPNOREMOVE: &str = "ARPNOREMOVE";
pub const IPROPNAME_ARPNOREPAIR: &str = "ARPNOREPAIR";
pub const IPROPNAME_ARPPRODUCTICON: &str = "ARPPRODUCTICON";
pub const IPROPNAME_ARPREADME: &str = "ARPREADME";
pub const IPROPNAME_ARPSETTINGSIDENTIFIER: &str = "MSIARPSETTINGSIDENTIFIER";
pub const IPROPNAME_ARPSHIMFLAGS: &str = "SHIMFLAGS";
pub const IPROPNAME_ARPSHIMSERVICEPACKLEVEL: &str = "SHIMSERVICEPACKLEVEL";
pub const IPROPNAME_ARPSHIMVERSIONNT: &str = "SHIMVERSIONNT";
pub const IPROPNAME_ARPSIZE: &str = "ARPSIZE";
pub const IPROPNAME_ARPSYSTEMCOMPONENT: &str = "ARPSYSTEMCOMPONENT";
pub const IPROPNAME_ARPURLINFOABOUT: &str = "ARPURLINFOABOUT";
pub const IPROPNAME_ARPURLUPDATEINFO: &str = "ARPURLUPDATEINFO";
pub const IPROPNAME_AVAILABLEFREEREG: &str = "AVAILABLEFREEREG";
pub const IPROPNAME_BORDERSIDE: &str = "BorderSide";
pub const IPROPNAME_BORDERTOP: &str = "BorderTop";
pub const IPROPNAME_CAPTIONHEIGHT: &str = "CaptionHeight";
pub const IPROPNAME_CARRYINGNDP: &str = "CARRYINGNDP";
pub const IPROPNAME_CHECKCRCS: &str = "MSICHECKCRCS";
pub const IPROPNAME_COLORBITS: &str = "ColorBits";
pub const IPROPNAME_COMMONAPPDATA_FOLDER: &str = "CommonAppDataFolder";
pub const IPROPNAME_COMMONFILES64_FOLDER: &str = "CommonFiles64Folder";
pub const IPROPNAME_COMMONFILES_FOLDER: &str = "CommonFilesFolder";
pub const IPROPNAME_COMPANYNAME: &str = "COMPANYNAME";
pub const IPROPNAME_COMPONENTADDDEFAULT: &str = "COMPADDDEFAULT";
pub const IPROPNAME_COMPONENTADDLOCAL: &str = "COMPADDLOCAL";
pub const IPROPNAME_COMPONENTADDSOURCE: &str = "COMPADDSOURCE";
pub const IPROPNAME_COMPUTERNAME: &str = "ComputerName";
pub const IPROPNAME_COSTINGCOMPLETE: &str = "CostingComplete";
pub const IPROPNAME_CUSTOMACTIONDATA: &str = "CustomActionData";
pub const IPROPNAME_DATE: &str = "Date";
pub const IPROPNAME_DATETIME: &str = "DateTime";
pub const IPROPNAME_DEFAULTUIFONT: &str = "DefaultUIFont";
pub const IPROPNAME_DESKTOP_FOLDER: &str = "DesktopFolder";
pub const IPROPNAME_DISABLEADVTSHORTCUTS: &str = "DISABLEADVTSHORTCUTS";
pub const IPROPNAME_DISABLEROLLBACK: &str = "DISABLEROLLBACK";
pub const IPROPNAME_DISKPROMPT: &str = "DiskPrompt";
pub const IPROPNAME_ENABLEUSERCONTROL: &str = "EnableUserControl";
pub const IPROPNAME_ENFORCE_UPGRADE_COMPONENT_RULES: &str = "MSIENFORCEUPGRADECOMPONENTRULES";
pub const IPROPNAME_EXECUTEACTION: &str = "EXECUTEACTION";
pub const IPROPNAME_EXECUTEMODE: &str = "EXECUTEMODE";
pub const IPROPNAME_FAVORITES_FOLDER: &str = "FavoritesFolder";
pub const IPROPNAME_FEATUREADDDEFAULT: &str = "ADDDEFAULT";
pub const IPROPNAME_FEATUREADDLOCAL: &str = "ADDLOCAL";
pub const IPROPNAME_FEATUREADDSOURCE: &str = "ADDSOURCE";
pub const IPROPNAME_FEATUREADVERTISE: &str = "ADVERTISE";
pub const IPROPNAME_FEATUREREMOVE: &str = "REMOVE";
pub const IPROPNAME_FILEADDDEFAULT: &str = "FILEADDDEFAULT";
pub const IPROPNAME_FILEADDLOCAL: &str = "FILEADDLOCAL";
pub const IPROPNAME_FILEADDSOURCE: &str = "FILEADDSOURCE";
pub const IPROPNAME_FONTS_FOLDER: &str = "FontsFolder";
pub const IPROPNAME_HIDDEN_PROPERTIES: &str = "MsiHiddenProperties";
pub const IPROPNAME_HIDECANCEL: &str = "MsiUIHideCancel";
pub const IPROPNAME_IA64: &str = "IA64";
pub const IPROPNAME_INSTALLED: &str = "Installed";
pub const IPROPNAME_INSTALLLANGUAGE: &str = "ProductLanguage";
pub const IPROPNAME_INSTALLLEVEL: &str = "INSTALLLEVEL";
pub const IPROPNAME_INSTALLPERUSER: &str = "MSIINSTALLPERUSER";
pub const IPROPNAME_INTEL: &str = "Intel";
pub const IPROPNAME_INTEL64: &str = "Intel64";
pub const IPROPNAME_INTERNALINSTALLEDPERUSER: &str = "MSIINTERNALINSTALLEDPERUSER";
pub const IPROPNAME_ISADMINPACKAGE: &str = "IsAdminPackage";
pub const IPROPNAME_LEFTUNIT: &str = "LeftUnit";
pub const IPROPNAME_LIMITUI: &str = "LIMITUI";
pub const IPROPNAME_LOCALAPPDATA_FOLDER: &str = "LocalAppDataFolder";
pub const IPROPNAME_LOGACTION: &str = "LOGACTION";
pub const IPROPNAME_LOGONUSER: &str = "LogonUser";
pub const IPROPNAME_MANUFACTURER: &str = "Manufacturer";
pub const IPROPNAME_MSIAMD64: &str = "MsiAMD64";
pub const IPROPNAME_MSIDISABLEEEUI: &str = "MSIDISABLEEEUI";
pub const IPROPNAME_MSIDISABLELUAPATCHING: &str = "MSIDISABLELUAPATCHING";
pub const IPROPNAME_MSIINSTANCEGUID: &str = "MSIINSTANCEGUID";
pub const IPROPNAME_MSILOGFILELOCATION: &str = "MsiLogFileLocation";
pub const IPROPNAME_MSILOGGINGMODE: &str = "MsiLogging";
pub const IPROPNAME_MSINEWINSTANCE: &str = "MSINEWINSTANCE";
pub const IPROPNAME_MSINODISABLEMEDIA: &str = "MSINODISABLEMEDIA";
pub const IPROPNAME_MSIPACKAGEDOWNLOADLOCALCOPY: &str = "MSIPACKAGEDOWNLOADLOCALCOPY";
pub const IPROPNAME_MSIPATCHDOWNLOADLOCALCOPY: &str = "MSIPATCHDOWNLOADLOCALCOPY";
pub const IPROPNAME_MSIPATCHREMOVE: &str = "MSIPATCHREMOVE";
pub const IPROPNAME_MSITABLETPC: &str = "MsiTabletPC";
pub const IPROPNAME_MSIX64: &str = "Msix64";
pub const IPROPNAME_MSI_FASTINSTALL: &str = "MSIFASTINSTALL";
pub const IPROPNAME_MSI_REBOOT_PENDING: &str = "MsiSystemRebootPending";
pub const IPROPNAME_MSI_RM_CONTROL: &str = "MSIRESTARTMANAGERCONTROL";
pub const IPROPNAME_MSI_RM_DISABLE_RESTART: &str = "MSIDISABLERMRESTART";
pub const IPROPNAME_MSI_RM_SESSION_KEY: &str = "MsiRestartManagerSessionKey";
pub const IPROPNAME_MSI_RM_SHUTDOWN: &str = "MSIRMSHUTDOWN";
pub const IPROPNAME_MSI_UAC_DEPLOYMENT_COMPLIANT: &str = "MSIDEPLOYMENTCOMPLIANT";
pub const IPROPNAME_MSI_UNINSTALL_SUPERSEDED_COMPONENTS: &str = "MSIUNINSTALLSUPERSEDEDCOMPONENTS";
pub const IPROPNAME_MSI_USE_REAL_ADMIN_DETECTION: &str = "MSIUSEREALADMINDETECTION";
pub const IPROPNAME_MYPICTURES_FOLDER: &str = "MyPicturesFolder";
pub const IPROPNAME_NETASSEMBLYSUPPORT: &str = "MsiNetAssemblySupport";
pub const IPROPNAME_NETHOOD_FOLDER: &str = "NetHoodFolder";
pub const IPROPNAME_NOCOMPANYNAME: &str = "NOCOMPANYNAME";
pub const IPROPNAME_NOUSERNAME: &str = "NOUSERNAME";
pub const IPROPNAME_NTPRODUCTTYPE: &str = "MsiNTProductType";
pub const IPROPNAME_NTSUITEBACKOFFICE: &str = "MsiNTSuiteBackOffice";
pub const IPROPNAME_NTSUITEDATACENTER: &str = "MsiNTSuiteDataCenter";
pub const IPROPNAME_NTSUITEENTERPRISE: &str = "MsiNTSuiteEnterprise";
pub const IPROPNAME_NTSUITEPERSONAL: &str = "MsiNTSuitePersonal";
pub const IPROPNAME_NTSUITESMALLBUSINESS: &str = "MsiNTSuiteSmallBusiness";
pub const IPROPNAME_NTSUITESMALLBUSINESSRESTRICTED: &str = "MsiNTSuiteSmallBusinessRestricted";
pub const IPROPNAME_NTSUITEWEBSERVER: &str = "MsiNTSuiteWebServer";
pub const IPROPNAME_OLEADVTSUPPORT: &str = "OLEAdvtSupport";
pub const IPROPNAME_OUTOFDISKSPACE: &str = "OutOfDiskSpace";
pub const IPROPNAME_OUTOFNORBDISKSPACE: &str = "OutOfNoRbDiskSpace";
pub const IPROPNAME_PATCH: &str = "PATCH";
pub const IPROPNAME_PATCHNEWPACKAGECODE: &str = "PATCHNEWPACKAGECODE";
pub const IPROPNAME_PATCHNEWSUMMARYCOMMENTS: &str = "PATCHNEWSUMMARYCOMMENTS";
pub const IPROPNAME_PATCHNEWSUMMARYSUBJECT: &str = "PATCHNEWSUMMARYSUBJECT";
pub const IPROPNAME_PERSONAL_FOLDER: &str = "PersonalFolder";
pub const IPROPNAME_PHYSICALMEMORY: &str = "PhysicalMemory";
pub const IPROPNAME_PIDKEY: &str = "PIDKEY";
pub const IPROPNAME_PIDTEMPLATE: &str = "PIDTemplate";
pub const IPROPNAME_PRESELECTED: &str = "Preselected";
pub const IPROPNAME_PRIMARYFOLDER: &str = "PRIMARYFOLDER";
pub const IPROPNAME_PRIMARYFOLDER_PATH: &str = "PrimaryVolumePath";
pub const IPROPNAME_PRIMARYFOLDER_SPACEAVAILABLE: &str = "PrimaryVolumeSpaceAvailable";
pub const IPROPNAME_PRIMARYFOLDER_SPACEREMAINING: &str = "PrimaryVolumeSpaceRemaining";
pub const IPROPNAME_PRIMARYFOLDER_SPACEREQUIRED: &str = "PrimaryVolumeSpaceRequired";
pub const IPROPNAME_PRINTHOOD_FOLDER: &str = "PrintHoodFolder";
pub const IPROPNAME_PRIVILEGED: &str = "Privileged";
pub const IPROPNAME_PRODUCTCODE: &str = "ProductCode";
pub const IPROPNAME_PRODUCTID: &str = "ProductID";
pub const IPROPNAME_PRODUCTLANGUAGE: &str = "PRODUCTLANGUAGE";
pub const IPROPNAME_PRODUCTNAME: &str = "ProductName";
pub const IPROPNAME_PRODUCTSTATE: &str = "ProductState";
pub const IPROPNAME_PRODUCTVERSION: &str = "ProductVersion";
pub const IPROPNAME_PROGRAMFILES64_FOLDER: &str = "ProgramFiles64Folder";
pub const IPROPNAME_PROGRAMFILES_FOLDER: &str = "ProgramFilesFolder";
pub const IPROPNAME_PROGRAMMENU_FOLDER: &str = "ProgramMenuFolder";
pub const IPROPNAME_PROGRESSONLY: &str = "MsiUIProgressOnly";
pub const IPROPNAME_PROMPTROLLBACKCOST: &str = "PROMPTROLLBACKCOST";
pub const IPROPNAME_REBOOT: &str = "REBOOT";
pub const IPROPNAME_REBOOTPROMPT: &str = "REBOOTPROMPT";
pub const IPROPNAME_RECENT_FOLDER: &str = "RecentFolder";
pub const IPROPNAME_REDIRECTEDDLLSUPPORT: &str = "RedirectedDllSupport";
pub const IPROPNAME_REINSTALL: &str = "REINSTALL";
pub const IPROPNAME_REINSTALLMODE: &str = "REINSTALLMODE";
pub const IPROPNAME_REMOTEADMINTS: &str = "RemoteAdminTS";
pub const IPROPNAME_REPLACEDINUSEFILES: &str = "ReplacedInUseFiles";
pub const IPROPNAME_RESTRICTEDUSERCONTROL: &str = "RestrictedUserControl";
pub const IPROPNAME_RESUME: &str = "RESUME";
pub const IPROPNAME_ROLLBACKDISABLED: &str = "RollbackDisabled";
pub const IPROPNAME_ROOTDRIVE: &str = "ROOTDRIVE";
pub const IPROPNAME_RUNNINGELEVATED: &str = "MsiRunningElevated";
pub const IPROPNAME_SCREENX: &str = "ScreenX";
pub const IPROPNAME_SCREENY: &str = "ScreenY";
pub const IPROPNAME_SENDTO_FOLDER: &str = "SendToFolder";
pub const IPROPNAME_SEQUENCE: &str = "SEQUENCE";
pub const IPROPNAME_SERVICEPACKLEVEL: &str = "ServicePackLevel";
pub const IPROPNAME_SERVICEPACKLEVELMINOR: &str = "ServicePackLevelMinor";
pub const IPROPNAME_SHAREDWINDOWS: &str = "SharedWindows";
pub const IPROPNAME_SHELLADVTSUPPORT: &str = "ShellAdvtSupport";
pub const IPROPNAME_SHORTFILENAMES: &str = "SHORTFILENAMES";
pub const IPROPNAME_SOURCEDIR: &str = "SourceDir";
pub const IPROPNAME_SOURCELIST: &str = "SOURCELIST";
pub const IPROPNAME_SOURCERESONLY: &str = "MsiUISourceResOnly";
pub const IPROPNAME_STARTMENU_FOLDER: &str = "StartMenuFolder";
pub const IPROPNAME_STARTUP_FOLDER: &str = "StartupFolder";
pub const IPROPNAME_SYSTEM16_FOLDER: &str = "System16Folder";
pub const IPROPNAME_SYSTEM64_FOLDER: &str = "System64Folder";
pub const IPROPNAME_SYSTEMLANGUAGEID: &str = "SystemLanguageID";
pub const IPROPNAME_SYSTEM_FOLDER: &str = "SystemFolder";
pub const IPROPNAME_TARGETDIR: &str = "TARGETDIR";
pub const IPROPNAME_TEMPLATE_AMD64: &str = "AMD64";
pub const IPROPNAME_TEMPLATE_FOLDER: &str = "TemplateFolder";
pub const IPROPNAME_TEMPLATE_X64: &str = "x64";
pub const IPROPNAME_TEMP_FOLDER: &str = "TempFolder";
pub const IPROPNAME_TERMSERVER: &str = "TerminalServer";
pub const IPROPNAME_TEXTHEIGHT: &str = "TextHeight";
pub const IPROPNAME_TEXTHEIGHT_CORRECTION: &str = "TextHeightCorrection";
pub const IPROPNAME_TEXTINTERNALLEADING: &str = "TextInternalLeading";
pub const IPROPNAME_TIME: &str = "Time";
pub const IPROPNAME_TRANSFORMS: &str = "TRANSFORMS";
pub const IPROPNAME_TRANSFORMSATSOURCE: &str = "TRANSFORMSATSOURCE";
pub const IPROPNAME_TRANSFORMSSECURE: &str = "TRANSFORMSSECURE";
pub const IPROPNAME_TRUEADMINUSER: &str = "MsiTrueAdminUser";
pub const IPROPNAME_TTCSUPPORT: &str = "TTCSupport";
pub const IPROPNAME_UACONLY: &str = "MsiUIUACOnly";
pub const IPROPNAME_UPDATESTARTED: &str = "UpdateStarted";
pub const IPROPNAME_UPGRADECODE: &str = "UpgradeCode";
pub const IPROPNAME_USERLANGUAGEID: &str = "UserLanguageID";
pub const IPROPNAME_USERNAME: &str = "USERNAME";
pub const IPROPNAME_USERSID: &str = "UserSID";
pub const IPROPNAME_VERSION9X: &str = "Version9X";
pub const IPROPNAME_VERSIONNT: &str = "VersionNT";
pub const IPROPNAME_VERSIONNT64: &str = "VersionNT64";
pub const IPROPNAME_VIRTUALMEMORY: &str = "VirtualMemory";
pub const IPROPNAME_WIN32ASSEMBLYSUPPORT: &str = "MsiWin32AssemblySupport";
pub const IPROPNAME_WINDOWSBUILD: &str = "WindowsBuild";
pub const IPROPNAME_WINDOWS_FOLDER: &str = "WindowsFolder";
pub const IPROPNAME_WINDOWS_VOLUME: &str = "WindowsVolume";
pub const IPROPVALUE_EXECUTEMODE_NONE: &str = "NONE";
pub const IPROPVALUE_EXECUTEMODE_SCRIPT: &str = "SCRIPT";
pub const IPROPVALUE_FEATURE_ALL: &str = "ALL";
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLE: &str = "Disable";
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLESHUTDOWN: &str = "DisableShutdown";
pub const IPROPVALUE_RBCOST_FAIL: &str = "F";
pub const IPROPVALUE_RBCOST_PROMPT: &str = "P";
pub const IPROPVALUE_RBCOST_SILENT: &str = "D";
pub const IPROPVALUE__CARRYINGNDP_URTREINSTALL: &str = "URTREINSTALL";
pub const IPROPVALUE__CARRYINGNDP_URTUPGRADE: &str = "URTUPGRADE";
pub type IValidate = *mut ::core::ffi::c_void;
pub const LIBID_MsmMergeTypeLib: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 182298671, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
pub const LOGALL: u32 = 15u32;
pub const LOGERR: u32 = 4u32;
pub const LOGINFO: u32 = 1u32;
pub const LOGNONE: u32 = 0u32;
pub const LOGPERFMESSAGES: u32 = 8u32;
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
pub const LOGWARN: u32 = 2u32;
pub type LPDISPLAYVAL = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, uitype: RESULTTYPES, szwval: ::windows_core_sys::PCWSTR, szwdescription: ::windows_core_sys::PCWSTR, szwlocation: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL>;
pub type LPEVALCOMCALLBACK = ::core::option::Option<unsafe extern "system" fn(istatus: STATUSTYPES, szdata: ::windows_core_sys::PCWSTR, pcontext: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL>;
pub const MAX_FEATURE_CHARS: u32 = 38u32;
pub const MAX_GUID_CHARS: u32 = 38u32;
pub type MSIADVERTISEOPTIONFLAGS = i32;
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = 1i32;
pub type MSIARCHITECTUREFLAGS = i32;
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = 1i32;
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = 2i32;
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = 4i32;
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = 8i32;
pub type MSIASSEMBLYINFO = u32;
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = 0u32;
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = 1u32;
pub type MSICODE = i32;
pub const MSICODE_PRODUCT: MSICODE = 0i32;
pub const MSICODE_PATCH: MSICODE = 1073741824i32;
pub type MSICOLINFO = i32;
pub const MSICOLINFO_NAMES: MSICOLINFO = 0i32;
pub const MSICOLINFO_TYPES: MSICOLINFO = 1i32;
pub type MSICONDITION = i32;
pub const MSICONDITION_FALSE: MSICONDITION = 0i32;
pub const MSICONDITION_TRUE: MSICONDITION = 1i32;
pub const MSICONDITION_NONE: MSICONDITION = 2i32;
pub const MSICONDITION_ERROR: MSICONDITION = 3i32;
pub type MSICOSTTREE = i32;
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = 0i32;
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = 1i32;
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = 2i32;
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = 3i32;
pub type MSIDBERROR = i32;
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = -3i32;
pub const MSIDBERROR_MOREDATA: MSIDBERROR = -2i32;
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = -1i32;
pub const MSIDBERROR_NOERROR: MSIDBERROR = 0i32;
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = 1i32;
pub const MSIDBERROR_REQUIRED: MSIDBERROR = 2i32;
pub const MSIDBERROR_BADLINK: MSIDBERROR = 3i32;
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = 4i32;
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = 5i32;
pub const MSIDBERROR_NOTINSET: MSIDBERROR = 6i32;
pub const MSIDBERROR_BADVERSION: MSIDBERROR = 7i32;
pub const MSIDBERROR_BADCASE: MSIDBERROR = 8i32;
pub const MSIDBERROR_BADGUID: MSIDBERROR = 9i32;
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = 10i32;
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = 11i32;
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = 12i32;
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = 13i32;
pub const MSIDBERROR_BADPATH: MSIDBERROR = 14i32;
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = 15i32;
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = 16i32;
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = 17i32;
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = 18i32;
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = 19i32;
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = 20i32;
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = 21i32;
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = 22i32;
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = 23i32;
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = 24i32;
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = 25i32;
pub const MSIDBERROR_BADCABINET: MSIDBERROR = 26i32;
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = 27i32;
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = 28i32;
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = 29i32;
pub type MSIDBSTATE = i32;
pub const MSIDBSTATE_ERROR: MSIDBSTATE = -1i32;
pub const MSIDBSTATE_READ: MSIDBSTATE = 0i32;
pub const MSIDBSTATE_WRITE: MSIDBSTATE = 1i32;
#[repr(C)]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl ::core::marker::Copy for MSIFILEHASHINFO {}
impl ::core::clone::Clone for MSIFILEHASHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MSIHANDLE = u32;
pub type MSIINSTALLCONTEXT = i32;
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = 0i32;
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = 0i32;
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = 1i32;
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = 2i32;
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = 4i32;
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = 7i32;
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = 8i32;
pub type MSIMODIFY = i32;
pub const MSIMODIFY_SEEK: MSIMODIFY = -1i32;
pub const MSIMODIFY_REFRESH: MSIMODIFY = 0i32;
pub const MSIMODIFY_INSERT: MSIMODIFY = 1i32;
pub const MSIMODIFY_UPDATE: MSIMODIFY = 2i32;
pub const MSIMODIFY_ASSIGN: MSIMODIFY = 3i32;
pub const MSIMODIFY_REPLACE: MSIMODIFY = 4i32;
pub const MSIMODIFY_MERGE: MSIMODIFY = 5i32;
pub const MSIMODIFY_DELETE: MSIMODIFY = 6i32;
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = 7i32;
pub const MSIMODIFY_VALIDATE: MSIMODIFY = 8i32;
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = 9i32;
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = 10i32;
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = 11i32;
pub type MSIOPENPACKAGEFLAGS = i32;
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = 1i32;
pub type MSIPATCHDATATYPE = i32;
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = 0i32;
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = 1i32;
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = 2i32;
#[repr(C)]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: ::windows_core_sys::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOA {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: ::windows_core_sys::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOW {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MSIPATCHSTATE = i32;
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = 0i32;
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = 1i32;
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = 2i32;
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = 4i32;
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = 8i32;
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = 15i32;
pub type MSIRUNMODE = i32;
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = 0i32;
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = 1i32;
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = 2i32;
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = 3i32;
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = 4i32;
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = 5i32;
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = 6i32;
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = 7i32;
pub const MSIRUNMODE_CABINET: MSIRUNMODE = 8i32;
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = 9i32;
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = 10i32;
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = 11i32;
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = 12i32;
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = 13i32;
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = 14i32;
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = 15i32;
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = 16i32;
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = 17i32;
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = 18i32;
pub type MSISOURCETYPE = i32;
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = 0i32;
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = 1i32;
pub const MSISOURCETYPE_URL: MSISOURCETYPE = 2i32;
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = 4i32;
pub type MSITRANSACTION = i32;
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = 1i32;
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = 2i32;
pub type MSITRANSACTIONSTATE = u32;
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = 0u32;
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = 1u32;
pub type MSITRANSFORM_ERROR = i32;
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = 1i32;
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = 2i32;
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = 4i32;
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = 8i32;
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = 16i32;
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = 32i32;
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = 256i32;
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = 0i32;
pub type MSITRANSFORM_VALIDATE = i32;
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = 1i32;
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = 2i32;
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = 4i32;
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = 8i32;
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = 16i32;
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = 32i32;
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = 64i32;
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 128i32;
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 256i32;
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = 512i32;
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = 1024i32;
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = 2048i32;
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
pub const MsmMerge: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 182298672, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
pub type PACKMAN_RUNTIME = i32;
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = 1i32;
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = 2i32;
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = 3i32;
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = 4i32;
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = 5i32;
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = 6i32;
#[repr(C)]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
impl ::core::marker::Copy for PATCH_IGNORE_RANGE {}
impl ::core::clone::Clone for PATCH_IGNORE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: ::windows_core_sys::PCSTR,
    pub OldFileNameW: ::windows_core_sys::PCWSTR,
    pub OldFileHandle: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_0 {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows_core_sys::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_A {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: ::win32_foundation_sys::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_H {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_H {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows_core_sys::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_W {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: ::windows_core_sys::PCSTR,
    pub OldFileSymbolPathArray: *mut ::windows_core_sys::PSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut ::core::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
impl ::core::marker::Copy for PATCH_OPTION_DATA {}
impl ::core::clone::Clone for PATCH_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[repr(C)]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
impl ::core::marker::Copy for PATCH_RETAIN_RANGE {}
impl ::core::clone::Clone for PATCH_RETAIN_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
pub const PID_APPNAME: u32 = 18u32;
pub const PID_AUTHOR: u32 = 4u32;
pub const PID_CHARCOUNT: u32 = 16u32;
pub const PID_COMMENTS: u32 = 6u32;
pub const PID_CREATE_DTM: u32 = 12u32;
pub const PID_EDITTIME: u32 = 10u32;
pub const PID_KEYWORDS: u32 = 5u32;
pub const PID_LASTAUTHOR: u32 = 8u32;
pub const PID_LASTPRINTED: u32 = 11u32;
pub const PID_LASTSAVE_DTM: u32 = 13u32;
pub const PID_MSIRESTRICT: u32 = 16u32;
pub const PID_MSISOURCE: u32 = 15u32;
pub const PID_MSIVERSION: u32 = 14u32;
pub const PID_PAGECOUNT: u32 = 14u32;
pub const PID_REVNUMBER: u32 = 9u32;
pub const PID_SUBJECT: u32 = 3u32;
pub const PID_TEMPLATE: u32 = 7u32;
pub const PID_THUMBNAIL: u32 = 17u32;
pub const PID_TITLE: u32 = 2u32;
pub const PID_WORDCOUNT: u32 = 15u32;
pub type PINSTALLUI_HANDLER_RECORD = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
#[repr(C)]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl ::core::marker::Copy for PMSIHANDLE {}
impl ::core::clone::Clone for PMSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PMSvc: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3118797308, data2: 58212, data3: 18810, data4: [161, 33, 183, 179, 97, 44, 237, 206] };
pub type PM_ACTIVATION_POLICY = i32;
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = 0i32;
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = 1i32;
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = 2i32;
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = 3i32;
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = 4i32;
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY = 5i32;
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = 6i32;
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = 7i32;
pub type PM_APPLICATION_HUBTYPE = i32;
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = 0i32;
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = 1i32;
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = 2i32;
pub type PM_APPLICATION_INSTALL_TYPE = i32;
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE = 0i32;
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE = 1i32;
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE = 2i32;
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE = 3i32;
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE = 4i32;
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE = 5i32;
pub type PM_APPLICATION_STATE = i32;
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = 0i32;
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = 1i32;
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = 2i32;
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = 3i32;
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = 4i32;
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = 5i32;
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = 6i32;
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = 7i32;
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE = 8i32;
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE = 9i32;
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE = 10i32;
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = 10i32;
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = 11i32;
pub type PM_APP_GENRE = i32;
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = 0i32;
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = 1i32;
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = 2i32;
#[repr(C)]
pub struct PM_BSATASKID {
    pub ProductID: ::windows_core_sys::GUID,
    pub TaskID: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_BSATASKID {}
impl ::core::clone::Clone for PM_BSATASKID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PM_BWTASKID {
    pub ProductID: ::windows_core_sys::GUID,
    pub TaskID: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_BWTASKID {}
impl ::core::clone::Clone for PM_BWTASKID {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PM_ENUM_APP_FILTER = i32;
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = 0i32;
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = 1i32;
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = 2i32;
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = 3i32;
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = 4i32;
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = 5i32;
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = 6i32;
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = 7i32;
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = 8i32;
pub type PM_ENUM_BSA_FILTER = i32;
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = 26i32;
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = 27i32;
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = 28i32;
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = 29i32;
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = 30i32;
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = 31i32;
pub type PM_ENUM_BW_FILTER = i32;
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = 31i32;
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = 32i32;
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = 33i32;
pub type PM_ENUM_EXTENSION_FILTER = i32;
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER = 17i32;
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER = 17i32;
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER = 18i32;
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER = 19i32;
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER = 20i32;
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER = 21i32;
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER = 22i32;
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER = 23i32;
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER = 24i32;
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER = 25i32;
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = 26i32;
#[repr(C)]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
impl ::core::marker::Copy for PM_ENUM_FILTER {}
impl ::core::clone::Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: ::windows_core_sys::GUID,
    pub TileProductID: ::windows_core_sys::GUID,
    pub AppTaskType: _tagAPPTASKTYPE,
    pub Consumer: PM_EXTENSIONCONSUMER,
    pub BSATask: PM_BSATASKID,
    pub BSAProductID: ::windows_core_sys::GUID,
    pub BWTask: PM_BWTASKID,
    pub ProtocolName: ::win32_foundation_sys::BSTR,
    pub FileType: ::win32_foundation_sys::BSTR,
    pub ContentType: ::win32_foundation_sys::BSTR,
    pub AppSupportedFileExtPID: ::windows_core_sys::GUID,
    pub ShareTargetFileType: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_ENUM_FILTER_0 {}
impl ::core::clone::Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PM_ENUM_TASK_FILTER = i32;
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = 12i32;
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = 13i32;
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = 14i32;
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = 15i32;
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = 16i32;
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = 17i32;
pub type PM_ENUM_TILE_FILTER = i32;
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = 8i32;
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = 9i32;
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = 10i32;
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = 11i32;
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = 12i32;
#[repr(C)]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: ::windows_core_sys::GUID,
    pub ExtensionID: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_EXTENSIONCONSUMER {}
impl ::core::clone::Clone for PM_EXTENSIONCONSUMER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PM_INSTALLINFO {
    pub ProductID: ::windows_core_sys::GUID,
    pub PackagePath: ::win32_foundation_sys::BSTR,
    pub InstanceID: ::windows_core_sys::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: ::win32_foundation_sys::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: ::windows_core_sys::GUID,
    pub MarketplaceAppVersion: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_INSTALLINFO {}
impl ::core::clone::Clone for PM_INSTALLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: ::win32_foundation_sys::BSTR,
    pub URIFragmentOrArgs: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_INVOCATIONINFO {}
impl ::core::clone::Clone for PM_INVOCATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PM_LIVETILE_RECURRENCE_TYPE = i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE = 0i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE = 1i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE = 2i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE = 2i32;
pub type PM_LOGO_SIZE = i32;
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = 0i32;
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = 1i32;
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = 2i32;
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = 3i32;
#[repr(C)]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_core_sys::GUID,
    pub AppTitle: ::win32_foundation_sys::BSTR,
    pub IconPath: ::win32_foundation_sys::BSTR,
    pub IsUninstallable: ::win32_foundation_sys::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: ::windows_core_sys::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: ::win32_foundation_sys::BOOL,
    pub IsModernLightUp: ::win32_foundation_sys::BOOL,
    pub LightUpSupportMask: u16,
}
impl ::core::marker::Copy for PM_STARTAPPBLOB {}
impl ::core::clone::Clone for PM_STARTAPPBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_core_sys::GUID,
    pub TileID: ::win32_foundation_sys::BSTR,
    pub TemplateType: TILE_TEMPLATE_TYPE,
    pub HubPosition: [u32; 32],
    pub HubVisibilityBitmask: u32,
    pub IsDefault: ::win32_foundation_sys::BOOL,
    pub TileType: PM_STARTTILE_TYPE,
    pub pbPropBlob: *mut u8,
    pub cbPropBlob: u32,
    pub IsRestoring: ::win32_foundation_sys::BOOL,
    pub IsModern: ::win32_foundation_sys::BOOL,
    pub InvocationInfo: PM_INVOCATIONINFO,
}
impl ::core::marker::Copy for PM_STARTTILEBLOB {}
impl ::core::clone::Clone for PM_STARTTILEBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PM_STARTTILE_TYPE = i32;
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = 1i32;
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = 2i32;
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = 3i32;
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = 4i32;
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = 5i32;
pub type PM_TASK_TRANSITION = i32;
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = 0i32;
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = 1i32;
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = 2i32;
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = 3i32;
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = 4i32;
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = 5i32;
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = 6i32;
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = 7i32;
pub type PM_TASK_TYPE = i32;
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = 0i32;
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = 1i32;
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = 2i32;
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = 3i32;
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = 4i32;
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = 5i32;
pub type PM_TILE_HUBTYPE = i32;
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = 1i32;
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = 268435456i32;
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = 536870912i32;
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = 1073741824i32;
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = -2147483648i32;
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = 16777216i32;
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = 33554432i32;
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = 67108864i32;
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = 67108865i32;
pub type PM_TILE_SIZE = i32;
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = 0i32;
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = 1i32;
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = 2i32;
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = 3i32;
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = 4i32;
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = 5i32;
#[repr(C)]
pub struct PM_UPDATEINFO {
    pub ProductID: ::windows_core_sys::GUID,
    pub PackagePath: ::win32_foundation_sys::BSTR,
    pub InstanceID: ::windows_core_sys::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: ::win32_foundation_sys::BSTR,
    pub DeploymentOptions: u32,
}
impl ::core::marker::Copy for PM_UPDATEINFO {}
impl ::core::clone::Clone for PM_UPDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: ::windows_core_sys::GUID,
    pub PackagePath: ::win32_foundation_sys::BSTR,
    pub InstanceID: ::windows_core_sys::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: ::win32_foundation_sys::BSTR,
}
impl ::core::marker::Copy for PM_UPDATEINFO_LEGACY {}
impl ::core::clone::Clone for PM_UPDATEINFO_LEGACY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PPATCH_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *mut ::core::ffi::c_void, currentposition: u32, maximumposition: u32) -> ::win32_foundation_sys::BOOL>;
pub type PPATCH_SYMLOAD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: ::windows_core_sys::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL>;
#[repr(C)]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl ::core::marker::Copy for PROTECTED_FILE_DATA {}
impl ::core::clone::Clone for PROTECTED_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type QUERYASMINFO_FLAGS = u32;
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = 1u32;
pub type REINSTALLMODE = i32;
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = 1i32;
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = 2i32;
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = 4i32;
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = 8i32;
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = 16i32;
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = 32i32;
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = 64i32;
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = 128i32;
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = 256i32;
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = 512i32;
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = 1024i32;
pub type RESULTTYPES = i32;
pub const ieUnknown: RESULTTYPES = 0i32;
pub const ieError: RESULTTYPES = 1i32;
pub const ieWarning: RESULTTYPES = 2i32;
pub const ieInfo: RESULTTYPES = 3i32;
pub type SCRIPTFLAGS = i32;
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = 1i32;
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = 4i32;
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = 8i32;
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = 32i32;
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = 64i32;
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = 128i32;
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = 256i32;
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = 384i32;
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = 416i32;
pub const SFC_DISABLE_ASK: u32 = 1u32;
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
pub const SFC_DISABLE_ONCE: u32 = 2u32;
pub const SFC_DISABLE_SETUP: u32 = 3u32;
pub const SFC_IDLE_TRIGGER: &str = "WFP_IDLE_TRIGGER";
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
pub const SFC_SCAN_NORMAL: u32 = 0u32;
pub const SFC_SCAN_ONCE: u32 = 2u32;
pub type STATUSTYPES = i32;
pub const ieStatusGetCUB: STATUSTYPES = 0i32;
pub const ieStatusICECount: STATUSTYPES = 1i32;
pub const ieStatusMerge: STATUSTYPES = 2i32;
pub const ieStatusSummaryInfo: STATUSTYPES = 3i32;
pub const ieStatusCreateEngine: STATUSTYPES = 4i32;
pub const ieStatusStarting: STATUSTYPES = 5i32;
pub const ieStatusRunICE: STATUSTYPES = 6i32;
pub const ieStatusShutdown: STATUSTYPES = 7i32;
pub const ieStatusSuccess: STATUSTYPES = 8i32;
pub const ieStatusFail: STATUSTYPES = 9i32;
pub const ieStatusCancel: STATUSTYPES = 10i32;
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
pub type TILE_TEMPLATE_TYPE = i32;
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = 0i32;
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = 5i32;
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = 13i32;
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = 14i32;
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = 1i32;
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = 2i32;
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = 3i32;
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = 4i32;
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = 7i32;
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = 10i32;
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = 11i32;
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = 12i32;
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = 15i32;
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = 16i32;
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = 17i32;
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = 18i32;
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = 19i32;
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = 20i32;
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = 21i32;
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = 22i32;
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = 23i32;
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = 24i32;
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = 25i32;
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = 26i32;
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = 27i32;
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = 28i32;
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = 29i32;
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = 30i32;
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 31i32;
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 32i32;
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = 33i32;
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = 34i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 35i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 36i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = 37i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = 38i32;
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = 39i32;
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = 40i32;
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = 41i32;
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = 42i32;
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = 43i32;
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = 44i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = 45i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = 46i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = 47i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = 48i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = 49i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = 50i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 51i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 52i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = 53i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = 54i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = 55i32;
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = 56i32;
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = 57i32;
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = 58i32;
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = 59i32;
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = 100i32;
pub const TXTLOG_BACKUP: u32 = 128u32;
pub const TXTLOG_CMI: u32 = 268435456u32;
pub const TXTLOG_COPYFILES: u32 = 8u32;
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
pub const TXTLOG_DETAILS: u32 = 5u32;
pub const TXTLOG_DEVINST: u32 = 1u32;
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
pub const TXTLOG_ERROR: u32 = 1u32;
pub const TXTLOG_FILEQ: u32 = 4u32;
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
pub const TXTLOG_INF: u32 = 2u32;
pub const TXTLOG_INFDB: u32 = 1024u32;
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
pub const TXTLOG_POLICY: u32 = 8388608u32;
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
pub const TXTLOG_SETUP: u32 = 134217728u32;
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
pub const TXTLOG_SIGVERIF: u32 = 32u32;
pub const TXTLOG_SUMMARY: u32 = 4u32;
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
pub const TXTLOG_TAB_1: u32 = 524288u32;
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
pub const TXTLOG_UI: u32 = 256u32;
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
pub const TXTLOG_UTIL: u32 = 512u32;
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
pub const TXTLOG_VERBOSE: u32 = 6u32;
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
pub const TXTLOG_WARNING: u32 = 2u32;
pub const UIALL: u32 = 32768u32;
pub const UILOGBITS: u32 = 15u32;
pub const UINONE: u32 = 0u32;
pub type USERINFOSTATE = i32;
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = -3i32;
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = -2i32;
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = -1i32;
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = 0i32;
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = 1i32;
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
pub const WARN_BASE: u32 = 3222294785u32;
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
pub const _WIN32_MSI: u32 = 500u32;
pub const _WIN32_MSM: u32 = 100u32;
#[repr(C)]
pub struct _tagAPPTASKTYPE {
    pub ProductID: ::windows_core_sys::GUID,
    pub TaskType: PM_TASK_TYPE,
}
impl ::core::marker::Copy for _tagAPPTASKTYPE {}
impl ::core::clone::Clone for _tagAPPTASKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const cchMaxInteger: i32 = 12i32;
pub type msidbAssemblyAttributes = i32;
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = 0i32;
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = 1i32;
pub type msidbClassAttributes = i32;
pub const msidbClassAttributesRelativePath: msidbClassAttributes = 1i32;
pub type msidbComponentAttributes = i32;
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes = 0i32;
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes = 1i32;
pub const msidbComponentAttributesOptional: msidbComponentAttributes = 2i32;
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes = 4i32;
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes = 8i32;
pub const msidbComponentAttributesPermanent: msidbComponentAttributes = 16i32;
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes = 32i32;
pub const msidbComponentAttributesTransitive: msidbComponentAttributes = 64i32;
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes = 128i32;
pub const msidbComponentAttributes64bit: msidbComponentAttributes = 256i32;
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes = 512i32;
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes = 1024i32;
pub const msidbComponentAttributesShared: msidbComponentAttributes = 2048i32;
pub type msidbControlAttributes = i32;
pub const msidbControlAttributesVisible: msidbControlAttributes = 1i32;
pub const msidbControlAttributesEnabled: msidbControlAttributes = 2i32;
pub const msidbControlAttributesSunken: msidbControlAttributes = 4i32;
pub const msidbControlAttributesIndirect: msidbControlAttributes = 8i32;
pub const msidbControlAttributesInteger: msidbControlAttributes = 16i32;
pub const msidbControlAttributesRTLRO: msidbControlAttributes = 32i32;
pub const msidbControlAttributesRightAligned: msidbControlAttributes = 64i32;
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = 128i32;
pub const msidbControlAttributesBiDi: msidbControlAttributes = 224i32;
pub const msidbControlAttributesTransparent: msidbControlAttributes = 65536i32;
pub const msidbControlAttributesNoPrefix: msidbControlAttributes = 131072i32;
pub const msidbControlAttributesNoWrap: msidbControlAttributes = 262144i32;
pub const msidbControlAttributesFormatSize: msidbControlAttributes = 524288i32;
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes = 1048576i32;
pub const msidbControlAttributesMultiline: msidbControlAttributes = 65536i32;
pub const msidbControlAttributesPasswordInput: msidbControlAttributes = 2097152i32;
pub const msidbControlAttributesProgress95: msidbControlAttributes = 65536i32;
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes = 65536i32;
pub const msidbControlAttributesFixedVolume: msidbControlAttributes = 131072i32;
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes = 262144i32;
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes = 524288i32;
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes = 1048576i32;
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes = 2097152i32;
pub const msidbControlShowRollbackCost: msidbControlAttributes = 4194304i32;
pub const msidbControlAttributesSorted: msidbControlAttributes = 65536i32;
pub const msidbControlAttributesComboList: msidbControlAttributes = 131072i32;
pub const msidbControlAttributesImageHandle: msidbControlAttributes = 65536i32;
pub const msidbControlAttributesPushLike: msidbControlAttributes = 131072i32;
pub const msidbControlAttributesBitmap: msidbControlAttributes = 262144i32;
pub const msidbControlAttributesIcon: msidbControlAttributes = 524288i32;
pub const msidbControlAttributesFixedSize: msidbControlAttributes = 1048576i32;
pub const msidbControlAttributesIconSize16: msidbControlAttributes = 2097152i32;
pub const msidbControlAttributesIconSize32: msidbControlAttributes = 4194304i32;
pub const msidbControlAttributesIconSize48: msidbControlAttributes = 6291456i32;
pub const msidbControlAttributesElevationShield: msidbControlAttributes = 8388608i32;
pub const msidbControlAttributesHasBorder: msidbControlAttributes = 16777216i32;
pub type msidbCustomActionType = i32;
pub const msidbCustomActionTypeDll: msidbCustomActionType = 1i32;
pub const msidbCustomActionTypeExe: msidbCustomActionType = 2i32;
pub const msidbCustomActionTypeTextData: msidbCustomActionType = 3i32;
pub const msidbCustomActionTypeJScript: msidbCustomActionType = 5i32;
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = 6i32;
pub const msidbCustomActionTypeInstall: msidbCustomActionType = 7i32;
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = 0i32;
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = 16i32;
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = 32i32;
pub const msidbCustomActionTypeProperty: msidbCustomActionType = 48i32;
pub const msidbCustomActionTypeContinue: msidbCustomActionType = 64i32;
pub const msidbCustomActionTypeAsync: msidbCustomActionType = 128i32;
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = 256i32;
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType = 512i32;
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = 768i32;
pub const msidbCustomActionTypeInScript: msidbCustomActionType = 1024i32;
pub const msidbCustomActionTypeRollback: msidbCustomActionType = 256i32;
pub const msidbCustomActionTypeCommit: msidbCustomActionType = 512i32;
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType = 2048i32;
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = 16384i32;
pub const msidbCustomActionType64BitScript: msidbCustomActionType = 4096i32;
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = 8192i32;
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType = 32768i32;
pub type msidbDialogAttributes = i32;
pub const msidbDialogAttributesVisible: msidbDialogAttributes = 1i32;
pub const msidbDialogAttributesModal: msidbDialogAttributes = 2i32;
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = 4i32;
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = 8i32;
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = 16i32;
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = 32i32;
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes = 64i32;
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = 128i32;
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = 256i32;
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = 512i32;
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = 896i32;
pub const msidbDialogAttributesError: msidbDialogAttributes = 65536i32;
pub type msidbEmbeddedUIAttributes = i32;
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = 1i32;
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = 2i32;
pub type msidbFeatureAttributes = i32;
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = 0i32;
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = 1i32;
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = 2i32;
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes = 4i32;
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes = 8i32;
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes = 16i32;
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes = 32i32;
pub type msidbFileAttributes = i32;
pub const msidbFileAttributesReadOnly: msidbFileAttributes = 1i32;
pub const msidbFileAttributesHidden: msidbFileAttributes = 2i32;
pub const msidbFileAttributesSystem: msidbFileAttributes = 4i32;
pub const msidbFileAttributesReserved0: msidbFileAttributes = 8i32;
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = 16i32;
pub const msidbFileAttributesReserved1: msidbFileAttributes = 64i32;
pub const msidbFileAttributesReserved2: msidbFileAttributes = 128i32;
pub const msidbFileAttributesReserved3: msidbFileAttributes = 256i32;
pub const msidbFileAttributesVital: msidbFileAttributes = 512i32;
pub const msidbFileAttributesChecksum: msidbFileAttributes = 1024i32;
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = 4096i32;
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = 8192i32;
pub const msidbFileAttributesCompressed: msidbFileAttributes = 16384i32;
pub const msidbFileAttributesReserved4: msidbFileAttributes = 32768i32;
pub type msidbIniFileAction = i32;
pub const msidbIniFileActionAddLine: msidbIniFileAction = 0i32;
pub const msidbIniFileActionCreateLine: msidbIniFileAction = 1i32;
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = 2i32;
pub const msidbIniFileActionAddTag: msidbIniFileAction = 3i32;
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = 4i32;
pub type msidbLocatorType = i32;
pub const msidbLocatorTypeDirectory: msidbLocatorType = 0i32;
pub const msidbLocatorTypeFileName: msidbLocatorType = 1i32;
pub const msidbLocatorTypeRawValue: msidbLocatorType = 2i32;
pub const msidbLocatorType64bit: msidbLocatorType = 16i32;
pub type msidbMoveFileOptions = i32;
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = 1i32;
pub type msidbODBCDataSourceRegistration = i32;
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration = 0i32;
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration = 1i32;
pub type msidbPatchAttributes = i32;
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = 1i32;
pub type msidbRegistryRoot = i32;
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = 0i32;
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = 1i32;
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = 2i32;
pub const msidbRegistryRootUsers: msidbRegistryRoot = 3i32;
pub type msidbRemoveFileInstallMode = i32;
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode = 1i32;
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode = 2i32;
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode = 3i32;
pub type msidbServiceConfigEvent = i32;
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = 1i32;
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = 2i32;
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = 4i32;
pub type msidbServiceControlEvent = i32;
pub const msidbServiceControlEventStart: msidbServiceControlEvent = 1i32;
pub const msidbServiceControlEventStop: msidbServiceControlEvent = 2i32;
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = 8i32;
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent = 16i32;
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent = 32i32;
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent = 128i32;
pub type msidbServiceInstallErrorControl = i32;
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl = 32768i32;
pub type msidbSumInfoSourceType = i32;
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = 1i32;
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = 2i32;
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = 4i32;
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = 8i32;
pub type msidbTextStyleStyleBits = i32;
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = 1i32;
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = 2i32;
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = 4i32;
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = 8i32;
pub type msidbUpgradeAttributes = i32;
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes = 1i32;
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = 2i32;
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes = 4i32;
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes = 256i32;
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes = 512i32;
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes = 1024i32;
pub type msifiFastInstallBits = i32;
pub const msifiFastInstallNoSR: msifiFastInstallBits = 1i32;
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = 2i32;
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = 4i32;
pub type msirbRebootReason = i32;
pub const msirbRebootUndeterminedReason: msirbRebootReason = 0i32;
pub const msirbRebootInUseFilesReason: msirbRebootReason = 1i32;
pub const msirbRebootScheduleRebootReason: msirbRebootReason = 2i32;
pub const msirbRebootForceRebootReason: msirbRebootReason = 3i32;
pub const msirbRebootCustomActionReason: msirbRebootReason = 4i32;
pub type msirbRebootType = i32;
pub const msirbRebootImmediate: msirbRebootType = 1i32;
pub const msirbRebootDeferred: msirbRebootType = 2i32;
pub type msmErrorType = i32;
pub const msmErrorLanguageUnsupported: msmErrorType = 1i32;
pub const msmErrorLanguageFailed: msmErrorType = 2i32;
pub const msmErrorExclusion: msmErrorType = 3i32;
pub const msmErrorTableMerge: msmErrorType = 4i32;
pub const msmErrorResequenceMerge: msmErrorType = 5i32;
pub const msmErrorFileCreate: msmErrorType = 6i32;
pub const msmErrorDirCreate: msmErrorType = 7i32;
pub const msmErrorFeatureRequired: msmErrorType = 8i32;
