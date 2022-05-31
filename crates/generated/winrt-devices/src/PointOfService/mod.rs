#[cfg(feature = "Provider")]
pub mod Provider;
#[repr(transparent)]
pub struct BarcodeScanner(::windows_core::IUnknown);
impl BarcodeScanner {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<BarcodeScannerCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerCapabilities>(result__)
        }
    }
    pub fn ClaimScannerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ClaimedBarcodeScanner>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClaimScannerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ClaimedBarcodeScanner>>(result__)
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHealthAsync)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSupportedSymbologiesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedSymbologiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    pub fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSymbologySupportedAsync)(::windows_core::Interface::as_raw(this), barcodesymbology, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSupportedProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn IsProfileSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, profile: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProfileSupported)(::windows_core::Interface::as_raw(this), profile.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn StatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IBarcodeScannerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows_core::Interface::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IBarcodeScannerStatics<R, F: FnOnce(&IBarcodeScannerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarcodeScannerStatics2<R, F: FnOnce(&IBarcodeScannerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BarcodeScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScanner {}
impl ::core::fmt::Debug for BarcodeScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScanner {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScanner;{bea33e06-b264-4f03-a9c1-45b20f01134f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScanner as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScanner";
}
impl ::core::convert::From<BarcodeScanner> for ::windows_core::IUnknown {
    fn from(value: BarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScanner> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScanner> for ::windows_core::IInspectable {
    fn from(value: BarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScanner> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BarcodeScanner> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScanner) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScanner> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScanner) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for BarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &BarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScanner {}
unsafe impl ::core::marker::Sync for BarcodeScanner {}
#[repr(transparent)]
pub struct BarcodeScannerCapabilities(::windows_core::IUnknown);
impl BarcodeScannerCapabilities {
    pub fn PowerReportingType(&self) -> ::windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosPowerReportingType>::zeroed();
            (::windows_core::Interface::vtable(this).PowerReportingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsImagePreviewSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsImagePreviewSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSoftwareTriggerSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerCapabilities1>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSoftwareTriggerSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVideoPreviewSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoPreviewSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerCapabilities {}
impl ::core::fmt::Debug for BarcodeScannerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerCapabilities;{c60691e4-f2c8-4420-a307-b12ef6622857})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerCapabilities";
}
impl ::core::convert::From<BarcodeScannerCapabilities> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerCapabilities> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerCapabilities> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerCapabilities> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerCapabilities {}
unsafe impl ::core::marker::Sync for BarcodeScannerCapabilities {}
#[repr(transparent)]
pub struct BarcodeScannerDataReceivedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows_core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Report)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerReport>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDataReceivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs;{4234a7e2-ed97-467d-ad2b-01e44313a929})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerDataReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDataReceivedEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerErrorOccurredEventArgs(::windows_core::IUnknown);
impl BarcodeScannerErrorOccurredEventArgs {
    pub fn PartialInputData(&self) -> ::windows_core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PartialInputData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerReport>(result__)
        }
    }
    pub fn IsRetriable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRetriable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ErrorData(&self) -> ::windows_core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerErrorOccurredEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs;{2cd2602f-cf3a-4002-a75a-c5ec468f0a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerErrorOccurredEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs";
}
impl ::core::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerErrorOccurredEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerImagePreviewReceivedEventArgs {
    #[cfg(feature = "winrt-storage")]
    pub fn Preview(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Preview)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerImagePreviewReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerImagePreviewReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerImagePreviewReceivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerImagePreviewReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerImagePreviewReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs;{f3b7de85-6e8b-434e-9f58-06ef26bc4baf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerImagePreviewReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerImagePreviewReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerImagePreviewReceivedEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerReport(::windows_core::IUnknown);
impl BarcodeScannerReport {
    pub fn ScanDataType(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ScanDataType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ScanData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScanData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ScanDataLabel(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScanDataLabel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateInstance<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(scandatatype: u32, scandata: Param1, scandatalabel: Param2) -> ::windows_core::Result<BarcodeScannerReport> {
        Self::IBarcodeScannerReportFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), scandatatype, scandata.into_param().abi(), scandatalabel.into_param().abi(), result__.as_mut_ptr()).from_abi::<BarcodeScannerReport>(result__)
        })
    }
    pub fn IBarcodeScannerReportFactory<R, F: FnOnce(&IBarcodeScannerReportFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BarcodeScannerReport, IBarcodeScannerReportFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BarcodeScannerReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerReport {}
impl ::core::fmt::Debug for BarcodeScannerReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerReport;{5ce4d8b0-a489-4b96-86c4-f0bf8a37753d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerReport";
}
impl ::core::convert::From<BarcodeScannerReport> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerReport> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerReport> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerReport> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerReport {}
unsafe impl ::core::marker::Sync for BarcodeScannerReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for BarcodeScannerStatus {}
impl ::core::clone::Clone for BarcodeScannerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeScannerStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BarcodeScannerStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeScannerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeScannerStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BarcodeScannerStatusUpdatedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<BarcodeScannerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BarcodeScannerStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerStatus>(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs;{355d8586-9c43-462b-a91a-816dc97f452c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerStatusUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStatusUpdatedEventArgs {}
pub struct BarcodeSymbologies;
impl BarcodeSymbologies {
    pub fn Unknown() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Unknown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean8() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean8)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean8Add2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean8Add2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean8Add5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean8Add5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Eanv() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Eanv)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn EanvAdd2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).EanvAdd2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn EanvAdd5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).EanvAdd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean13() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean13)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean13Add2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean13Add2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean13Add5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean13Add5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Isbn() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Isbn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IsbnAdd5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IsbnAdd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ismn() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ismn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IsmnAdd2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IsmnAdd2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IsmnAdd5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IsmnAdd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Issn() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Issn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IssnAdd2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IssnAdd2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IssnAdd5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IssnAdd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean99() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean99)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean99Add2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean99Add2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ean99Add5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ean99Add5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Upca() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Upca)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UpcaAdd2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UpcaAdd2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UpcaAdd5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UpcaAdd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Upce() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Upce)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UpceAdd2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UpceAdd2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UpceAdd5() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UpceAdd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UpcCoupon() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UpcCoupon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TfStd() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TfStd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TfDis() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TfDis)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TfInt() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TfInt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TfInd() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TfInd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TfMat() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TfMat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TfIata() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TfIata)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Gs1DatabarType1() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Gs1DatabarType1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Gs1DatabarType2() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Gs1DatabarType2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Gs1DatabarType3() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Gs1DatabarType3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code39() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code39)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code39Ex() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code39Ex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Trioptic39() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Trioptic39)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code32() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code32)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Pzn() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Pzn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code93() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code93)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code93Ex() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code93Ex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code128() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code128)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Gs1128() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Gs1128)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Gs1128Coupon() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Gs1128Coupon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UccEan128() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UccEan128)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Sisac() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Sisac)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Isbt() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Isbt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Codabar() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Codabar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code11() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code11)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Msi() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Msi)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Plessey() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Plessey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Telepen() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Telepen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code16k() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code16k)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn CodablockA() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CodablockA)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn CodablockF() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CodablockF)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Codablock128() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Codablock128)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Code49() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code49)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Aztec() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Aztec)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn DataCode() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DataCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn DataMatrix() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DataMatrix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn HanXin() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HanXin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Maxicode() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Maxicode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn MicroPdf417() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MicroPdf417)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn MicroQr() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MicroQr)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Pdf417() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Pdf417)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Qr() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Qr)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn MsTag() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MsTag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ccab() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ccab)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ccc() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ccc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Tlc39() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Tlc39)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn AusPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AusPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn CanPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CanPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn ChinaPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ChinaPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn DutchKix() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DutchKix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn InfoMail() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).InfoMail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn ItalianPost25() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ItalianPost25)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn ItalianPost39() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ItalianPost39)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn JapanPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).JapanPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn KoreanPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).KoreanPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn SwedenPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SwedenPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UkPost() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UkPost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UsIntelligent() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UsIntelligent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UsIntelligentPkg() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UsIntelligentPkg)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UsPlanet() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UsPlanet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn UsPostNet() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UsPostNet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Us4StateFics() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Us4StateFics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn OcrA() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OcrA)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn OcrB() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OcrB)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Micr() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Micr)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn ExtendedBase() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedBase)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn GetName(scandatatype: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetName)(::windows_core::Interface::as_raw(this), scandatatype, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Gs1DWCode() -> ::windows_core::Result<u32> {
        Self::IBarcodeSymbologiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Gs1DWCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IBarcodeSymbologiesStatics<R, F: FnOnce(&IBarcodeSymbologiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarcodeSymbologiesStatics2<R, F: FnOnce(&IBarcodeSymbologiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for BarcodeSymbologies {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologies";
}
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(::windows_core::IUnknown);
impl BarcodeSymbologyAttributes {
    pub fn IsCheckDigitValidationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitValidationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCheckDigitValidationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitValidationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCheckDigitTransmissionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitTransmissionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCheckDigitTransmissionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitTransmissionSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DecodeLength1(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DecodeLength1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDecodeLength1(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodeLength1)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecodeLength2(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DecodeLength2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDecodeLength2(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodeLength2)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecodeLengthKind(&self) -> ::windows_core::Result<BarcodeSymbologyDecodeLengthKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BarcodeSymbologyDecodeLengthKind>::zeroed();
            (::windows_core::Interface::vtable(this).DecodeLengthKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeSymbologyDecodeLengthKind>(result__)
        }
    }
    pub fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodeLengthKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeLengthSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecodeLengthSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeSymbologyAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeSymbologyAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeSymbologyAttributes {}
impl ::core::fmt::Debug for BarcodeSymbologyAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeSymbologyAttributes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeSymbologyAttributes;{66413a78-ab7a-4ada-8ece-936014b2ead7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeSymbologyAttributes as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologyAttributes";
}
impl ::core::convert::From<BarcodeSymbologyAttributes> for ::windows_core::IUnknown {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributes> for ::windows_core::IUnknown {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeSymbologyAttributes> for ::windows_core::IInspectable {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributes> for ::windows_core::IInspectable {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributes {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributes {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: Self = Self(0i32);
    pub const Discrete: Self = Self(1i32);
    pub const Range: Self = Self(2i32);
}
impl ::core::marker::Copy for BarcodeSymbologyDecodeLengthKind {}
impl ::core::clone::Clone for BarcodeSymbologyDecodeLengthKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeSymbologyDecodeLengthKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BarcodeSymbologyDecodeLengthKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeSymbologyDecodeLengthKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyDecodeLengthKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeSymbologyDecodeLengthKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeSymbologyDecodeLengthKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CashDrawer(::windows_core::IUnknown);
impl CashDrawer {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<CashDrawerCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawerCapabilities>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawerStatus>(result__)
        }
    }
    pub fn IsDrawerOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDrawerOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DrawerEventSource(&self) -> ::windows_core::Result<CashDrawerEventSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DrawerEventSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawerEventSource>(result__)
        }
    }
    pub fn ClaimDrawerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ClaimedCashDrawer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClaimDrawerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ClaimedCashDrawer>>(result__)
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHealthAsync)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn StatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICashDrawerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows_core::Interface::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ICashDrawerStatics<R, F: FnOnce(&ICashDrawerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CashDrawer, ICashDrawerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICashDrawerStatics2<R, F: FnOnce(&ICashDrawerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CashDrawer, ICashDrawerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CashDrawer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawer {}
impl ::core::fmt::Debug for CashDrawer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawer;{9f88f5c8-de54-4aee-a890-920bcbfe30fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawer {
    type Vtable = ICashDrawer_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawer";
}
impl ::core::convert::From<CashDrawer> for ::windows_core::IUnknown {
    fn from(value: CashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawer> for ::windows_core::IUnknown {
    fn from(value: &CashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawer> for ::windows_core::IInspectable {
    fn from(value: CashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawer> for ::windows_core::IInspectable {
    fn from(value: &CashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CashDrawer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: CashDrawer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &CashDrawer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for CashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &CashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CashDrawer {}
unsafe impl ::core::marker::Sync for CashDrawer {}
#[repr(transparent)]
pub struct CashDrawerCapabilities(::windows_core::IUnknown);
impl CashDrawerCapabilities {
    pub fn PowerReportingType(&self) -> ::windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosPowerReportingType>::zeroed();
            (::windows_core::Interface::vtable(this).PowerReportingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatusReportingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatusReportingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatusMultiDrawerDetectSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDrawerOpenSensorAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDrawerOpenSensorAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerCapabilities {}
impl ::core::fmt::Debug for CashDrawerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCapabilities;{0bc6de0b-e8e7-4b1f-b1d1-3e501ad08247})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCapabilities";
}
impl ::core::convert::From<CashDrawerCapabilities> for ::windows_core::IUnknown {
    fn from(value: CashDrawerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCapabilities> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerCapabilities> for ::windows_core::IInspectable {
    fn from(value: CashDrawerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCapabilities> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerCapabilities {}
unsafe impl ::core::marker::Sync for CashDrawerCapabilities {}
#[repr(transparent)]
pub struct CashDrawerCloseAlarm(::windows_core::IUnknown);
impl CashDrawerCloseAlarm {
    pub fn SetAlarmTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlarmTimeout)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AlarmTimeout(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AlarmTimeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetBeepFrequency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBeepFrequency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BeepFrequency(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BeepFrequency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetBeepDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBeepDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeepDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).BeepDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetBeepDelay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBeepDelay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeepDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).BeepDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn AlarmTimeoutExpired<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AlarmTimeoutExpired)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAlarmTimeoutExpired<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAlarmTimeoutExpired)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerCloseAlarm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerCloseAlarm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerCloseAlarm {}
impl ::core::fmt::Debug for CashDrawerCloseAlarm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerCloseAlarm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerCloseAlarm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCloseAlarm;{6bf88cc7-6f63-430e-ab3b-95d75ffbe87f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerCloseAlarm as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCloseAlarm";
}
impl ::core::convert::From<CashDrawerCloseAlarm> for ::windows_core::IUnknown {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCloseAlarm> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerCloseAlarm> for ::windows_core::IInspectable {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCloseAlarm> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerCloseAlarm {}
unsafe impl ::core::marker::Sync for CashDrawerCloseAlarm {}
#[repr(transparent)]
pub struct CashDrawerClosedEventArgs(::windows_core::IUnknown);
impl CashDrawerClosedEventArgs {
    pub fn CashDrawer(&self) -> ::windows_core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CashDrawer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawer>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerClosedEventArgs {}
impl ::core::fmt::Debug for CashDrawerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerClosedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerClosedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerEventSourceEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerClosedEventArgs";
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CashDrawerClosedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CashDrawerClosedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::core::convert::TryInto::<ICashDrawerEventSourceEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerClosedEventArgs {}
#[repr(transparent)]
pub struct CashDrawerEventSource(::windows_core::IUnknown);
impl CashDrawerEventSource {
    pub fn DrawerClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DrawerClosed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDrawerClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDrawerClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DrawerOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DrawerOpened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDrawerOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDrawerOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CashDrawerEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerEventSource {}
impl ::core::fmt::Debug for CashDrawerEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerEventSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerEventSource;{e006e46c-f2f9-442f-8dd6-06c10a4227ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerEventSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerEventSource";
}
impl ::core::convert::From<CashDrawerEventSource> for ::windows_core::IUnknown {
    fn from(value: CashDrawerEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerEventSource> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerEventSource> for ::windows_core::IInspectable {
    fn from(value: CashDrawerEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerEventSource> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerEventSource {}
unsafe impl ::core::marker::Sync for CashDrawerEventSource {}
#[repr(transparent)]
pub struct CashDrawerOpenedEventArgs(::windows_core::IUnknown);
impl CashDrawerOpenedEventArgs {
    pub fn CashDrawer(&self) -> ::windows_core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CashDrawer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawer>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerOpenedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerOpenedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerOpenedEventArgs {}
impl ::core::fmt::Debug for CashDrawerOpenedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerOpenedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerOpenedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerOpenedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerOpenedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerEventSourceEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerOpenedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerOpenedEventArgs";
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CashDrawerOpenedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CashDrawerOpenedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::core::convert::TryInto::<ICashDrawerEventSourceEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CashDrawerOpenedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerOpenedEventArgs {}
#[repr(transparent)]
pub struct CashDrawerStatus(::windows_core::IUnknown);
impl CashDrawerStatus {
    pub fn StatusKind(&self) -> ::windows_core::Result<CashDrawerStatusKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CashDrawerStatusKind>::zeroed();
            (::windows_core::Interface::vtable(this).StatusKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawerStatusKind>(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerStatus {}
impl ::core::fmt::Debug for CashDrawerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatus;{6bbd78bf-dca1-4e06-99eb-5af6a5aec108})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatus";
}
impl ::core::convert::From<CashDrawerStatus> for ::windows_core::IUnknown {
    fn from(value: CashDrawerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatus> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerStatus> for ::windows_core::IInspectable {
    fn from(value: CashDrawerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatus> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerStatus {}
unsafe impl ::core::marker::Sync for CashDrawerStatus {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for CashDrawerStatusKind {}
impl ::core::clone::Clone for CashDrawerStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CashDrawerStatusKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CashDrawerStatusKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CashDrawerStatusKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatusKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerStatusKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.CashDrawerStatusKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CashDrawerStatusUpdatedEventArgs(::windows_core::IUnknown);
impl CashDrawerStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawerStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for CashDrawerStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs;{30aae98a-0d70-459c-9553-87e124c52488})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICashDrawerStatusUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs";
}
impl ::core::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerStatusUpdatedEventArgs {}
#[repr(transparent)]
pub struct ClaimedBarcodeScanner(::windows_core::IUnknown);
impl ClaimedBarcodeScanner {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDisabledOnDataReceived)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDisabledOnDataReceived(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledOnDataReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecodeDataEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeDataEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecodeDataEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EnableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DisableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn RetainDevice(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RetainDevice)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetActiveSymbologiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<u32>>>(&self, symbologies: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetActiveSymbologiesAsync)(::windows_core::Interface::as_raw(this), symbologies.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(&self, statistics: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStatisticsAsync)(::windows_core::Interface::as_raw(this), statistics.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetActiveProfileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, profile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetActiveProfileAsync)(::windows_core::Interface::as_raw(this), profile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DataReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDataReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TriggerPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TriggerPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTriggerPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTriggerPressed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TriggerReleased<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TriggerReleased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTriggerReleased<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTriggerReleased)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ImagePreviewReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ImagePreviewReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImagePreviewReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveImagePreviewReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorOccurred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveErrorOccurred)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StartSoftwareTriggerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartSoftwareTriggerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn StopSoftwareTriggerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopSoftwareTriggerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeSymbologyAttributes>> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSymbologyAttributesAsync)(::windows_core::Interface::as_raw(this), barcodesymbology, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeSymbologyAttributes>>(result__)
        }
    }
    pub fn SetSymbologyAttributesAsync<'a, Param1: ::windows_core::IntoParam<'a, BarcodeSymbologyAttributes>>(&self, barcodesymbology: u32, attributes: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetSymbologyAttributesAsync)(::windows_core::Interface::as_raw(this), barcodesymbology, attributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ShowVideoPreviewAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowVideoPreviewAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn HideVideoPreview(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).HideVideoPreview)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVideoPreviewShownOnEnable)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVideoPreviewShownOnEnable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoPreviewShownOnEnable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedBarcodeScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedBarcodeScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedBarcodeScanner {}
impl ::core::fmt::Debug for ClaimedBarcodeScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedBarcodeScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedBarcodeScanner {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScanner;{4a63b49c-8fa4-4332-bb26-945d11d81e0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedBarcodeScanner as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScanner";
}
impl ::core::convert::From<ClaimedBarcodeScanner> for ::windows_core::IUnknown {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScanner> for ::windows_core::IUnknown {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedBarcodeScanner> for ::windows_core::IInspectable {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScanner> for ::windows_core::IInspectable {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedBarcodeScanner> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedBarcodeScanner) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedBarcodeScanner> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedBarcodeScanner) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedBarcodeScanner {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScanner {}
#[repr(transparent)]
pub struct ClaimedBarcodeScannerClosedEventArgs(::windows_core::IUnknown);
impl ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedBarcodeScannerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedBarcodeScannerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedBarcodeScannerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedBarcodeScannerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs;{cf7d5489-a22c-4c65-a901-88d77d833954})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedBarcodeScannerClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs";
}
impl ::core::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScannerClosedEventArgs {}
#[repr(transparent)]
pub struct ClaimedCashDrawer(::windows_core::IUnknown);
impl ClaimedCashDrawer {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDrawerOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDrawerOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CloseAlarm(&self) -> ::windows_core::Result<CashDrawerCloseAlarm> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloseAlarm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawerCloseAlarm>(result__)
        }
    }
    pub fn OpenDrawerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenDrawerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn EnableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DisableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RetainDeviceAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetainDeviceAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(&self, statistics: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStatisticsAsync)(::windows_core::Interface::as_raw(this), statistics.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedCashDrawer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedCashDrawer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedCashDrawer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedCashDrawer {}
impl ::core::fmt::Debug for ClaimedCashDrawer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedCashDrawer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedCashDrawer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawer;{ca3f99af-abb8-42c1-8a84-5c66512f5a75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedCashDrawer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawer";
}
impl ::core::convert::From<ClaimedCashDrawer> for ::windows_core::IUnknown {
    fn from(value: ClaimedCashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawer> for ::windows_core::IUnknown {
    fn from(value: &ClaimedCashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedCashDrawer> for ::windows_core::IInspectable {
    fn from(value: ClaimedCashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawer> for ::windows_core::IInspectable {
    fn from(value: &ClaimedCashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedCashDrawer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedCashDrawer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedCashDrawer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedCashDrawer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ClaimedCashDrawer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedCashDrawer {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawer {}
#[repr(transparent)]
pub struct ClaimedCashDrawerClosedEventArgs(::windows_core::IUnknown);
impl ClaimedCashDrawerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedCashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedCashDrawerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedCashDrawerClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedCashDrawerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedCashDrawerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs;{cc573f33-3f34-4c5c-baae-deadf16cd7fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedCashDrawerClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs";
}
impl ::core::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawerClosedEventArgs {}
#[repr(transparent)]
pub struct ClaimedJournalPrinter(::windows_core::IUnknown);
impl ClaimedJournalPrinter {
    pub fn CreateJob(&self) -> ::windows_core::Result<JournalPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateJob)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JournalPrintJob>(result__)
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCharactersPerLine)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLineSpacing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineSpacing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLetterQuality)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLetterQuality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorCartridge)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows_core::Result<PosPrinterColorCartridge> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCartridge>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCoverOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeRemoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHeadCleaning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadyToPrint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ValidateData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateData)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedJournalPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedJournalPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedJournalPrinter {}
impl ::core::fmt::Debug for ClaimedJournalPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedJournalPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedJournalPrinter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedJournalPrinter;{67ea0630-517d-487f-9fdf-d2e0a0a264a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedJournalPrinter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedJournalPrinter";
}
impl ::core::convert::From<ClaimedJournalPrinter> for ::windows_core::IUnknown {
    fn from(value: ClaimedJournalPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedJournalPrinter> for ::windows_core::IUnknown {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedJournalPrinter> for ::windows_core::IInspectable {
    fn from(value: ClaimedJournalPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedJournalPrinter> for ::windows_core::IInspectable {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedJournalPrinter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedJournalPrinter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedJournalPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::core::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedJournalPrinter {}
unsafe impl ::core::marker::Sync for ClaimedJournalPrinter {}
#[repr(transparent)]
pub struct ClaimedLineDisplay(::windows_core::IUnknown);
impl ClaimedLineDisplay {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    pub fn PhysicalDeviceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PhysicalDeviceDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceControlDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceControlDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceControlVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceControlVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceServiceVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceServiceVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DefaultWindow(&self) -> ::windows_core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultWindow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayWindow>(result__)
        }
    }
    pub fn RetainDevice(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RetainDevice)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedLineDisplay, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHealthAsync)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn CheckPowerStatusAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckPowerStatusAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    pub fn StatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedScreenSizesInCharacters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Size>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedScreenSizesInCharacters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Size>>(result__)
        }
    }
    pub fn MaxBitmapSizeInPixels(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBitmapSizeInPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharacterSets(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<i32>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharacterSets)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    pub fn CustomGlyphs(&self) -> ::windows_core::Result<LineDisplayCustomGlyphs> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CustomGlyphs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayCustomGlyphs>(result__)
        }
    }
    pub fn GetAttributes(&self) -> ::windows_core::Result<LineDisplayAttributes> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayAttributes>(result__)
        }
    }
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows_core::IntoParam<'a, LineDisplayAttributes>>(&self, attributes: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateAttributesAsync)(::windows_core::Interface::as_raw(this), attributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetDescriptorAsync)(::windows_core::Interface::as_raw(this), descriptor, descriptorstate, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryClearDescriptorsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryClearDescriptorsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryCreateWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, viewport: Param0, windowsize: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplayWindow>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateWindowAsync)(::windows_core::Interface::as_raw(this), viewport.into_param().abi(), windowsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplayWindow>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryStoreStorageFileBitmapAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, bitmap: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryStoreStorageFileBitmapAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryStoreStorageFileBitmapWithAlignmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryStoreStorageFileBitmapWithAlignmentAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryStoreStorageFileBitmapWithAlignmentAndWidthAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ClaimedLineDisplay>> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows_core::Interface::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IClaimedLineDisplayStatics<R, F: FnOnce(&IClaimedLineDisplayStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ClaimedLineDisplay, IClaimedLineDisplayStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ClaimedLineDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedLineDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedLineDisplay {}
impl ::core::fmt::Debug for ClaimedLineDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedLineDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedLineDisplay {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplay;{120ac970-9a75-4acf-aae7-09972bcf8794})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedLineDisplay as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplay";
}
impl ::core::convert::From<ClaimedLineDisplay> for ::windows_core::IUnknown {
    fn from(value: ClaimedLineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplay> for ::windows_core::IUnknown {
    fn from(value: &ClaimedLineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedLineDisplay> for ::windows_core::IInspectable {
    fn from(value: ClaimedLineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplay> for ::windows_core::IInspectable {
    fn from(value: &ClaimedLineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedLineDisplay> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedLineDisplay) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedLineDisplay> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedLineDisplay) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ClaimedLineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedLineDisplay {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplay {}
#[repr(transparent)]
pub struct ClaimedLineDisplayClosedEventArgs(::windows_core::IUnknown);
impl ClaimedLineDisplayClosedEventArgs {}
impl ::core::clone::Clone for ClaimedLineDisplayClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedLineDisplayClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedLineDisplayClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedLineDisplayClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedLineDisplayClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs;{f915f364-d3d5-4f10-b511-90939edfacd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedLineDisplayClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs";
}
impl ::core::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplayClosedEventArgs {}
#[repr(transparent)]
pub struct ClaimedMagneticStripeReader(::windows_core::IUnknown);
impl ClaimedMagneticStripeReader {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDisabledOnDataReceived)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDisabledOnDataReceived(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledOnDataReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecodeDataEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeDataEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecodeDataEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDeviceAuthenticated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDeviceAuthenticated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataEncryptionAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataEncryptionAlgorithm(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DataEncryptionAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTracksToRead)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TracksToRead(&self) -> ::windows_core::Result<MagneticStripeReaderTrackIds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderTrackIds>::zeroed();
            (::windows_core::Interface::vtable(this).TracksToRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackIds>(result__)
        }
    }
    pub fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsTransmitSentinelsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTransmitSentinelsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTransmitSentinelsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EnableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DisableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn RetainDevice(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RetainDevice)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorReportingType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveDeviceAuthenticationDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
    pub fn AuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticateDeviceAsync)(::windows_core::Interface::as_raw(this), responsetoken.len() as u32, ::core::mem::transmute(responsetoken.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeAuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeAuthenticateDeviceAsync)(::windows_core::Interface::as_raw(this), responsetoken.len() as u32, ::core::mem::transmute(responsetoken.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn UpdateKeyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0, keyname: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateKeyAsync)(::windows_core::Interface::as_raw(this), key.into_param().abi(), keyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(&self, statistics: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStatisticsAsync)(::windows_core::Interface::as_raw(this), statistics.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn BankCardDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BankCardDataReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBankCardDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBankCardDataReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AamvaCardDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AamvaCardDataReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAamvaCardDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAamvaCardDataReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VendorSpecificDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VendorSpecificDataReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVendorSpecificDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVendorSpecificDataReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ClaimedMagneticStripeReader>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorOccurred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveErrorOccurred)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedMagneticStripeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedMagneticStripeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedMagneticStripeReader {}
impl ::core::fmt::Debug for ClaimedMagneticStripeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedMagneticStripeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedMagneticStripeReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReader;{475ca8f3-9417-48bc-b9d7-4163a7844c02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedMagneticStripeReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReader";
}
impl ::core::convert::From<ClaimedMagneticStripeReader> for ::windows_core::IUnknown {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReader> for ::windows_core::IUnknown {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedMagneticStripeReader> for ::windows_core::IInspectable {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReader> for ::windows_core::IInspectable {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedMagneticStripeReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedMagneticStripeReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedMagneticStripeReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedMagneticStripeReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReader {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReader {}
#[repr(transparent)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(::windows_core::IUnknown);
impl ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::clone::Clone for ClaimedMagneticStripeReaderClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedMagneticStripeReaderClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedMagneticStripeReaderClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedMagneticStripeReaderClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs;{14ada93a-adcd-4c80-acda-c3eaed2647e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedMagneticStripeReaderClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs";
}
impl ::core::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReaderClosedEventArgs {}
#[repr(transparent)]
pub struct ClaimedPosPrinter(::windows_core::IUnknown);
impl ClaimedPosPrinter {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCharacterSet(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharacterSet)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCoverOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCharacterSetMappingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCharacterSetMappingEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMapMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MapMode(&self) -> ::windows_core::Result<PosPrinterMapMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterMapMode>::zeroed();
            (::windows_core::Interface::vtable(this).MapMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterMapMode>(result__)
        }
    }
    pub fn Receipt(&self) -> ::windows_core::Result<ClaimedReceiptPrinter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Receipt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClaimedReceiptPrinter>(result__)
        }
    }
    pub fn Slip(&self) -> ::windows_core::Result<ClaimedSlipPrinter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Slip)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClaimedSlipPrinter>(result__)
        }
    }
    pub fn Journal(&self) -> ::windows_core::Result<ClaimedJournalPrinter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Journal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClaimedJournalPrinter>(result__)
        }
    }
    pub fn EnableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DisableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RetainDeviceAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetainDeviceAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(&self, statistics: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStatisticsAsync)(::windows_core::Interface::as_raw(this), statistics.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedPosPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedPosPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedPosPrinter {}
impl ::core::fmt::Debug for ClaimedPosPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedPosPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedPosPrinter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinter;{6d64ce0c-e03e-4b14-a38e-c28c34b86353})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedPosPrinter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinter";
}
impl ::core::convert::From<ClaimedPosPrinter> for ::windows_core::IUnknown {
    fn from(value: ClaimedPosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinter> for ::windows_core::IUnknown {
    fn from(value: &ClaimedPosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedPosPrinter> for ::windows_core::IInspectable {
    fn from(value: ClaimedPosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinter> for ::windows_core::IInspectable {
    fn from(value: &ClaimedPosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedPosPrinter> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedPosPrinter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedPosPrinter> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedPosPrinter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ClaimedPosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedPosPrinter {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinter {}
#[repr(transparent)]
pub struct ClaimedPosPrinterClosedEventArgs(::windows_core::IUnknown);
impl ClaimedPosPrinterClosedEventArgs {}
impl ::core::clone::Clone for ClaimedPosPrinterClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedPosPrinterClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedPosPrinterClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedPosPrinterClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedPosPrinterClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs;{e2b7a27b-4d40-471d-92ed-63375b18c788})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedPosPrinterClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs";
}
impl ::core::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinterClosedEventArgs {}
#[repr(transparent)]
pub struct ClaimedReceiptPrinter(::windows_core::IUnknown);
impl ClaimedReceiptPrinter {
    pub fn SidewaysMaxLines(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidewaysMaxLines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidewaysMaxChars(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidewaysMaxChars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn LinesToPaperCut(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LinesToPaperCut)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PageSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).PageSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn PrintArea(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).PrintArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn CreateJob(&self) -> ::windows_core::Result<ReceiptPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateJob)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ReceiptPrintJob>(result__)
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCharactersPerLine)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLineSpacing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineSpacing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLetterQuality)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLetterQuality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorCartridge)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows_core::Result<PosPrinterColorCartridge> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCartridge>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCoverOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeRemoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHeadCleaning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadyToPrint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ValidateData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateData)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedReceiptPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedReceiptPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedReceiptPrinter {}
impl ::core::fmt::Debug for ClaimedReceiptPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedReceiptPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedReceiptPrinter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedReceiptPrinter;{9ad27a74-dd61-4ee2-9837-5b5d72d538b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedReceiptPrinter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedReceiptPrinter";
}
impl ::core::convert::From<ClaimedReceiptPrinter> for ::windows_core::IUnknown {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedReceiptPrinter> for ::windows_core::IUnknown {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedReceiptPrinter> for ::windows_core::IInspectable {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedReceiptPrinter> for ::windows_core::IInspectable {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedReceiptPrinter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedReceiptPrinter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::core::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedReceiptPrinter {}
unsafe impl ::core::marker::Sync for ClaimedReceiptPrinter {}
#[repr(transparent)]
pub struct ClaimedSlipPrinter(::windows_core::IUnknown);
impl ClaimedSlipPrinter {
    pub fn SidewaysMaxLines(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidewaysMaxLines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidewaysMaxChars(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidewaysMaxChars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxLines(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxLines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn LinesNearEndToEnd(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LinesNearEndToEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PrintSide(&self) -> ::windows_core::Result<PosPrinterPrintSide> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterPrintSide>::zeroed();
            (::windows_core::Interface::vtable(this).PrintSide)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterPrintSide>(result__)
        }
    }
    pub fn PageSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).PageSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn PrintArea(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).PrintArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn OpenJaws(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OpenJaws)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CloseJaws(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CloseJaws)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InsertSlipAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InsertSlipAsync)(::windows_core::Interface::as_raw(this), timeout.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RemoveSlipAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveSlipAsync)(::windows_core::Interface::as_raw(this), timeout.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ChangePrintSide)(::windows_core::Interface::as_raw(this), printside).ok() }
    }
    pub fn CreateJob(&self) -> ::windows_core::Result<SlipPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateJob)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SlipPrintJob>(result__)
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCharactersPerLine)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLineSpacing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineSpacing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLetterQuality)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLetterQuality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorCartridge)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows_core::Result<PosPrinterColorCartridge> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCartridge>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCoverOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeRemoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHeadCleaning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadyToPrint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ValidateData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateData)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedSlipPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedSlipPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedSlipPrinter {}
impl ::core::fmt::Debug for ClaimedSlipPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedSlipPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClaimedSlipPrinter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedSlipPrinter;{bd5deff2-af90-4e8a-b77b-e3ae9ca63a7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
    const IID: ::windows_core::GUID = <IClaimedSlipPrinter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedSlipPrinter";
}
impl ::core::convert::From<ClaimedSlipPrinter> for ::windows_core::IUnknown {
    fn from(value: ClaimedSlipPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedSlipPrinter> for ::windows_core::IUnknown {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedSlipPrinter> for ::windows_core::IInspectable {
    fn from(value: ClaimedSlipPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedSlipPrinter> for ::windows_core::IInspectable {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows_core::Error;
    fn try_from(value: ClaimedSlipPrinter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows_core::Error;
    fn try_from(value: &ClaimedSlipPrinter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedSlipPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::core::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedSlipPrinter {}
unsafe impl ::core::marker::Sync for ClaimedSlipPrinter {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScanner(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbea33e06_b264_4f03_a9c1_45b20f01134f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClaimScannerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetSupportedSymbologiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSupportedSymbologiesAsync: usize,
    pub IsSymbologySupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    RetrieveStatisticsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSupportedProfiles: usize,
    pub IsProfileSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScanner2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScanner2 {
    type Vtable = IBarcodeScanner2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89215167_8cee_436d_89ab_8dfb43bb4286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc60691e4_f2c8_4420_a307_b12ef6622857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsImagePreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities1(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerCapabilities1 {
    type Vtable = IBarcodeScannerCapabilities1_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e5ab3e9_0e2c_472f_a1cc_ee8054b6a684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSoftwareTriggerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerCapabilities2 {
    type Vtable = IBarcodeScannerCapabilities2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf211cfec_e1a1_4ea8_9abc_92b1596270ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsVideoPreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDataReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4234a7e2_ed97_467d_ad2b_01e44313a929);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerErrorOccurredEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2cd2602f_cf3a_4002_a75a_c5ec468f0a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PartialInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsRetriable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3b7de85_6e8b_434e_9f58_06ef26bc4baf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Preview: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ce4d8b0_a489_4b96_86c4_f0bf8a37753d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScanDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub ScanData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ScanData: usize,
    #[cfg(feature = "winrt-storage")]
    pub ScanDataLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ScanDataLabel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerReportFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerReportFactory {
    type Vtable = IBarcodeScannerReportFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2547326_2013_457c_8963_49c15dca78ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scandatatype: u32, scandata: ::windows_core::RawPtr, scandatalabel: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStatics {
    type Vtable = IBarcodeScannerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d115f6f_da49_41e8_8c8c_f0cb62a9c4fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStatics2 {
    type Vtable = IBarcodeScannerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8652473_a36f_4007_b1d0_279ebe92a656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x355d8586_9c43_462b_a91a_816dc97f452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BarcodeScannerStatus) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeSymbologiesStatics {
    type Vtable = IBarcodeSymbologiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca8549bb_06d2_43f4_a44b_c620679fd8d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean8Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean8Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Eanv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub EanvAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub EanvAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean13Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean13Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Isbn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsbnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ismn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsmnAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsmnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Issn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IssnAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IssnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean99: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean99Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ean99Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Upca: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UpcaAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UpcaAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Upce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UpceAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UpceAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UpcCoupon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TfStd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TfDis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TfInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TfInd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TfMat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TfIata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Gs1DatabarType1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Gs1DatabarType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Gs1DatabarType3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code39Ex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Trioptic39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Pzn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code93: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code93Ex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Gs1128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Gs1128Coupon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UccEan128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Sisac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Isbt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Codabar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Msi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Plessey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Telepen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code16k: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CodablockA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CodablockF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Codablock128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Code49: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Aztec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DataCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DataMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub HanXin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Maxicode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MicroPdf417: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MicroQr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Pdf417: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Qr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MsTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ccab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ccc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Tlc39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AusPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ChinaPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DutchKix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub InfoMail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ItalianPost25: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ItalianPost39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub JapanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub KoreanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SwedenPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UkPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsIntelligent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsIntelligentPkg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsPlanet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsPostNet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Us4StateFics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OcrA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OcrB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Micr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scandatatype: u32, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeSymbologiesStatics2 {
    type Vtable = IBarcodeSymbologiesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b7518f4_99d0_40bf_9424_b91d6dd4c6e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Gs1DWCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributes(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66413a78_ab7a_4ada_8ece_936014b2ead7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DecodeLength1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDecodeLength1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DecodeLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDecodeLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DecodeLengthKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows_core::HRESULT,
    pub SetDecodeLengthKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BarcodeSymbologyDecodeLengthKind) -> ::windows_core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawer {
    type Vtable = ICashDrawer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f88f5c8_de54_4aee_a890_920bcbfe30fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DrawerEventSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClaimDrawerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStatisticsAsync: usize,
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bc6de0b_e8e7_4b1f_b1d1_3e501ad08247);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatusReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatusMultiDrawerDetectSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDrawerOpenSensorAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerCloseAlarm(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bf88cc7_6f63_430e_ab3b_95d75ffbe87f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAlarmTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AlarmTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetBeepFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BeepFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetBeepDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub BeepDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetBeepDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub BeepDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AlarmTimeoutExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAlarmTimeoutExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerEventSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe006e46c_f2f9_442f_8dd6_06c10a4227ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DrawerClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDrawerClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DrawerOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDrawerOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICashDrawerEventSourceEventArgs(::windows_core::IUnknown);
impl ICashDrawerEventSourceEventArgs {
    pub fn CashDrawer(&self) -> ::windows_core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CashDrawer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CashDrawer>(result__)
        }
    }
}
impl ::core::convert::From<ICashDrawerEventSourceEventArgs> for ::windows_core::IUnknown {
    fn from(value: ICashDrawerEventSourceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICashDrawerEventSourceEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ICashDrawerEventSourceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICashDrawerEventSourceEventArgs> for ::windows_core::IInspectable {
    fn from(value: ICashDrawerEventSourceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICashDrawerEventSourceEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ICashDrawerEventSourceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICashDrawerEventSourceEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICashDrawerEventSourceEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICashDrawerEventSourceEventArgs {}
impl ::core::fmt::Debug for ICashDrawerEventSourceEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICashDrawerEventSourceEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICashDrawerEventSourceEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{69cb3bc1-147f-421c-9c23-090123bb786c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICashDrawerEventSourceEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69cb3bc1_147f_421c_9c23_090123bb786c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSourceEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CashDrawer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerStatics {
    type Vtable = ICashDrawerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfa0955a_d437_4fff_b547_dda969a4f883);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerStatics2 {
    type Vtable = ICashDrawerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e818121_8c42_40e8_9c0e_40297048104c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bbd78bf_dca1_4e06_99eb_5af6a5aec108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StatusKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CashDrawerStatusKind) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatusUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30aae98a_0d70_459c_9553_87e124c52488);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a63b49c_8fa4_4332_bb26_945d11d81e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SetActiveSymbologiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbologies: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetActiveSymbologiesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UpdateStatisticsAsync: usize,
    pub SetActiveProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TriggerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTriggerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TriggerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTriggerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ImagePreviewReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveImagePreviewReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner1(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedBarcodeScanner1 {
    type Vtable = IClaimedBarcodeScanner1_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf61aad0c_8551_42b4_998c_970c20210a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedBarcodeScanner2 {
    type Vtable = IClaimedBarcodeScanner2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3b59e8c_2d8b_4f70_8af3_3448bedd5fe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, attributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedBarcodeScanner3 {
    type Vtable = IClaimedBarcodeScanner3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6ceb430_712e_45fc_8b86_cd55f5aef79d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShowVideoPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HideVideoPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetIsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedBarcodeScanner4 {
    type Vtable = IClaimedBarcodeScanner4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d501f97_376a_41a8_a230_2f37c1949dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScannerClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf7d5489_a22c_4c65_a901_88d77d833954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca3f99af_abb8_42c1_8a84_5c66512f5a75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CloseAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenDrawerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UpdateStatisticsAsync: usize,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedCashDrawer2 {
    type Vtable = IClaimedCashDrawer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cbab5a2_de42_4d5b_b0c1_9b57a2ba89c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawerClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc573f33_3f34_4c5c_baae_deadf16cd7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedJournalPrinter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67ea0630_517d_487f_9fdf_d2e0a0a264a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x120ac970_9a75_4acf_aae7_09972bcf8794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedLineDisplay2 {
    type Vtable = IClaimedLineDisplay2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa31c75ed_41f5_4e76_a074_795e47a46e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStatisticsAsync: usize,
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedScreenSizesInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedScreenSizesInCharacters: usize,
    pub MaxBitmapSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedCharacterSets: usize,
    pub CustomGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySetDescriptorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryClearDescriptorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryCreateWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::winrt_foundation::Rect, windowsize: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub TryStoreStorageFileBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryStoreStorageFileBitmapAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryStoreStorageFileBitmapWithAlignmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryStoreStorageFileBitmapWithAlignmentAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedLineDisplay3 {
    type Vtable = IClaimedLineDisplay3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x642ecd92_e9d4_4ecc_af75_329c274cd18f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplayClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf915f364_d3d5_4f10_b511_90939edfacd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplayStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedLineDisplayStatics {
    type Vtable = IClaimedLineDisplayStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78ca98fb_8b6b_4973_86f0_3e570c351825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x475ca8f3_9417_48bc_b9d7_4163a7844c02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDeviceAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTracksToRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderTrackIds) -> ::windows_core::HRESULT,
    pub TracksToRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackIds) -> ::windows_core::HRESULT,
    pub SetIsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetErrorReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderErrorReportingType) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub RetrieveDeviceAuthenticationDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    RetrieveDeviceAuthenticationDataAsync: usize,
    pub AuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeAuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, keyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UpdateStatisticsAsync: usize,
    pub BankCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBankCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AamvaCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAamvaCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub VendorSpecificDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVendorSpecificDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedMagneticStripeReader2 {
    type Vtable = IClaimedMagneticStripeReader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x236fafdf_e2dc_4d7d_9c78_060df2bf2928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14ada93a_adcd_4c80_acda_c3eaed2647e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d64ce0c_e03e_4b14_a38e_c28c34b86353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetMapMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterMapMode) -> ::windows_core::HRESULT,
    pub MapMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMapMode) -> ::windows_core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UpdateStatisticsAsync: usize,
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedPosPrinter2 {
    type Vtable = IClaimedPosPrinter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bf7a3d5_5198_437a_82df_589993fa77e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinterClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2b7a27b_4d40_471d_92ed_63375b18c788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedReceiptPrinter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ad27a74_dd61_4ee2_9837_5b5d72d538b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub LinesToPaperCut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub PrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedSlipPrinter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd5deff2_af90_4e8a_b77b_e3ae9ca63a7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub LinesNearEndToEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PrintSide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterPrintSide) -> ::windows_core::HRESULT,
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub PrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub OpenJaws: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CloseJaws: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertSlipAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveSlipAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChangePrintSide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printside: PosPrinterPrintSide) -> ::windows_core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICommonClaimedPosPrinterStation(::windows_core::IUnknown);
impl ICommonClaimedPosPrinterStation {
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharactersPerLine)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineSpacing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineSpacing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).LineWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLetterQuality)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLetterQuality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorCartridge)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows_core::Result<PosPrinterColorCartridge> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCartridge>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCoverOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeRemoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCartridgeEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHeadCleaning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadyToPrint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ValidateData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateData)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICommonClaimedPosPrinterStation> for ::windows_core::IUnknown {
    fn from(value: ICommonClaimedPosPrinterStation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonClaimedPosPrinterStation> for ::windows_core::IUnknown {
    fn from(value: &ICommonClaimedPosPrinterStation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommonClaimedPosPrinterStation> for ::windows_core::IInspectable {
    fn from(value: ICommonClaimedPosPrinterStation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonClaimedPosPrinterStation> for ::windows_core::IInspectable {
    fn from(value: &ICommonClaimedPosPrinterStation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICommonClaimedPosPrinterStation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonClaimedPosPrinterStation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonClaimedPosPrinterStation {}
impl ::core::fmt::Debug for ICommonClaimedPosPrinterStation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonClaimedPosPrinterStation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICommonClaimedPosPrinterStation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b7eb66a8-fe8a-4cfb-8b42-e35b280cb27c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICommonClaimedPosPrinterStation {
    type Vtable = ICommonClaimedPosPrinterStation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7eb66a8_fe8a_4cfb_8b42_e35b280cb27c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonClaimedPosPrinterStation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetCharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub LineWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetIsLetterQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsLetterQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPaperNearEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetColorCartridge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows_core::HRESULT,
    pub ColorCartridge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows_core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCartridgeRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCartridgeEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHeadCleaning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPaperEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReadyToPrint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ValidateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICommonPosPrintStationCapabilities(::windows_core::IUnknown);
impl ICommonPosPrintStationCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrinterPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDualColorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows_core::Result<PosPrinterColorCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows_core::Result<PosPrinterCartridgeSensors> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterCartridgeSensors>::zeroed();
            (::windows_core::Interface::vtable(this).CartridgeSensors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBoldSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsItalicSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::convert::From<ICommonPosPrintStationCapabilities> for ::windows_core::IUnknown {
    fn from(value: ICommonPosPrintStationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonPosPrintStationCapabilities> for ::windows_core::IUnknown {
    fn from(value: &ICommonPosPrintStationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommonPosPrintStationCapabilities> for ::windows_core::IInspectable {
    fn from(value: ICommonPosPrintStationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonPosPrintStationCapabilities> for ::windows_core::IInspectable {
    fn from(value: &ICommonPosPrintStationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICommonPosPrintStationCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonPosPrintStationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonPosPrintStationCapabilities {}
impl ::core::fmt::Debug for ICommonPosPrintStationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonPosPrintStationCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICommonPosPrintStationCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{de5b52ca-e02e-40e9-9e5e-1b488e6aacfc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICommonPosPrintStationCapabilities {
    type Vtable = ICommonPosPrintStationCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde5b52ca_e02e_40e9_9e5e_1b488e6aacfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonPosPrintStationCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPrinterPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDualColorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ColorCartridgeCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows_core::HRESULT,
    pub CartridgeSensors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows_core::HRESULT,
    pub IsBoldSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsItalicSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDoubleHighPrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDoubleHighDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPaperEmptySensorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPaperNearEndSensorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedCharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedCharactersPerLine: usize,
}
#[repr(transparent)]
pub struct ICommonReceiptSlipCapabilities(::windows_core::IUnknown);
impl ICommonReceiptSlipCapabilities {
    pub fn IsBarcodeSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBarcodeSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLeft90RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRight90RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Is180RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrintAreaSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> ::windows_core::Result<PosPrinterRuledLineCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterRuledLineCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).RuledLineCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBarcodeRotations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBitmapRotations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBitmapRotations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    pub fn IsPrinterPresent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrinterPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDualColorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows_core::Result<PosPrinterColorCapabilities> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterCartridgeSensors>::zeroed();
            (::windows_core::Interface::vtable(this).CartridgeSensors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBoldSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsItalicSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::convert::From<ICommonReceiptSlipCapabilities> for ::windows_core::IUnknown {
    fn from(value: ICommonReceiptSlipCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonReceiptSlipCapabilities> for ::windows_core::IUnknown {
    fn from(value: &ICommonReceiptSlipCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommonReceiptSlipCapabilities> for ::windows_core::IInspectable {
    fn from(value: ICommonReceiptSlipCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonReceiptSlipCapabilities> for ::windows_core::IInspectable {
    fn from(value: &ICommonReceiptSlipCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: ICommonReceiptSlipCapabilities) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: &ICommonReceiptSlipCapabilities) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ICommonReceiptSlipCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonReceiptSlipCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonReceiptSlipCapabilities {}
impl ::core::fmt::Debug for ICommonReceiptSlipCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonReceiptSlipCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICommonReceiptSlipCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{09286b8b-9873-4d05-bfbe-4727a6038f69}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICommonReceiptSlipCapabilities {
    type Vtable = ICommonReceiptSlipCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09286b8b_9873_4d05_bfbe_4727a6038f69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonReceiptSlipCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBarcodeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBitmapSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsLeft90RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRight90RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Is180RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPrintAreaSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RuledLineCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedBarcodeRotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedBarcodeRotations: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedBitmapRotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedBitmapRotations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrintJob(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJournalPrintJob {
    type Vtable = IJournalPrintJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f4f2864_f3f0_55d0_8c39_74cc91783eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrintJob_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, printoptions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows_core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrinterCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b5ccc43_e047_4463_bb58_17b5ba1d8056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrinterCapabilities2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJournalPrinterCapabilities2 {
    type Vtable = IJournalPrinterCapabilities2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03b0b645_33b8_533b_baaa_a4389283ab0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplay(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplay {
    type Vtable = ILineDisplay_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24f5df4e_3c99_44e2_b73f_e51be3637a8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ClaimAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplay2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplay2 {
    type Vtable = ILineDisplay2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc296a628_ef44_40f3_bd1c_b04c6a5cdc7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayAttributes(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc17de99c_229a_4c14_a6f1_b4e4b1fead92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayAttributes_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub BlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub ScreenSizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub SetScreenSizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCurrentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a15b5d1_8dc5_4b9c_9172_303e47b70c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows_core::HRESULT,
    pub CanChangeScreenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanDisplayBitmaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanReadCharacterAtCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanMapCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanDisplayCustomGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows_core::HRESULT,
    pub CanBlink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows_core::HRESULT,
    pub CanChangeBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBrightnessSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCursorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHorizontalMarqueeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVerticalMarqueeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsInterCharacterWaitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SupportedWindows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCursor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecdffc45_754a_4e3b_ab2b_151181085605);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanCustomize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBlinkSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBlockSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHalfBlockSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReverseSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsOtherSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCursorAttributes(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e2d54fe_4ffd_4190_aae1_ce285f20c896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBlinkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsBlinkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CursorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayCursorType) -> ::windows_core::HRESULT,
    pub SetCursorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LineDisplayCursorType) -> ::windows_core::HRESULT,
    pub IsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCustomGlyphs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2257f63c_f263_44f1_a1a0_e750a6a0ec54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedGlyphCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedGlyphCodes: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryRedefineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphcode: u32, glyphdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryRedefineAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayMarquee(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3d33e3e_f46a_4b7a_bc21_53eb3b57f8b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayMarquee_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayMarqueeFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LineDisplayMarqueeFormat) -> ::windows_core::HRESULT,
    pub RepeatWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetRepeatWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub ScrollWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetScrollWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub TryStartScrollingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryStopScrollingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayStatics {
    type Vtable = ILineDisplayStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x022dc0b6_11b0_4690_9547_0b39c5af2114);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayStatics2 {
    type Vtable = ILineDisplayStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x600c3f1c_77ab_4968_a7de_c02ff169f2cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StatisticsCategorySelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatisticsCategorySelector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb521c46b_9274_4d24_94f3_b6017b832444);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UnifiedPosStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ManufacturerStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatusUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddd57c1a_86fb_4eba_93d1_6f5eda52b752);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayPowerStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStoredBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf621515b_d81e_43ba_bf1b_bcfa3c785ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EscapeSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayWindow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd21feef4_2364_4be5_bee1_851680af4964);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub InterCharacterWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetInterCharacterWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub TryRefreshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryDisplayTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayattribute: LineDisplayTextAttribute, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryDisplayTextAtPositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayattribute: LineDisplayTextAttribute, startposition: ::winrt_foundation::Point, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryDisplayTextNormalAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryScrollTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryClearTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayWindow2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineDisplayWindow2 {
    type Vtable = ILineDisplayWindow2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa95ce2e6_bdd8_4365_8e11_de94de8dff02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Cursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Marquee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReadCharacterAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryDisplayStoredBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub TryDisplayStorageFileBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryDisplayStorageFileBitmapAtCursorAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryDisplayStorageFileBitmapAtPointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, offsetinpixels: ::winrt_foundation::Point, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryDisplayStorageFileBitmapAtPointAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TryDisplayStorageFileBitmapAtPointWithWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, offsetinpixels: ::winrt_foundation::Point, widthinpixels: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TryDisplayStorageFileBitmapAtPointWithWidthAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a92b015_47c3_468a_9333_0c6517574883);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SupportedCardTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows_core::HRESULT,
    pub DeviceAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClaimReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    RetrieveStatisticsAsync: usize,
    pub GetErrorReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows_core::HRESULT,
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a4bbd51_c316_4910_87f3_7a62ba862d31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LicenseNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Restrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Endorsements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BirthDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HairColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EyeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e958823_a31a_4763_882c_23725e39b08e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AccountNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MiddleInitial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7128809c_c440_44a2_a467_469175d02896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CardAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportedEncryptionAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows_core::HRESULT,
    pub IsIsoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsJisOneSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsJisTwoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTrackDataMaskingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTransmitSentinelsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderCardTypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderCardTypesStatics {
    type Vtable = IMagneticStripeReaderCardTypesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x528f2c5d_2986_474f_8454_7ccd05928d5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Bank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Aamva: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    type Vtable = IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53b57350_c3db_4754_9c00_41392374a109);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub None: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TripleDesDukpt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fedf95d_2c84_41ad_b778_f2356a789ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Track1Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_core::HRESULT,
    pub Track2Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_core::HRESULT,
    pub Track3Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_core::HRESULT,
    pub Track4Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PartialInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a5b6047_99b0_4188_bef1_eddf79f78fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CardType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Track1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Track2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Track3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Track4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
    #[cfg(feature = "winrt-storage")]
    pub CardAuthenticationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CardAuthenticationData: usize,
    pub CardAuthenticationDataLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub AdditionalSecurityInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    AdditionalSecurityInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderStatics {
    type Vtable = IMagneticStripeReaderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc45fab4a_efd7_4760_a5ce_15b0e47e94eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderStatics2 {
    type Vtable = IMagneticStripeReaderStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cadc362_d667_48fa_86bc_f5ae1189262b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09cc6bb0_3262_401d_9e8a_e80d6358906b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderStatus) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderTrackData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x104cf671_4a9d_446e_abc5_20402307ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Data: usize,
    #[cfg(feature = "winrt-storage")]
    pub DiscretionaryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    DiscretionaryData: usize,
    #[cfg(feature = "winrt-storage")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    EncryptedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf0a5514_59cc_4a60_99e8_99a53dace5aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinter {
    type Vtable = IPosPrinter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a03c10e_9a19_4a01_994f_12dfad6adcbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedCharacterSets: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedTypeFaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedTypeFaces: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClaimPrinterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStatisticsAsync: usize,
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinter2 {
    type Vtable = IPosPrinter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x248475e8_8b98_5517_8e48_760e86f68987);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedBarcodeSymbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedBarcodeSymbologies: usize,
    pub GetFontProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeface: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcde95721_4380_4985_adc5_39db30cd93bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows_core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DefaultCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub HasCoverSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanMapCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTransactionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterCharacterSetIdsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterCharacterSetIdsStatics {
    type Vtable = IPosPrinterCharacterSetIdsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c709eff_709a_4fe7_b215_06a748a38b39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Utf16LE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ascii: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Ansi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterFontProperty(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7f4e93a_f8ac_5f04_84d2_29b16d8a633c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsScalableToAnySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub CharacterSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CharacterSizes: usize,
}
#[repr(transparent)]
pub struct IPosPrinterJob(::windows_core::IUnknown);
impl IPosPrinterJob {
    pub fn Print<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintLine)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintNewline)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::convert::From<IPosPrinterJob> for ::windows_core::IUnknown {
    fn from(value: IPosPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPosPrinterJob> for ::windows_core::IUnknown {
    fn from(value: &IPosPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPosPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPosPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPosPrinterJob> for ::windows_core::IInspectable {
    fn from(value: IPosPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPosPrinterJob> for ::windows_core::IInspectable {
    fn from(value: &IPosPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPosPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPosPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPosPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPosPrinterJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPosPrinterJob {}
impl ::core::fmt::Debug for IPosPrinterJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPosPrinterJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPosPrinterJob {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{9a94005c-0615-4591-a58f-30f87edfe2e4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPosPrinterJob {
    type Vtable = IPosPrinterJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a94005c_0615_4591_a58f_30f87edfe2e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterJob_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PrintLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PrintNewline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExecuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterPrintOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a2e16fd_1d02_5a58_9d59_bfcde76fde86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CharacterHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCharacterHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Italic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Underline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ReverseVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetReverseVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Superscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Subscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DoubleWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDoubleWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DoubleHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDoubleHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Alignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterAlignment) -> ::windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterAlignment) -> ::windows_core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bcba359_1cef_40b2_9ecb_f927f856ae3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterStatics {
    type Vtable = IPosPrinterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ce0d4ea_132f_4cdf_a64a_2d0d7c96a85b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterStatics2 {
    type Vtable = IPosPrinterStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeecd2c1c_b0d0_42e7_b137_b89b16244d41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1f0c730_da40_4328_bf76_5156fa33b747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StatusKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterStatusKind) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatusUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2edb87df_13a6_428d_ba81_b0e7c3e5a3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReceiptOrSlipJob(::windows_core::IUnknown);
impl IReceiptOrSlipJob {
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBarcodeRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrintRotation)(::windows_core::Interface::as_raw(this), value, includebitmaps).ok() }
    }
    pub fn SetPrintArea<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrintArea)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmap<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomAlignedBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintSavedBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DrawRuledLine)(::windows_core::Interface::as_raw(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBarcode)(::windows_core::Interface::as_raw(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBarcodeCustomAlign)(::windows_core::Interface::as_raw(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintCustomAlignedBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    pub fn Print<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintLine)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintNewline)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::convert::From<IReceiptOrSlipJob> for ::windows_core::IUnknown {
    fn from(value: IReceiptOrSlipJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReceiptOrSlipJob> for ::windows_core::IUnknown {
    fn from(value: &IReceiptOrSlipJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReceiptOrSlipJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IReceiptOrSlipJob> for ::windows_core::IInspectable {
    fn from(value: IReceiptOrSlipJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReceiptOrSlipJob> for ::windows_core::IInspectable {
    fn from(value: &IReceiptOrSlipJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IReceiptOrSlipJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: IReceiptOrSlipJob) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: &IReceiptOrSlipJob) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for &IReceiptOrSlipJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IReceiptOrSlipJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReceiptOrSlipJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReceiptOrSlipJob {}
impl ::core::fmt::Debug for IReceiptOrSlipJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReceiptOrSlipJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IReceiptOrSlipJob {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{532199be-c8c3-4dc2-89e9-5c4a37b34ddc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IReceiptOrSlipJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x532199be_c8c3_4dc2_89e9_5c4a37b34ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptOrSlipJob_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetBarcodeRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows_core::HRESULT,
    pub SetPrintRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_core::HRESULT,
    pub SetPrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub SetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows_core::RawPtr, alignment: PosPrinterAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetBitmap: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows_core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetCustomAlignedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows_core::RawPtr, alignmentdistance: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetCustomAlignedBitmap: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows_core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetBitmapCustomWidthCustomAlign: usize,
    pub PrintSavedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows_core::HRESULT,
    pub DrawRuledLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positionlist: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_core::HRESULT,
    pub PrintBarcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_core::HRESULT,
    pub PrintBarcodeCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub PrintBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, alignment: PosPrinterAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrintBitmap: usize,
    #[cfg(feature = "winrt-graphics")]
    pub PrintBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrintBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "winrt-graphics")]
    pub PrintCustomAlignedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, alignmentdistance: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrintCustomAlignedBitmap: usize,
    #[cfg(feature = "winrt-graphics")]
    pub PrintBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrintBitmapCustomWidthCustomAlign: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrintJob(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa96066e_acad_4b79_9d0f_c0cfc08dc77b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MarkFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: PosPrinterMarkFeedKind) -> ::windows_core::HRESULT,
    pub CutPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentage: f64) -> ::windows_core::HRESULT,
    pub CutPaperDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrintJob2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReceiptPrintJob2 {
    type Vtable = IReceiptPrintJob2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cbc12e3_9e29_5179_bcd8_1811d3b9a10e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StampPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, printoptions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows_core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8f0b58f_51a8_43fc_9bd5_8de272a6415b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanCutPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStampSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MarkFeedCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReceiptPrinterCapabilities2 {
    type Vtable = IReceiptPrinterCapabilities2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20030638_8a2c_55ac_9a7b_7576d8869e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrintJob(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlipPrintJob {
    type Vtable = ISlipPrintJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d88f95d_6131_5a4b_b7d5_8ef2da7b4165);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrintJob_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, printoptions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows_core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrinterCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99b16399_488c_4157_8ac2_9f57f708d3db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsFullLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBothSidesPrintingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrinterCapabilities2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlipPrinterCapabilities2 {
    type Vtable = ISlipPrinterCapabilities2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ff89671_2d1a_5000_87c2_b0851bfdf07e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnifiedPosErrorData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b998c3a_555c_4889_8ed8_c599bb3a712a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorSeverity) -> ::windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorReason) -> ::windows_core::HRESULT,
    pub ExtendedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnifiedPosErrorDataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnifiedPosErrorDataFactory {
    type Vtable = IUnifiedPosErrorDataFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b982551_1ffe_451b_a368_63e0ce465f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct JournalPrintJob(::windows_core::IUnknown);
impl JournalPrintJob {
    pub fn Print<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FeedPaperByLine)(::windows_core::Interface::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FeedPaperByMapModeUnit)(::windows_core::Interface::as_raw(this), distance).ok() }
    }
    pub fn Print2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintLine)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintNewline)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for JournalPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JournalPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JournalPrintJob {}
impl ::core::fmt::Debug for JournalPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JournalPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JournalPrintJob {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrintJob;{9a94005c-0615-4591-a58f-30f87edfe2e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JournalPrintJob {
    type Vtable = IPosPrinterJob_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterJob as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrintJob";
}
impl ::core::convert::From<JournalPrintJob> for ::windows_core::IUnknown {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrintJob> for ::windows_core::IUnknown {
    fn from(value: &JournalPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JournalPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JournalPrintJob> for ::windows_core::IInspectable {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrintJob> for ::windows_core::IInspectable {
    fn from(value: &JournalPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JournalPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<JournalPrintJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: JournalPrintJob) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JournalPrintJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: &JournalPrintJob) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for JournalPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for &JournalPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JournalPrintJob {}
unsafe impl ::core::marker::Sync for JournalPrintJob {}
#[repr(transparent)]
pub struct JournalPrinterCapabilities(::windows_core::IUnknown);
impl JournalPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrinterPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDualColorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows_core::Result<PosPrinterColorCapabilities> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterCartridgeSensors>::zeroed();
            (::windows_core::Interface::vtable(this).CartridgeSensors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBoldSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsItalicSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReverseVideoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStrikethroughSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSuperscriptSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSubscriptSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for JournalPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JournalPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JournalPrinterCapabilities {}
impl ::core::fmt::Debug for JournalPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JournalPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JournalPrinterCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrinterCapabilities;{3b5ccc43-e047-4463-bb58-17b5ba1d8056})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IJournalPrinterCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrinterCapabilities";
}
impl ::core::convert::From<JournalPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: JournalPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JournalPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: JournalPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: JournalPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: &JournalPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &JournalPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JournalPrinterCapabilities {}
unsafe impl ::core::marker::Sync for JournalPrinterCapabilities {}
#[repr(transparent)]
pub struct LineDisplay(::windows_core::IUnknown);
impl LineDisplay {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    pub fn PhysicalDeviceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PhysicalDeviceDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceControlDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceControlDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceControlVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceControlVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeviceServiceVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceServiceVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ClaimAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ClaimedLineDisplay>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClaimAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        }
    }
    pub fn CheckPowerStatusAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows_core::Interface::cast::<ILineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckPowerStatusAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows_core::Interface::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn StatisticsCategorySelector() -> ::windows_core::Result<LineDisplayStatisticsCategorySelector> {
        Self::ILineDisplayStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StatisticsCategorySelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayStatisticsCategorySelector>(result__)
        })
    }
    pub fn ILineDisplayStatics<R, F: FnOnce(&ILineDisplayStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LineDisplay, ILineDisplayStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILineDisplayStatics2<R, F: FnOnce(&ILineDisplayStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LineDisplay, ILineDisplayStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LineDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplay {}
impl ::core::fmt::Debug for LineDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplay {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplay;{24f5df4e-3c99-44e2-b73f-e51be3637a8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplay {
    type Vtable = ILineDisplay_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplay as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplay";
}
impl ::core::convert::From<LineDisplay> for ::windows_core::IUnknown {
    fn from(value: LineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplay> for ::windows_core::IUnknown {
    fn from(value: &LineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplay> for ::windows_core::IInspectable {
    fn from(value: LineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplay> for ::windows_core::IInspectable {
    fn from(value: &LineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LineDisplay> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: LineDisplay) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LineDisplay> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &LineDisplay) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for LineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &LineDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LineDisplay {}
unsafe impl ::core::marker::Sync for LineDisplay {}
#[repr(transparent)]
pub struct LineDisplayAttributes(::windows_core::IUnknown);
impl LineDisplayAttributes {
    pub fn IsPowerNotifyEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPowerNotifyEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPowerNotifyEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BlinkRate(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).BlinkRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetBlinkRate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBlinkRate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ScreenSizeInCharacters(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenSizeInCharacters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SetScreenSizeInCharacters<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScreenSizeInCharacters)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CharacterSet(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSet(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharacterSet)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCharacterSetMappingEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCharacterSetMappingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentWindow(&self) -> ::windows_core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentWindow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayWindow>(result__)
        }
    }
    pub fn SetCurrentWindow<'a, Param0: ::windows_core::IntoParam<'a, LineDisplayWindow>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrentWindow)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LineDisplayAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayAttributes {}
impl ::core::fmt::Debug for LineDisplayAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayAttributes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayAttributes;{c17de99c-229a-4c14-a6f1-b4e4b1fead92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayAttributes as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayAttributes";
}
impl ::core::convert::From<LineDisplayAttributes> for ::windows_core::IUnknown {
    fn from(value: LineDisplayAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayAttributes> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayAttributes> for ::windows_core::IInspectable {
    fn from(value: LineDisplayAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayAttributes> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayAttributes {}
#[repr(transparent)]
pub struct LineDisplayCapabilities(::windows_core::IUnknown);
impl LineDisplayCapabilities {
    pub fn IsStatisticsReportingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PowerReportingType(&self) -> ::windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosPowerReportingType>::zeroed();
            (::windows_core::Interface::vtable(this).PowerReportingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    pub fn CanChangeScreenSize(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanChangeScreenSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanDisplayBitmaps(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanDisplayBitmaps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanReadCharacterAtCursor(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanReadCharacterAtCursor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanMapCharacterSets(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanMapCharacterSets)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanDisplayCustomGlyphs(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanDisplayCustomGlyphs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanReverse(&self) -> ::windows_core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LineDisplayTextAttributeGranularity>::zeroed();
            (::windows_core::Interface::vtable(this).CanReverse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    pub fn CanBlink(&self) -> ::windows_core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LineDisplayTextAttributeGranularity>::zeroed();
            (::windows_core::Interface::vtable(this).CanBlink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    pub fn CanChangeBlinkRate(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanChangeBlinkRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBrightnessSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBrightnessSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCursorSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCursorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHorizontalMarqueeSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHorizontalMarqueeSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVerticalMarqueeSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVerticalMarqueeSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsInterCharacterWaitSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInterCharacterWaitSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedDescriptors(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDescriptors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SupportedWindows(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedWindows)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCapabilities {}
impl ::core::fmt::Debug for LineDisplayCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCapabilities;{5a15b5d1-8dc5-4b9c-9172-303e47b70c55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCapabilities";
}
impl ::core::convert::From<LineDisplayCapabilities> for ::windows_core::IUnknown {
    fn from(value: LineDisplayCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCapabilities> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCapabilities> for ::windows_core::IInspectable {
    fn from(value: LineDisplayCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCapabilities> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCapabilities {}
unsafe impl ::core::marker::Sync for LineDisplayCapabilities {}
#[repr(transparent)]
pub struct LineDisplayCursor(::windows_core::IUnknown);
impl LineDisplayCursor {
    pub fn CanCustomize(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCustomize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBlinkSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlinkSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBlockSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlockSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHalfBlockSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHalfBlockSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReverseSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReverseSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsOtherSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOtherSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetAttributes(&self) -> ::windows_core::Result<LineDisplayCursorAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayCursorAttributes>(result__)
        }
    }
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows_core::IntoParam<'a, LineDisplayCursorAttributes>>(&self, attributes: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateAttributesAsync)(::windows_core::Interface::as_raw(this), attributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCursor {}
impl ::core::fmt::Debug for LineDisplayCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayCursor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursor;{ecdffc45-754a-4e3b-ab2b-151181085605})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayCursor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursor";
}
impl ::core::convert::From<LineDisplayCursor> for ::windows_core::IUnknown {
    fn from(value: LineDisplayCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursor> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCursor> for ::windows_core::IInspectable {
    fn from(value: LineDisplayCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursor> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCursor {}
unsafe impl ::core::marker::Sync for LineDisplayCursor {}
#[repr(transparent)]
pub struct LineDisplayCursorAttributes(::windows_core::IUnknown);
impl LineDisplayCursorAttributes {
    pub fn IsBlinkEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlinkEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsBlinkEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBlinkEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CursorType(&self) -> ::windows_core::Result<LineDisplayCursorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LineDisplayCursorType>::zeroed();
            (::windows_core::Interface::vtable(this).CursorType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayCursorType>(result__)
        }
    }
    pub fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCursorType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoAdvanceEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoAdvanceEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAutoAdvanceEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LineDisplayCursorAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCursorAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCursorAttributes {}
impl ::core::fmt::Debug for LineDisplayCursorAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursorAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayCursorAttributes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursorAttributes;{4e2d54fe-4ffd-4190-aae1-ce285f20c896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayCursorAttributes as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursorAttributes";
}
impl ::core::convert::From<LineDisplayCursorAttributes> for ::windows_core::IUnknown {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursorAttributes> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCursorAttributes> for ::windows_core::IInspectable {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursorAttributes> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCursorAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayCursorAttributes {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayCursorType(pub i32);
impl LineDisplayCursorType {
    pub const None: Self = Self(0i32);
    pub const Block: Self = Self(1i32);
    pub const HalfBlock: Self = Self(2i32);
    pub const Underline: Self = Self(3i32);
    pub const Reverse: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for LineDisplayCursorType {}
impl ::core::clone::Clone for LineDisplayCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayCursorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayCursorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayCursorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursorType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayCursorType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayCursorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LineDisplayCustomGlyphs(::windows_core::IUnknown);
impl LineDisplayCustomGlyphs {
    pub fn SizeInPixels(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).SizeInPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedGlyphCodes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedGlyphCodes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryRedefineAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, glyphcode: u32, glyphdata: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRedefineAsync)(::windows_core::Interface::as_raw(this), glyphcode, glyphdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCustomGlyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCustomGlyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCustomGlyphs {}
impl ::core::fmt::Debug for LineDisplayCustomGlyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCustomGlyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayCustomGlyphs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCustomGlyphs;{2257f63c-f263-44f1-a1a0-e750a6a0ec54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayCustomGlyphs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCustomGlyphs";
}
impl ::core::convert::From<LineDisplayCustomGlyphs> for ::windows_core::IUnknown {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCustomGlyphs> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCustomGlyphs> for ::windows_core::IInspectable {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCustomGlyphs> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCustomGlyphs {}
unsafe impl ::core::marker::Sync for LineDisplayCustomGlyphs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Blink: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayDescriptorState {}
impl ::core::clone::Clone for LineDisplayDescriptorState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayDescriptorState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayDescriptorState {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayDescriptorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayDescriptorState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayDescriptorState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayDescriptorState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayHorizontalAlignment {}
impl ::core::clone::Clone for LineDisplayHorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayHorizontalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayHorizontalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayHorizontalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayHorizontalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayHorizontalAlignment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayHorizontalAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LineDisplayMarquee(::windows_core::IUnknown);
impl LineDisplayMarquee {
    pub fn Format(&self) -> ::windows_core::Result<LineDisplayMarqueeFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LineDisplayMarqueeFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayMarqueeFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepeatWaitInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RepeatWaitInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetRepeatWaitInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepeatWaitInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ScrollWaitInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).ScrollWaitInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetScrollWaitInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScrollWaitInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryStartScrollingAsync)(::windows_core::Interface::as_raw(this), direction, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryStopScrollingAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryStopScrollingAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayMarquee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayMarquee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayMarquee {}
impl ::core::fmt::Debug for LineDisplayMarquee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayMarquee").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayMarquee {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayMarquee;{a3d33e3e-f46a-4b7a-bc21-53eb3b57f8b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayMarquee as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayMarquee";
}
impl ::core::convert::From<LineDisplayMarquee> for ::windows_core::IUnknown {
    fn from(value: LineDisplayMarquee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayMarquee> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayMarquee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayMarquee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayMarquee> for ::windows_core::IInspectable {
    fn from(value: LineDisplayMarquee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayMarquee> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayMarquee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayMarquee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayMarquee {}
unsafe impl ::core::marker::Sync for LineDisplayMarquee {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: Self = Self(0i32);
    pub const Walk: Self = Self(1i32);
    pub const Place: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayMarqueeFormat {}
impl ::core::clone::Clone for LineDisplayMarqueeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayMarqueeFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayMarqueeFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayMarqueeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayMarqueeFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayMarqueeFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayMarqueeFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Online: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Offline: Self = Self(3i32);
    pub const OffOrOffline: Self = Self(4i32);
}
impl ::core::marker::Copy for LineDisplayPowerStatus {}
impl ::core::clone::Clone for LineDisplayPowerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayPowerStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayPowerStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayPowerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayPowerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayPowerStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayPowerStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: Self = Self(0i32);
    pub const Down: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayScrollDirection {}
impl ::core::clone::Clone for LineDisplayScrollDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayScrollDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayScrollDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayScrollDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayScrollDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayScrollDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayScrollDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LineDisplayStatisticsCategorySelector(::windows_core::IUnknown);
impl LineDisplayStatisticsCategorySelector {
    pub fn AllStatistics(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AllStatistics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn UnifiedPosStatistics(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UnifiedPosStatistics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ManufacturerStatistics(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ManufacturerStatistics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStatisticsCategorySelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStatisticsCategorySelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStatisticsCategorySelector {}
impl ::core::fmt::Debug for LineDisplayStatisticsCategorySelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStatisticsCategorySelector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayStatisticsCategorySelector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector;{b521c46b-9274-4d24-94f3-b6017b832444})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayStatisticsCategorySelector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector";
}
impl ::core::convert::From<LineDisplayStatisticsCategorySelector> for ::windows_core::IUnknown {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayStatisticsCategorySelector> for ::windows_core::IInspectable {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayStatisticsCategorySelector {}
unsafe impl ::core::marker::Sync for LineDisplayStatisticsCategorySelector {}
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(::windows_core::IUnknown);
impl LineDisplayStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<LineDisplayPowerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LineDisplayPowerStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayPowerStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for LineDisplayStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs;{ddd57c1a-86fb-4eba-93d1-6f5eda52b752})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayStatusUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs";
}
impl ::core::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for LineDisplayStatusUpdatedEventArgs {}
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(::windows_core::IUnknown);
impl LineDisplayStoredBitmap {
    pub fn EscapeSequence(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EscapeSequence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TryDeleteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStoredBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStoredBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStoredBitmap {}
impl ::core::fmt::Debug for LineDisplayStoredBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStoredBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayStoredBitmap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStoredBitmap;{f621515b-d81e-43ba-bf1b-bcfa3c785ba0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayStoredBitmap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStoredBitmap";
}
impl ::core::convert::From<LineDisplayStoredBitmap> for ::windows_core::IUnknown {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStoredBitmap> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayStoredBitmap> for ::windows_core::IInspectable {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStoredBitmap> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayStoredBitmap {}
unsafe impl ::core::marker::Sync for LineDisplayStoredBitmap {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: Self = Self(0i32);
    pub const Blink: Self = Self(1i32);
    pub const Reverse: Self = Self(2i32);
    pub const ReverseBlink: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayTextAttribute {}
impl ::core::clone::Clone for LineDisplayTextAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayTextAttribute {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayTextAttribute {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayTextAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayTextAttribute {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttribute;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
    pub const PerCharacter: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayTextAttributeGranularity {}
impl ::core::clone::Clone for LineDisplayTextAttributeGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayTextAttributeGranularity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayTextAttributeGranularity {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayTextAttributeGranularity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttributeGranularity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayTextAttributeGranularity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttributeGranularity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayVerticalAlignment {}
impl ::core::clone::Clone for LineDisplayVerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayVerticalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LineDisplayVerticalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayVerticalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayVerticalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayVerticalAlignment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayVerticalAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LineDisplayWindow(::windows_core::IUnknown);
impl LineDisplayWindow {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SizeInCharacters(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).SizeInCharacters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn InterCharacterWaitInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).InterCharacterWaitInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetInterCharacterWaitInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterCharacterWaitInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TryRefreshAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRefreshAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryDisplayTextAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayTextAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), displayattribute, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryDisplayTextAtPositionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute, startposition: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayTextAtPositionAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), displayattribute, startposition.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryDisplayTextNormalAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayTextNormalAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryScrollTextAsync)(::windows_core::Interface::as_raw(this), direction, numberofcolumnsorrows, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryClearTextAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryClearTextAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Cursor(&self) -> ::windows_core::Result<LineDisplayCursor> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Cursor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayCursor>(result__)
        }
    }
    pub fn Marquee(&self) -> ::windows_core::Result<LineDisplayMarquee> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Marquee)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LineDisplayMarquee>(result__)
        }
    }
    pub fn ReadCharacterAtCursorAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadCharacterAtCursorAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn TryDisplayStoredBitmapAtCursorAsync<'a, Param0: ::windows_core::IntoParam<'a, LineDisplayStoredBitmap>>(&self, bitmap: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayStoredBitmapAtCursorAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryDisplayStorageFileBitmapAtCursorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, bitmap: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryDisplayStorageFileBitmapAtPointAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtPointAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1, widthinpixels: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisplayStorageFileBitmapAtPointWithWidthAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), widthinpixels, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayWindow {}
impl ::core::fmt::Debug for LineDisplayWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LineDisplayWindow {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayWindow;{d21feef4-2364-4be5-bee1-851680af4964})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
    const IID: ::windows_core::GUID = <ILineDisplayWindow as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayWindow";
}
impl ::core::convert::From<LineDisplayWindow> for ::windows_core::IUnknown {
    fn from(value: LineDisplayWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayWindow> for ::windows_core::IUnknown {
    fn from(value: &LineDisplayWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LineDisplayWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayWindow> for ::windows_core::IInspectable {
    fn from(value: LineDisplayWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayWindow> for ::windows_core::IInspectable {
    fn from(value: &LineDisplayWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LineDisplayWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LineDisplayWindow> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: LineDisplayWindow) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LineDisplayWindow> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &LineDisplayWindow) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for LineDisplayWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &LineDisplayWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LineDisplayWindow {}
unsafe impl ::core::marker::Sync for LineDisplayWindow {}
#[repr(transparent)]
pub struct MagneticStripeReader(::windows_core::IUnknown);
impl MagneticStripeReader {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<MagneticStripeReaderCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderCapabilities>(result__)
        }
    }
    pub fn SupportedCardTypes(&self) -> ::windows_core::Result<::windows_core::Array<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u32>>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCardTypes)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn DeviceAuthenticationProtocol(&self) -> ::windows_core::Result<MagneticStripeReaderAuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderAuthenticationProtocol>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAuthenticationProtocol)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderAuthenticationProtocol>(result__)
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHealthAsync)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ClaimReaderAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ClaimedMagneticStripeReader>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClaimReaderAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ClaimedMagneticStripeReader>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
    pub fn GetErrorReportingType(&self) -> ::windows_core::Result<MagneticStripeReaderErrorReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderErrorReportingType>::zeroed();
            (::windows_core::Interface::vtable(this).GetErrorReportingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderErrorReportingType>(result__)
        }
    }
    pub fn StatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMagneticStripeReaderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows_core::Interface::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IMagneticStripeReaderStatics<R, F: FnOnce(&IMagneticStripeReaderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMagneticStripeReaderStatics2<R, F: FnOnce(&IMagneticStripeReaderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MagneticStripeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReader {}
impl ::core::fmt::Debug for MagneticStripeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReader;{1a92b015-47c3-468a-9333-0c6517574883})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReader";
}
impl ::core::convert::From<MagneticStripeReader> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReader> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReader> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReader> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MagneticStripeReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MagneticStripeReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MagneticStripeReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MagneticStripeReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MagneticStripeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReader {}
unsafe impl ::core::marker::Sync for MagneticStripeReader {}
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows_core::IUnknown);
impl MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Report)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    pub fn LicenseNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Restrictions(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Restrictions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Class(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Class)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Endorsements(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Endorsements)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn BirthDate(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BirthDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FirstName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Surname(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Surname)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Suffix(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Suffix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Gender(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Gender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn HairColor(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HairColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EyeColor(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EyeColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Weight(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Weight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn City(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).City)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAamvaCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs;{0a4bbd51-c316-4910-87f3-7a62ba862d31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderAamvaCardDataReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationLevel {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderAuthenticationLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagneticStripeReaderAuthenticationLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderAuthenticationLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderAuthenticationLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: Self = Self(0i32);
    pub const ChallengeResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationProtocol {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderAuthenticationProtocol {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagneticStripeReaderAuthenticationProtocol {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderAuthenticationProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderAuthenticationProtocol {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationProtocol;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(::windows_core::IUnknown);
impl MagneticStripeReaderBankCardDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Report)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    pub fn AccountNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FirstName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MiddleInitial(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MiddleInitial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Surname(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Surname)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Suffix(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Suffix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderBankCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderBankCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs;{2e958823-a31a-4763-882c-23725e39b08e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderBankCardDataReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderBankCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderBankCardDataReceivedEventArgs {}
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(::windows_core::IUnknown);
impl MagneticStripeReaderCapabilities {
    pub fn CardAuthentication(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CardAuthentication)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SupportedEncryptionAlgorithms(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedEncryptionAlgorithms)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AuthenticationLevel(&self) -> ::windows_core::Result<MagneticStripeReaderAuthenticationLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderAuthenticationLevel>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderAuthenticationLevel>(result__)
        }
    }
    pub fn IsIsoSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIsoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsJisOneSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsJisOneSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsJisTwoSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsJisTwoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PowerReportingType(&self) -> ::windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosPowerReportingType>::zeroed();
            (::windows_core::Interface::vtable(this).PowerReportingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTrackDataMaskingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTrackDataMaskingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTransmitSentinelsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTransmitSentinelsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderCapabilities {}
impl ::core::fmt::Debug for MagneticStripeReaderCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderCapabilities;{7128809c-c440-44a2-a467-469175d02896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCapabilities";
}
impl ::core::convert::From<MagneticStripeReaderCapabilities> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderCapabilities> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderCapabilities> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderCapabilities> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderCapabilities {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderCapabilities {}
pub struct MagneticStripeReaderCardTypes;
impl MagneticStripeReaderCardTypes {
    pub fn Unknown() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Unknown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Bank() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Bank)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Aamva() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Aamva)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn ExtendedBase() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedBase)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IMagneticStripeReaderCardTypesStatics<R, F: FnOnce(&IMagneticStripeReaderCardTypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MagneticStripeReaderCardTypes, IMagneticStripeReaderCardTypesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MagneticStripeReaderCardTypes {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCardTypes";
}
pub struct MagneticStripeReaderEncryptionAlgorithms;
impl MagneticStripeReaderEncryptionAlgorithms {
    pub fn None() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).None)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn TripleDesDukpt() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TripleDesDukpt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn ExtendedBase() -> ::windows_core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedBase)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IMagneticStripeReaderEncryptionAlgorithmsStatics<R, F: FnOnce(&IMagneticStripeReaderEncryptionAlgorithmsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MagneticStripeReaderEncryptionAlgorithms, IMagneticStripeReaderEncryptionAlgorithmsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MagneticStripeReaderEncryptionAlgorithms {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderEncryptionAlgorithms";
}
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(::windows_core::IUnknown);
impl MagneticStripeReaderErrorOccurredEventArgs {
    pub fn Track1Status(&self) -> ::windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderTrackErrorType>::zeroed();
            (::windows_core::Interface::vtable(this).Track1Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    pub fn Track2Status(&self) -> ::windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderTrackErrorType>::zeroed();
            (::windows_core::Interface::vtable(this).Track2Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    pub fn Track3Status(&self) -> ::windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderTrackErrorType>::zeroed();
            (::windows_core::Interface::vtable(this).Track3Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    pub fn Track4Status(&self) -> ::windows_core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderTrackErrorType>::zeroed();
            (::windows_core::Interface::vtable(this).Track4Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    pub fn ErrorData(&self) -> ::windows_core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
    pub fn PartialInputData(&self) -> ::windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PartialInputData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderErrorOccurredEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs;{1fedf95d-2c84-41ad-b778-f2356a789ab1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderErrorOccurredEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: Self = Self(0i32);
    pub const TrackLevel: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderErrorReportingType {}
impl ::core::clone::Clone for MagneticStripeReaderErrorReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderErrorReportingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagneticStripeReaderErrorReportingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderErrorReportingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorReportingType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderErrorReportingType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderErrorReportingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderReport(::windows_core::IUnknown);
impl MagneticStripeReaderReport {
    pub fn CardType(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CardType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Track1(&self) -> ::windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Track1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    pub fn Track2(&self) -> ::windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Track2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    pub fn Track3(&self) -> ::windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Track3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    pub fn Track4(&self) -> ::windows_core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Track4)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CardAuthenticationData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CardAuthenticationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn CardAuthenticationDataLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CardAuthenticationDataLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn AdditionalSecurityInformation(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalSecurityInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderReport {}
impl ::core::fmt::Debug for MagneticStripeReaderReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderReport;{6a5b6047-99b0-4188-bef1-eddf79f78fe6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderReport";
}
impl ::core::convert::From<MagneticStripeReaderReport> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderReport> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderReport> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderReport> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderReport {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const Authenticated: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderStatus {}
impl ::core::clone::Clone for MagneticStripeReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagneticStripeReaderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(::windows_core::IUnknown);
impl MagneticStripeReaderStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<MagneticStripeReaderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagneticStripeReaderStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderStatus>(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs;{09cc6bb0-3262-401d-9e8a-e80d6358906b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderStatusUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderStatusUpdatedEventArgs {}
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(::windows_core::IUnknown);
impl MagneticStripeReaderTrackData {
    #[cfg(feature = "winrt-storage")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn DiscretionaryData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DiscretionaryData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn EncryptedData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncryptedData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderTrackData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderTrackData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderTrackData {}
impl ::core::fmt::Debug for MagneticStripeReaderTrackData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderTrackData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderTrackData;{104cf671-4a9d-446e-abc5-20402307ba36})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderTrackData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderTrackData";
}
impl ::core::convert::From<MagneticStripeReaderTrackData> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderTrackData> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderTrackData> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderTrackData> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderTrackData {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderTrackData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderTrackErrorType(pub i32);
impl MagneticStripeReaderTrackErrorType {
    pub const None: Self = Self(0i32);
    pub const StartSentinelError: Self = Self(1i32);
    pub const EndSentinelError: Self = Self(2i32);
    pub const ParityError: Self = Self(3i32);
    pub const LrcError: Self = Self(4i32);
    pub const Unknown: Self = Self(-1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackErrorType {}
impl ::core::clone::Clone for MagneticStripeReaderTrackErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderTrackErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagneticStripeReaderTrackErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderTrackErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackErrorType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderTrackErrorType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackErrorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: Self = Self(0i32);
    pub const Track1: Self = Self(1i32);
    pub const Track2: Self = Self(2i32);
    pub const Track3: Self = Self(4i32);
    pub const Track4: Self = Self(8i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackIds {}
impl ::core::clone::Clone for MagneticStripeReaderTrackIds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderTrackIds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagneticStripeReaderTrackIds {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderTrackIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackIds").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderTrackIds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackIds;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows_core::IUnknown);
impl MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows_core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Report)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs;{af0a5514-59cc-4a60-99e8-99a53dace5aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: Self = Self(1u32);
    pub const IP: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PosConnectionTypes {}
impl ::core::clone::Clone for PosConnectionTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosConnectionTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosConnectionTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosConnectionTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosConnectionTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosConnectionTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosConnectionTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosConnectionTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosConnectionTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosConnectionTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PosConnectionTypes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosConnectionTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinter(::windows_core::IUnknown);
impl PosPrinter {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<PosPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterCapabilities>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharacterSets(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharacterSets)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedTypeFaces(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTypeFaces)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterStatus>(result__)
        }
    }
    pub fn ClaimPrinterAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ClaimedPosPrinter>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClaimPrinterAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ClaimedPosPrinter>>(result__)
        }
    }
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHealthAsync)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStatisticsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatisticsAsync)(::windows_core::Interface::as_raw(this), statisticscategories.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn StatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBarcodeSymbologies(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBarcodeSymbologies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn GetFontProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, typeface: Param0) -> ::windows_core::Result<PosPrinterFontProperty> {
        let this = &::windows_core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFontProperty)(::windows_core::Interface::as_raw(this), typeface.into_param().abi(), result__.as_mut_ptr()).from_abi::<PosPrinterFontProperty>(result__)
        }
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPosPrinterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows_core::Interface::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IPosPrinterStatics<R, F: FnOnce(&IPosPrinterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PosPrinter, IPosPrinterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPosPrinterStatics2<R, F: FnOnce(&IPosPrinterStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PosPrinter, IPosPrinterStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PosPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinter {}
impl ::core::fmt::Debug for PosPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinter;{2a03c10e-9a19-4a01-994f-12dfad6adcbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinter {
    type Vtable = IPosPrinter_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinter";
}
impl ::core::convert::From<PosPrinter> for ::windows_core::IUnknown {
    fn from(value: PosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinter> for ::windows_core::IUnknown {
    fn from(value: &PosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinter> for ::windows_core::IInspectable {
    fn from(value: PosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinter> for ::windows_core::IInspectable {
    fn from(value: &PosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PosPrinter> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PosPrinter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PosPrinter> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PosPrinter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PosPrinter {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PosPrinter {}
unsafe impl ::core::marker::Sync for PosPrinter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterAlignment {}
impl ::core::clone::Clone for PosPrinterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterAlignment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterBarcodeTextPosition {}
impl ::core::clone::Clone for PosPrinterBarcodeTextPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterBarcodeTextPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterBarcodeTextPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterBarcodeTextPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterBarcodeTextPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterBarcodeTextPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterBarcodeTextPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinterCapabilities(::windows_core::IUnknown);
impl PosPrinterCapabilities {
    pub fn PowerReportingType(&self) -> ::windows_core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosPowerReportingType>::zeroed();
            (::windows_core::Interface::vtable(this).PowerReportingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsReportingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DefaultCharacterSet(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultCharacterSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn HasCoverSensor(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasCoverSensor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanMapCharacterSet(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanMapCharacterSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTransactionSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTransactionSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Receipt(&self) -> ::windows_core::Result<ReceiptPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Receipt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ReceiptPrinterCapabilities>(result__)
        }
    }
    pub fn Slip(&self) -> ::windows_core::Result<SlipPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Slip)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SlipPrinterCapabilities>(result__)
        }
    }
    pub fn Journal(&self) -> ::windows_core::Result<JournalPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Journal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JournalPrinterCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterCapabilities {}
impl ::core::fmt::Debug for PosPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterCapabilities;{cde95721-4380-4985-adc5-39db30cd93bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCapabilities";
}
impl ::core::convert::From<PosPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: PosPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: &PosPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: PosPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: &PosPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterCapabilities {}
unsafe impl ::core::marker::Sync for PosPrinterCapabilities {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: Self = Self(0u32);
    pub const Removed: Self = Self(1u32);
    pub const Empty: Self = Self(2u32);
    pub const HeadCleaning: Self = Self(4u32);
    pub const NearEnd: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterCartridgeSensors {}
impl ::core::clone::Clone for PosPrinterCartridgeSensors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterCartridgeSensors {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterCartridgeSensors {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterCartridgeSensors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterCartridgeSensors").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterCartridgeSensors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterCartridgeSensors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterCartridgeSensors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterCartridgeSensors {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterCartridgeSensors;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct PosPrinterCharacterSetIds;
impl PosPrinterCharacterSetIds {
    pub fn Utf16LE() -> ::windows_core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Utf16LE)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ascii() -> ::windows_core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ascii)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn Ansi() -> ::windows_core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Ansi)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IPosPrinterCharacterSetIdsStatics<R, F: FnOnce(&IPosPrinterCharacterSetIdsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PosPrinterCharacterSetIds, IPosPrinterCharacterSetIdsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PosPrinterCharacterSetIds {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCharacterSetIds";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterColorCapabilities(pub u32);
impl PosPrinterColorCapabilities {
    pub const None: Self = Self(0u32);
    pub const Primary: Self = Self(1u32);
    pub const Custom1: Self = Self(2u32);
    pub const Custom2: Self = Self(4u32);
    pub const Custom3: Self = Self(8u32);
    pub const Custom4: Self = Self(16u32);
    pub const Custom5: Self = Self(32u32);
    pub const Custom6: Self = Self(64u32);
    pub const Cyan: Self = Self(128u32);
    pub const Magenta: Self = Self(256u32);
    pub const Yellow: Self = Self(512u32);
    pub const Full: Self = Self(1024u32);
}
impl ::core::marker::Copy for PosPrinterColorCapabilities {}
impl ::core::clone::Clone for PosPrinterColorCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterColorCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterColorCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterColorCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterColorCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterColorCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterColorCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterColorCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterColorCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterColorCartridge(pub i32);
impl PosPrinterColorCartridge {
    pub const Unknown: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Custom1: Self = Self(2i32);
    pub const Custom2: Self = Self(3i32);
    pub const Custom3: Self = Self(4i32);
    pub const Custom4: Self = Self(5i32);
    pub const Custom5: Self = Self(6i32);
    pub const Custom6: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
    pub const Magenta: Self = Self(9i32);
    pub const Yellow: Self = Self(10i32);
}
impl ::core::marker::Copy for PosPrinterColorCartridge {}
impl ::core::clone::Clone for PosPrinterColorCartridge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterColorCartridge {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterColorCartridge {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterColorCartridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterColorCartridge").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterColorCartridge {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCartridge;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinterFontProperty(::windows_core::IUnknown);
impl PosPrinterFontProperty {
    pub fn TypeFace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TypeFace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsScalableToAnySize(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScalableToAnySize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CharacterSizes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SizeUInt32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSizes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SizeUInt32>>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterFontProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterFontProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterFontProperty {}
impl ::core::fmt::Debug for PosPrinterFontProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterFontProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterFontProperty {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterFontProperty;{a7f4e93a-f8ac-5f04-84d2-29b16d8a633c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterFontProperty as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterFontProperty";
}
impl ::core::convert::From<PosPrinterFontProperty> for ::windows_core::IUnknown {
    fn from(value: PosPrinterFontProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterFontProperty> for ::windows_core::IUnknown {
    fn from(value: &PosPrinterFontProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterFontProperty> for ::windows_core::IInspectable {
    fn from(value: PosPrinterFontProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterFontProperty> for ::windows_core::IInspectable {
    fn from(value: &PosPrinterFontProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterFontProperty {}
unsafe impl ::core::marker::Sync for PosPrinterFontProperty {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: Self = Self(0i32);
    pub const Vertical: Self = Self(1i32);
}
impl ::core::marker::Copy for PosPrinterLineDirection {}
impl ::core::clone::Clone for PosPrinterLineDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterLineDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterLineDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterLineDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterLineDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterLineDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: Self = Self(0i32);
    pub const DoubleSolid: Self = Self(1i32);
    pub const Broken: Self = Self(2i32);
    pub const Chain: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterLineStyle {}
impl ::core::clone::Clone for PosPrinterLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterLineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterLineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterLineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterLineStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: Self = Self(0i32);
    pub const Twips: Self = Self(1i32);
    pub const English: Self = Self(2i32);
    pub const Metric: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMapMode {}
impl ::core::clone::Clone for PosPrinterMapMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMapMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterMapMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMapMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMapMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterMapMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMapMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: Self = Self(0u32);
    pub const ToTakeUp: Self = Self(1u32);
    pub const ToCutter: Self = Self(2u32);
    pub const ToCurrentTopOfForm: Self = Self(4u32);
    pub const ToNextTopOfForm: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedCapabilities {}
impl ::core::clone::Clone for PosPrinterMarkFeedCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMarkFeedCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterMarkFeedCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMarkFeedCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterMarkFeedCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterMarkFeedCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterMarkFeedCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: Self = Self(0i32);
    pub const ToCutter: Self = Self(1i32);
    pub const ToCurrentTopOfForm: Self = Self(2i32);
    pub const ToNextTopOfForm: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedKind {}
impl ::core::clone::Clone for PosPrinterMarkFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMarkFeedKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterMarkFeedKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMarkFeedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterMarkFeedKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinterPrintOptions(::windows_core::IUnknown);
impl PosPrinterPrintOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PosPrinterPrintOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TypeFace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TypeFace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTypeFace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTypeFace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CharacterHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharacterHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCharacterHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharacterHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bold(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Bold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetBold(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBold)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Italic(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Italic)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetItalic(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetItalic)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Underline(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Underline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUnderline(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnderline)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverseVideo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ReverseVideo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetReverseVideo(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReverseVideo)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Strikethrough(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Strikethrough)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStrikethrough(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStrikethrough)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Superscript(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Superscript)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSuperscript(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuperscript)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Subscript(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Subscript)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSubscript(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubscript)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DoubleWide(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DoubleWide)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDoubleWide(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDoubleWide)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DoubleHigh(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DoubleHigh)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDoubleHigh(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDoubleHigh)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Alignment(&self) -> ::windows_core::Result<PosPrinterAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).Alignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterAlignment>(result__)
        }
    }
    pub fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCharacterSet(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharacterSet)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PosPrinterPrintOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterPrintOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterPrintOptions {}
impl ::core::fmt::Debug for PosPrinterPrintOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterPrintOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterPrintOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterPrintOptions;{0a2e16fd-1d02-5a58-9d59-bfcde76fde86})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterPrintOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterPrintOptions";
}
impl ::core::convert::From<PosPrinterPrintOptions> for ::windows_core::IUnknown {
    fn from(value: PosPrinterPrintOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterPrintOptions> for ::windows_core::IUnknown {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterPrintOptions> for ::windows_core::IInspectable {
    fn from(value: PosPrinterPrintOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterPrintOptions> for ::windows_core::IInspectable {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterPrintOptions {}
unsafe impl ::core::marker::Sync for PosPrinterPrintOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: Self = Self(0i32);
    pub const Side1: Self = Self(1i32);
    pub const Side2: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterPrintSide {}
impl ::core::clone::Clone for PosPrinterPrintSide {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterPrintSide {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterPrintSide {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterPrintSide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterPrintSide").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterPrintSide {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterPrintSide;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(::windows_core::IUnknown);
impl PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::clone::Clone for PosPrinterReleaseDeviceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterReleaseDeviceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::fmt::Debug for PosPrinterReleaseDeviceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterReleaseDeviceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs;{2bcba359-1cef-40b2-9ecb-f927f856ae3c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterReleaseDeviceRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs";
}
impl ::core::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterReleaseDeviceRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: Self = Self(0i32);
    pub const Right90: Self = Self(1i32);
    pub const Left90: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterRotation {}
impl ::core::clone::Clone for PosPrinterRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterRotation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for PosPrinterRuledLineCapabilities {}
impl ::core::clone::Clone for PosPrinterRuledLineCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterRuledLineCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterRuledLineCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterRuledLineCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterRuledLineCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterRuledLineCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterRuledLineCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterRuledLineCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRuledLineCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinterStatus(::windows_core::IUnknown);
impl PosPrinterStatus {
    pub fn StatusKind(&self) -> ::windows_core::Result<PosPrinterStatusKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterStatusKind>::zeroed();
            (::windows_core::Interface::vtable(this).StatusKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterStatusKind>(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterStatus {}
impl ::core::fmt::Debug for PosPrinterStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatus;{d1f0c730-da40-4328-bf76-5156fa33b747})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatus";
}
impl ::core::convert::From<PosPrinterStatus> for ::windows_core::IUnknown {
    fn from(value: PosPrinterStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatus> for ::windows_core::IUnknown {
    fn from(value: &PosPrinterStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinterStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterStatus> for ::windows_core::IInspectable {
    fn from(value: PosPrinterStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatus> for ::windows_core::IInspectable {
    fn from(value: &PosPrinterStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinterStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterStatus {}
unsafe impl ::core::marker::Sync for PosPrinterStatus {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for PosPrinterStatusKind {}
impl ::core::clone::Clone for PosPrinterStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterStatusKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PosPrinterStatusKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterStatusKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatusKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterStatusKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterStatusKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PosPrinterStatusUpdatedEventArgs(::windows_core::IUnknown);
impl PosPrinterStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for PosPrinterStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs;{2edb87df-13a6-428d-ba81-b0e7c3e5a3cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPosPrinterStatusUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs";
}
impl ::core::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterStatusUpdatedEventArgs {}
#[repr(transparent)]
pub struct ReceiptPrintJob(::windows_core::IUnknown);
impl ReceiptPrintJob {
    pub fn Print<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintLine)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintNewline)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBarcodeRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrintRotation)(::windows_core::Interface::as_raw(this), value, includebitmaps).ok() }
    }
    pub fn SetPrintArea<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrintArea)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmap<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomAlignedBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintSavedBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DrawRuledLine)(::windows_core::Interface::as_raw(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintBarcode)(::windows_core::Interface::as_raw(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintBarcodeCustomAlign)(::windows_core::Interface::as_raw(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintCustomAlignedBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    pub fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MarkFeed)(::windows_core::Interface::as_raw(this), kind).ok() }
    }
    pub fn CutPaper(&self, percentage: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CutPaper)(::windows_core::Interface::as_raw(this), percentage).ok() }
    }
    pub fn CutPaperDefault(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CutPaperDefault)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StampPaper(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StampPaper)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Print2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FeedPaperByLine)(::windows_core::Interface::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FeedPaperByMapModeUnit)(::windows_core::Interface::as_raw(this), distance).ok() }
    }
}
impl ::core::clone::Clone for ReceiptPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReceiptPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReceiptPrintJob {}
impl ::core::fmt::Debug for ReceiptPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReceiptPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ReceiptPrintJob {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrintJob;{aa96066e-acad-4b79-9d0f-c0cfc08dc77b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
    const IID: ::windows_core::GUID = <IReceiptPrintJob as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrintJob";
}
impl ::core::convert::From<ReceiptPrintJob> for ::windows_core::IUnknown {
    fn from(value: ReceiptPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrintJob> for ::windows_core::IUnknown {
    fn from(value: &ReceiptPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReceiptPrintJob> for ::windows_core::IInspectable {
    fn from(value: ReceiptPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrintJob> for ::windows_core::IInspectable {
    fn from(value: &ReceiptPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows_core::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows_core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IReceiptOrSlipJob> for ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IReceiptOrSlipJob> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IReceiptOrSlipJob> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IReceiptOrSlipJob> {
        ::core::convert::TryInto::<IReceiptOrSlipJob>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ReceiptPrintJob {}
unsafe impl ::core::marker::Sync for ReceiptPrintJob {}
#[repr(transparent)]
pub struct ReceiptPrinterCapabilities(::windows_core::IUnknown);
impl ReceiptPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrinterPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDualColorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows_core::Result<PosPrinterColorCapabilities> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterCartridgeSensors>::zeroed();
            (::windows_core::Interface::vtable(this).CartridgeSensors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBoldSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsItalicSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn IsBarcodeSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBarcodeSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLeft90RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRight90RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Is180RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrintAreaSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> ::windows_core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterRuledLineCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).RuledLineCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBarcodeRotations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBitmapRotations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBitmapRotations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    pub fn CanCutPaper(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCutPaper)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStampSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStampSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MarkFeedCapabilities(&self) -> ::windows_core::Result<PosPrinterMarkFeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterMarkFeedCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).MarkFeedCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterMarkFeedCapabilities>(result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReverseVideoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStrikethroughSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSuperscriptSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSubscriptSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ReceiptPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReceiptPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReceiptPrinterCapabilities {}
impl ::core::fmt::Debug for ReceiptPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReceiptPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ReceiptPrinterCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrinterCapabilities;{b8f0b58f-51a8-43fc-9bd5-8de272a6415b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IReceiptPrinterCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrinterCapabilities";
}
impl ::core::convert::From<ReceiptPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReceiptPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonReceiptSlipCapabilities> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonReceiptSlipCapabilities> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::core::convert::TryInto::<ICommonReceiptSlipCapabilities>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ReceiptPrinterCapabilities {}
unsafe impl ::core::marker::Sync for ReceiptPrinterCapabilities {}
#[repr(C)]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for SizeUInt32 {}
impl ::core::clone::Clone for SizeUInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SizeUInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SizeUInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows_core::Abi for SizeUInt32 {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for SizeUInt32 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.PointOfService.SizeUInt32;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for SizeUInt32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SizeUInt32>()) == 0 }
    }
}
impl ::core::cmp::Eq for SizeUInt32 {}
impl ::core::default::Default for SizeUInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct SlipPrintJob(::windows_core::IUnknown);
impl SlipPrintJob {
    pub fn Print<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintLine)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PrintNewline)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExecuteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBarcodeRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrintRotation)(::windows_core::Interface::as_raw(this), value, includebitmaps).ok() }
    }
    pub fn SetPrintArea<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrintArea)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmap<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomAlignedBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(::windows_core::Interface::as_raw(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintSavedBitmap)(::windows_core::Interface::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DrawRuledLine)(::windows_core::Interface::as_raw(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBarcode)(::windows_core::Interface::as_raw(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBarcodeCustomAlign)(::windows_core::Interface::as_raw(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintCustomAlignedBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    pub fn Print2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Print)(::windows_core::Interface::as_raw(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FeedPaperByLine)(::windows_core::Interface::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FeedPaperByMapModeUnit)(::windows_core::Interface::as_raw(this), distance).ok() }
    }
}
impl ::core::clone::Clone for SlipPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlipPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlipPrintJob {}
impl ::core::fmt::Debug for SlipPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlipPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SlipPrintJob {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrintJob;{532199be-c8c3-4dc2-89e9-5c4a37b34ddc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SlipPrintJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
    const IID: ::windows_core::GUID = <IReceiptOrSlipJob as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrintJob";
}
impl ::core::convert::From<SlipPrintJob> for ::windows_core::IUnknown {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrintJob> for ::windows_core::IUnknown {
    fn from(value: &SlipPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlipPrintJob> for ::windows_core::IInspectable {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrintJob> for ::windows_core::IInspectable {
    fn from(value: &SlipPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: SlipPrintJob) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows_core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPosPrinterJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SlipPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows_core::Error;
    fn try_from(value: SlipPrintJob) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows_core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IReceiptOrSlipJob> for SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IReceiptOrSlipJob> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IReceiptOrSlipJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows_core::Param<'a, IReceiptOrSlipJob> {
        ::core::convert::TryInto::<IReceiptOrSlipJob>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlipPrintJob {}
unsafe impl ::core::marker::Sync for SlipPrintJob {}
#[repr(transparent)]
pub struct SlipPrinterCapabilities(::windows_core::IUnknown);
impl SlipPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrinterPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDualColorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows_core::Result<PosPrinterColorCapabilities> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterColorCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).ColorCartridgeCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows_core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterCartridgeSensors>::zeroed();
            (::windows_core::Interface::vtable(this).CartridgeSensors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBoldSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsItalicSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighPrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperEmptySensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCharactersPerLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn IsBarcodeSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBarcodeSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLeft90RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRight90RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Is180RotationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrintAreaSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> ::windows_core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PosPrinterRuledLineCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).RuledLineCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBarcodeRotations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedBitmapRotations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows_core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBitmapRotations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    pub fn IsFullLengthSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFullLengthSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBothSidesPrintingSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBothSidesPrintingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReverseVideoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStrikethroughSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSuperscriptSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSubscriptSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SlipPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlipPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlipPrinterCapabilities {}
impl ::core::fmt::Debug for SlipPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlipPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SlipPrinterCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrinterCapabilities;{99b16399-488c-4157-8ac2-9f57f708d3db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <ISlipPrinterCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrinterCapabilities";
}
impl ::core::convert::From<SlipPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: SlipPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrinterCapabilities> for ::windows_core::IUnknown {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlipPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: SlipPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrinterCapabilities> for ::windows_core::IInspectable {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows_core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonReceiptSlipCapabilities> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommonReceiptSlipCapabilities> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::core::convert::TryInto::<ICommonReceiptSlipCapabilities>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlipPrinterCapabilities {}
unsafe impl ::core::marker::Sync for SlipPrinterCapabilities {}
#[repr(transparent)]
pub struct UnifiedPosErrorData(::windows_core::IUnknown);
impl UnifiedPosErrorData {
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Severity(&self) -> ::windows_core::Result<UnifiedPosErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosErrorSeverity>::zeroed();
            (::windows_core::Interface::vtable(this).Severity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosErrorSeverity>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<UnifiedPosErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UnifiedPosErrorReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnifiedPosErrorReason>(result__)
        }
    }
    pub fn ExtendedReason(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(message: Param0, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows_core::Result<UnifiedPosErrorData> {
        Self::IUnifiedPosErrorDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), message.into_param().abi(), severity, reason, extendedreason, result__.as_mut_ptr()).from_abi::<UnifiedPosErrorData>(result__)
        })
    }
    pub fn IUnifiedPosErrorDataFactory<R, F: FnOnce(&IUnifiedPosErrorDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UnifiedPosErrorData, IUnifiedPosErrorDataFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UnifiedPosErrorData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnifiedPosErrorData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnifiedPosErrorData {}
impl ::core::fmt::Debug for UnifiedPosErrorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnifiedPosErrorData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.UnifiedPosErrorData;{2b998c3a-555c-4889-8ed8-c599bb3a712a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
    const IID: ::windows_core::GUID = <IUnifiedPosErrorData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.UnifiedPosErrorData";
}
impl ::core::convert::From<UnifiedPosErrorData> for ::windows_core::IUnknown {
    fn from(value: UnifiedPosErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnifiedPosErrorData> for ::windows_core::IUnknown {
    fn from(value: &UnifiedPosErrorData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnifiedPosErrorData> for ::windows_core::IInspectable {
    fn from(value: UnifiedPosErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnifiedPosErrorData> for ::windows_core::IInspectable {
    fn from(value: &UnifiedPosErrorData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnifiedPosErrorData {}
unsafe impl ::core::marker::Sync for UnifiedPosErrorData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosErrorReason(pub i32);
impl UnifiedPosErrorReason {
    pub const UnknownErrorReason: Self = Self(0i32);
    pub const NoService: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
    pub const Illegal: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
    pub const Closed: Self = Self(5i32);
    pub const Offline: Self = Self(6i32);
    pub const Failure: Self = Self(7i32);
    pub const Timeout: Self = Self(8i32);
    pub const Busy: Self = Self(9i32);
    pub const Extended: Self = Self(10i32);
}
impl ::core::marker::Copy for UnifiedPosErrorReason {}
impl ::core::clone::Clone for UnifiedPosErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosErrorReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UnifiedPosErrorReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosErrorReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnifiedPosErrorReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosErrorSeverity(pub i32);
impl UnifiedPosErrorSeverity {
    pub const UnknownErrorSeverity: Self = Self(0i32);
    pub const Warning: Self = Self(1i32);
    pub const Recoverable: Self = Self(2i32);
    pub const Unrecoverable: Self = Self(3i32);
    pub const AssistanceRequired: Self = Self(4i32);
    pub const Fatal: Self = Self(5i32);
}
impl ::core::marker::Copy for UnifiedPosErrorSeverity {}
impl ::core::clone::Clone for UnifiedPosErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosErrorSeverity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UnifiedPosErrorSeverity {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosErrorSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorSeverity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnifiedPosErrorSeverity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorSeverity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: Self = Self(0i32);
    pub const POSInternal: Self = Self(1i32);
    pub const External: Self = Self(2i32);
    pub const Interactive: Self = Self(3i32);
}
impl ::core::marker::Copy for UnifiedPosHealthCheckLevel {}
impl ::core::clone::Clone for UnifiedPosHealthCheckLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosHealthCheckLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UnifiedPosHealthCheckLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosHealthCheckLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosHealthCheckLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnifiedPosHealthCheckLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosHealthCheckLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Advanced: Self = Self(2i32);
}
impl ::core::marker::Copy for UnifiedPosPowerReportingType {}
impl ::core::clone::Clone for UnifiedPosPowerReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosPowerReportingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UnifiedPosPowerReportingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosPowerReportingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosPowerReportingType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnifiedPosPowerReportingType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosPowerReportingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
