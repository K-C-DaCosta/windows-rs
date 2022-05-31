#[repr(transparent)]
pub struct AppInstallItem(::windows_core::IUnknown);
impl AppInstallItem {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InstallType(&self) -> ::windows_core::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallType>::zeroed();
            (::windows_core::Interface::vtable(this).InstallType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallType>(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUserInitiated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows_core::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallStatus>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Restart(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Restart)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Completed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppInstallItem, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppInstallItem, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CancelWithTelemetry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, correlationvector: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CancelWithTelemetry)(::windows_core::Interface::as_raw(this), correlationvector.into_param().abi()).ok() }
    }
    pub fn PauseWithTelemetry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, correlationvector: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PauseWithTelemetry)(::windows_core::Interface::as_raw(this), correlationvector.into_param().abi()).ok() }
    }
    pub fn RestartWithTelemetry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, correlationvector: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RestartWithTelemetry)(::windows_core::Interface::as_raw(this), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    pub fn ItemOperationsMightAffectOtherItems(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ItemOperationsMightAffectOtherItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLaunchAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PinToStartAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToStartAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallationToastNotificationMode>::zeroed();
            (::windows_core::Interface::vtable(this).CompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallationToastNotificationMode>::zeroed();
            (::windows_core::Interface::vtable(this).InstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppInstallItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallItem {}
impl ::core::fmt::Debug for AppInstallItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem;{49d3dfab-168a-4cbf-a93a-9e448c82737d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
impl ::core::convert::From<AppInstallItem> for ::windows_core::IUnknown {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallItem> for ::windows_core::IUnknown {
    fn from(value: &AppInstallItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallItem> for ::windows_core::IInspectable {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallItem> for ::windows_core::IInspectable {
    fn from(value: &AppInstallItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallItem {}
unsafe impl ::core::marker::Sync for AppInstallItem {}
#[repr(transparent)]
pub struct AppInstallManager(::windows_core::IUnknown);
impl AppInstallManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppInstallManager, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AppInstallItems(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    pub fn Cancel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), productid.into_param().abi()).ok() }
    }
    pub fn Pause<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this), productid.into_param().abi()).ok() }
    }
    pub fn Restart<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Restart)(::windows_core::Interface::as_raw(this), productid.into_param().abi()).ok() }
    }
    pub fn ItemCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveItemCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ItemStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveItemStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AutoUpdateSetting(&self) -> ::windows_core::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AutoUpdateSetting>::zeroed();
            (::windows_core::Interface::vtable(this).AutoUpdateSetting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutoUpdateSetting>(result__)
        }
    }
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoUpdateSetting)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionIdentity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AcquisitionIdentity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAcquisitionIdentity<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAcquisitionIdentity)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn GetIsApplicableAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsApplicableAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn StartAppInstallAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAppInstallAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    pub fn UpdateAppByPackageFamilyNameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    pub fn SearchForUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SearchForAllUpdatesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    pub fn IsStoreBlockedByPolicyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storeclientname: Param0, storeclientpublisher: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsStoreBlockedByPolicyAsync)(::windows_core::Interface::as_raw(this), storeclientname.into_param().abi(), storeclientpublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetIsAppAllowedToInstallAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAppAllowedToInstallAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn StartAppInstallWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param5: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param6: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool, catalogid: Param4, bundleid: Param5, correlationvector: Param6) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAppInstallWithTelemetryAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, catalogid.into_param().abi(), bundleid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0, correlationvector: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameWithTelemetryAsync)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    pub fn SearchForUpdatesWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, skuid: Param1, catalogid: Param2, correlationvector: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesWithTelemetryAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SearchForAllUpdatesWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, correlationvector: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesWithTelemetryAsync)(::windows_core::Interface::as_raw(this), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, skuid: Param1, catalogid: Param2, correlationvector: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAppAllowedToInstallWithTelemetryAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CancelWithTelemetry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CancelWithTelemetry)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn PauseWithTelemetry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PauseWithTelemetry)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn RestartWithTelemetry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RestartWithTelemetry)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-management"))]
    pub fn StartProductInstallAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param6: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param7: ::windows_core::IntoParam<'a, ::winrt_management::Deployment::PackageVolume>>(&self, productid: Param0, catalogid: Param1, flightid: Param2, clientid: Param3, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: Param6, targetvolume: Param7) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-management", feature = "winrt-system"))]
    pub fn StartProductInstallForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param7: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param8: ::windows_core::IntoParam<'a, ::winrt_management::Deployment::PackageVolume>>(
        &self,
        user: Param0,
        productid: Param1,
        catalogid: Param2,
        flightid: Param3,
        clientid: Param4,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: Param7,
        targetvolume: Param8,
    ) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UpdateAppByPackageFamilyNameForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, packagefamilyname: Param1, correlationvector: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn SearchForUpdatesForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2, catalogid: Param3, correlationvector: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub fn SearchForAllUpdatesForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, correlationvector: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetIsAppAllowedToInstallForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2, catalogid: Param3, correlationvector: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAppAllowedToInstallForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetIsApplicableForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsApplicableForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn MoveToFrontOfDownloadQueue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).MoveToFrontOfDownloadQueue)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn GetFreeUserEntitlementAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFreeUserEntitlementAsync)(::windows_core::Interface::as_raw(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetFreeUserEntitlementForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, storeid: Param1, campaignid: Param2, correlationvector: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFreeUserEntitlementForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    pub fn GetFreeDeviceEntitlementAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFreeDeviceEntitlementAsync)(::windows_core::Interface::as_raw(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallItemsWithGroupSupport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, AppUpdateOptions>>(&self, correlationvector: Param0, clientid: Param1, updateoptions: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsAsync)(::windows_core::Interface::as_raw(this), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, AppUpdateOptions>>(&self, user: Param0, correlationvector: Param1, clientid: Param2, updateoptions: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    pub fn SearchForUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, AppUpdateOptions>>(&self, productid: Param0, skuid: Param1, correlationvector: Param2, clientid: Param3, updateoptions: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param5: ::windows_core::IntoParam<'a, AppUpdateOptions>>(&self, user: Param0, productid: Param1, skuid: Param2, correlationvector: Param3, clientid: Param4, updateoptions: Param5) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StartProductInstallWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, AppInstallOptions>>(&self, productid: Param0, flightid: Param1, clientid: Param2, correlationvector: Param3, installoptions: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallWithOptionsAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub fn StartProductInstallWithOptionsForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param5: ::windows_core::IntoParam<'a, AppInstallOptions>>(&self, user: Param0, productid: Param1, flightid: Param2, clientid: Param3, correlationvector: Param4, installoptions: Param5) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallWithOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    pub fn GetIsPackageIdentityAllowedToInstallAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, correlationvector: Param0, packageidentityname: Param1, publishercertificatename: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallAsync)(::windows_core::Interface::as_raw(this), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, user: Param0, correlationvector: Param1, packageidentityname: Param2, publishercertificatename: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CanInstallForAllUsers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanInstallForAllUsers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManager {}
impl ::core::fmt::Debug for AppInstallManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager;{9353e170-8441-4b45-bd72-7c2fa925beee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
impl ::core::convert::From<AppInstallManager> for ::windows_core::IUnknown {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManager> for ::windows_core::IUnknown {
    fn from(value: &AppInstallManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallManager> for ::windows_core::IInspectable {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManager> for ::windows_core::IInspectable {
    fn from(value: &AppInstallManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallManager {}
unsafe impl ::core::marker::Sync for AppInstallManager {}
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(::windows_core::IUnknown);
impl AppInstallManagerItemEventArgs {
    pub fn Item(&self) -> ::windows_core::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallItem>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManagerItemEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManagerItemEventArgs {}
impl ::core::fmt::Debug for AppInstallManagerItemEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManagerItemEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs;{bc505743-4674-4dd1-957e-c25682086a14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallManagerItemEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
impl ::core::convert::From<AppInstallManagerItemEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManagerItemEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallManagerItemEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManagerItemEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallManagerItemEventArgs {}
unsafe impl ::core::marker::Sync for AppInstallManagerItemEventArgs {}
#[repr(transparent)]
pub struct AppInstallOptions(::windows_core::IUnknown);
impl AppInstallOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppInstallOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CatalogId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CatalogId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCatalogId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCatalogId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ForceUseOfNonRemovableStorage(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceUseOfNonRemovableStorage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceUseOfNonRemovableStorage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowForcedAppRestart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Repair(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Repair)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRepair(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepair)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-management")]
    pub fn TargetVolume(&self) -> ::windows_core::Result<::winrt_management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_management::Deployment::PackageVolume>(result__)
        }
    }
    #[cfg(feature = "winrt-management")]
    pub fn SetTargetVolume<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_management::Deployment::PackageVolume>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetVolume)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLaunchAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PinToStartAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToStartAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallationToastNotificationMode>::zeroed();
            (::windows_core::Interface::vtable(this).CompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallationToastNotificationMode>::zeroed();
            (::windows_core::Interface::vtable(this).InstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallForAllUsers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InstallForAllUsers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallForAllUsers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallForAllUsers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageButDoNotInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StageButDoNotInstall)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageButDoNotInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStageButDoNotInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CampaignId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CampaignId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCampaignId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCampaignId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExtendedCampaignId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedCampaignId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetExtendedCampaignId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendedCampaignId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallOptions {}
impl ::core::fmt::Debug for AppInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions;{c9808300-1cb8-4eb6-8c9f-6a30c64a5b51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
impl ::core::convert::From<AppInstallOptions> for ::windows_core::IUnknown {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallOptions> for ::windows_core::IUnknown {
    fn from(value: &AppInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallOptions> for ::windows_core::IInspectable {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallOptions> for ::windows_core::IInspectable {
    fn from(value: &AppInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallOptions {}
unsafe impl ::core::marker::Sync for AppInstallOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppInstallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppInstallStatus(::windows_core::IUnknown);
impl AppInstallStatus {
    pub fn InstallState(&self) -> ::windows_core::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallState>::zeroed();
            (::windows_core::Interface::vtable(this).InstallState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallState>(result__)
        }
    }
    pub fn DownloadSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn BytesDownloaded(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesDownloaded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PercentComplete(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PercentComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn ReadyForLaunch(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ReadyForLaunch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStaged(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaged)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallStatus {}
impl ::core::fmt::Debug for AppInstallStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus;{936dccfa-2450-4126-88b1-6127a644dd5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
impl ::core::convert::From<AppInstallStatus> for ::windows_core::IUnknown {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallStatus> for ::windows_core::IUnknown {
    fn from(value: &AppInstallStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallStatus> for ::windows_core::IInspectable {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallStatus> for ::windows_core::IInspectable {
    fn from(value: &AppInstallStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallStatus {}
unsafe impl ::core::marker::Sync for AppInstallStatus {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppInstallType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallationToastNotificationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppInstallationToastNotificationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallationToastNotificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallationToastNotificationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppUpdateOptions(::windows_core::IUnknown);
impl AppUpdateOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppUpdateOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CatalogId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CatalogId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCatalogId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCatalogId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowForcedAppRestart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticallyDownloadAndInstallUpdateIfFound)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutomaticallyDownloadAndInstallUpdateIfFound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppUpdateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUpdateOptions {}
impl ::core::fmt::Debug for AppUpdateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUpdateOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppUpdateOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions;{26f0b02f-c2f3-4aea-af8c-6308dd9db85f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
    const IID: ::windows_core::GUID = <IAppUpdateOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
impl ::core::convert::From<AppUpdateOptions> for ::windows_core::IUnknown {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUpdateOptions> for ::windows_core::IUnknown {
    fn from(value: &AppUpdateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppUpdateOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppUpdateOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUpdateOptions> for ::windows_core::IInspectable {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUpdateOptions> for ::windows_core::IInspectable {
    fn from(value: &AppUpdateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppUpdateOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppUpdateOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUpdateOptions {}
unsafe impl ::core::marker::Sync for AppUpdateOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutoUpdateSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoUpdateSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GetEntitlementResult(::windows_core::IUnknown);
impl GetEntitlementResult {
    pub fn Status(&self) -> ::windows_core::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GetEntitlementStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GetEntitlementStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for GetEntitlementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetEntitlementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetEntitlementResult {}
impl ::core::fmt::Debug for GetEntitlementResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult;{74fc843f-1a9e-4609-8e4d-819086d08a3d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
    const IID: ::windows_core::GUID = <IGetEntitlementResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
impl ::core::convert::From<GetEntitlementResult> for ::windows_core::IUnknown {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetEntitlementResult> for ::windows_core::IUnknown {
    fn from(value: &GetEntitlementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GetEntitlementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GetEntitlementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GetEntitlementResult> for ::windows_core::IInspectable {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetEntitlementResult> for ::windows_core::IInspectable {
    fn from(value: &GetEntitlementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GetEntitlementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GetEntitlementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GetEntitlementResult {}
unsafe impl ::core::marker::Sync for GetEntitlementResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GetEntitlementStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GetEntitlementStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GetEntitlementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem2 {
    type Vtable = IAppInstallItem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3972af8_40c0_4fd7_aa6c_0aa13ca6188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem3 {
    type Vtable = IAppInstallItem3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f3dc998_dd47_433c_9234_560172d67a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Children: usize,
    pub ItemOperationsMightAffectOtherItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem4 {
    type Vtable = IAppInstallItem4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2d1ce12_71ff_4fc8_b540_453d4b37e1d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem5 {
    type Vtable = IAppInstallItem5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5510e7cc_4076_4a0b_9472_c21d9d380e55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9353e170_8441_4b45_bd72_7c2fa925beee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AppInstallItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AppInstallItems: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows_core::HRESULT,
    pub SetAutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows_core::HRESULT,
    pub AcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetIsApplicableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartAppInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateAppByPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SearchForUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SearchForAllUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SearchForAllUpdatesAsync: usize,
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeclientname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, storeclientpublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsAppAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager2 {
    type Vtable = IAppInstallManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16937851_ed37_480d_8314_52e27c03f04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartAppInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, bundleid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateAppByPackageFamilyNameWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SearchForUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SearchForAllUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SearchForAllUpdatesWithTelemetryAsync: usize,
    pub GetIsAppAllowedToInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager3 {
    type Vtable = IAppInstallManager3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95b24b17_e96a_4d0e_84e1_c8cb417a0178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-management"))]
    pub StartProductInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-management")))]
    StartProductInstallAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-management", feature = "winrt-system"))]
    pub StartProductInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-management", feature = "winrt-system")))]
    StartProductInstallForUserAsync: usize,
    #[cfg(feature = "winrt-system")]
    pub UpdateAppByPackageFamilyNameForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    UpdateAppByPackageFamilyNameForUserAsync: usize,
    #[cfg(feature = "winrt-system")]
    pub SearchForUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    SearchForUpdatesForUserAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub SearchForAllUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-system")))]
    SearchForAllUpdatesForUserAsync: usize,
    #[cfg(feature = "winrt-system")]
    pub GetIsAppAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetIsAppAllowedToInstallForUserAsync: usize,
    #[cfg(feature = "winrt-system")]
    pub GetIsApplicableForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetIsApplicableForUserAsync: usize,
    pub MoveToFrontOfDownloadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager4 {
    type Vtable = IAppInstallManager4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x260a2a16_5a9e_4ebd_b944_f2ba75c31159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub GetFreeUserEntitlementForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetFreeUserEntitlementForUserAsync: usize,
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager5 {
    type Vtable = IAppInstallManager5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cd7be4c_1be9_4f7f_b675_aa1d64a529b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AppInstallItemsWithGroupSupport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager6 {
    type Vtable = IAppInstallManager6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9e7d408_f27a_4471_b2f4_e76efcbebcca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SearchForAllUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, updateoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub SearchForAllUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, updateoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-system")))]
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    pub SearchForUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, updateoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub SearchForUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, updateoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, installoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StartProductInstallWithOptionsAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub StartProductInstallWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, installoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-system")))]
    StartProductInstallWithOptionsForUserAsync: usize,
    pub GetIsPackageIdentityAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub GetIsPackageIdentityAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetIsPackageIdentityAllowedToInstallForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager7 {
    type Vtable = IAppInstallManager7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5ee7b30_d5e4_49a3_9853_3db03203321d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc505743_4674_4dd1_957e_c25682086a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRepair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-management")]
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-management"))]
    TargetVolume: usize,
    #[cfg(feature = "winrt-management")]
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-management"))]
    SetTargetVolume: usize,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallOptions2 {
    type Vtable = IAppInstallOptions2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a04c0d7_c94b_425e_95b4_bf27faeaee89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub InstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub StageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x936dccfa_2450_4126_88b1_6127a644dd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows_core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallStatus2 {
    type Vtable = IAppInstallStatus2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96e7818a_5e92_4aa9_8edc_58fed4b87e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
    pub ReadyForLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallStatus3 {
    type Vtable = IAppInstallStatus3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb880c56_837b_4b4c_9ebb_6d44a0a96307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUpdateOptions2 {
    type Vtable = IAppUpdateOptions2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4646e08_ed26_4bf9_9679_48f628e53df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetEntitlementResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetEntitlementResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows_core::HRESULT,
}
