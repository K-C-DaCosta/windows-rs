#[repr(transparent)]
pub struct DesignerAppExitedEventArgs(::windows_core::IUnknown);
impl DesignerAppExitedEventArgs {
    pub fn ExitCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for DesignerAppExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesignerAppExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppExitedEventArgs {}
impl ::core::fmt::Debug for DesignerAppExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppExitedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesignerAppExitedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs;{f6aac86a-0cad-410c-8f62-dc2936151c74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDesignerAppExitedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesignerAppExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppExitedEventArgs";
}
impl ::core::convert::From<DesignerAppExitedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DesignerAppExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppExitedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DesignerAppExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesignerAppExitedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DesignerAppExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppExitedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DesignerAppExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesignerAppExitedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesignerAppExitedEventArgs {}
unsafe impl ::core::marker::Sync for DesignerAppExitedEventArgs {}
#[repr(transparent)]
pub struct DesignerAppManager(::windows_core::IUnknown);
impl DesignerAppManager {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DesignerAppExited<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DesignerAppExited)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDesignerAppExited<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDesignerAppExited)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateNewViewAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, initialviewstate: DesignerAppViewState, initialviewsize: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DesignerAppView>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNewViewAsync)(::windows_core::Interface::as_raw(this), initialviewstate, initialviewsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DesignerAppView>>(result__)
        }
    }
    pub fn LoadObjectIntoAppAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, dllname: Param0, classid: Param1, initializationdata: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadObjectIntoAppAsync)(::windows_core::Interface::as_raw(this), dllname.into_param().abi(), classid.into_param().abi(), initializationdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appusermodelid: Param0) -> ::windows_core::Result<DesignerAppManager> {
        Self::IDesignerAppManagerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), appusermodelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<DesignerAppManager>(result__)
        })
    }
    pub fn IDesignerAppManagerFactory<R, F: FnOnce(&IDesignerAppManagerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DesignerAppManager, IDesignerAppManagerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DesignerAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesignerAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppManager {}
impl ::core::fmt::Debug for DesignerAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesignerAppManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppManager;{a6272caa-d5c6-40cb-abd9-27ba43831bb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesignerAppManager {
    type Vtable = IDesignerAppManager_Vtbl;
    const IID: ::windows_core::GUID = <IDesignerAppManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesignerAppManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppManager";
}
impl ::core::convert::From<DesignerAppManager> for ::windows_core::IUnknown {
    fn from(value: DesignerAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppManager> for ::windows_core::IUnknown {
    fn from(value: &DesignerAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesignerAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesignerAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesignerAppManager> for ::windows_core::IInspectable {
    fn from(value: DesignerAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppManager> for ::windows_core::IInspectable {
    fn from(value: &DesignerAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesignerAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesignerAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DesignerAppManager> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: DesignerAppManager) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesignerAppManager> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &DesignerAppManager) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for DesignerAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &DesignerAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesignerAppManager {}
unsafe impl ::core::marker::Sync for DesignerAppManager {}
#[repr(transparent)]
pub struct DesignerAppView(::windows_core::IUnknown);
impl DesignerAppView {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ViewState(&self) -> ::windows_core::Result<DesignerAppViewState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DesignerAppViewState>::zeroed();
            (::windows_core::Interface::vtable(this).ViewState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DesignerAppViewState>(result__)
        }
    }
    pub fn ViewSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn UpdateViewAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, viewstate: DesignerAppViewState, viewsize: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateViewAsync)(::windows_core::Interface::as_raw(this), viewstate, viewsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for DesignerAppView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesignerAppView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesignerAppView {}
impl ::core::fmt::Debug for DesignerAppView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesignerAppView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesignerAppView;{5c777cea-dd71-4a84-a56f-dacb4b14706f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesignerAppView {
    type Vtable = IDesignerAppView_Vtbl;
    const IID: ::windows_core::GUID = <IDesignerAppView as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesignerAppView {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesignerAppView";
}
impl ::core::convert::From<DesignerAppView> for ::windows_core::IUnknown {
    fn from(value: DesignerAppView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppView> for ::windows_core::IUnknown {
    fn from(value: &DesignerAppView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesignerAppView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesignerAppView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesignerAppView> for ::windows_core::IInspectable {
    fn from(value: DesignerAppView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesignerAppView> for ::windows_core::IInspectable {
    fn from(value: &DesignerAppView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesignerAppView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesignerAppView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DesignerAppView> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: DesignerAppView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesignerAppView> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &DesignerAppView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for DesignerAppView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &DesignerAppView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesignerAppView {}
unsafe impl ::core::marker::Sync for DesignerAppView {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for DesignerAppViewState {}
impl ::core::clone::Clone for DesignerAppViewState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DesignerAppViewState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DesignerAppViewState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DesignerAppViewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesignerAppViewState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesignerAppViewState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.DesignerAppViewState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DesktopWindowXamlSource(::windows_core::IUnknown);
impl DesktopWindowXamlSource {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn HasFocus(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasFocus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TakeFocusRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TakeFocusRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTakeFocusRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTakeFocusRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GotFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGotFocus)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NavigateFocus<'a, Param0: ::windows_core::IntoParam<'a, XamlSourceFocusNavigationRequest>>(&self, request: Param0) -> ::windows_core::Result<XamlSourceFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NavigateFocus)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationResult>(result__)
        }
    }
    pub fn new() -> ::windows_core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<DesktopWindowXamlSource>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<DesktopWindowXamlSource>(result__)
        })
    }
    pub fn IDesktopWindowXamlSourceFactory<R, F: FnOnce(&IDesktopWindowXamlSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DesktopWindowXamlSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowXamlSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowXamlSource {}
impl ::core::fmt::Debug for DesktopWindowXamlSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowXamlSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSource;{d585bfe1-00ff-51be-ba1d-a1329956ea0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
    const IID: ::windows_core::GUID = <IDesktopWindowXamlSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSource {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
}
impl ::core::convert::From<DesktopWindowXamlSource> for ::windows_core::IUnknown {
    fn from(value: DesktopWindowXamlSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSource> for ::windows_core::IUnknown {
    fn from(value: &DesktopWindowXamlSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesktopWindowXamlSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowXamlSource> for ::windows_core::IInspectable {
    fn from(value: DesktopWindowXamlSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSource> for ::windows_core::IInspectable {
    fn from(value: &DesktopWindowXamlSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesktopWindowXamlSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DesktopWindowXamlSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: DesktopWindowXamlSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopWindowXamlSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &DesktopWindowXamlSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for DesktopWindowXamlSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &DesktopWindowXamlSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSource {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSource {}
#[repr(transparent)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(::windows_core::IUnknown);
impl DesktopWindowXamlSourceGotFocusEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for DesktopWindowXamlSourceGotFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowXamlSourceGotFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowXamlSourceGotFocusEventArgs {}
impl ::core::fmt::Debug for DesktopWindowXamlSourceGotFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowXamlSourceGotFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesktopWindowXamlSourceGotFocusEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs;{39be4849-d9cc-5b70-8f05-1ad9a4aaa342})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDesktopWindowXamlSourceGotFocusEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSourceGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
}
impl ::core::convert::From<DesktopWindowXamlSourceGotFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceGotFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowXamlSourceGotFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceGotFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DesktopWindowXamlSourceGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesktopWindowXamlSourceGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceGotFocusEventArgs {}
#[repr(transparent)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows_core::IUnknown);
impl DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
impl ::core::fmt::Debug for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowXamlSourceTakeFocusRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs;{fe61e4b9-a7af-52b3-bdb9-c3305c0b8df2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDesktopWindowXamlSourceTakeFocusRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
}
impl ::core::convert::From<DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowXamlSourceTakeFocusRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DesktopWindowXamlSourceTakeFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
#[repr(transparent)]
pub struct ElementCompositionPreview(::windows_core::IUnknown);
impl ElementCompositionPreview {
    #[cfg(feature = "winrt-ui")]
    pub fn GetElementVisual<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows_core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetElementVisual)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetElementChildVisual<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows_core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetElementChildVisual)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetElementChildVisual<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>, Param1: ::windows_core::IntoParam<'a, super::super::Composition::Visual>>(element: Param0, visual: Param1) -> ::windows_core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetElementChildVisual)(::windows_core::Interface::as_raw(this), element.into_param().abi(), visual.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "winrt-ui", feature = "winrt-ui"))]
    pub fn GetScrollViewerManipulationPropertySet<'a, Param0: ::windows_core::IntoParam<'a, super::Controls::ScrollViewer>>(scrollviewer: Param0) -> ::windows_core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetScrollViewerManipulationPropertySet)(::windows_core::Interface::as_raw(this), scrollviewer.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetImplicitShowAnimation<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>, Param1: ::windows_core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows_core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetImplicitShowAnimation)(::windows_core::Interface::as_raw(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetImplicitHideAnimation<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>, Param1: ::windows_core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>>(element: Param0, animation: Param1) -> ::windows_core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetImplicitHideAnimation)(::windows_core::Interface::as_raw(this), element.into_param().abi(), animation.into_param().abi()).ok() })
    }
    pub fn SetIsTranslationEnabled<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsTranslationEnabled)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetPointerPositionPropertySet<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(targetelement: Param0) -> ::windows_core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPointerPositionPropertySet)(::windows_core::Interface::as_raw(this), targetelement.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetAppWindowContent<'a, Param0: ::windows_core::IntoParam<'a, super::super::WindowManagement::AppWindow>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(appwindow: Param0, xamlcontent: Param1) -> ::windows_core::Result<()> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetAppWindowContent)(::windows_core::Interface::as_raw(this), appwindow.into_param().abi(), xamlcontent.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetAppWindowContent<'a, Param0: ::windows_core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(appwindow: Param0) -> ::windows_core::Result<super::UIElement> {
        Self::IElementCompositionPreviewStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppWindowContent)(::windows_core::Interface::as_raw(this), appwindow.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        })
    }
    pub fn IElementCompositionPreviewStatics<R, F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IElementCompositionPreviewStatics2<R, F: FnOnce(&IElementCompositionPreviewStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IElementCompositionPreviewStatics3<R, F: FnOnce(&IElementCompositionPreviewStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ElementCompositionPreview, IElementCompositionPreviewStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ElementCompositionPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ElementCompositionPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ElementCompositionPreview {}
impl ::core::fmt::Debug for ElementCompositionPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositionPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.ElementCompositionPreview;{b6f1a676-cfe6-46ac-acf6-c4687bb65e60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
    const IID: ::windows_core::GUID = <IElementCompositionPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.ElementCompositionPreview";
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows_core::IUnknown {
    fn from(value: ElementCompositionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows_core::IUnknown {
    fn from(value: &ElementCompositionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ElementCompositionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ElementCompositionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows_core::IInspectable {
    fn from(value: ElementCompositionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows_core::IInspectable {
    fn from(value: &ElementCompositionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ElementCompositionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ElementCompositionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppExitedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppExitedEventArgs {
    type Vtable = IDesignerAppExitedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6aac86a_0cad_410c_8f62_dc2936151c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppExitedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppManager {
    type Vtable = IDesignerAppManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6272caa_d5c6_40cb_abd9_27ba43831bb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DesignerAppExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDesignerAppExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CreateNewViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialviewstate: DesignerAppViewState, initialviewsize: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoadObjectIntoAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dllname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, classid: ::windows_core::GUID, initializationdata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppManagerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppManagerFactory {
    type Vtable = IDesignerAppManagerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f9d633b_1266_4c0e_8499_0db85bbd4c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppManagerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignerAppView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignerAppView {
    type Vtable = IDesignerAppView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c777cea_dd71_4a84_a56f_dacb4b14706f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignerAppView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ViewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DesignerAppViewState) -> ::windows_core::HRESULT,
    pub ViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub UpdateViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewstate: DesignerAppViewState, viewsize: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd585bfe1_00ff_51be_ba1d_a1329956ea0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TakeFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NavigateFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceFactory {
    type Vtable = IDesktopWindowXamlSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cd61dc0_2561_56e1_8e75_6e44173805e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39be4849_d9cc_5b70_8f05_1ad9a4aaa342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe61e4b9_a7af_52b3_bdb9_c3305c0b8df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6f1a676_cfe6_46ac_acf6_c4687bb65e60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c92b38_ec99_4c55_bc85_a1c180b27646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub GetElementVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetElementVisual: usize,
    #[cfg(feature = "winrt-ui")]
    pub GetElementChildVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetElementChildVisual: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetElementChildVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, visual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetElementChildVisual: usize,
    #[cfg(all(feature = "winrt-ui", feature = "winrt-ui"))]
    pub GetScrollViewerManipulationPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollviewer: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-ui", feature = "winrt-ui")))]
    GetScrollViewerManipulationPropertySet: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics2 {
    type Vtable = IElementCompositionPreviewStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24148fbb_23d6_4f37_ba0c_0733e799722d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub SetImplicitShowAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetImplicitShowAnimation: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetImplicitHideAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub GetPointerPositionPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetelement: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetPointerPositionPropertySet: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics3 {
    type Vtable = IElementCompositionPreviewStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x843bc4c3_c105_59fe_a3d1_373c1d3e6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub SetAppWindowContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::windows_core::RawPtr, xamlcontent: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetAppWindowContent: usize,
    #[cfg(feature = "winrt-ui")]
    pub GetAppWindowContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetAppWindowContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsXamlManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56096c31_1aa0_5288_8818_6e74a2dcaff5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsXamlManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsXamlManagerStatics {
    type Vtable = IWindowsXamlManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28258a12_7d82_505b_b210_712b04a58882);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InitializeForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbb93bb5_1496_5a80_ac00_e757359755e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XamlSourceFocusNavigationReason) -> ::windows_core::HRESULT,
    pub HintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationRequestFactory {
    type Vtable = IXamlSourceFocusNavigationRequestFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe746ab8f_b4ef_5390_97e5_cc0a2779c574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceWithHintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceWithHintRectAndCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: ::winrt_foundation::Rect, correlationid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88d55a5f_9603_5d8f_9cc7_d1c4070d9801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WasFocusMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationResultFactory {
    type Vtable = IXamlSourceFocusNavigationResultFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43bbadbf_f9e1_5527_b8c5_09339ff2ca76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusmoved: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUIPresenter {
    type Vtable = IXamlUIPresenter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa714944a_1619_4fc6_b31b_89512ef022a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RootElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRootElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ThemeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetThemeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ThemeResourcesXaml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetThemeResourcesXaml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32) -> ::windows_core::HRESULT,
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXamlUIPresenterHost(::windows_core::IUnknown);
impl IXamlUIPresenterHost {
    pub fn ResolveFileResource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, path: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveFileResource)(::windows_core::Interface::as_raw(this), path.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IXamlUIPresenterHost> for ::windows_core::IUnknown {
    fn from(value: IXamlUIPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost> for ::windows_core::IUnknown {
    fn from(value: &IXamlUIPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXamlUIPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXamlUIPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlUIPresenterHost> for ::windows_core::IInspectable {
    fn from(value: IXamlUIPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost> for ::windows_core::IInspectable {
    fn from(value: &IXamlUIPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXamlUIPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXamlUIPresenterHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlUIPresenterHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlUIPresenterHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlUIPresenterHost {}
impl ::core::fmt::Debug for IXamlUIPresenterHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlUIPresenterHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXamlUIPresenterHost {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{aafb84cd-9f6d-4f80-ac2c-0e6cb9f31659}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXamlUIPresenterHost {
    type Vtable = IXamlUIPresenterHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaafb84cd_9f6d_4f80_ac2c_0e6cb9f31659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResolveFileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXamlUIPresenterHost2(::windows_core::IUnknown);
impl IXamlUIPresenterHost2 {
    pub fn GetGenericXamlFilePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetGenericXamlFilePath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IXamlUIPresenterHost2> for ::windows_core::IUnknown {
    fn from(value: IXamlUIPresenterHost2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost2> for ::windows_core::IUnknown {
    fn from(value: &IXamlUIPresenterHost2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlUIPresenterHost2> for ::windows_core::IInspectable {
    fn from(value: IXamlUIPresenterHost2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost2> for ::windows_core::IInspectable {
    fn from(value: &IXamlUIPresenterHost2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXamlUIPresenterHost2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlUIPresenterHost2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlUIPresenterHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlUIPresenterHost2 {}
impl ::core::fmt::Debug for IXamlUIPresenterHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlUIPresenterHost2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXamlUIPresenterHost2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{61595672-7ca4-4a21-b56a-88f4812388ca}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXamlUIPresenterHost2 {
    type Vtable = IXamlUIPresenterHost2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61595672_7ca4_4a21_b56a_88f4812388ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetGenericXamlFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXamlUIPresenterHost3(::windows_core::IUnknown);
impl IXamlUIPresenterHost3 {
    pub fn ResolveDictionaryResource<'a, Param0: ::windows_core::IntoParam<'a, super::ResourceDictionary>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, dictionary: Param0, dictionarykey: Param1, suggestedvalue: Param2) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveDictionaryResource)(::windows_core::Interface::as_raw(this), dictionary.into_param().abi(), dictionarykey.into_param().abi(), suggestedvalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IXamlUIPresenterHost3> for ::windows_core::IUnknown {
    fn from(value: IXamlUIPresenterHost3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost3> for ::windows_core::IUnknown {
    fn from(value: &IXamlUIPresenterHost3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlUIPresenterHost3> for ::windows_core::IInspectable {
    fn from(value: IXamlUIPresenterHost3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlUIPresenterHost3> for ::windows_core::IInspectable {
    fn from(value: &IXamlUIPresenterHost3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXamlUIPresenterHost3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlUIPresenterHost3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlUIPresenterHost3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlUIPresenterHost3 {}
impl ::core::fmt::Debug for IXamlUIPresenterHost3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlUIPresenterHost3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXamlUIPresenterHost3 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b14292bf-7320-41bb-9f26-4d6fd34db45a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXamlUIPresenterHost3 {
    type Vtable = IXamlUIPresenterHost3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb14292bf_7320_41bb_9f26_4d6fd34db45a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterHost3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResolveDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: ::windows_core::RawPtr, dictionarykey: *mut ::core::ffi::c_void, suggestedvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUIPresenterStatics {
    type Vtable = IXamlUIPresenterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71eaeac8_45e1_4192_85aa_3a422edd23cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CompleteTimelinesAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCompleteTimelinesAutomatically: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NotifyWindowSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUIPresenterStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUIPresenterStatics2 {
    type Vtable = IXamlUIPresenterStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c6b68d2_cf1c_4f53_bf09_6a745f7a9703);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUIPresenterStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub GetFlyoutPlacementTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtarget: ::windows_core::RawPtr, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: *mut bool, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetFlyoutPlacementTargetInfo: usize,
    #[cfg(feature = "winrt-ui")]
    pub GetFlyoutPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtargetbounds: ::winrt_foundation::Rect, controlsize: ::winrt_foundation::Size, mincontrolsize: ::winrt_foundation::Size, containerrect: ::winrt_foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetFlyoutPlacement: usize,
}
#[repr(transparent)]
pub struct WindowsXamlManager(::windows_core::IUnknown);
impl WindowsXamlManager {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InitializeForCurrentThread() -> ::windows_core::Result<WindowsXamlManager> {
        Self::IWindowsXamlManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InitializeForCurrentThread)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WindowsXamlManager>(result__)
        })
    }
    pub fn IWindowsXamlManagerStatics<R, F: FnOnce(&IWindowsXamlManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WindowsXamlManager, IWindowsXamlManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowsXamlManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowsXamlManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsXamlManager {}
impl ::core::fmt::Debug for WindowsXamlManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsXamlManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.WindowsXamlManager;{56096c31-1aa0-5288-8818-6e74a2dcaff5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
    const IID: ::windows_core::GUID = <IWindowsXamlManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowsXamlManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.WindowsXamlManager";
}
impl ::core::convert::From<WindowsXamlManager> for ::windows_core::IUnknown {
    fn from(value: WindowsXamlManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowsXamlManager> for ::windows_core::IUnknown {
    fn from(value: &WindowsXamlManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowsXamlManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowsXamlManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowsXamlManager> for ::windows_core::IInspectable {
    fn from(value: WindowsXamlManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowsXamlManager> for ::windows_core::IInspectable {
    fn from(value: &WindowsXamlManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowsXamlManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowsXamlManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WindowsXamlManager> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: WindowsXamlManager) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WindowsXamlManager> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &WindowsXamlManager) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for WindowsXamlManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &WindowsXamlManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WindowsXamlManager {}
unsafe impl ::core::marker::Sync for WindowsXamlManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
    pub const Left: Self = Self(7i32);
    pub const Up: Self = Self(8i32);
    pub const Right: Self = Self(9i32);
    pub const Down: Self = Self(10i32);
}
impl ::core::marker::Copy for XamlSourceFocusNavigationReason {}
impl ::core::clone::Clone for XamlSourceFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlSourceFocusNavigationReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XamlSourceFocusNavigationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for XamlSourceFocusNavigationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlSourceFocusNavigationReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationRequest(::windows_core::IUnknown);
impl XamlSourceFocusNavigationRequest {
    pub fn Reason(&self) -> ::windows_core::Result<XamlSourceFocusNavigationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XamlSourceFocusNavigationReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationReason>(result__)
        }
    }
    pub fn HintRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).HintRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn CreateInstance(reason: XamlSourceFocusNavigationReason) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    pub fn CreateInstanceWithHintRect<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHintRect)(::windows_core::Interface::as_raw(this), reason, hintrect.into_param().abi(), result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    pub fn CreateInstanceWithHintRectAndCorrelationId<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(reason: XamlSourceFocusNavigationReason, hintrect: Param1, correlationid: Param2) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHintRectAndCorrelationId)(::windows_core::Interface::as_raw(this), reason, hintrect.into_param().abi(), correlationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationRequest>(result__)
        })
    }
    pub fn IXamlSourceFocusNavigationRequestFactory<R, F: FnOnce(&IXamlSourceFocusNavigationRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlSourceFocusNavigationRequest, IXamlSourceFocusNavigationRequestFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlSourceFocusNavigationRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlSourceFocusNavigationRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlSourceFocusNavigationRequest {}
impl ::core::fmt::Debug for XamlSourceFocusNavigationRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlSourceFocusNavigationRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest;{fbb93bb5-1496-5a80-ac00-e757359755e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
    const IID: ::windows_core::GUID = <IXamlSourceFocusNavigationRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlSourceFocusNavigationRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
}
impl ::core::convert::From<XamlSourceFocusNavigationRequest> for ::windows_core::IUnknown {
    fn from(value: XamlSourceFocusNavigationRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationRequest> for ::windows_core::IUnknown {
    fn from(value: &XamlSourceFocusNavigationRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlSourceFocusNavigationRequest> for ::windows_core::IInspectable {
    fn from(value: XamlSourceFocusNavigationRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationRequest> for ::windows_core::IInspectable {
    fn from(value: &XamlSourceFocusNavigationRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlSourceFocusNavigationRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationRequest {}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationResult(::windows_core::IUnknown);
impl XamlSourceFocusNavigationResult {
    pub fn WasFocusMoved(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).WasFocusMoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CreateInstance(focusmoved: bool) -> ::windows_core::Result<XamlSourceFocusNavigationResult> {
        Self::IXamlSourceFocusNavigationResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), focusmoved, result__.as_mut_ptr()).from_abi::<XamlSourceFocusNavigationResult>(result__)
        })
    }
    pub fn IXamlSourceFocusNavigationResultFactory<R, F: FnOnce(&IXamlSourceFocusNavigationResultFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlSourceFocusNavigationResult, IXamlSourceFocusNavigationResultFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlSourceFocusNavigationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlSourceFocusNavigationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlSourceFocusNavigationResult {}
impl ::core::fmt::Debug for XamlSourceFocusNavigationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlSourceFocusNavigationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult;{88d55a5f-9603-5d8f-9cc7-d1c4070d9801})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
    const IID: ::windows_core::GUID = <IXamlSourceFocusNavigationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlSourceFocusNavigationResult {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
}
impl ::core::convert::From<XamlSourceFocusNavigationResult> for ::windows_core::IUnknown {
    fn from(value: XamlSourceFocusNavigationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationResult> for ::windows_core::IUnknown {
    fn from(value: &XamlSourceFocusNavigationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlSourceFocusNavigationResult> for ::windows_core::IInspectable {
    fn from(value: XamlSourceFocusNavigationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlSourceFocusNavigationResult> for ::windows_core::IInspectable {
    fn from(value: &XamlSourceFocusNavigationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlSourceFocusNavigationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationResult {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationResult {}
#[repr(transparent)]
pub struct XamlUIPresenter(::windows_core::IUnknown);
impl XamlUIPresenter {
    pub fn RootElement(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RootElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetRootElement<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRootElement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ThemeKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ThemeKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetThemeKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThemeKey)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ThemeResourcesXaml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ThemeResourcesXaml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetThemeResourcesXaml<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThemeResourcesXaml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetSize(&self, width: i32, height: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), width, height).ok() }
    }
    pub fn Render(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Render)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Present(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Present)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CompleteTimelinesAutomatically() -> ::windows_core::Result<bool> {
        Self::IXamlUIPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteTimelinesAutomatically)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetCompleteTimelinesAutomatically(value: bool) -> ::windows_core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetCompleteTimelinesAutomatically)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn SetHost<'a, Param0: ::windows_core::IntoParam<'a, IXamlUIPresenterHost>>(host: Param0) -> ::windows_core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetHost)(::windows_core::Interface::as_raw(this), host.into_param().abi()).ok() })
    }
    pub fn NotifyWindowSizeChanged() -> ::windows_core::Result<()> {
        Self::IXamlUIPresenterStatics(|this| unsafe { (::windows_core::Interface::vtable(this).NotifyWindowSizeChanged)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetFlyoutPlacementTargetInfo<'a, Param0: ::windows_core::IntoParam<'a, super::FrameworkElement>>(placementtarget: Param0, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: &mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: &mut bool) -> ::windows_core::Result<::winrt_foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).GetFlyoutPlacementTargetInfo)(::windows_core::Interface::as_raw(this), placementtarget.into_param().abi(), preferredplacement, targetpreferredplacement, allowfallbacks, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetFlyoutPlacement<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(placementtargetbounds: Param0, controlsize: Param1, mincontrolsize: Param2, containerrect: Param3, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: &mut super::Controls::Primitives::FlyoutPlacementMode) -> ::windows_core::Result<::winrt_foundation::Rect> {
        Self::IXamlUIPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).GetFlyoutPlacement)(::windows_core::Interface::as_raw(this), placementtargetbounds.into_param().abi(), controlsize.into_param().abi(), mincontrolsize.into_param().abi(), containerrect.into_param().abi(), targetpreferredplacement, allowfallbacks, chosenplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        })
    }
    pub fn IXamlUIPresenterStatics<R, F: FnOnce(&IXamlUIPresenterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlUIPresenter, IXamlUIPresenterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlUIPresenterStatics2<R, F: FnOnce(&IXamlUIPresenterStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlUIPresenter, IXamlUIPresenterStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlUIPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlUIPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlUIPresenter {}
impl ::core::fmt::Debug for XamlUIPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlUIPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlUIPresenter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Hosting.XamlUIPresenter;{a714944a-1619-4fc6-b31b-89512ef022a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlUIPresenter {
    type Vtable = IXamlUIPresenter_Vtbl;
    const IID: ::windows_core::GUID = <IXamlUIPresenter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlUIPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.XamlUIPresenter";
}
impl ::core::convert::From<XamlUIPresenter> for ::windows_core::IUnknown {
    fn from(value: XamlUIPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUIPresenter> for ::windows_core::IUnknown {
    fn from(value: &XamlUIPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlUIPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlUIPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlUIPresenter> for ::windows_core::IInspectable {
    fn from(value: XamlUIPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUIPresenter> for ::windows_core::IInspectable {
    fn from(value: &XamlUIPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlUIPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlUIPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlUIPresenter {}
unsafe impl ::core::marker::Sync for XamlUIPresenter {}
