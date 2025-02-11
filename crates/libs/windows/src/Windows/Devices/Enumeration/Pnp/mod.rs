#[doc(hidden)]
#[repr(transparent)]
pub struct IPnpObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPnpObject {
    type Vtable = IPnpObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IPnpObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c66258_733b_4a8f_93a3_db078ac870c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObject_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PnpObjectType) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPnpObjectStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPnpObjectStatics {
    type Vtable = IPnpObjectStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPnpObjectStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c32a3d_d168_4660_bbf3_a733b14b6e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestedproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcher: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherAqsFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPnpObjectUpdate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPnpObjectUpdate {
    type Vtable = IPnpObjectUpdate_Vtbl;
}
unsafe impl ::windows::core::Interface for IPnpObjectUpdate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f59e812_001e_4844_bcc6_432886856a17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectUpdate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PnpObjectType) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPnpObjectWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPnpObjectWatcher {
    type Vtable = IPnpObjectWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IPnpObjectWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83c95ca8_4772_4a7a_aca8_e48c42a89c44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DeviceWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct PnpObject(::windows::core::IUnknown);
impl PnpObject {
    pub fn Type(&self) -> ::windows::core::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PnpObjectType>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    pub fn Update(&self, updateinfo: &PnpObjectUpdate) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Update)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updateinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsync<'a, P0, E0>(r#type: PnpObjectType, id: &::windows::core::HSTRING, requestedproperties: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObject>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromIdAsync)(::windows::core::Vtable::as_raw(this), r#type, ::core::mem::transmute_copy(id), requestedproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObject>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync<'a, P0, E0>(r#type: PnpObjectType, requestedproperties: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsync)(::windows::core::Vtable::as_raw(this), r#type, requestedproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilter<'a, P0, E0>(r#type: PnpObjectType, requestedproperties: P0, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsyncAqsFilter)(::windows::core::Vtable::as_raw(this), r#type, requestedproperties.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(aqsfilter), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcher<'a, P0, E0>(r#type: PnpObjectType, requestedproperties: P0) -> ::windows::core::Result<PnpObjectWatcher>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcher)(::windows::core::Vtable::as_raw(this), r#type, requestedproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<PnpObjectWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherAqsFilter<'a, P0, E0>(r#type: PnpObjectType, requestedproperties: P0, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<PnpObjectWatcher>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcherAqsFilter)(::windows::core::Vtable::as_raw(this), r#type, requestedproperties.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(aqsfilter), result__.as_mut_ptr()).from_abi::<PnpObjectWatcher>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPnpObjectStatics<R, F: FnOnce(&IPnpObjectStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PnpObject, IPnpObjectStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PnpObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PnpObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PnpObject {}
impl ::core::fmt::Debug for PnpObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObject;{95c66258-733b-4a8f-93a3-db078ac870c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PnpObject {
    type Vtable = IPnpObject_Vtbl;
}
unsafe impl ::windows::core::Interface for PnpObject {
    const IID: ::windows::core::GUID = <IPnpObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PnpObject {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObject";
}
::windows::core::interface_hierarchy!(PnpObject, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PnpObject {}
unsafe impl ::core::marker::Sync for PnpObject {}
#[doc = "*Required features: `\"Devices_Enumeration_Pnp\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PnpObjectCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PnpObjectCollection {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<PnpObject>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<PnpObject>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterator<PnpObject>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<PnpObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi::<PnpObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &PnpObject, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<PnpObject>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PnpObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PnpObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PnpObjectCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PnpObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PnpObjectCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.Pnp.PnpObject;{95c66258-733b-4a8f-93a3-db078ac870c1})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PnpObjectCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<PnpObject>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PnpObjectCollection {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IVectorView<PnpObject> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PnpObjectCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PnpObjectCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PnpObjectCollection> for super::super::super::Foundation::Collections::IIterable<PnpObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: PnpObjectCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PnpObjectCollection> for super::super::super::Foundation::Collections::IIterable<PnpObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PnpObjectCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&PnpObjectCollection> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PnpObjectCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PnpObjectCollection> for super::super::super::Foundation::Collections::IVectorView<PnpObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: PnpObjectCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PnpObjectCollection> for super::super::super::Foundation::Collections::IVectorView<PnpObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PnpObjectCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&PnpObjectCollection> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PnpObjectCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PnpObjectCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PnpObjectCollection {}
#[doc = "*Required features: `\"Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct PnpObjectUpdate(::windows::core::IUnknown);
impl PnpObjectUpdate {
    pub fn Type(&self) -> ::windows::core::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PnpObjectType>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for PnpObjectUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PnpObjectUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PnpObjectUpdate {}
impl ::core::fmt::Debug for PnpObjectUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectUpdate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObjectUpdate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectUpdate;{6f59e812-001e-4844-bcc6-432886856a17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PnpObjectUpdate {
    type Vtable = IPnpObjectUpdate_Vtbl;
}
unsafe impl ::windows::core::Interface for PnpObjectUpdate {
    const IID: ::windows::core::GUID = <IPnpObjectUpdate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PnpObjectUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectUpdate";
}
::windows::core::interface_hierarchy!(PnpObjectUpdate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PnpObjectUpdate {}
unsafe impl ::core::marker::Sync for PnpObjectUpdate {}
#[doc = "*Required features: `\"Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
pub struct PnpObjectWatcher(::windows::core::IUnknown);
impl PnpObjectWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated(&self, handler: &super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Updated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<super::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DeviceWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PnpObjectWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PnpObjectWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PnpObjectWatcher {}
impl ::core::fmt::Debug for PnpObjectWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObjectWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectWatcher;{83c95ca8-4772-4a7a-aca8-e48c42a89c44})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PnpObjectWatcher {
    type Vtable = IPnpObjectWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for PnpObjectWatcher {
    const IID: ::windows::core::GUID = <IPnpObjectWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PnpObjectWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectWatcher";
}
::windows::core::interface_hierarchy!(PnpObjectWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PnpObjectWatcher {}
unsafe impl ::core::marker::Sync for PnpObjectWatcher {}
#[doc = "*Required features: `\"Devices_Enumeration_Pnp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PnpObjectType(pub i32);
impl PnpObjectType {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
}
impl ::core::marker::Copy for PnpObjectType {}
impl ::core::clone::Clone for PnpObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PnpObjectType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PnpObjectType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PnpObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObjectType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Pnp.PnpObjectType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
