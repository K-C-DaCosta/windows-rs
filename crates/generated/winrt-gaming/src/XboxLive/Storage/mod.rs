#[repr(transparent)]
pub struct GameSaveBlobGetResult(::windows_core::IUnknown);
impl GameSaveBlobGetResult {
    pub fn Status(&self) -> ::windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameSaveErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn Value(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobGetResult {}
impl ::core::fmt::Debug for GameSaveBlobGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveBlobGetResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult;{917281e0-7201-4953-aa2c-4008f03aef45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveBlobGetResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
}
impl ::core::convert::From<GameSaveBlobGetResult> for ::windows_core::IUnknown {
    fn from(value: GameSaveBlobGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobGetResult> for ::windows_core::IUnknown {
    fn from(value: &GameSaveBlobGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveBlobGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveBlobGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveBlobGetResult> for ::windows_core::IInspectable {
    fn from(value: GameSaveBlobGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobGetResult> for ::windows_core::IInspectable {
    fn from(value: &GameSaveBlobGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveBlobGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveBlobGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobGetResult {}
unsafe impl ::core::marker::Sync for GameSaveBlobGetResult {}
#[repr(transparent)]
pub struct GameSaveBlobInfo(::windows_core::IUnknown);
impl GameSaveBlobInfo {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobInfo {}
impl ::core::fmt::Debug for GameSaveBlobInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveBlobInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo;{add38034-baf0-4645-b6d0-46edaffb3c2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveBlobInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
}
impl ::core::convert::From<GameSaveBlobInfo> for ::windows_core::IUnknown {
    fn from(value: GameSaveBlobInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobInfo> for ::windows_core::IUnknown {
    fn from(value: &GameSaveBlobInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveBlobInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveBlobInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveBlobInfo> for ::windows_core::IInspectable {
    fn from(value: GameSaveBlobInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobInfo> for ::windows_core::IInspectable {
    fn from(value: &GameSaveBlobInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveBlobInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveBlobInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobInfo {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfo {}
#[repr(transparent)]
pub struct GameSaveBlobInfoGetResult(::windows_core::IUnknown);
impl GameSaveBlobInfoGetResult {
    pub fn Status(&self) -> ::windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameSaveErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Value(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GameSaveBlobInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GameSaveBlobInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobInfoGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobInfoGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobInfoGetResult {}
impl ::core::fmt::Debug for GameSaveBlobInfoGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobInfoGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveBlobInfoGetResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult;{c7578582-3697-42bf-989c-665d923b5231})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveBlobInfoGetResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
}
impl ::core::convert::From<GameSaveBlobInfoGetResult> for ::windows_core::IUnknown {
    fn from(value: GameSaveBlobInfoGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobInfoGetResult> for ::windows_core::IUnknown {
    fn from(value: &GameSaveBlobInfoGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveBlobInfoGetResult> for ::windows_core::IInspectable {
    fn from(value: GameSaveBlobInfoGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobInfoGetResult> for ::windows_core::IInspectable {
    fn from(value: &GameSaveBlobInfoGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobInfoGetResult {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfoGetResult {}
#[repr(transparent)]
pub struct GameSaveBlobInfoQuery(::windows_core::IUnknown);
impl GameSaveBlobInfoQuery {
    pub fn GetBlobInfoAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBlobInfoAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>(result__)
        }
    }
    pub fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBlobInfoWithIndexAndMaxAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>(result__)
        }
    }
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveBlobInfoQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveBlobInfoQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveBlobInfoQuery {}
impl ::core::fmt::Debug for GameSaveBlobInfoQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveBlobInfoQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveBlobInfoQuery {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery;{9fdd74b2-eeee-447b-a9d2-7f96c0f83208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveBlobInfoQuery as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
}
impl ::core::convert::From<GameSaveBlobInfoQuery> for ::windows_core::IUnknown {
    fn from(value: GameSaveBlobInfoQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobInfoQuery> for ::windows_core::IUnknown {
    fn from(value: &GameSaveBlobInfoQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveBlobInfoQuery> for ::windows_core::IInspectable {
    fn from(value: GameSaveBlobInfoQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveBlobInfoQuery> for ::windows_core::IInspectable {
    fn from(value: &GameSaveBlobInfoQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobInfoQuery {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfoQuery {}
#[repr(transparent)]
pub struct GameSaveContainer(::windows_core::IUnknown);
impl GameSaveContainer {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Provider(&self) -> ::windows_core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Provider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveProvider>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn SubmitUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::winrt_storage::Streams::IBuffer>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, blobstowrite: Param0, blobstodelete: Param1, displayname: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubmitUpdatesAsync)(::windows_core::Interface::as_raw(this), blobstowrite.into_param().abi(), blobstodelete.into_param().abi(), displayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn ReadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::winrt_storage::Streams::IBuffer>>>(&self, blobstoread: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), blobstoread.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, blobstoread: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveBlobGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), blobstoread.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveBlobGetResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SubmitPropertySetUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, blobstowrite: Param0, blobstodelete: Param1, displayname: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubmitPropertySetUpdatesAsync)(::windows_core::Interface::as_raw(this), blobstowrite.into_param().abi(), blobstodelete.into_param().abi(), displayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    pub fn CreateBlobInfoQuery<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, blobnameprefix: Param0) -> ::windows_core::Result<GameSaveBlobInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBlobInfoQuery)(::windows_core::Interface::as_raw(this), blobnameprefix.into_param().abi(), result__.as_mut_ptr()).from_abi::<GameSaveBlobInfoQuery>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainer {}
impl ::core::fmt::Debug for GameSaveContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveContainer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainer;{c3c08f89-563f-4ecd-9c6f-33fd0e323d10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveContainer {
    type Vtable = IGameSaveContainer_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveContainer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainer";
}
impl ::core::convert::From<GameSaveContainer> for ::windows_core::IUnknown {
    fn from(value: GameSaveContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainer> for ::windows_core::IUnknown {
    fn from(value: &GameSaveContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveContainer> for ::windows_core::IInspectable {
    fn from(value: GameSaveContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainer> for ::windows_core::IInspectable {
    fn from(value: &GameSaveContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveContainer {}
unsafe impl ::core::marker::Sync for GameSaveContainer {}
#[repr(transparent)]
pub struct GameSaveContainerInfo(::windows_core::IUnknown);
impl GameSaveContainerInfo {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TotalSize(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LastModifiedTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastModifiedTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn NeedsSync(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).NeedsSync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainerInfo {}
impl ::core::fmt::Debug for GameSaveContainerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveContainerInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo;{b7e27300-155d-4bb4-b2ba-930306f391b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveContainerInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
}
impl ::core::convert::From<GameSaveContainerInfo> for ::windows_core::IUnknown {
    fn from(value: GameSaveContainerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainerInfo> for ::windows_core::IUnknown {
    fn from(value: &GameSaveContainerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveContainerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveContainerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveContainerInfo> for ::windows_core::IInspectable {
    fn from(value: GameSaveContainerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainerInfo> for ::windows_core::IInspectable {
    fn from(value: &GameSaveContainerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveContainerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveContainerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveContainerInfo {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfo {}
#[repr(transparent)]
pub struct GameSaveContainerInfoGetResult(::windows_core::IUnknown);
impl GameSaveContainerInfoGetResult {
    pub fn Status(&self) -> ::windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameSaveErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Value(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GameSaveContainerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GameSaveContainerInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainerInfoGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainerInfoGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainerInfoGetResult {}
impl ::core::fmt::Debug for GameSaveContainerInfoGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainerInfoGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveContainerInfoGetResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult;{ffc50d74-c581-4f9d-9e39-30a10c1e4c50})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveContainerInfoGetResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
}
impl ::core::convert::From<GameSaveContainerInfoGetResult> for ::windows_core::IUnknown {
    fn from(value: GameSaveContainerInfoGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainerInfoGetResult> for ::windows_core::IUnknown {
    fn from(value: &GameSaveContainerInfoGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveContainerInfoGetResult> for ::windows_core::IInspectable {
    fn from(value: GameSaveContainerInfoGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainerInfoGetResult> for ::windows_core::IInspectable {
    fn from(value: &GameSaveContainerInfoGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveContainerInfoGetResult {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfoGetResult {}
#[repr(transparent)]
pub struct GameSaveContainerInfoQuery(::windows_core::IUnknown);
impl GameSaveContainerInfoQuery {
    pub fn GetContainerInfoAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContainerInfoAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>(result__)
        }
    }
    pub fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContainerInfoWithIndexAndMaxAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>(result__)
        }
    }
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveContainerInfoQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveContainerInfoQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveContainerInfoQuery {}
impl ::core::fmt::Debug for GameSaveContainerInfoQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveContainerInfoQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveContainerInfoQuery {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery;{3c94e863-6f80-4327-9327-ffc11afd42b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveContainerInfoQuery as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
}
impl ::core::convert::From<GameSaveContainerInfoQuery> for ::windows_core::IUnknown {
    fn from(value: GameSaveContainerInfoQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainerInfoQuery> for ::windows_core::IUnknown {
    fn from(value: &GameSaveContainerInfoQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveContainerInfoQuery> for ::windows_core::IInspectable {
    fn from(value: GameSaveContainerInfoQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveContainerInfoQuery> for ::windows_core::IInspectable {
    fn from(value: &GameSaveContainerInfoQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveContainerInfoQuery {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfoQuery {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Abort: Self = Self(-2147467260i32);
    pub const InvalidContainerName: Self = Self(-2138898431i32);
    pub const NoAccess: Self = Self(-2138898430i32);
    pub const OutOfLocalStorage: Self = Self(-2138898429i32);
    pub const UserCanceled: Self = Self(-2138898428i32);
    pub const UpdateTooBig: Self = Self(-2138898427i32);
    pub const QuotaExceeded: Self = Self(-2138898426i32);
    pub const ProvidedBufferTooSmall: Self = Self(-2138898425i32);
    pub const BlobNotFound: Self = Self(-2138898424i32);
    pub const NoXboxLiveInfo: Self = Self(-2138898423i32);
    pub const ContainerNotInSync: Self = Self(-2138898422i32);
    pub const ContainerSyncFailed: Self = Self(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: Self = Self(-2138898420i32);
    pub const ObjectExpired: Self = Self(-2138898419i32);
}
impl ::core::marker::Copy for GameSaveErrorStatus {}
impl ::core::clone::Clone for GameSaveErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameSaveErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameSaveErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameSaveErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveErrorStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GameSaveOperationResult(::windows_core::IUnknown);
impl GameSaveOperationResult {
    pub fn Status(&self) -> ::windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameSaveErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveOperationResult {}
impl ::core::fmt::Debug for GameSaveOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveOperationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveOperationResult;{cf0f1a05-24a0-4582-9a55-b1bbbb9388d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveOperationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
}
impl ::core::convert::From<GameSaveOperationResult> for ::windows_core::IUnknown {
    fn from(value: GameSaveOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveOperationResult> for ::windows_core::IUnknown {
    fn from(value: &GameSaveOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveOperationResult> for ::windows_core::IInspectable {
    fn from(value: GameSaveOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveOperationResult> for ::windows_core::IInspectable {
    fn from(value: &GameSaveOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveOperationResult {}
unsafe impl ::core::marker::Sync for GameSaveOperationResult {}
#[repr(transparent)]
pub struct GameSaveProvider(::windows_core::IUnknown);
impl GameSaveProvider {
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn CreateContainer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<GameSaveContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateContainer)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<GameSaveContainer>(result__)
        }
    }
    pub fn DeleteContainerAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteContainerAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    pub fn CreateContainerInfoQuery(&self) -> ::windows_core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateContainerInfoQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveContainerInfoQuery>(result__)
        }
    }
    pub fn CreateContainerInfoQueryWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, containernameprefix: Param0) -> ::windows_core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateContainerInfoQueryWithName)(::windows_core::Interface::as_raw(this), containernameprefix.into_param().abi(), result__.as_mut_ptr()).from_abi::<GameSaveContainerInfoQuery>(result__)
        }
    }
    pub fn GetRemainingBytesInQuotaAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRemainingBytesInQuotaAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<i64>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ContainersChangedSinceLastSync(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContainersChangedSinceLastSync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, serviceconfigid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), serviceconfigid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveProviderGetResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetSyncOnDemandForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, serviceconfigid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSyncOnDemandForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), serviceconfigid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameSaveProviderGetResult>>(result__)
        })
    }
    pub fn IGameSaveProviderStatics<R, F: FnOnce(&IGameSaveProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameSaveProvider, IGameSaveProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GameSaveProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveProvider {}
impl ::core::fmt::Debug for GameSaveProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProvider;{90a60394-80fe-4211-97f8-a5de14dd95d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveProvider {
    type Vtable = IGameSaveProvider_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProvider";
}
impl ::core::convert::From<GameSaveProvider> for ::windows_core::IUnknown {
    fn from(value: GameSaveProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveProvider> for ::windows_core::IUnknown {
    fn from(value: &GameSaveProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveProvider> for ::windows_core::IInspectable {
    fn from(value: GameSaveProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveProvider> for ::windows_core::IInspectable {
    fn from(value: &GameSaveProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveProvider {}
unsafe impl ::core::marker::Sync for GameSaveProvider {}
#[repr(transparent)]
pub struct GameSaveProviderGetResult(::windows_core::IUnknown);
impl GameSaveProviderGetResult {
    pub fn Status(&self) -> ::windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameSaveErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameSaveProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for GameSaveProviderGetResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameSaveProviderGetResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameSaveProviderGetResult {}
impl ::core::fmt::Debug for GameSaveProviderGetResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameSaveProviderGetResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameSaveProviderGetResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult;{3ab90816-d393-4d65-ac16-41c3e67ab945})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_Vtbl;
    const IID: ::windows_core::GUID = <IGameSaveProviderGetResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
}
impl ::core::convert::From<GameSaveProviderGetResult> for ::windows_core::IUnknown {
    fn from(value: GameSaveProviderGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveProviderGetResult> for ::windows_core::IUnknown {
    fn from(value: &GameSaveProviderGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameSaveProviderGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameSaveProviderGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameSaveProviderGetResult> for ::windows_core::IInspectable {
    fn from(value: GameSaveProviderGetResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameSaveProviderGetResult> for ::windows_core::IInspectable {
    fn from(value: &GameSaveProviderGetResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameSaveProviderGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameSaveProviderGetResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameSaveProviderGetResult {}
unsafe impl ::core::marker::Sync for GameSaveProviderGetResult {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobGetResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x917281e0_7201_4953_aa2c_4008f03aef45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobGetResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadd38034_baf0_4645_b6d0_46edaffb3c2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobInfoGetResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7578582_3697_42bf_989c_665d923b5231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoGetResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveBlobInfoQuery(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fdd74b2_eeee_447b_a9d2_7f96c0f83208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoQuery_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetBlobInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBlobInfoWithIndexAndMaxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveContainer {
    type Vtable = IGameSaveContainer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3c08f89_563f_4ecd_9c6f_33fd0e323d10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Provider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub SubmitUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstowrite: ::windows_core::RawPtr, blobstodelete: ::windows_core::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    SubmitUpdatesAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub ReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstoread: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    ReadAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstoread: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SubmitPropertySetUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobstowrite: ::windows_core::RawPtr, blobstodelete: ::windows_core::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SubmitPropertySetUpdatesAsync: usize,
    pub CreateBlobInfoQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobnameprefix: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainerInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7e27300_155d_4bb4_b2ba_930306f391b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub NeedsSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainerInfoGetResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffc50d74_c581_4f9d_9e39_30a10c1e4c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoGetResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveContainerInfoQuery(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c94e863_6f80_4327_9327_ffc11afd42b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoQuery_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetContainerInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetContainerInfoWithIndexAndMaxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf0f1a05_24a0_4582_9a55_b1bbbb9388d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveProvider {
    type Vtable = IGameSaveProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90a60394_80fe_4211_97f8_a5de14dd95d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
    pub CreateContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateContainerInfoQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateContainerInfoQueryWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containernameprefix: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRemainingBytesInQuotaAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ContainersChangedSinceLastSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ContainersChangedSinceLastSync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveProviderGetResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ab90816_d393_4d65_ac16_41c3e67ab945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderGetResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameSaveErrorStatus) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameSaveProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameSaveProviderStatics {
    type Vtable = IGameSaveProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd01d3ed0_7b03_449d_8cbd_3402842a1048);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub GetForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, serviceconfigid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetForUserAsync: usize,
    #[cfg(feature = "winrt-system")]
    pub GetSyncOnDemandForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, serviceconfigid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetSyncOnDemandForUserAsync: usize,
}
