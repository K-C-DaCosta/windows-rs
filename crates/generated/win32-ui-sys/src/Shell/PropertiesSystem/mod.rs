#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "win32-system-sys")]
    pub fn ClearPropVariantArray(rgpropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, cvars: u32);
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn ClearVariantArray(pvars: *mut ::win32_system_sys::Com::VARIANT, cvars: u32);
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromBooleanVector(prgf: *const ::win32_foundation_sys::BOOL, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromCLSID(clsid: *const ::windows_core_sys::GUID, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromFileTime(pftin: *const ::win32_foundation_sys::FILETIME, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromFileTimeVector(prgft: *const ::win32_foundation_sys::FILETIME, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromGUIDAsString(guid: *const ::windows_core_sys::GUID, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromPropVariantVectorElem(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromResource(hinst: ::win32_foundation_sys::HINSTANCE, id: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-ui-sys"))]
    pub fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromStringAsVector(psz: ::windows_core_sys::PCWSTR, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromStringVector(prgsz: *const ::windows_core_sys::PWSTR, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitPropVariantVectorFromPropVariant(propvarsingle: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ppropvarvector: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromBooleanArray(prgf: *const ::win32_foundation_sys::BOOL, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromFileTime(pft: *const ::win32_foundation_sys::FILETIME, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromFileTimeArray(prgft: *const ::win32_foundation_sys::FILETIME, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromGUIDAsString(guid: *const ::windows_core_sys::GUID, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromInt16Array(prgn: *const i16, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromInt32Array(prgn: *const i32, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromInt64Array(prgn: *const i64, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromResource(hinst: ::win32_foundation_sys::HINSTANCE, id: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys", feature = "win32-ui-sys"))]
    pub fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromStringArray(prgsz: *const ::windows_core_sys::PWSTR, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn InitVariantFromVariantArrayElem(varin: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    pub fn PSCreateAdapterFromPropertyStore(pps: IPropertyStore, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSCreateDelayedMultiplexPropertyStore(flags: GETPROPERTYSTOREFLAGS, pdpsf: IDelayedPropertyStoreFactory, rgstoreids: *const u32, cstores: u32, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSCreateMemoryPropertyStore(riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::windows_core_sys::IUnknown, cstores: u32, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSCreatePropertyStoreFromObject(punk: ::windows_core_sys::IUnknown, grfmode: u32, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSCreatePropertyStoreFromPropertySetStorage(ppss: ::win32_system_sys::Com::StructuredStorage::IPropertySetStorage, grfmode: u32, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: ::windows_core_sys::PWSTR, cchtext: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn PSFormatPropertyValue(pps: IPropertyStore, ppd: IPropertyDescription, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn PSGetItemPropertyHandler(punkitem: ::windows_core_sys::IUnknown, freadwrite: ::win32_foundation_sys::BOOL, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSGetItemPropertyHandlerWithCreateObject(punkitem: ::windows_core_sys::IUnknown, freadwrite: ::win32_foundation_sys::BOOL, punkcreateobject: ::windows_core_sys::IUnknown, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSGetNamedPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: ::windows_core_sys::PCWSTR, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    pub fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSGetPropertyDescriptionByName(pszcanonicalname: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn PSGetPropertyDescriptionListFromString(pszproplist: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    pub fn PSGetPropertyKeyFromName(pszname: ::windows_core_sys::PCWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows_core_sys::HRESULT;
    pub fn PSGetPropertySystem(riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSGetPropertyValue(pps: IPropertyStore, ppd: IPropertyDescription, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    pub fn PSLookupPropertyHandlerCLSID(pszfilepath: ::windows_core_sys::PCWSTR, pclsid: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_Delete(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadBOOL(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadBSTR(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadDWORD(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadGUID(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadInt(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadLONG(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadPOINTL(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::win32_foundation_sys::POINTL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadPOINTS(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::win32_foundation_sys::POINTS) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadPropertyKey(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut PROPERTYKEY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadRECTL(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::win32_foundation_sys::RECTL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadSHORT(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadStr(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: ::windows_core_sys::PWSTR, charactercount: i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadStrAlloc(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadStream(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut ::win32_system_sys::Com::IStream) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn PSPropertyBag_ReadType(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, var: *mut ::win32_system_sys::Com::VARIANT, r#type: u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadULONGLONG(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_ReadUnknown(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteBOOL(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteBSTR(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteDWORD(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteGUID(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *const ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteInt(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteLONG(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WritePOINTL(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *const ::win32_foundation_sys::POINTL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WritePOINTS(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *const ::win32_foundation_sys::POINTS) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WritePropertyKey(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *const PROPERTYKEY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteRECTL(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: *const ::win32_foundation_sys::RECTL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteSHORT(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteStr(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteStream(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: ::win32_system_sys::Com::IStream) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteULONGLONG(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, value: u64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSPropertyBag_WriteUnknown(propbag: ::win32_system_sys::Com::StructuredStorage::IPropertyBag, propname: ::windows_core_sys::PCWSTR, punk: ::windows_core_sys::IUnknown) -> ::windows_core_sys::HRESULT;
    pub fn PSPropertyKeyFromString(pszstring: ::windows_core_sys::PCWSTR, pkey: *mut PROPERTYKEY) -> ::windows_core_sys::HRESULT;
    pub fn PSRefreshPropertySchema() -> ::windows_core_sys::HRESULT;
    pub fn PSRegisterPropertySchema(pszpath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PSSetPropertyValue(pps: IPropertyStore, ppd: IPropertyDescription, propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    pub fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: ::windows_core_sys::PWSTR, cch: u32) -> ::windows_core_sys::HRESULT;
    pub fn PSUnregisterPropertySchema(pszpath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn PifMgr_CloseProperties(hprops: ::win32_foundation_sys::HANDLE, flopt: u32) -> ::win32_foundation_sys::HANDLE;
    pub fn PifMgr_GetProperties(hprops: ::win32_foundation_sys::HANDLE, pszgroup: ::windows_core_sys::PCSTR, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
    pub fn PifMgr_OpenProperties(pszapp: ::windows_core_sys::PCWSTR, pszpif: ::windows_core_sys::PCWSTR, hinf: u32, flopt: u32) -> ::win32_foundation_sys::HANDLE;
    pub fn PifMgr_SetProperties(hprops: ::win32_foundation_sys::HANDLE, pszgroup: ::windows_core_sys::PCSTR, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantChangeType(ppropvardest: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantCompareEx(propvar1: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, propvar2: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetBooleanElem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pfval: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetDoubleElem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetElementCount(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetFileTimeElem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pftval: *mut ::win32_foundation_sys::FILETIME) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetInt16Elem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetInt32Elem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetInt64Elem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetStringElem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppszval: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetUInt16Elem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetUInt32Elem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantGetUInt64Elem(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToBSTR(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToBoolean(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pfret: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToBooleanVector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgf: *mut ::win32_foundation_sys::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToBooleanVectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut ::win32_foundation_sys::BOOL, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToBooleanWithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, fdefault: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToBuffer(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToDouble(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pdblret: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToDoubleVector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToDoubleVectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToDoubleWithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToFileTime(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS, pftout: *mut ::win32_foundation_sys::FILETIME) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToFileTimeVector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgft: *mut ::win32_foundation_sys::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToFileTimeVectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut ::win32_foundation_sys::FILETIME, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToGUID(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pguid: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt16(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, piret: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt16Vector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt16VectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt16WithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt32(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, plret: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt32Vector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt32VectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt32WithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt64(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pllret: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt64Vector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt64VectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToInt64WithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-ui-sys"))]
    pub fn PropVariantToStrRet(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToString(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, psz: ::windows_core_sys::PWSTR, cch: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToStringAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ppszout: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToStringVector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgsz: *mut ::windows_core_sys::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToStringVectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut ::windows_core_sys::PWSTR, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToStringWithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pszdefault: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::PWSTR;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt16(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, puiret: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt16Vector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt16VectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt16WithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt32(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pulret: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt32Vector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt32VectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt32WithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt64(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pullret: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt64Vector(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt64VectorAlloc(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToUInt64WithDefault(propvarin: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn PropVariantToVariant(ppropvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, pvar: *mut ::win32_system_sys::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn PropVariantToWinRTPropertyValue(propvar: *const ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn SHAddDefaultPropertiesByExt(pszext: ::windows_core_sys::PCWSTR, ppropstore: IPropertyStore) -> ::windows_core_sys::HRESULT;
    pub fn SHGetPropertyStoreForWindow(hwnd: ::win32_foundation_sys::HWND, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-ui-sys")]
    pub fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn SHGetPropertyStoreFromParsingName(pszpath: ::windows_core_sys::PCWSTR, pbc: ::win32_system_sys::Com::IBindCtx, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn SHPropStgCreate(psstg: ::win32_system_sys::Com::StructuredStorage::IPropertySetStorage, fmtid: *const ::windows_core_sys::GUID, pclsid: *const ::windows_core_sys::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::win32_system_sys::Com::StructuredStorage::IPropertyStorage, pucodepage: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn SHPropStgReadMultiple(pps: ::win32_system_sys::Com::StructuredStorage::IPropertyStorage, ucodepage: u32, cpspec: u32, rgpspec: *const ::win32_system_sys::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn SHPropStgWriteMultiple(pps: ::win32_system_sys::Com::StructuredStorage::IPropertyStorage, pucodepage: *mut u32, cpspec: u32, rgpspec: *const ::win32_system_sys::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantCompare(var1: *const ::win32_system_sys::Com::VARIANT, var2: *const ::win32_system_sys::Com::VARIANT) -> i32;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetBooleanElem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pfval: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetDoubleElem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetElementCount(varin: *const ::win32_system_sys::Com::VARIANT) -> u32;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetInt16Elem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetInt32Elem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetInt64Elem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetStringElem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, ppszval: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetUInt16Elem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetUInt32Elem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantGetUInt64Elem(var: *const ::win32_system_sys::Com::VARIANT, ielem: u32, pnval: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToBoolean(varin: *const ::win32_system_sys::Com::VARIANT, pfret: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToBooleanArray(var: *const ::win32_system_sys::Com::VARIANT, prgf: *mut ::win32_foundation_sys::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToBooleanArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgf: *mut *mut ::win32_foundation_sys::BOOL, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToBooleanWithDefault(varin: *const ::win32_system_sys::Com::VARIANT, fdefault: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToBuffer(varin: *const ::win32_system_sys::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToDosDateTime(varin: *const ::win32_system_sys::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToDouble(varin: *const ::win32_system_sys::Com::VARIANT, pdblret: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToDoubleArray(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToDoubleArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToDoubleWithDefault(varin: *const ::win32_system_sys::Com::VARIANT, dbldefault: f64) -> f64;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToFileTime(varin: *const ::win32_system_sys::Com::VARIANT, stfout: PSTIME_FLAGS, pftout: *mut ::win32_foundation_sys::FILETIME) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToGUID(varin: *const ::win32_system_sys::Com::VARIANT, pguid: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt16(varin: *const ::win32_system_sys::Com::VARIANT, piret: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt16Array(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt16ArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt16WithDefault(varin: *const ::win32_system_sys::Com::VARIANT, idefault: i16) -> i16;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt32(varin: *const ::win32_system_sys::Com::VARIANT, plret: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt32Array(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt32ArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt32WithDefault(varin: *const ::win32_system_sys::Com::VARIANT, ldefault: i32) -> i32;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt64(varin: *const ::win32_system_sys::Com::VARIANT, pllret: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt64Array(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt64ArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToInt64WithDefault(varin: *const ::win32_system_sys::Com::VARIANT, lldefault: i64) -> i64;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToPropVariant(pvar: *const ::win32_system_sys::Com::VARIANT, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys", feature = "win32-ui-sys"))]
    pub fn VariantToStrRet(varin: *const ::win32_system_sys::Com::VARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToString(varin: *const ::win32_system_sys::Com::VARIANT, pszbuf: ::windows_core_sys::PWSTR, cchbuf: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToStringAlloc(varin: *const ::win32_system_sys::Com::VARIANT, ppszbuf: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToStringArray(var: *const ::win32_system_sys::Com::VARIANT, prgsz: *mut ::windows_core_sys::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToStringArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgsz: *mut *mut ::windows_core_sys::PWSTR, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToStringWithDefault(varin: *const ::win32_system_sys::Com::VARIANT, pszdefault: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::PWSTR;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt16(varin: *const ::win32_system_sys::Com::VARIANT, puiret: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt16Array(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt16ArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt16WithDefault(varin: *const ::win32_system_sys::Com::VARIANT, uidefault: u16) -> u16;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt32(varin: *const ::win32_system_sys::Com::VARIANT, pulret: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt32Array(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt32ArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt32WithDefault(varin: *const ::win32_system_sys::Com::VARIANT, uldefault: u32) -> u32;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt64(varin: *const ::win32_system_sys::Com::VARIANT, pullret: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt64Array(var: *const ::win32_system_sys::Com::VARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt64ArrayAlloc(var: *const ::win32_system_sys::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-system-sys", feature = "win32-system-sys"))]
    pub fn VariantToUInt64WithDefault(varin: *const ::win32_system_sys::Com::VARIANT, ulldefault: u64) -> u64;
    #[cfg(feature = "win32-system-sys")]
    pub fn WinRTPropertyValueToPropVariant(punkpropertyvalue: ::windows_core_sys::IUnknown, ppropvar: *mut ::win32_system_sys::Com::StructuredStorage::PROPVARIANT) -> ::windows_core_sys::HRESULT;
}
pub type DRAWPROGRESSFLAGS = u32;
pub const DPF_NONE: DRAWPROGRESSFLAGS = 0u32;
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = 1u32;
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = 2u32;
pub const DPF_ERROR: DRAWPROGRESSFLAGS = 4u32;
pub const DPF_WARNING: DRAWPROGRESSFLAGS = 8u32;
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = 16u32;
pub type GETPROPERTYSTOREFLAGS = u32;
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = 0u32;
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1u32;
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = 2u32;
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = 4u32;
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 8u32;
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = 16u32;
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = 32u32;
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = 64u32;
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = 128u32;
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = 256u32;
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = 512u32;
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1024u32;
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = 2048u32;
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 4096u32;
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = 8191u32;
pub type ICreateObject = *mut ::core::ffi::c_void;
pub type IDelayedPropertyStoreFactory = *mut ::core::ffi::c_void;
pub type IInitializeWithFile = *mut ::core::ffi::c_void;
pub type IInitializeWithStream = *mut ::core::ffi::c_void;
pub type INamedPropertyStore = *mut ::core::ffi::c_void;
pub type IObjectWithPropertyKey = *mut ::core::ffi::c_void;
pub type IPersistSerializedPropStorage = *mut ::core::ffi::c_void;
pub type IPersistSerializedPropStorage2 = *mut ::core::ffi::c_void;
pub type IPropertyChange = *mut ::core::ffi::c_void;
pub type IPropertyChangeArray = *mut ::core::ffi::c_void;
pub type IPropertyDescription = *mut ::core::ffi::c_void;
pub type IPropertyDescription2 = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionAliasInfo = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionList = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionRelatedPropertyInfo = *mut ::core::ffi::c_void;
pub type IPropertyDescriptionSearchInfo = *mut ::core::ffi::c_void;
pub type IPropertyEnumType = *mut ::core::ffi::c_void;
pub type IPropertyEnumType2 = *mut ::core::ffi::c_void;
pub type IPropertyEnumTypeList = *mut ::core::ffi::c_void;
pub type IPropertyStore = *mut ::core::ffi::c_void;
pub type IPropertyStoreCache = *mut ::core::ffi::c_void;
pub type IPropertyStoreCapabilities = *mut ::core::ffi::c_void;
pub type IPropertyStoreFactory = *mut ::core::ffi::c_void;
pub type IPropertySystem = *mut ::core::ffi::c_void;
pub type IPropertySystemChangeNotify = *mut ::core::ffi::c_void;
pub type IPropertyUI = *mut ::core::ffi::c_void;
pub const InMemoryPropertyStore: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2583879698, data2: 25347, data3: 19998, data4: [185, 161, 99, 15, 128, 37, 146, 197] };
pub const InMemoryPropertyStoreMarshalByValue: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3570011693, data2: 28071, data3: 19317, data4: [169, 124, 95, 48, 111, 14, 174, 220] };
pub type PDOPSTATUS = i32;
pub const PDOPS_RUNNING: PDOPSTATUS = 1i32;
pub const PDOPS_PAUSED: PDOPSTATUS = 2i32;
pub const PDOPS_CANCELLED: PDOPSTATUS = 3i32;
pub const PDOPS_STOPPED: PDOPSTATUS = 4i32;
pub const PDOPS_ERRORS: PDOPSTATUS = 5i32;
pub type PKA_FLAGS = u32;
pub const PKA_SET: PKA_FLAGS = 0u32;
pub const PKA_APPEND: PKA_FLAGS = 1u32;
pub const PKA_DELETE: PKA_FLAGS = 2u32;
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
pub type PLACEHOLDER_STATES = u32;
pub const PS_NONE: PLACEHOLDER_STATES = 0u32;
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = 1u32;
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = 2u32;
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = 4u32;
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = 8u32;
pub const PS_DEFAULT: PLACEHOLDER_STATES = 7u32;
pub const PS_ALL: PLACEHOLDER_STATES = 15u32;
pub type PROPDESC_AGGREGATION_TYPE = i32;
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = 0i32;
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = 1i32;
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = 2i32;
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = 3i32;
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = 4i32;
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = 5i32;
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = 6i32;
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = 7i32;
pub type PROPDESC_COLUMNINDEX_TYPE = i32;
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = 0i32;
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = 1i32;
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = 2i32;
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = 3i32;
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = 4i32;
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = 5i32;
pub type PROPDESC_CONDITION_TYPE = i32;
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = 0i32;
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = 1i32;
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = 2i32;
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = 3i32;
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = 4i32;
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = 5i32;
pub type PROPDESC_DISPLAYTYPE = i32;
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = 0i32;
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = 1i32;
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = 2i32;
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = 3i32;
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = 4i32;
pub type PROPDESC_ENUMFILTER = i32;
pub const PDEF_ALL: PROPDESC_ENUMFILTER = 0i32;
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = 1i32;
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = 2i32;
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = 3i32;
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = 4i32;
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = 5i32;
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = 6i32;
pub type PROPDESC_FORMAT_FLAGS = u32;
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = 0u32;
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = 1u32;
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = 2u32;
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = 4u32;
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = 8u32;
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = 16u32;
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = 32u32;
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = 64u32;
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = 128u32;
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = 256u32;
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = 512u32;
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = 1024u32;
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = 2048u32;
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = 4096u32;
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = 8192u32;
pub type PROPDESC_GROUPING_RANGE = i32;
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = 0i32;
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = 1i32;
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = 2i32;
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = 3i32;
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = 4i32;
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = 5i32;
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = 6i32;
pub type PROPDESC_RELATIVEDESCRIPTION_TYPE = i32;
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = 0i32;
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 1i32;
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 2i32;
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = 3i32;
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 4i32;
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = 5i32;
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 6i32;
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = 7i32;
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 8i32;
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = 9i32;
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = 10i32;
pub type PROPDESC_SEARCHINFO_FLAGS = u32;
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = 0u32;
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = 1u32;
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = 2u32;
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = 4u32;
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = 8u32;
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = 16u32;
pub type PROPDESC_SORTDESCRIPTION = i32;
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = 0i32;
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = 1i32;
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = 2i32;
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = 3i32;
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = 4i32;
pub type PROPDESC_TYPE_FLAGS = u32;
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = 0u32;
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = 1u32;
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = 2u32;
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = 4u32;
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = 8u32;
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = 16u32;
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = 32u32;
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = 64u32;
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = 128u32;
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = 256u32;
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = 512u32;
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = 1024u32;
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = 2048u32;
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = 4096u32;
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = 2147483648u32;
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = 2147491839u32;
pub type PROPDESC_VIEW_FLAGS = u32;
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = 0u32;
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = 1u32;
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = 2u32;
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = 4u32;
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = 8u32;
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = 16u32;
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = 32u32;
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = 64u32;
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = 128u32;
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = 256u32;
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = 512u32;
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = 2048u32;
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = 4096u32;
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = 7167u32;
pub type PROPENUMTYPE = i32;
pub const PET_DISCRETEVALUE: PROPENUMTYPE = 0i32;
pub const PET_RANGEDVALUE: PROPENUMTYPE = 1i32;
pub const PET_DEFAULTVALUE: PROPENUMTYPE = 2i32;
pub const PET_ENDRANGE: PROPENUMTYPE = 3i32;
#[repr(C)]
pub struct PROPERTYKEY {
    pub fmtid: ::windows_core_sys::GUID,
    pub pid: u32,
}
impl ::core::marker::Copy for PROPERTYKEY {}
impl ::core::clone::Clone for PROPERTYKEY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPERTYUI_FLAGS = u32;
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = 0u32;
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = 1u32;
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = 2u32;
pub type PROPERTYUI_FORMAT_FLAGS = u32;
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = 0u32;
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = 1u32;
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = 2u32;
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = 4u32;
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = 8u32;
pub type PROPERTYUI_NAME_FLAGS = u32;
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = 0u32;
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = 1u32;
#[repr(C, packed(1))]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [::win32_foundation_sys::CHAR; 30],
    pub achCmdLine: [::win32_foundation_sys::CHAR; 128],
    pub achWorkDir: [::win32_foundation_sys::CHAR; 64],
    pub wHotKey: u16,
    pub achIconFile: [::win32_foundation_sys::CHAR; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [::win32_foundation_sys::CHAR; 80],
    pub achPIFFile: [::win32_foundation_sys::CHAR; 260],
}
impl ::core::marker::Copy for PROPPRG {}
impl ::core::clone::Clone for PROPPRG {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPVAR_CHANGE_FLAGS = u32;
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = 0u32;
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = 1u32;
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = 2u32;
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = 4u32;
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = 8u32;
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = 16u32;
pub type PROPVAR_COMPARE_FLAGS = u32;
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = 0u32;
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = 1u32;
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = 2u32;
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = 4u32;
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = 8u32;
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = 16u32;
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = 32u32;
pub type PROPVAR_COMPARE_UNIT = i32;
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = 0i32;
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = 1i32;
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = 2i32;
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = 3i32;
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = 4i32;
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = 5i32;
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = 6i32;
pub type PSC_STATE = i32;
pub const PSC_NORMAL: PSC_STATE = 0i32;
pub const PSC_NOTINSOURCE: PSC_STATE = 1i32;
pub const PSC_DIRTY: PSC_STATE = 2i32;
pub const PSC_READONLY: PSC_STATE = 3i32;
pub type PSTIME_FLAGS = u32;
pub const PSTF_UTC: PSTIME_FLAGS = 0u32;
pub const PSTF_LOCAL: PSTIME_FLAGS = 1u32;
pub const PropertySystem: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3096870789, data2: 22702, data3: 20294, data4: [159, 178, 93, 121, 4, 121, 143, 75] };
#[repr(C)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
pub type SYNC_ENGINE_STATE_FLAGS = u32;
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = 0u32;
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = 1u32;
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = 2u32;
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = 4u32;
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = 8u32;
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = 16u32;
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = 32u32;
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = 64u32;
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = 128u32;
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = 256u32;
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = 511u32;
pub type SYNC_TRANSFER_STATUS = u32;
pub const STS_NONE: SYNC_TRANSFER_STATUS = 0u32;
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = 1u32;
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = 2u32;
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = 4u32;
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = 8u32;
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = 16u32;
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = 32u32;
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = 64u32;
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = 128u32;
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = 256u32;
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = 512u32;
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = 1024u32;
pub type _PERSIST_SPROPSTORE_FLAGS = i32;
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = 0i32;
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = 1i32;
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = 2i32;
