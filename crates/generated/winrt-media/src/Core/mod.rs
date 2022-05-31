#[cfg(feature = "Preview")]
pub mod Preview;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: Self = Self(0i32);
    pub const DownmixTo2Channels: Self = Self(1i32);
    pub const DownmixTo6Channels: Self = Self(2i32);
    pub const DownmixTo8Channels: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioDecoderDegradation {}
impl ::core::clone::Clone for AudioDecoderDegradation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDecoderDegradation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioDecoderDegradation {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioDecoderDegradation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioDecoderDegradation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDecoderDegradationReason {}
impl ::core::clone::Clone for AudioDecoderDegradationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDecoderDegradationReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioDecoderDegradationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioDecoderDegradationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradationReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioDecoderDegradationReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradationReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioStreamDescriptor(::windows_core::IUnknown);
impl AudioStreamDescriptor {
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn SetLeadingEncoderPadding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLeadingEncoderPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LeadingEncoderPadding(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LeadingEncoderPadding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetTrailingEncoderPadding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTrailingEncoderPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TrailingEncoderPadding(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrailingEncoderPadding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<AudioStreamDescriptor> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioStreamDescriptor>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(encodingproperties: Param0) -> ::windows_core::Result<AudioStreamDescriptor> {
        Self::IAudioStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioStreamDescriptor>(result__)
        })
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IAudioStreamDescriptorFactory<R, F: FnOnce(&IAudioStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioStreamDescriptor, IAudioStreamDescriptorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioStreamDescriptor {}
impl ::core::fmt::Debug for AudioStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioStreamDescriptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioStreamDescriptor;{1e3692e4-4027-4847-a70b-df1d9a2a7b04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <IAudioStreamDescriptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.AudioStreamDescriptor";
}
impl ::core::convert::From<AudioStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: AudioStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: &AudioStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: AudioStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: &AudioStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioStreamDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioStreamDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for &AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioStreamDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioStreamDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor2> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor2> for &AudioStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor2> {
        ::core::convert::TryInto::<IMediaStreamDescriptor2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioStreamDescriptor {}
unsafe impl ::core::marker::Sync for AudioStreamDescriptor {}
#[repr(transparent)]
pub struct AudioTrack(::windows_core::IUnknown);
impl AudioTrack {
    pub fn OpenFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpenFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn GetEncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetEncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SupportInfo(&self) -> ::windows_core::Result<AudioTrackSupportInfo> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioTrackSupportInfo>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaTrackKind>::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaTrackKind>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrack {}
impl ::core::fmt::Debug for AudioTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioTrack {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioTrack {
    type Vtable = IMediaTrack_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTrack as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrack {
    const NAME: &'static str = "Windows.Media.Core.AudioTrack";
}
impl ::core::convert::From<AudioTrack> for ::windows_core::IUnknown {
    fn from(value: AudioTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrack> for ::windows_core::IUnknown {
    fn from(value: &AudioTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioTrack> for ::windows_core::IInspectable {
    fn from(value: AudioTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrack> for ::windows_core::IInspectable {
    fn from(value: &AudioTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioTrack> for IMediaTrack {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioTrack) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioTrack> for IMediaTrack {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioTrack) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaTrack> for AudioTrack {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaTrack> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaTrack> for &AudioTrack {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaTrack> {
        ::core::convert::TryInto::<IMediaTrack>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioTrack {}
unsafe impl ::core::marker::Sync for AudioTrack {}
#[repr(transparent)]
pub struct AudioTrackOpenFailedEventArgs(::windows_core::IUnknown);
impl AudioTrackOpenFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioTrackOpenFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrackOpenFailedEventArgs {}
impl ::core::fmt::Debug for AudioTrackOpenFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrackOpenFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrackOpenFailedEventArgs;{eeddb9b9-bb7c-4112-bf76-9384676f824b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAudioTrackOpenFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackOpenFailedEventArgs";
}
impl ::core::convert::From<AudioTrackOpenFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AudioTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackOpenFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AudioTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioTrackOpenFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AudioTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackOpenFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AudioTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for AudioTrackOpenFailedEventArgs {}
#[repr(transparent)]
pub struct AudioTrackSupportInfo(::windows_core::IUnknown);
impl AudioTrackSupportInfo {
    pub fn DecoderStatus(&self) -> ::windows_core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaDecoderStatus>::zeroed();
            (::windows_core::Interface::vtable(this).DecoderStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDecoderStatus>(result__)
        }
    }
    pub fn Degradation(&self) -> ::windows_core::Result<AudioDecoderDegradation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioDecoderDegradation>::zeroed();
            (::windows_core::Interface::vtable(this).Degradation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDecoderDegradation>(result__)
        }
    }
    pub fn DegradationReason(&self) -> ::windows_core::Result<AudioDecoderDegradationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioDecoderDegradationReason>::zeroed();
            (::windows_core::Interface::vtable(this).DegradationReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDecoderDegradationReason>(result__)
        }
    }
    pub fn MediaSourceStatus(&self) -> ::windows_core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaSourceStatus>::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioTrackSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrackSupportInfo {}
impl ::core::fmt::Debug for AudioTrackSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrackSupportInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioTrackSupportInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrackSupportInfo;{178beff7-cc39-44a6-b951-4a5653f073fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAudioTrackSupportInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackSupportInfo";
}
impl ::core::convert::From<AudioTrackSupportInfo> for ::windows_core::IUnknown {
    fn from(value: AudioTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackSupportInfo> for ::windows_core::IUnknown {
    fn from(value: &AudioTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioTrackSupportInfo> for ::windows_core::IInspectable {
    fn from(value: AudioTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackSupportInfo> for ::windows_core::IInspectable {
    fn from(value: &AudioTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioTrackSupportInfo {}
unsafe impl ::core::marker::Sync for AudioTrackSupportInfo {}
#[repr(transparent)]
pub struct ChapterCue(::windows_core::IUnknown);
impl ChapterCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ChapterCue, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ChapterCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChapterCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChapterCue {}
impl ::core::fmt::Debug for ChapterCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChapterCue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ChapterCue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.ChapterCue;{72a98001-d38a-4c0a-8fa6-75cddaf4664c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ChapterCue {
    type Vtable = IChapterCue_Vtbl;
    const IID: ::windows_core::GUID = <IChapterCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ChapterCue {
    const NAME: &'static str = "Windows.Media.Core.ChapterCue";
}
impl ::core::convert::From<ChapterCue> for ::windows_core::IUnknown {
    fn from(value: ChapterCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChapterCue> for ::windows_core::IUnknown {
    fn from(value: &ChapterCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ChapterCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ChapterCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChapterCue> for ::windows_core::IInspectable {
    fn from(value: ChapterCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChapterCue> for ::windows_core::IInspectable {
    fn from(value: &ChapterCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ChapterCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ChapterCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ChapterCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: ChapterCue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ChapterCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: &ChapterCue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for ChapterCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for &ChapterCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ChapterCue {}
unsafe impl ::core::marker::Sync for ChapterCue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecCategory {}
impl ::core::clone::Clone for CodecCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CodecCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CodecCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for CodecCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CodecCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CodecInfo(::windows_core::IUnknown);
impl CodecInfo {
    pub fn Kind(&self) -> ::windows_core::Result<CodecKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CodecKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CodecKind>(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<CodecCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CodecCategory>::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CodecCategory>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Subtypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Subtypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsTrusted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTrusted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecInfo {}
impl ::core::fmt::Debug for CodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CodecInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.CodecInfo;{51e89f85-ea97-499c-86ac-4ce5e73f3a42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CodecInfo {
    type Vtable = ICodecInfo_Vtbl;
    const IID: ::windows_core::GUID = <ICodecInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CodecInfo {
    const NAME: &'static str = "Windows.Media.Core.CodecInfo";
}
impl ::core::convert::From<CodecInfo> for ::windows_core::IUnknown {
    fn from(value: CodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecInfo> for ::windows_core::IUnknown {
    fn from(value: &CodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CodecInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CodecInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CodecInfo> for ::windows_core::IInspectable {
    fn from(value: CodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecInfo> for ::windows_core::IInspectable {
    fn from(value: &CodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CodecInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CodecInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CodecInfo {}
unsafe impl ::core::marker::Sync for CodecInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecKind {}
impl ::core::clone::Clone for CodecKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CodecKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CodecKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CodecKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CodecKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CodecQuery(::windows_core::IUnknown);
impl CodecQuery {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CodecQuery, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, kind: CodecKind, category: CodecCategory, subtype: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<CodecInfo>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), kind, category, subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<CodecInfo>>>(result__)
        }
    }
}
impl ::core::clone::Clone for CodecQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CodecQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecQuery {}
impl ::core::fmt::Debug for CodecQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CodecQuery {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.CodecQuery;{222a953a-af61-4e04-808a-a4634e2f3ac4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CodecQuery {
    type Vtable = ICodecQuery_Vtbl;
    const IID: ::windows_core::GUID = <ICodecQuery as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CodecQuery {
    const NAME: &'static str = "Windows.Media.Core.CodecQuery";
}
impl ::core::convert::From<CodecQuery> for ::windows_core::IUnknown {
    fn from(value: CodecQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecQuery> for ::windows_core::IUnknown {
    fn from(value: &CodecQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CodecQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CodecQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CodecQuery> for ::windows_core::IInspectable {
    fn from(value: CodecQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecQuery> for ::windows_core::IInspectable {
    fn from(value: &CodecQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CodecQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CodecQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CodecQuery {}
unsafe impl ::core::marker::Sync for CodecQuery {}
pub struct CodecSubtypes;
impl CodecSubtypes {
    pub fn VideoFormatDV25() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDV25)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatDV50() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDV50)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatDvc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatDvh1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvh1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatDvhD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvhD)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatDvsd() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvsd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatDvsl() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvsl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatH263() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH263)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatH264() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH264)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatH265() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH265)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatH264ES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH264ES)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatHevc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatHevc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatHevcES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatHevcES)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatM4S2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatM4S2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMjpg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMjpg)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMP43() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP43)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMP4S() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP4S)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMP4V() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP4V)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMpeg2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMpeg2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatVP80() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatVP80)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatVP90() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatVP90)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMpg1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMpg1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMss1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMss1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatMss2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMss2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatWmv1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatWmv2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatWmv3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormatWvc1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWvc1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn VideoFormat420O() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormat420O)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatAac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAac)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatAdts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAdts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatAlac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAlac)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatAmrNB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrNB)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatAmrWB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrWB)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatAmrWP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrWP)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatDolbyAC3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyAC3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatDolbyAC3Spdif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyAC3Spdif)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatDolbyDDPlus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyDDPlus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatDrm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDrm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatDts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatFlac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatFlac)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatFloat() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatFloat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatMP3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMP3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatMPeg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMPeg)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatMsp1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMsp1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatOpus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatOpus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatPcm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatPcm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatWmaSpdif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWmaSpdif)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatWMAudioLossless() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioLossless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatWMAudioV8() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioV8)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AudioFormatWMAudioV9() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioV9)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ICodecSubtypesStatics<R, F: FnOnce(&ICodecSubtypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CodecSubtypes, ICodecSubtypesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CodecSubtypes {
    const NAME: &'static str = "Windows.Media.Core.CodecSubtypes";
}
#[repr(transparent)]
pub struct DataCue(::windows_core::IUnknown);
impl DataCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DataCue, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::PropertySet> {
        let this = &::windows_core::Interface::cast::<IDataCue2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::PropertySet>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DataCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataCue {}
impl ::core::fmt::Debug for DataCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DataCue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.DataCue;{7c7f676d-1fbc-4e2d-9a87-ee38bd1dc637})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DataCue {
    type Vtable = IDataCue_Vtbl;
    const IID: ::windows_core::GUID = <IDataCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DataCue {
    const NAME: &'static str = "Windows.Media.Core.DataCue";
}
impl ::core::convert::From<DataCue> for ::windows_core::IUnknown {
    fn from(value: DataCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataCue> for ::windows_core::IUnknown {
    fn from(value: &DataCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DataCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DataCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataCue> for ::windows_core::IInspectable {
    fn from(value: DataCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataCue> for ::windows_core::IInspectable {
    fn from(value: &DataCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DataCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DataCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DataCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: DataCue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DataCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: &DataCue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for DataCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for &DataCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DataCue {}
unsafe impl ::core::marker::Sync for DataCue {}
#[repr(transparent)]
pub struct FaceDetectedEventArgs(::windows_core::IUnknown);
impl FaceDetectedEventArgs {
    pub fn ResultFrame(&self) -> ::windows_core::Result<FaceDetectionEffectFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResultFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FaceDetectionEffectFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for FaceDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectedEventArgs {}
impl ::core::fmt::Debug for FaceDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FaceDetectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectedEventArgs;{19918426-c65b-46ba-85f8-13880576c90a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectedEventArgs";
}
impl ::core::convert::From<FaceDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: FaceDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FaceDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FaceDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FaceDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: FaceDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FaceDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FaceDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FaceDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FaceDetectedEventArgs {}
unsafe impl ::core::marker::Sync for FaceDetectedEventArgs {}
#[repr(transparent)]
pub struct FaceDetectionEffect(::windows_core::IUnknown);
impl FaceDetectionEffect {
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDesiredDetectionInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDetectionInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DesiredDetectionInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDetectionInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn FaceDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FaceDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFaceDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFaceDetected)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FaceDetectionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetectionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionEffect {}
impl ::core::fmt::Debug for FaceDetectionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FaceDetectionEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffect;{ae15ebd2-0542-42a9-bc90-f283a29f46c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FaceDetectionEffect {
    type Vtable = IFaceDetectionEffect_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetectionEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectionEffect {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffect";
}
impl ::core::convert::From<FaceDetectionEffect> for ::windows_core::IUnknown {
    fn from(value: FaceDetectionEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffect> for ::windows_core::IUnknown {
    fn from(value: &FaceDetectionEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FaceDetectionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FaceDetectionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetectionEffect> for ::windows_core::IInspectable {
    fn from(value: FaceDetectionEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffect> for ::windows_core::IInspectable {
    fn from(value: &FaceDetectionEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FaceDetectionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FaceDetectionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FaceDetectionEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: FaceDetectionEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FaceDetectionEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: &FaceDetectionEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for FaceDetectionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for &FaceDetectionEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FaceDetectionEffect {}
unsafe impl ::core::marker::Sync for FaceDetectionEffect {}
#[cfg(feature = "winrt-media")]
#[repr(transparent)]
pub struct FaceDetectionEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "winrt-media")]
impl FaceDetectionEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FaceDetectionEffectDefinition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetDetectionMode(&self, value: FaceDetectionMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDetectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DetectionMode(&self) -> ::windows_core::Result<FaceDetectionMode> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FaceDetectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).DetectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FaceDetectionMode>(result__)
        }
    }
    pub fn SetSynchronousDetectionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSynchronousDetectionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SynchronousDetectionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SynchronousDetectionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::clone::Clone for FaceDetectionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::cmp::PartialEq for FaceDetectionEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::cmp::Eq for FaceDetectionEffectDefinition {}
#[cfg(feature = "winrt-media")]
impl ::core::fmt::Debug for FaceDetectionEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::windows_core::RuntimeType for FaceDetectionEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::windows_core::Interface for FaceDetectionEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-media")]
impl ::windows_core::RuntimeName for FaceDetectionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectDefinition";
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<FaceDetectionEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: FaceDetectionEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<&FaceDetectionEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &FaceDetectionEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<FaceDetectionEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: FaceDetectionEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<&FaceDetectionEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &FaceDetectionEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<FaceDetectionEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: FaceDetectionEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&FaceDetectionEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &FaceDetectionEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for &FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IVideoEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::core::marker::Send for FaceDetectionEffectDefinition {}
#[cfg(feature = "winrt-media")]
unsafe impl ::core::marker::Sync for FaceDetectionEffectDefinition {}
#[repr(transparent)]
pub struct FaceDetectionEffectFrame(::windows_core::IUnknown);
impl FaceDetectionEffectFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn DetectedFaces(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DetectedFaces)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetSystemRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::clone::Clone for FaceDetectionEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetectionEffectFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionEffectFrame {}
impl ::core::fmt::Debug for FaceDetectionEffectFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffectFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FaceDetectionEffectFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffectFrame;{8ab08993-5dc8-447b-a247-5270bd802ece})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrame_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetectionEffectFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectionEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectFrame";
}
impl ::core::convert::From<FaceDetectionEffectFrame> for ::windows_core::IUnknown {
    fn from(value: FaceDetectionEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffectFrame> for ::windows_core::IUnknown {
    fn from(value: &FaceDetectionEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetectionEffectFrame> for ::windows_core::IInspectable {
    fn from(value: FaceDetectionEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffectFrame> for ::windows_core::IInspectable {
    fn from(value: &FaceDetectionEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FaceDetectionEffectFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: FaceDetectionEffectFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FaceDetectionEffectFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &FaceDetectionEffectFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FaceDetectionEffectFrame> for super::IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: FaceDetectionEffectFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FaceDetectionEffectFrame> for super::IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &FaceDetectionEffectFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaFrame> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaFrame> for &FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaFrame> {
        ::core::convert::TryInto::<super::IMediaFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FaceDetectionEffectFrame {}
unsafe impl ::core::marker::Sync for FaceDetectionEffectFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
}
impl ::core::marker::Copy for FaceDetectionMode {}
impl ::core::clone::Clone for FaceDetectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FaceDetectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FaceDetectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FaceDetectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FaceDetectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.FaceDetectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HighDynamicRangeControl(::windows_core::IUnknown);
impl HighDynamicRangeControl {
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HighDynamicRangeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HighDynamicRangeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HighDynamicRangeControl {}
impl ::core::fmt::Debug for HighDynamicRangeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HighDynamicRangeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HighDynamicRangeControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.HighDynamicRangeControl;{55f1a7ae-d957-4dc9-9d1c-8553a82a7d99})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControl_Vtbl;
    const IID: ::windows_core::GUID = <IHighDynamicRangeControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HighDynamicRangeControl {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeControl";
}
impl ::core::convert::From<HighDynamicRangeControl> for ::windows_core::IUnknown {
    fn from(value: HighDynamicRangeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeControl> for ::windows_core::IUnknown {
    fn from(value: &HighDynamicRangeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HighDynamicRangeControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HighDynamicRangeControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HighDynamicRangeControl> for ::windows_core::IInspectable {
    fn from(value: HighDynamicRangeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeControl> for ::windows_core::IInspectable {
    fn from(value: &HighDynamicRangeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HighDynamicRangeControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HighDynamicRangeControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HighDynamicRangeControl {}
unsafe impl ::core::marker::Sync for HighDynamicRangeControl {}
#[repr(transparent)]
pub struct HighDynamicRangeOutput(::windows_core::IUnknown);
impl HighDynamicRangeOutput {
    pub fn Certainty(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Certainty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn FrameControllers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::Devices::Core::FrameController>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameControllers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::Devices::Core::FrameController>>(result__)
        }
    }
}
impl ::core::clone::Clone for HighDynamicRangeOutput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HighDynamicRangeOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HighDynamicRangeOutput {}
impl ::core::fmt::Debug for HighDynamicRangeOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HighDynamicRangeOutput").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HighDynamicRangeOutput {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.HighDynamicRangeOutput;{0f57806b-253b-4119-bb40-3a90e51384f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutput_Vtbl;
    const IID: ::windows_core::GUID = <IHighDynamicRangeOutput as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HighDynamicRangeOutput {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeOutput";
}
impl ::core::convert::From<HighDynamicRangeOutput> for ::windows_core::IUnknown {
    fn from(value: HighDynamicRangeOutput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeOutput> for ::windows_core::IUnknown {
    fn from(value: &HighDynamicRangeOutput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HighDynamicRangeOutput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HighDynamicRangeOutput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HighDynamicRangeOutput> for ::windows_core::IInspectable {
    fn from(value: HighDynamicRangeOutput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeOutput> for ::windows_core::IInspectable {
    fn from(value: &HighDynamicRangeOutput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HighDynamicRangeOutput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HighDynamicRangeOutput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HighDynamicRangeOutput {}
unsafe impl ::core::marker::Sync for HighDynamicRangeOutput {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e3692e4_4027_4847_a70b_df1d9a2a7b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EncodingProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptor2 {
    type Vtable = IAudioStreamDescriptor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e68f1f6_a448_497b_8840_85082665acf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetLeadingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LeadingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTrailingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrailingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptor3 {
    type Vtable = IAudioStreamDescriptor3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d220da1_8e83_44ef_8973_2f63e993f36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptorFactory {
    type Vtable = IAudioStreamDescriptorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a86ce9e_4cb1_4380_8e0c_83504b7f5bf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrack(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioTrack {
    type Vtable = IAudioTrack_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf23b6e77_3ef7_40de_b943_068b1321701d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrack_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub GetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "winrt-media")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrackOpenFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeeddb9b9_bb7c_4112_bf76_9384676f824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackOpenFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrackSupportInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x178beff7_cc39_44a6_b951_4a5653f073fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows_core::HRESULT,
    pub Degradation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradation) -> ::windows_core::HRESULT,
    pub DegradationReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradationReason) -> ::windows_core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChapterCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChapterCue {
    type Vtable = IChapterCue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72a98001_d38a_4c0a_8fa6_75cddaf4664c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChapterCue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICodecInfo {
    type Vtable = ICodecInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51e89f85_ea97_499c_86ac_4ce5e73f3a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CodecKind) -> ::windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CodecCategory) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Subtypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Subtypes: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsTrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecQuery(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICodecQuery {
    type Vtable = ICodecQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x222a953a_af61_4e04_808a_a4634e2f3ac4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecQuery_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: CodecKind, category: CodecCategory, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecSubtypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICodecSubtypesStatics {
    type Vtable = ICodecSubtypesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa66ac4f2_888b_4224_8cf6_2a8d4eb02382);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecSubtypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoFormatDV25: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDV50: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvh1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvhD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvsd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH263: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH264: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH265: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH264ES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatHevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatHevcES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatM4S2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMjpg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP43: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP4S: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP4V: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMpeg2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatVP80: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatVP90: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMpg1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMss1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMss2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWvc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormat420O: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAdts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrNB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrWB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrWP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyAC3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyAC3Spdif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyDDPlus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDrm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatFlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatFloat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMP3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMPeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMsp1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatOpus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatPcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWmaSpdif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioLossless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioV8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioV9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataCue {
    type Vtable = IDataCue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c7f676d_1fbc_4e2d_9a87_ee38bd1dc637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetData: usize,
    #[cfg(feature = "winrt-storage")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataCue2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataCue2 {
    type Vtable = IDataCue2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc561b15_95f2_49e8_96f1_8dd5dac68d93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCue2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19918426_c65b_46ba_85f8_13880576c90a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectionEffect {
    type Vtable = IFaceDetectionEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae15ebd2_0542_42a9_bc90_f283a29f46c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDesiredDetectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DesiredDetectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub FaceDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFaceDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectionEffectDefinition {
    type Vtable = IFaceDetectionEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43dca081_b848_4f33_b702_1fd2624fb016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetDetectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FaceDetectionMode) -> ::windows_core::HRESULT,
    pub DetectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FaceDetectionMode) -> ::windows_core::HRESULT,
    pub SetSynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffectFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ab08993_5dc8_447b_a247_5270bd802ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub DetectedFaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-media")))]
    DetectedFaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHighDynamicRangeControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55f1a7ae_d957_4dc9_9d1c_8553a82a7d99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHighDynamicRangeControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHighDynamicRangeOutput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutput_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f57806b_253b_4119_bb40_3a90e51384f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHighDynamicRangeOutput_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Certainty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub FrameControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-media")))]
    FrameControllers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageCue {
    type Vtable = IImageCue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52828282_367b_440b_9116_3c84570dd270);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageCue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows_core::HRESULT,
    pub Extent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetSoftwareBitmap: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SoftwareBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25bc45e1_9b08_4c2e_a855_4542f1a75deb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub RandomAccessStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    RandomAccessStream: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLightFusionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLightFusionResult {
    type Vtable = ILowLightFusionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78edbe35_27a0_42e0_9cd3_738d2089de9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLightFusionResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    Frame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLightFusionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLightFusionStatics {
    type Vtable = ILowLightFusionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5305016d_c29e_40e2_87a9_9e1fd2f192f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLightFusionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub SupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-graphics")))]
    SupportedBitmapPixelFormats: usize,
    pub MaxSupportedFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub FuseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameset: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-graphics")))]
    FuseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBinder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBinder {
    type Vtable = IMediaBinder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b7e40aa_de07_424f_83f1_f1de46c4fa2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBinder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Binding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb61cb25a_1b6d_4630_a86d_2f0837f712e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaBinder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetStream: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetStreamReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBindingEventArgs2 {
    type Vtable = IMediaBindingEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0464cceb_bb5a_482f_b8ba_f0284c696567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub SetAdaptiveMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetAdaptiveMediaSource: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetStorageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetStorageFile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBindingEventArgs3 {
    type Vtable = IMediaBindingEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8eb475e_19be_44fc_a5ed_7aba315037f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-networking")]
    pub SetDownloadOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    SetDownloadOperation: usize,
}
#[repr(transparent)]
pub struct IMediaCue(::windows_core::IUnknown);
impl IMediaCue {
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaCue> for ::windows_core::IUnknown {
    fn from(value: IMediaCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaCue> for ::windows_core::IUnknown {
    fn from(value: &IMediaCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaCue> for ::windows_core::IInspectable {
    fn from(value: IMediaCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaCue> for ::windows_core::IInspectable {
    fn from(value: &IMediaCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaCue {}
impl ::core::fmt::Debug for IMediaCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaCue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaCue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{c7d15e5d-59dc-431f-a0ee-27744323b36d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaCue {
    type Vtable = IMediaCue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7d15e5d_59dc_431f_a0ee_27744323b36d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCueEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCueEventArgs {
    type Vtable = IMediaCueEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd12f47f7_5fa4_4e68_9fe5_32160dcee57e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCueEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Cue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaSource(::windows_core::IUnknown);
impl IMediaSource {}
impl ::core::convert::From<IMediaSource> for ::windows_core::IUnknown {
    fn from(value: IMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaSource> for ::windows_core::IUnknown {
    fn from(value: &IMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaSource> for ::windows_core::IInspectable {
    fn from(value: IMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaSource> for ::windows_core::IInspectable {
    fn from(value: &IMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaSource {}
impl ::core::fmt::Debug for IMediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e7bfb599-a09d-4c21-bcdf-20af4f86b3d9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaSource {
    type Vtable = IMediaSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7bfb599_a09d_4c21_bcdf_20af4f86b3d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource2 {
    type Vtable = IMediaSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2eb61048_655f_4c37_b813_b4e45dfa0abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenOperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenOperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CustomProperties: usize,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ExternalTimedTextSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ExternalTimedTextSources: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ExternalTimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ExternalTimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource3 {
    type Vtable = IMediaSource3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb59f0d9b_4b6e_41ed_bbb4_7c7509a994ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource4 {
    type Vtable = IMediaSource4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdafad57_8eff_4c63_85a6_84de0ae3e4f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub AdaptiveMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    AdaptiveMediaSource: usize,
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MseStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource5 {
    type Vtable = IMediaSource5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x331a22ae_ed2e_4a22_94c8_b743a92b3022);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-networking")]
    pub DownloadOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    DownloadOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61e1ea97_1916_4810_b7f4_b642be829596);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAppServiceConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InitializeMediaStreamSourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInitializeMediaStreamSourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceAppServiceConnectionFactory {
    type Vtable = IMediaSourceAppServiceConnectionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65b912eb_80b9_44f9_9c1e_e120f6d92838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAppServiceConnectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserviceconnection: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceError {
    type Vtable = IMediaSourceError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c0a8965_37c5_4e9d_8d21_1cdee90cecc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceError_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceOpenOperationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc682ceb_e281_477c_a8e0_1acd654114c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceOpenOperationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a30af82_9071_4bac_bc39_ca2a93b717a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows_core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics {
    type Vtable = IMediaSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf77d6fa4_4652_410e_b1d8_e9a5e245a45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub CreateFromAdaptiveMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateFromAdaptiveMediaSource: usize,
    pub CreateFromMediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromMseStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromIMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStorageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStorageFile: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStream: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStreamReference: usize,
    pub CreateFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics2 {
    type Vtable = IMediaSourceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeee161a4_7f13_4896_b8cb_df0de5bcb9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromMediaBinder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics3 {
    type Vtable = IMediaSourceStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x453a30d6_2bea_4122_9f73_eace04526e35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub CreateFromMediaFrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framesource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateFromMediaFrameSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics4 {
    type Vtable = IMediaSourceStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x281b3bfc_e50a_4428_a500_9c4ed918d3f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-networking")]
    pub CreateFromDownloadOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperation: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    CreateFromDownloadOperation: usize,
}
#[repr(transparent)]
pub struct IMediaStreamDescriptor(::windows_core::IUnknown);
impl IMediaStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: IMediaStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: &IMediaStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: IMediaStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: &IMediaStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaStreamDescriptor {}
impl ::core::fmt::Debug for IMediaStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaStreamDescriptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{80f16e6e-92f7-451e-97d2-afd80742da70}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaStreamDescriptor {
    type Vtable = IMediaStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80f16e6e_92f7_451e_97d2_afd80742da70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaStreamDescriptor2(::windows_core::IUnknown);
impl IMediaStreamDescriptor2 {
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaStreamDescriptor2> for ::windows_core::IUnknown {
    fn from(value: IMediaStreamDescriptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor2> for ::windows_core::IUnknown {
    fn from(value: &IMediaStreamDescriptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaStreamDescriptor2> for ::windows_core::IInspectable {
    fn from(value: IMediaStreamDescriptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor2> for ::windows_core::IInspectable {
    fn from(value: &IMediaStreamDescriptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IMediaStreamDescriptor2> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: IMediaStreamDescriptor2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMediaStreamDescriptor2> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: &IMediaStreamDescriptor2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for &IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IMediaStreamDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaStreamDescriptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaStreamDescriptor2 {}
impl ::core::fmt::Debug for IMediaStreamDescriptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaStreamDescriptor2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaStreamDescriptor2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5073010f-e8b2-4071-b00b-ebf337a76b58}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaStreamDescriptor2 {
    type Vtable = IMediaStreamDescriptor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5073010f_e8b2_4071_b00b_ebf337a76b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSample(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSample {
    type Vtable = IMediaStreamSample_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c8db627_4b80_4361_9837_6cb7481ad9d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSample_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Processed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Buffer: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ExtendedProperties: usize,
    pub Protection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDecodeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DecodeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetKeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub KeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Discontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSample2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSample2 {
    type Vtable = IMediaStreamSample2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45078691_fce8_4746_a1c8_10c25d3d7cd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSample2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub Direct3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    Direct3D11Surface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleProtectionProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4eb88292_ecdf_493e_841d_dd4add7caca2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleProtectionProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetKeyIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetKeyIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetInitializationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetInitializationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetSubSampleMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetSubSampleMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSampleStatics {
    type Vtable = IMediaStreamSampleStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfdf218f_a6cf_4579_be41_73dd941ad972);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, timestamp: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromBuffer: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, count: u32, timestamp: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSampleStatics2 {
    type Vtable = IMediaStreamSampleStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9efe9521_6d46_494c_a2f8_d662922e2dd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: ::windows_core::RawPtr, timestamp: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    CreateFromDirect3D11Surface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource {
    type Vtable = IMediaStreamSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3712d543_45eb_4138_aa62_c01e26f3843f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Starting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Paused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SampleRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSampleRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SwitchStreamsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSwitchStreamsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NotifyError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstatus: MediaStreamSourceErrorStatus) -> ::windows_core::HRESULT,
    pub AddStreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub SetMediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetMediaProtectionManager: usize,
    #[cfg(feature = "winrt-media")]
    pub MediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    MediaProtectionManager: usize,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetCanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBufferTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub BufferTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetBufferedRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startoffset: ::winrt_foundation::TimeSpan, endoffset: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    MusicProperties: usize,
    #[cfg(feature = "winrt-storage")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    VideoProperties: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetThumbnail: usize,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
    pub AddProtectionKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamdescriptor: ::windows_core::RawPtr, keyIdentifier_array_size: u32, keyidentifier: *const u8, licenseData_array_size: u32, licensedata: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource2 {
    type Vtable = IMediaStreamSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec55d0ad_2e6a_4f74_adbb_b562d1533849);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SampleRendered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSampleRendered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource3 {
    type Vtable = IMediaStreamSource3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a2a2746_3ddd_4ddf_a121_94045ecf9440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetMaxSupportedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxSupportedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource4 {
    type Vtable = IMediaStreamSource4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d0cfcab_830d_417c_a3a9_2454fd6415c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd8c7eb2_4816_4e24_88f0_491ef7386406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceClosedRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x907c00e9_18a3_4951_887a_2c1eebd5c69e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceClosedRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaStreamSourceClosedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceFactory {
    type Vtable = IMediaStreamSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef77e0d9_d158_4b7a_863f_203342fbfd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows_core::RawPtr, descriptor2: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRenderedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d697b05_d4f2_4c7a_9dfe_8d6cd0b3ee84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRenderedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SampleLag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4db341a9_3501_4d9b_83f9_8f235c822532);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Sample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportSampleProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progress: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7895cc02_f982_43c8_9d16_c62d999319be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10f9bb9e_71c5_492f_847f_0da1f35e81f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf41468f2_c274_4940_a5bb_28a572452fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a9093e4_35c4_4b1b_a791_0d99db56dd1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetActualStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f1356a5_6340_4dc4_9910_068ed9f598f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41b8808e_38a9_4ec3_9ba0_b69b85501e90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldStreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NewStreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbee3d835_a505_4f9a_b943_2b8cb1b4bbd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42202b72_6ea1_4677_981e_350a0da412aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaTrack(::windows_core::IUnknown);
impl IMediaTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaTrackKind>::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaTrackKind>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaTrack> for ::windows_core::IUnknown {
    fn from(value: IMediaTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaTrack> for ::windows_core::IUnknown {
    fn from(value: &IMediaTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaTrack> for ::windows_core::IInspectable {
    fn from(value: IMediaTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaTrack> for ::windows_core::IInspectable {
    fn from(value: &IMediaTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaTrack {}
impl ::core::fmt::Debug for IMediaTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaTrack {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{03e1fafc-c931-491a-b46b-c10ee8c256b7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaTrack {
    type Vtable = IMediaTrack_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTrack_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseSourceBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseSourceBuffer {
    type Vtable = IMseSourceBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c1aa3e3_df8d_4079_a3fe_6849184b4e2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseSourceBuffer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UpdateStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdateStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UpdateEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdateEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Aborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MseAppendMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MseAppendMode) -> ::windows_core::HRESULT,
    pub IsUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Buffered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Buffered: usize,
    pub TimestampOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetTimestampOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AppendWindowStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetAppendWindowStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AppendWindowEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAppendWindowEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub AppendBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    AppendBuffer: usize,
    #[cfg(feature = "winrt-storage")]
    pub AppendStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    AppendStream: usize,
    #[cfg(feature = "winrt-storage")]
    pub AppendStreamMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, maxsize: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    AppendStreamMaxSize: usize,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::winrt_foundation::TimeSpan, end: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseSourceBufferList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseSourceBufferList {
    type Vtable = IMseSourceBufferList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95fae8e7_a8e7_4ebf_8927_145e940ba511);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseSourceBufferList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceBufferAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSourceBufferAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SourceBufferRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSourceBufferRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Buffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Buffers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseStreamSource {
    type Vtable = IMseStreamSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0b4198d_02f4_4923_88dd_81bc3f360ffa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Opened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Ended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SourceBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ActiveSourceBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MseReadyState) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddSourceBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveSourceBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: MseEndOfStreamStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseStreamSource2 {
    type Vtable = IMseStreamSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66f57d37_f9e7_418a_9cde_a020e956552b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LiveSeekableRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLiveSeekableRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseStreamSourceStatics {
    type Vtable = IMseStreamSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x465c679d_d570_43ce_ba21_0bff5f3fbd0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc04ba319_ca41_4813_bffd_7b08b0ed2557);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HighDynamicRangeAnalyzer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredAnalysisInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DesiredAnalysisInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SceneAnalyzed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSceneAnalyzed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b10e4c_7fd9_42e1_85eb_6572c297c987);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub FrameControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    FrameControlValues: usize,
    pub HighDynamicRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalysisEffectFrame2 {
    type Vtable = ISceneAnalysisEffectFrame2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d4e29be_061f_47ae_9915_02524b5f9a5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectFrame2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AnalysisRecommendation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneAnalysisRecommendation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalyzedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x146b9588_2851_45e4_ad55_44cf8df8db4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalyzedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISingleSelectMediaTrackList(::windows_core::IUnknown);
impl ISingleSelectMediaTrackList {
    pub fn SelectedIndexChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSelectedIndexChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::convert::From<ISingleSelectMediaTrackList> for ::windows_core::IUnknown {
    fn from(value: ISingleSelectMediaTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISingleSelectMediaTrackList> for ::windows_core::IUnknown {
    fn from(value: &ISingleSelectMediaTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISingleSelectMediaTrackList> for ::windows_core::IInspectable {
    fn from(value: ISingleSelectMediaTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISingleSelectMediaTrackList> for ::windows_core::IInspectable {
    fn from(value: &ISingleSelectMediaTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISingleSelectMediaTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISingleSelectMediaTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISingleSelectMediaTrackList {}
impl ::core::fmt::Debug for ISingleSelectMediaTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISingleSelectMediaTrackList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISingleSelectMediaTrackList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{77206f1f-c34f-494f-8077-2bad9ff4ecf1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISingleSelectMediaTrackList {
    type Vtable = ISingleSelectMediaTrackList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77206f1f_c34f_494f_8077_2bad9ff4ecf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleSelectMediaTrackList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSelectedIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechCue {
    type Vtable = ISpeechCue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaee254dc_1725_4bad_8043_a98499b017a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStartPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEndPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataStreamDescriptor {
    type Vtable = ITimedMetadataStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x133336bf_296a_463e_9ff9_01cd25691408);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EncodingProperties: usize,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataStreamDescriptorFactory {
    type Vtable = ITimedMetadataStreamDescriptorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc027de30_7362_4ff9_98b1_2dfd0b8d1cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrack(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrack {
    type Vtable = ITimedMetadataTrack_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e6aed9e_f67a_49a9_b330_cf03b0e9cf07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CueEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCueEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CueExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCueExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TrackFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTrackFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Cues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Cues: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ActiveCues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ActiveCues: usize,
    pub TimedMetadataKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataKind) -> ::windows_core::HRESULT,
    pub DispatchType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddCue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveCue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrack2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrack2 {
    type Vtable = ITimedMetadataTrack2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21b4b648_9f9d_40ba_a8f3_1a92753aef0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3767915_4114_4819_b9d9_dd76089e72f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackError_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackErrorCode) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrackFactory {
    type Vtable = ITimedMetadataTrackFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8dd57611_97b3_4e1f_852c_0f482c81ad26);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, kind: TimedMetadataKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa57fc9d1_6789_4d4d_b07f_84b4f31acb70);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITimedMetadataTrackProvider(::windows_core::IUnknown);
impl ITimedMetadataTrackProvider {
    #[cfg(feature = "winrt-foundation")]
    pub fn TimedMetadataTracks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataTracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TimedMetadataTrack>>(result__)
        }
    }
}
impl ::core::convert::From<ITimedMetadataTrackProvider> for ::windows_core::IUnknown {
    fn from(value: ITimedMetadataTrackProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimedMetadataTrackProvider> for ::windows_core::IUnknown {
    fn from(value: &ITimedMetadataTrackProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITimedMetadataTrackProvider> for ::windows_core::IInspectable {
    fn from(value: ITimedMetadataTrackProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimedMetadataTrackProvider> for ::windows_core::IInspectable {
    fn from(value: &ITimedMetadataTrackProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITimedMetadataTrackProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITimedMetadataTrackProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimedMetadataTrackProvider {}
impl ::core::fmt::Debug for ITimedMetadataTrackProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimedMetadataTrackProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ITimedMetadataTrackProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3b7f2024-f74e-4ade-93c5-219da05b6856}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ITimedMetadataTrackProvider {
    type Vtable = ITimedMetadataTrackProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b7f2024_f74e_4ade_93c5_219da05b6856);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextBouten(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextBouten {
    type Vtable = ITimedTextBouten_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9062783_5597_5092_820c_8f738e0f774a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextBouten_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextBoutenType) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Color: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetColor: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenPosition) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextBoutenPosition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextCue {
    type Vtable = ITimedTextCue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51c79e51_3b86_494d_b359_bb2ea7aca9a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextCue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CueRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCueRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CueStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCueStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Lines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Lines: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextLine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextLine {
    type Vtable = ITimedTextLine_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x978d7ce2_7308_4c66_be50_65777289f5df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextLine_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Subformats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Subformats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextRegion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextRegion {
    type Vtable = ITimedTextRegion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ed0881f_8a06_4222_9f59_b21bf40124b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextRegion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows_core::HRESULT,
    pub Extent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Background: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Background: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetBackground: usize,
    pub WritingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWritingMode) -> ::windows_core::HRESULT,
    pub SetWritingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWritingMode) -> ::windows_core::HRESULT,
    pub DisplayAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDisplayAlignment) -> ::windows_core::HRESULT,
    pub SetDisplayAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDisplayAlignment) -> ::windows_core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
    pub IsOverflowClipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOverflowClipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPadding) -> ::windows_core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPadding) -> ::windows_core::HRESULT,
    pub TextWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWrapping) -> ::windows_core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWrapping) -> ::windows_core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ScrollMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextScrollMode) -> ::windows_core::HRESULT,
    pub SetScrollMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextScrollMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextRuby(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextRuby {
    type Vtable = ITimedTextRuby_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10335c29_5b3c_5693_9959_d05a0bd24628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextRuby_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyPosition) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyPosition) -> ::windows_core::HRESULT,
    pub Align: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyAlign) -> ::windows_core::HRESULT,
    pub SetAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyAlign) -> ::windows_core::HRESULT,
    pub Reserve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyReserve) -> ::windows_core::HRESULT,
    pub SetReserve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyReserve) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSource {
    type Vtable = ITimedTextSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4ed9ba6_101f_404d_a949_82f33fcd93b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Resolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveResolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceResolveResultEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48907c9c_dcd8_4c33_9ad3_6cdce7b1c566);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceResolveResultEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Tracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Tracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSourceStatics {
    type Vtable = ITimedTextSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e311853_9aba_4ac4_bb98_2fb176c3bfdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStream: usize,
    pub CreateFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStreamWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStreamWithLanguage: usize,
    pub CreateFromUriWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSourceStatics2 {
    type Vtable = ITimedTextSourceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb66b7602_923e_43fa_9633_587075812db5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStreamWithIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, indexstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStreamWithIndex: usize,
    pub CreateFromUriWithIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, indexuri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromStreamWithIndexAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, indexstream: ::windows_core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromStreamWithIndexAndLanguage: usize,
    pub CreateFromUriWithIndexAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, indexuri: ::windows_core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextStyle {
    type Vtable = ITimedTextStyle_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb2384d_a825_40c2_a7f5_281eaedf3b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
    pub FontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWeight) -> ::windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWeight) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Foreground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Foreground: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetForeground: usize,
    #[cfg(feature = "winrt-ui")]
    pub Background: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Background: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetBackground: usize,
    pub IsBackgroundAlwaysShown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsBackgroundAlwaysShown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFlowDirection) -> ::windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextFlowDirection) -> ::windows_core::HRESULT,
    pub LineAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextLineAlignment) -> ::windows_core::HRESULT,
    pub SetLineAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextLineAlignment) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub OutlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    OutlineColor: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetOutlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetOutlineColor: usize,
    pub OutlineThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetOutlineThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
    pub OutlineRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetOutlineRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextStyle2 {
    type Vtable = ITimedTextStyle2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x655f492d_6111_4787_89cc_686fece57e14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFontStyle) -> ::windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextFontStyle) -> ::windows_core::HRESULT,
    pub IsUnderlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsUnderlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsLineThroughEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLineThroughEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsOverlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOverlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextStyle3 {
    type Vtable = ITimedTextStyle3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf803f93b_3e99_595e_bbb7_78a2fa13c270);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Ruby: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Bouten: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsTextCombined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsTextCombined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub FontAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFontAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSubformat(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSubformat {
    type Vtable = ITimedTextSubformat_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd713502f_3261_4722_a0c2_b937b2390f14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSubformat_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetStartIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SubformatStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSubformatStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStabilizationEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0808a650_9698_4e57_877b_bd7cb2ee0f8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStabilizationEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-media", feature = "winrt-media", feature = "winrt-media"))]
    pub GetRecommendedStreamConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controller: ::windows_core::RawPtr, desiredproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-media", feature = "winrt-media")))]
    GetRecommendedStreamConfiguration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x187eff28_67bb_4713_b900_4168da164529);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12ee0d55_9c2b_4440_8057_2c7a90f0cbec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EncodingProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamDescriptor2 {
    type Vtable = IVideoStreamDescriptor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b306e10_453e_4088_832d_c36fa4f94af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamDescriptorFactory {
    type Vtable = IVideoStreamDescriptorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x494ef6d1_bb75_43d2_9e5e_7b79a3afced4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrack(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTrack {
    type Vtable = IVideoTrack_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99f3b7f3_e298_4396_bb6a_a51be6a2a20a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrack_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub GetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "winrt-media")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrackOpenFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7679e231_04f9_4c82_a4ee_8602c8bb4754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackOpenFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrackSupportInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bb534a0_fc5f_450d_8ff0_778d590486de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows_core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ImageCue(::windows_core::IUnknown);
impl ImageCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ImageCue, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextPoint>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextPoint>(result__)
        }
    }
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, TimedTextPoint>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Extent(&self) -> ::windows_core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextSize>::zeroed();
            (::windows_core::Interface::vtable(this).Extent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextSize>(result__)
        }
    }
    pub fn SetExtent<'a, Param0: ::windows_core::IntoParam<'a, TimedTextSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetSoftwareBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSoftwareBitmap)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageCue {}
impl ::core::fmt::Debug for ImageCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageCue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageCue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.ImageCue;{52828282-367b-440b-9116-3c84570dd270})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageCue {
    type Vtable = IImageCue_Vtbl;
    const IID: ::windows_core::GUID = <IImageCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageCue {
    const NAME: &'static str = "Windows.Media.Core.ImageCue";
}
impl ::core::convert::From<ImageCue> for ::windows_core::IUnknown {
    fn from(value: ImageCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageCue> for ::windows_core::IUnknown {
    fn from(value: &ImageCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageCue> for ::windows_core::IInspectable {
    fn from(value: ImageCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageCue> for ::windows_core::IInspectable {
    fn from(value: &ImageCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageCue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageCue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for ImageCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for &ImageCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageCue {}
unsafe impl ::core::marker::Sync for ImageCue {}
#[repr(transparent)]
pub struct InitializeMediaStreamSourceRequestedEventArgs(::windows_core::IUnknown);
impl InitializeMediaStreamSourceRequestedEventArgs {
    pub fn Source(&self) -> ::windows_core::Result<MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSource>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn RandomAccessStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RandomAccessStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for InitializeMediaStreamSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InitializeMediaStreamSourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InitializeMediaStreamSourceRequestedEventArgs {}
impl ::core::fmt::Debug for InitializeMediaStreamSourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InitializeMediaStreamSourceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InitializeMediaStreamSourceRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs;{25bc45e1-9b08-4c2e-a855-4542f1a75deb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInitializeMediaStreamSourceRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InitializeMediaStreamSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs";
}
impl ::core::convert::From<InitializeMediaStreamSourceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InitializeMediaStreamSourceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InitializeMediaStreamSourceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InitializeMediaStreamSourceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InitializeMediaStreamSourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for InitializeMediaStreamSourceRequestedEventArgs {}
pub struct LowLightFusion;
impl LowLightFusion {
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub fn SupportedBitmapPixelFormats() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::Imaging::BitmapPixelFormat>> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    pub fn MaxSupportedFrameCount() -> ::windows_core::Result<i32> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedFrameCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub fn FuseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_graphics::Imaging::SoftwareBitmap>>>(frameset: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FuseAsync)(::windows_core::Interface::as_raw(this), frameset.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>(result__)
        })
    }
    pub fn ILowLightFusionStatics<R, F: FnOnce(&ILowLightFusionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LowLightFusion, ILowLightFusionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for LowLightFusion {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusion";
}
#[repr(transparent)]
pub struct LowLightFusionResult(::windows_core::IUnknown);
impl LowLightFusionResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn Frame(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLightFusionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLightFusionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLightFusionResult {}
impl ::core::fmt::Debug for LowLightFusionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLightFusionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LowLightFusionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.LowLightFusionResult;{78edbe35-27a0-42e0-9cd3-738d2089de9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LowLightFusionResult {
    type Vtable = ILowLightFusionResult_Vtbl;
    const IID: ::windows_core::GUID = <ILowLightFusionResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLightFusionResult {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusionResult";
}
impl ::core::convert::From<LowLightFusionResult> for ::windows_core::IUnknown {
    fn from(value: LowLightFusionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLightFusionResult> for ::windows_core::IUnknown {
    fn from(value: &LowLightFusionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LowLightFusionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LowLightFusionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLightFusionResult> for ::windows_core::IInspectable {
    fn from(value: LowLightFusionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLightFusionResult> for ::windows_core::IInspectable {
    fn from(value: &LowLightFusionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LowLightFusionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LowLightFusionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LowLightFusionResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: LowLightFusionResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LowLightFusionResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &LowLightFusionResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for LowLightFusionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &LowLightFusionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LowLightFusionResult {}
unsafe impl ::core::marker::Sync for LowLightFusionResult {}
#[repr(transparent)]
pub struct MediaBinder(::windows_core::IUnknown);
impl MediaBinder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaBinder, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Binding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Binding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBinding<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBinding)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Token(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Token)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToken)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBinder {}
impl ::core::fmt::Debug for MediaBinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBinder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBinder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaBinder;{2b7e40aa-de07-424f-83f1-f1de46c4fa2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBinder {
    type Vtable = IMediaBinder_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBinder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBinder {
    const NAME: &'static str = "Windows.Media.Core.MediaBinder";
}
impl ::core::convert::From<MediaBinder> for ::windows_core::IUnknown {
    fn from(value: MediaBinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBinder> for ::windows_core::IUnknown {
    fn from(value: &MediaBinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBinder> for ::windows_core::IInspectable {
    fn from(value: MediaBinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBinder> for ::windows_core::IInspectable {
    fn from(value: &MediaBinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBinder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBinder {}
unsafe impl ::core::marker::Sync for MediaBinder {}
#[repr(transparent)]
pub struct MediaBindingEventArgs(::windows_core::IUnknown);
impl MediaBindingEventArgs {
    pub fn Canceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBindingEventArgs, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MediaBinder(&self) -> ::windows_core::Result<MediaBinder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaBinder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBinder>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, stream: Param0, contenttype: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), contenttype.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetStreamReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, stream: Param0, contenttype: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamReference)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), contenttype.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetAdaptiveMediaSource<'a, Param0: ::windows_core::IntoParam<'a, super::Streaming::Adaptive::AdaptiveMediaSource>>(&self, mediasource: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAdaptiveMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetStorageFile<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStorageFile)(::windows_core::Interface::as_raw(this), file.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-networking")]
    pub fn SetDownloadOperation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_networking::BackgroundTransfer::DownloadOperation>>(&self, downloadoperation: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaBindingEventArgs3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDownloadOperation)(::windows_core::Interface::as_raw(this), downloadoperation.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaBindingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBindingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBindingEventArgs {}
impl ::core::fmt::Debug for MediaBindingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBindingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBindingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaBindingEventArgs;{b61cb25a-1b6d-4630-a86d-2f0837f712e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBindingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBindingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaBindingEventArgs";
}
impl ::core::convert::From<MediaBindingEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaBindingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBindingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaBindingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBindingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBindingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBindingEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaBindingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBindingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaBindingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBindingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBindingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBindingEventArgs {}
unsafe impl ::core::marker::Sync for MediaBindingEventArgs {}
#[repr(transparent)]
pub struct MediaCueEventArgs(::windows_core::IUnknown);
impl MediaCueEventArgs {
    pub fn Cue(&self) -> ::windows_core::Result<IMediaCue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Cue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IMediaCue>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCueEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCueEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCueEventArgs {}
impl ::core::fmt::Debug for MediaCueEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCueEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCueEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaCueEventArgs;{d12f47f7-5fa4-4e68-9fe5-32160dcee57e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCueEventArgs {
    type Vtable = IMediaCueEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCueEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaCueEventArgs";
}
impl ::core::convert::From<MediaCueEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaCueEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCueEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaCueEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCueEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCueEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCueEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaCueEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCueEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaCueEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCueEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCueEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCueEventArgs {}
unsafe impl ::core::marker::Sync for MediaCueEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaDecoderStatus {}
impl ::core::clone::Clone for MediaDecoderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaDecoderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaDecoderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaDecoderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDecoderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaDecoderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaDecoderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaSource(::windows_core::IUnknown);
impl MediaSource {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OpenOperationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OpenOperationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpenOperationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenOperationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CustomProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CustomProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn IsOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExternalTimedTextSources(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IObservableVector<TimedTextSource>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExternalTimedTextSources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IObservableVector<TimedTextSource>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExternalTimedMetadataTracks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IObservableVector<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExternalTimedMetadataTracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IObservableVector<TimedMetadataTrack>>(result__)
        }
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaSourceState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceState>(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn AdaptiveMediaSource(&self) -> ::windows_core::Result<super::Streaming::Adaptive::AdaptiveMediaSource> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdaptiveMediaSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Streaming::Adaptive::AdaptiveMediaSource>(result__)
        }
    }
    pub fn MediaStreamSource(&self) -> ::windows_core::Result<MediaStreamSource> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaStreamSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSource>(result__)
        }
    }
    pub fn MseStreamSource(&self) -> ::windows_core::Result<MseStreamSource> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MseStreamSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MseStreamSource>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn OpenAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-networking")]
    pub fn DownloadOperation(&self) -> ::windows_core::Result<::winrt_networking::BackgroundTransfer::DownloadOperation> {
        let this = &::windows_core::Interface::cast::<IMediaSource5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_networking::BackgroundTransfer::DownloadOperation>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateFromAdaptiveMediaSource<'a, Param0: ::windows_core::IntoParam<'a, super::Streaming::Adaptive::AdaptiveMediaSource>>(mediasource: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAdaptiveMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    pub fn CreateFromMediaStreamSource<'a, Param0: ::windows_core::IntoParam<'a, MediaStreamSource>>(mediasource: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaStreamSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    pub fn CreateFromMseStreamSource<'a, Param0: ::windows_core::IntoParam<'a, MseStreamSource>>(mediasource: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMseStreamSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    pub fn CreateFromIMediaSource<'a, Param0: ::windows_core::IntoParam<'a, IMediaSource>>(mediasource: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStorageFile<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(file: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStorageFile)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(stream: Param0, contenttype: Param1) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), contenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStreamReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(stream: Param0, contenttype: Param1) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamReference)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), contenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    pub fn CreateFromUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    pub fn CreateFromMediaBinder<'a, Param0: ::windows_core::IntoParam<'a, MediaBinder>>(binder: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaBinder)(::windows_core::Interface::as_raw(this), binder.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateFromMediaFrameSource<'a, Param0: ::windows_core::IntoParam<'a, super::Capture::Frames::MediaFrameSource>>(framesource: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaFrameSource)(::windows_core::Interface::as_raw(this), framesource.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    #[cfg(feature = "winrt-networking")]
    pub fn CreateFromDownloadOperation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_networking::BackgroundTransfer::DownloadOperation>>(downloadoperation: Param0) -> ::windows_core::Result<MediaSource> {
        Self::IMediaSourceStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDownloadOperation)(::windows_core::Interface::as_raw(this), downloadoperation.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSource>(result__)
        })
    }
    pub fn IMediaSourceStatics<R, F: FnOnce(&IMediaSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaSource, IMediaSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaSourceStatics2<R, F: FnOnce(&IMediaSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaSource, IMediaSourceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaSourceStatics3<R, F: FnOnce(&IMediaSourceStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaSource, IMediaSourceStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaSourceStatics4<R, F: FnOnce(&IMediaSourceStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaSource, IMediaSourceStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSource {}
impl ::core::fmt::Debug for MediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSource;{2eb61048-655f-4c37-b813-b4e45dfa0abe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaSource {
    type Vtable = IMediaSource2_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSource2 as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSource {
    const NAME: &'static str = "Windows.Media.Core.MediaSource";
}
impl ::core::convert::From<MediaSource> for ::windows_core::IUnknown {
    fn from(value: MediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSource> for ::windows_core::IUnknown {
    fn from(value: &MediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSource> for ::windows_core::IInspectable {
    fn from(value: MediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSource> for ::windows_core::IInspectable {
    fn from(value: &MediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<MediaSource> for super::Playback::IMediaPlaybackSource {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&MediaSource> for super::Playback::IMediaPlaybackSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Playback::IMediaPlaybackSource> for MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::Playback::IMediaPlaybackSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Playback::IMediaPlaybackSource> for &MediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::Playback::IMediaPlaybackSource> {
        ::core::convert::TryInto::<super::Playback::IMediaPlaybackSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaSource {}
unsafe impl ::core::marker::Sync for MediaSource {}
#[repr(transparent)]
pub struct MediaSourceAppServiceConnection(::windows_core::IUnknown);
impl MediaSourceAppServiceConnection {
    pub fn InitializeMediaStreamSourceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).InitializeMediaStreamSourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInitializeMediaStreamSourceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInitializeMediaStreamSourceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::AppService::AppServiceConnection>>(appserviceconnection: Param0) -> ::windows_core::Result<MediaSourceAppServiceConnection> {
        Self::IMediaSourceAppServiceConnectionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), appserviceconnection.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaSourceAppServiceConnection>(result__)
        })
    }
    pub fn IMediaSourceAppServiceConnectionFactory<R, F: FnOnce(&IMediaSourceAppServiceConnectionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaSourceAppServiceConnection, IMediaSourceAppServiceConnectionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaSourceAppServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceAppServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceAppServiceConnection {}
impl ::core::fmt::Debug for MediaSourceAppServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAppServiceConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceAppServiceConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceAppServiceConnection;{61e1ea97-1916-4810-b7f4-b642be829596})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnection_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceAppServiceConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceAppServiceConnection {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceAppServiceConnection";
}
impl ::core::convert::From<MediaSourceAppServiceConnection> for ::windows_core::IUnknown {
    fn from(value: MediaSourceAppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAppServiceConnection> for ::windows_core::IUnknown {
    fn from(value: &MediaSourceAppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceAppServiceConnection> for ::windows_core::IInspectable {
    fn from(value: MediaSourceAppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAppServiceConnection> for ::windows_core::IInspectable {
    fn from(value: &MediaSourceAppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct MediaSourceError(::windows_core::IUnknown);
impl MediaSourceError {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaSourceError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceError {}
impl ::core::fmt::Debug for MediaSourceError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceError;{5c0a8965-37c5-4e9d-8d21-1cdee90cecc6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaSourceError {
    type Vtable = IMediaSourceError_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceError {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceError";
}
impl ::core::convert::From<MediaSourceError> for ::windows_core::IUnknown {
    fn from(value: MediaSourceError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceError> for ::windows_core::IUnknown {
    fn from(value: &MediaSourceError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaSourceError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaSourceError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceError> for ::windows_core::IInspectable {
    fn from(value: MediaSourceError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceError> for ::windows_core::IInspectable {
    fn from(value: &MediaSourceError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaSourceError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaSourceError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaSourceError {}
unsafe impl ::core::marker::Sync for MediaSourceError {}
#[repr(transparent)]
pub struct MediaSourceOpenOperationCompletedEventArgs(::windows_core::IUnknown);
impl MediaSourceOpenOperationCompletedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<MediaSourceError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceError>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaSourceOpenOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceOpenOperationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceOpenOperationCompletedEventArgs {}
impl ::core::fmt::Debug for MediaSourceOpenOperationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceOpenOperationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceOpenOperationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs;{fc682ceb-e281-477c-a8e0-1acd654114c8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceOpenOperationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs";
}
impl ::core::convert::From<MediaSourceOpenOperationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaSourceOpenOperationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceOpenOperationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaSourceOpenOperationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceOpenOperationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaSourceOpenOperationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceOpenOperationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaSourceOpenOperationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaSourceOpenOperationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceOpenOperationCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaSourceState {}
impl ::core::clone::Clone for MediaSourceState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaSourceState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaSourceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(::windows_core::IUnknown);
impl MediaSourceStateChangedEventArgs {
    pub fn OldState(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaSourceState>::zeroed();
            (::windows_core::Interface::vtable(this).OldState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceState>(result__)
        }
    }
    pub fn NewState(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaSourceState>::zeroed();
            (::windows_core::Interface::vtable(this).NewState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceState>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaSourceStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceStateChangedEventArgs {}
impl ::core::fmt::Debug for MediaSourceStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceStateChangedEventArgs;{0a30af82-9071-4bac-bc39-ca2a93b717a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceStateChangedEventArgs";
}
impl ::core::convert::From<MediaSourceStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaSourceStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaSourceStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaSourceStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaSourceStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaSourceStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaSourceStatus {}
impl ::core::clone::Clone for MediaSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaSourceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaStreamSample(::windows_core::IUnknown);
impl MediaStreamSample {
    pub fn Processed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSample, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Processed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveProcessed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProcessed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Buffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::Buffer>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<MediaStreamSamplePropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSamplePropertySet>(result__)
        }
    }
    pub fn Protection(&self) -> ::windows_core::Result<MediaStreamSampleProtectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Protection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSampleProtectionProperties>(result__)
        }
    }
    pub fn SetDecodeTimestamp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodeTimestamp)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DecodeTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DecodeTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetKeyFrame(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyFrame)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyFrame(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Discontinuous(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Discontinuous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn Direct3D11Surface(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSample2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Surface)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(buffer: Param0, timestamp: Param1) -> ::windows_core::Result<MediaStreamSample> {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaStreamSample>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(stream: Param0, count: u32, timestamp: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaStreamSample>> {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), count, timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaStreamSample>>(result__)
        })
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn CreateFromDirect3D11Surface<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(surface: Param0, timestamp: Param1) -> ::windows_core::Result<MediaStreamSample> {
        Self::IMediaStreamSampleStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDirect3D11Surface)(::windows_core::Interface::as_raw(this), surface.into_param().abi(), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaStreamSample>(result__)
        })
    }
    pub fn IMediaStreamSampleStatics<R, F: FnOnce(&IMediaStreamSampleStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaStreamSampleStatics2<R, F: FnOnce(&IMediaStreamSampleStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaStreamSample {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSample {}
impl ::core::fmt::Debug for MediaStreamSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSample").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSample {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSample;{5c8db627-4b80-4361-9837-6cb7481ad9d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSample {
    type Vtable = IMediaStreamSample_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSample as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSample {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSample";
}
impl ::core::convert::From<MediaStreamSample> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSample) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSample> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSample) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSample {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSample {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSample> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSample) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSample> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSample) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSample {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSample {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSample {}
unsafe impl ::core::marker::Sync for MediaStreamSample {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct MediaStreamSamplePropertySet(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl MediaStreamSamplePropertySet {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for MediaStreamSamplePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for MediaStreamSamplePropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for MediaStreamSamplePropertySet {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for MediaStreamSamplePropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSamplePropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for MediaStreamSamplePropertySet {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSamplePropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};g16;cinterface(IInspectable)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for MediaStreamSamplePropertySet {
    type Vtable = ::winrt_foundation::Collections::IMap_Vtbl<::windows_core::GUID, ::windows_core::IInspectable>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for MediaStreamSamplePropertySet {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSamplePropertySet";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for MediaStreamSamplePropertySet {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &MediaStreamSamplePropertySet {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaStreamSamplePropertySet> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSamplePropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaStreamSamplePropertySet> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSamplePropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaStreamSamplePropertySet> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSamplePropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaStreamSamplePropertySet> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSamplePropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<MediaStreamSamplePropertySet> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaStreamSamplePropertySet) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&MediaStreamSamplePropertySet> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaStreamSamplePropertySet) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> for &MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<MediaStreamSamplePropertySet> for ::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaStreamSamplePropertySet) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&MediaStreamSamplePropertySet> for ::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaStreamSamplePropertySet) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable>> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable>> for &MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for MediaStreamSamplePropertySet {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for MediaStreamSamplePropertySet {}
#[repr(transparent)]
pub struct MediaStreamSampleProtectionProperties(::windows_core::IUnknown);
impl MediaStreamSampleProtectionProperties {
    pub fn SetKeyIdentifier(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyIdentifier)(::windows_core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn GetKeyIdentifier(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetKeyIdentifier)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetInitializationVector(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitializationVector)(::windows_core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn GetInitializationVector(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetInitializationVector)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetSubSampleMapping(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubSampleMapping)(::windows_core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn GetSubSampleMapping(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetSubSampleMapping)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSampleProtectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSampleProtectionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSampleProtectionProperties {}
impl ::core::fmt::Debug for MediaStreamSampleProtectionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSampleProtectionProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSampleProtectionProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSampleProtectionProperties;{4eb88292-ecdf-493e-841d-dd4add7caca2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionProperties_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSampleProtectionProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSampleProtectionProperties {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSampleProtectionProperties";
}
impl ::core::convert::From<MediaStreamSampleProtectionProperties> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSampleProtectionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSampleProtectionProperties> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSampleProtectionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSampleProtectionProperties> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSampleProtectionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSampleProtectionProperties> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSampleProtectionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSampleProtectionProperties {}
unsafe impl ::core::marker::Sync for MediaStreamSampleProtectionProperties {}
#[repr(transparent)]
pub struct MediaStreamSource(::windows_core::IUnknown);
impl MediaStreamSource {
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Starting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Starting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Paused<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Paused)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePaused<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePaused)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SampleRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SampleRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSampleRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSampleRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SwitchStreamsRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SwitchStreamsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSwitchStreamsRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSwitchStreamsRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NotifyError(&self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyError)(::windows_core::Interface::as_raw(this), errorstatus).ok() }
    }
    pub fn AddStreamDescriptor<'a, Param0: ::windows_core::IntoParam<'a, IMediaStreamDescriptor>>(&self, descriptor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStreamDescriptor)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetMediaProtectionManager<'a, Param0: ::windows_core::IntoParam<'a, super::Protection::MediaProtectionManager>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaProtectionManager)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn MediaProtectionManager(&self) -> ::windows_core::Result<super::Protection::MediaProtectionManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaProtectionManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Protection::MediaProtectionManager>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetCanSeek(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanSeek)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetBufferTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBufferTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BufferTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).BufferTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetBufferedRange<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, startoffset: Param0, endoffset: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBufferedRange)(::windows_core::Interface::as_raw(this), startoffset.into_param().abi(), endoffset.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<::winrt_storage::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::FileProperties::MusicProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<::winrt_storage::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::FileProperties::VideoProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetThumbnail<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn AddProtectionKey<'a, Param0: ::windows_core::IntoParam<'a, IMediaStreamDescriptor>>(&self, streamdescriptor: Param0, keyidentifier: &[u8], licensedata: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddProtectionKey)(::windows_core::Interface::as_raw(this), streamdescriptor.into_param().abi(), keyidentifier.len() as u32, ::core::mem::transmute(keyidentifier.as_ptr()), licensedata.len() as u32, ::core::mem::transmute(licensedata.as_ptr())).ok() }
    }
    pub fn SampleRendered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SampleRendered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSampleRendered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSampleRendered)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetMaxSupportedPlaybackRate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSupportedPlaybackRate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxSupportedPlaybackRate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedPlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetIsLive(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLive)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLive(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CreateFromDescriptor<'a, Param0: ::windows_core::IntoParam<'a, IMediaStreamDescriptor>>(descriptor: Param0) -> ::windows_core::Result<MediaStreamSource> {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDescriptor)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaStreamSource>(result__)
        })
    }
    pub fn CreateFromDescriptors<'a, Param0: ::windows_core::IntoParam<'a, IMediaStreamDescriptor>, Param1: ::windows_core::IntoParam<'a, IMediaStreamDescriptor>>(descriptor: Param0, descriptor2: Param1) -> ::windows_core::Result<MediaStreamSource> {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDescriptors)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), descriptor2.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaStreamSource>(result__)
        })
    }
    pub fn IMediaStreamSourceFactory<R, F: FnOnce(&IMediaStreamSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaStreamSource, IMediaStreamSourceFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSource {}
impl ::core::fmt::Debug for MediaStreamSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSource;{3712d543-45eb-4138-aa62-c01e26f3843f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSource {
    type Vtable = IMediaStreamSource_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSource";
}
impl ::core::convert::From<MediaStreamSource> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSource> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSource> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSource> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaStreamSource> for IMediaSource {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaStreamSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaStreamSource> for IMediaSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaStreamSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaSource> for MediaStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaSource> for &MediaStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaSource> {
        ::core::convert::TryInto::<IMediaSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaStreamSource {}
unsafe impl ::core::marker::Sync for MediaStreamSource {}
#[repr(transparent)]
pub struct MediaStreamSourceClosedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceClosedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceClosedRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceClosedRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceClosedEventArgs;{cd8c7eb2-4816-4e24-88f0-491ef7386406})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceClosedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceClosedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaStreamSourceClosedReason(pub i32);
impl MediaStreamSourceClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const AppReportedError: Self = Self(2i32);
    pub const UnsupportedProtectionSystem: Self = Self(3i32);
    pub const ProtectionSystemFailure: Self = Self(4i32);
    pub const UnsupportedEncodingFormat: Self = Self(5i32);
    pub const MissingSampleRequestedEventHandler: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaStreamSourceClosedReason {}
impl ::core::clone::Clone for MediaStreamSourceClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaStreamSourceClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaStreamSourceClosedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaStreamSourceClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceClosedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceClosedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(::windows_core::IUnknown);
impl MediaStreamSourceClosedRequest {
    pub fn Reason(&self) -> ::windows_core::Result<MediaStreamSourceClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaStreamSourceClosedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceClosedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceClosedRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedRequest {}
impl ::core::fmt::Debug for MediaStreamSourceClosedRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceClosedRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceClosedRequest;{907c00e9-18a3-4951-887a-2c1eebd5c69e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceClosedRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceClosedRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedRequest";
}
impl ::core::convert::From<MediaStreamSourceClosedRequest> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceClosedRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedRequest> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceClosedRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceClosedRequest> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceClosedRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedRequest> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceClosedRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceClosedRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: Self = Self(0i32);
    pub const OutOfMemory: Self = Self(1i32);
    pub const FailedToOpenFile: Self = Self(2i32);
    pub const FailedToConnectToServer: Self = Self(3i32);
    pub const ConnectionToServerLost: Self = Self(4i32);
    pub const UnspecifiedNetworkError: Self = Self(5i32);
    pub const DecodeError: Self = Self(6i32);
    pub const UnsupportedMediaFormat: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaStreamSourceErrorStatus {}
impl ::core::clone::Clone for MediaStreamSourceErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaStreamSourceErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaStreamSourceErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaStreamSourceErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceErrorStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRenderedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceSampleRenderedEventArgs {
    pub fn SampleLag(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SampleLag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRenderedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRenderedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRenderedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRenderedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSampleRenderedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs;{9d697b05-d4f2-4c7a-9dfe-8d6cd0b3ee84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRenderedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRenderedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceSampleRenderedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSampleRenderedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRenderedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRenderedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRenderedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSampleRenderedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRenderedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRenderedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRenderedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRenderedEventArgs {}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequest(::windows_core::IUnknown);
impl MediaStreamSourceSampleRequest {
    pub fn StreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StreamDescriptor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IMediaStreamDescriptor>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceSampleRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceSampleRequestDeferral>(result__)
        }
    }
    pub fn SetSample<'a, Param0: ::windows_core::IntoParam<'a, MediaStreamSample>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSample)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Sample(&self) -> ::windows_core::Result<MediaStreamSample> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sample)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSample>(result__)
        }
    }
    pub fn ReportSampleProgress(&self, progress: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportSampleProgress)(::windows_core::Interface::as_raw(this), progress).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequest {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequest;{4db341a9-3501-4d9b-83f9-8f235c822532})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequest";
}
impl ::core::convert::From<MediaStreamSourceSampleRequest> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSampleRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequest> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRequest> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSampleRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequest> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequest {}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestDeferral(::windows_core::IUnknown);
impl MediaStreamSourceSampleRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequestDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequestDeferral;{7895cc02-f982-43c8-9d16-c62d999319be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestDeferral";
}
impl ::core::convert::From<MediaStreamSourceSampleRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSampleRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSampleRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestDeferral {}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceSampleRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceSampleRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceSampleRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequestedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs;{10f9bb9e-71c5-492f-847f-0da1f35e81f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceSampleRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSampleRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSampleRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestedEventArgs {}
#[repr(transparent)]
pub struct MediaStreamSourceStartingEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceStartingEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceStartingRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceStartingRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingEventArgs;{f41468f2-c274-4940-a5bb-28a572452fa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingEventArgs";
}
impl ::core::convert::From<MediaStreamSourceStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingEventArgs {}
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequest(::windows_core::IUnknown);
impl MediaStreamSourceStartingRequest {
    pub fn StartPosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceStartingRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceStartingRequestDeferral>(result__)
        }
    }
    pub fn SetActualStartPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, position: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetActualStartPosition)(::windows_core::Interface::as_raw(this), position.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceStartingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingRequest {}
impl ::core::fmt::Debug for MediaStreamSourceStartingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceStartingRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingRequest;{2a9093e4-35c4-4b1b-a791-0d99db56dd1d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequest";
}
impl ::core::convert::From<MediaStreamSourceStartingRequest> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceStartingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequest> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceStartingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceStartingRequest> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceStartingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequest> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceStartingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequest {}
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequestDeferral(::windows_core::IUnknown);
impl MediaStreamSourceStartingRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceStartingRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceStartingRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceStartingRequestDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingRequestDeferral;{3f1356a5-6340-4dc4-9910-068ed9f598f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequestDeferral";
}
impl ::core::convert::From<MediaStreamSourceStartingRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceStartingRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceStartingRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceStartingRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceStartingRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceStartingRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequestDeferral {}
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequest(::windows_core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequest {
    pub fn OldStreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OldStreamDescriptor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IMediaStreamDescriptor>(result__)
        }
    }
    pub fn NewStreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewStreamDescriptor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IMediaStreamDescriptor>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceSwitchStreamsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceSwitchStreamsRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequest {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest;{41b8808e-38a9-4ec3-9ba0-b69b85501e90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest";
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequest> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSwitchStreamsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequest> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSwitchStreamsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequest> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSwitchStreamsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequest> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSwitchStreamsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequest {}
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestDeferral(::windows_core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequestDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral;{bee3d835-a505-4f9a-b943-2b8cb1b4bbd9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral";
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestDeferral {}
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceSwitchStreamsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaStreamSourceSwitchStreamsRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs;{42202b72-6ea1-4677-981e-350a0da412aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaTrackKind {}
impl ::core::clone::Clone for MediaTrackKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaTrackKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaTrackKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaTrackKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTrackKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaTrackKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaTrackKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
impl ::core::marker::Copy for MseAppendMode {}
impl ::core::clone::Clone for MseAppendMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MseAppendMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MseAppendMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MseAppendMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseAppendMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MseAppendMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseAppendMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MseEndOfStreamStatus {}
impl ::core::clone::Clone for MseEndOfStreamStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MseEndOfStreamStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MseEndOfStreamStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MseEndOfStreamStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseEndOfStreamStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MseEndOfStreamStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseEndOfStreamStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
impl ::core::marker::Copy for MseReadyState {}
impl ::core::clone::Clone for MseReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MseReadyState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MseReadyState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MseReadyState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseReadyState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MseReadyState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseReadyState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MseSourceBuffer(::windows_core::IUnknown);
impl MseSourceBuffer {
    pub fn UpdateStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdateStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Updated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UpdateEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateEnded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdateEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Aborted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Aborted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAborted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAborted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Mode(&self) -> ::windows_core::Result<MseAppendMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MseAppendMode>::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MseAppendMode>(result__)
        }
    }
    pub fn SetMode(&self, value: MseAppendMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsUpdating(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUpdating)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Buffered(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MseTimeRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Buffered)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MseTimeRange>>(result__)
        }
    }
    pub fn TimestampOffset(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TimestampOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetTimestampOffset<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestampOffset)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AppendWindowStart(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AppendWindowStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetAppendWindowStart<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendWindowStart)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AppendWindowEnd(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppendWindowEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetAppendWindowEnd<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendWindowEnd)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn AppendBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn AppendStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, stream: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn AppendStreamMaxSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, stream: Param0, maxsize: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendStreamMaxSize)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), maxsize).ok() }
    }
    pub fn Abort(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Abort)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, start: Param0, end: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), start.into_param().abi(), end.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MseSourceBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MseSourceBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseSourceBuffer {}
impl ::core::fmt::Debug for MseSourceBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseSourceBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MseSourceBuffer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseSourceBuffer;{0c1aa3e3-df8d-4079-a3fe-6849184b4e2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MseSourceBuffer {
    type Vtable = IMseSourceBuffer_Vtbl;
    const IID: ::windows_core::GUID = <IMseSourceBuffer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MseSourceBuffer {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBuffer";
}
impl ::core::convert::From<MseSourceBuffer> for ::windows_core::IUnknown {
    fn from(value: MseSourceBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBuffer> for ::windows_core::IUnknown {
    fn from(value: &MseSourceBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MseSourceBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MseSourceBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MseSourceBuffer> for ::windows_core::IInspectable {
    fn from(value: MseSourceBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBuffer> for ::windows_core::IInspectable {
    fn from(value: &MseSourceBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MseSourceBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MseSourceBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MseSourceBuffer {}
unsafe impl ::core::marker::Sync for MseSourceBuffer {}
#[repr(transparent)]
pub struct MseSourceBufferList(::windows_core::IUnknown);
impl MseSourceBufferList {
    pub fn SourceBufferAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBufferList, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceBufferAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSourceBufferAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBufferAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SourceBufferRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseSourceBufferList, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceBufferRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSourceBufferRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBufferRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Buffers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MseSourceBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Buffers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MseSourceBuffer>>(result__)
        }
    }
}
impl ::core::clone::Clone for MseSourceBufferList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MseSourceBufferList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseSourceBufferList {}
impl ::core::fmt::Debug for MseSourceBufferList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseSourceBufferList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MseSourceBufferList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseSourceBufferList;{95fae8e7-a8e7-4ebf-8927-145e940ba511})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MseSourceBufferList {
    type Vtable = IMseSourceBufferList_Vtbl;
    const IID: ::windows_core::GUID = <IMseSourceBufferList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MseSourceBufferList {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBufferList";
}
impl ::core::convert::From<MseSourceBufferList> for ::windows_core::IUnknown {
    fn from(value: MseSourceBufferList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBufferList> for ::windows_core::IUnknown {
    fn from(value: &MseSourceBufferList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MseSourceBufferList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MseSourceBufferList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MseSourceBufferList> for ::windows_core::IInspectable {
    fn from(value: MseSourceBufferList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBufferList> for ::windows_core::IInspectable {
    fn from(value: &MseSourceBufferList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MseSourceBufferList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MseSourceBufferList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MseSourceBufferList {}
unsafe impl ::core::marker::Sync for MseSourceBufferList {}
#[repr(transparent)]
pub struct MseStreamSource(::windows_core::IUnknown);
impl MseStreamSource {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MseStreamSource, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Opened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Ended<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Ended)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SourceBuffers(&self) -> ::windows_core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceBuffers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MseSourceBufferList>(result__)
        }
    }
    pub fn ActiveSourceBuffers(&self) -> ::windows_core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveSourceBuffers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MseSourceBufferList>(result__)
        }
    }
    pub fn ReadyState(&self) -> ::windows_core::Result<MseReadyState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MseReadyState>::zeroed();
            (::windows_core::Interface::vtable(this).ReadyState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MseReadyState>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AddSourceBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, mimetype: Param0) -> ::windows_core::Result<MseSourceBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddSourceBuffer)(::windows_core::Interface::as_raw(this), mimetype.into_param().abi(), result__.as_mut_ptr()).from_abi::<MseSourceBuffer>(result__)
        }
    }
    pub fn RemoveSourceBuffer<'a, Param0: ::windows_core::IntoParam<'a, MseSourceBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    pub fn EndOfStream(&self, status: MseEndOfStreamStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndOfStream)(::windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn LiveSeekableRange(&self) -> ::windows_core::Result<::winrt_foundation::IReference<MseTimeRange>> {
        let this = &::windows_core::Interface::cast::<IMseStreamSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LiveSeekableRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<MseTimeRange>>(result__)
        }
    }
    pub fn SetLiveSeekableRange<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<MseTimeRange>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMseStreamSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLiveSeekableRange)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsContentTypeSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contenttype: Param0) -> ::windows_core::Result<bool> {
        Self::IMseStreamSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsContentTypeSupported)(::windows_core::Interface::as_raw(this), contenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IMseStreamSourceStatics<R, F: FnOnce(&IMseStreamSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MseStreamSource, IMseStreamSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MseStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MseStreamSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseStreamSource {}
impl ::core::fmt::Debug for MseStreamSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseStreamSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MseStreamSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseStreamSource;{b0b4198d-02f4-4923-88dd-81bc3f360ffa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MseStreamSource {
    type Vtable = IMseStreamSource_Vtbl;
    const IID: ::windows_core::GUID = <IMseStreamSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MseStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MseStreamSource";
}
impl ::core::convert::From<MseStreamSource> for ::windows_core::IUnknown {
    fn from(value: MseStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseStreamSource> for ::windows_core::IUnknown {
    fn from(value: &MseStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MseStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MseStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MseStreamSource> for ::windows_core::IInspectable {
    fn from(value: MseStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseStreamSource> for ::windows_core::IInspectable {
    fn from(value: &MseStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MseStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MseStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MseStreamSource> for IMediaSource {
    type Error = ::windows_core::Error;
    fn try_from(value: MseStreamSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MseStreamSource> for IMediaSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &MseStreamSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaSource> for MseStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaSource> for &MseStreamSource {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaSource> {
        ::core::convert::TryInto::<IMediaSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MseStreamSource {}
unsafe impl ::core::marker::Sync for MseStreamSource {}
#[repr(C)]
pub struct MseTimeRange {
    pub Start: ::winrt_foundation::TimeSpan,
    pub End: ::winrt_foundation::TimeSpan,
}
impl ::core::marker::Copy for MseTimeRange {}
impl ::core::clone::Clone for MseTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MseTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MseTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
unsafe impl ::windows_core::Abi for MseTimeRange {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for MseTimeRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.MseTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for MseTimeRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MseTimeRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for MseTimeRange {}
impl ::core::default::Default for MseTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct SceneAnalysisEffect(::windows_core::IUnknown);
impl SceneAnalysisEffect {
    #[cfg(feature = "winrt-foundation")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
    pub fn HighDynamicRangeAnalyzer(&self) -> ::windows_core::Result<HighDynamicRangeControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HighDynamicRangeAnalyzer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HighDynamicRangeControl>(result__)
        }
    }
    pub fn SetDesiredAnalysisInterval<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAnalysisInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DesiredAnalysisInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAnalysisInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SceneAnalyzed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SceneAnalyzed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSceneAnalyzed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSceneAnalyzed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SceneAnalysisEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneAnalysisEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisEffect {}
impl ::core::fmt::Debug for SceneAnalysisEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneAnalysisEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffect;{c04ba319-ca41-4813-bffd-7b08b0ed2557})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffect_Vtbl;
    const IID: ::windows_core::GUID = <ISceneAnalysisEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalysisEffect {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffect";
}
impl ::core::convert::From<SceneAnalysisEffect> for ::windows_core::IUnknown {
    fn from(value: SceneAnalysisEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffect> for ::windows_core::IUnknown {
    fn from(value: &SceneAnalysisEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneAnalysisEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneAnalysisEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneAnalysisEffect> for ::windows_core::IInspectable {
    fn from(value: SceneAnalysisEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffect> for ::windows_core::IInspectable {
    fn from(value: &SceneAnalysisEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneAnalysisEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneAnalysisEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneAnalysisEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneAnalysisEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneAnalysisEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneAnalysisEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for SceneAnalysisEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for &SceneAnalysisEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneAnalysisEffect {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffect {}
#[cfg(feature = "winrt-media")]
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "winrt-media")]
impl SceneAnalysisEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneAnalysisEffectDefinition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::clone::Clone for SceneAnalysisEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::cmp::PartialEq for SceneAnalysisEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::cmp::Eq for SceneAnalysisEffectDefinition {}
#[cfg(feature = "winrt-media")]
impl ::core::fmt::Debug for SceneAnalysisEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::windows_core::RuntimeType for SceneAnalysisEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::windows_core::Interface for SceneAnalysisEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-media")]
impl ::windows_core::RuntimeName for SceneAnalysisEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectDefinition";
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<SceneAnalysisEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: SceneAnalysisEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<&SceneAnalysisEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &SceneAnalysisEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<SceneAnalysisEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: SceneAnalysisEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<&SceneAnalysisEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &SceneAnalysisEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<SceneAnalysisEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneAnalysisEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&SceneAnalysisEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneAnalysisEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for &SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IVideoEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::core::marker::Send for SceneAnalysisEffectDefinition {}
#[cfg(feature = "winrt-media")]
unsafe impl ::core::marker::Sync for SceneAnalysisEffectDefinition {}
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(::windows_core::IUnknown);
impl SceneAnalysisEffectFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetSystemRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn FrameControlValues(&self) -> ::windows_core::Result<super::Capture::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameControlValues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Capture::CapturedFrameControlValues>(result__)
        }
    }
    pub fn HighDynamicRange(&self) -> ::windows_core::Result<HighDynamicRangeOutput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HighDynamicRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HighDynamicRangeOutput>(result__)
        }
    }
    pub fn AnalysisRecommendation(&self) -> ::windows_core::Result<SceneAnalysisRecommendation> {
        let this = &::windows_core::Interface::cast::<ISceneAnalysisEffectFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneAnalysisRecommendation>::zeroed();
            (::windows_core::Interface::vtable(this).AnalysisRecommendation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneAnalysisRecommendation>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneAnalysisEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneAnalysisEffectFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisEffectFrame {}
impl ::core::fmt::Debug for SceneAnalysisEffectFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffectFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneAnalysisEffectFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffectFrame;{d8b10e4c-7fd9-42e1-85eb-6572c297c987})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrame_Vtbl;
    const IID: ::windows_core::GUID = <ISceneAnalysisEffectFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalysisEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectFrame";
}
impl ::core::convert::From<SceneAnalysisEffectFrame> for ::windows_core::IUnknown {
    fn from(value: SceneAnalysisEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffectFrame> for ::windows_core::IUnknown {
    fn from(value: &SceneAnalysisEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneAnalysisEffectFrame> for ::windows_core::IInspectable {
    fn from(value: SceneAnalysisEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffectFrame> for ::windows_core::IInspectable {
    fn from(value: &SceneAnalysisEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneAnalysisEffectFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneAnalysisEffectFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneAnalysisEffectFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneAnalysisEffectFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneAnalysisEffectFrame> for super::IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneAnalysisEffectFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneAnalysisEffectFrame> for super::IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneAnalysisEffectFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaFrame> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaFrame> for &SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaFrame> {
        ::core::convert::TryInto::<super::IMediaFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneAnalysisEffectFrame {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffectFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAnalysisRecommendation {}
impl ::core::clone::Clone for SceneAnalysisRecommendation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneAnalysisRecommendation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SceneAnalysisRecommendation {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneAnalysisRecommendation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisRecommendation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneAnalysisRecommendation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.SceneAnalysisRecommendation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(::windows_core::IUnknown);
impl SceneAnalyzedEventArgs {
    pub fn ResultFrame(&self) -> ::windows_core::Result<SceneAnalysisEffectFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResultFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneAnalysisEffectFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneAnalyzedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneAnalyzedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalyzedEventArgs {}
impl ::core::fmt::Debug for SceneAnalyzedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalyzedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneAnalyzedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalyzedEventArgs;{146b9588-2851-45e4-ad55-44cf8df8db4d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISceneAnalyzedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalyzedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalyzedEventArgs";
}
impl ::core::convert::From<SceneAnalyzedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SceneAnalyzedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalyzedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SceneAnalyzedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneAnalyzedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SceneAnalyzedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalyzedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SceneAnalyzedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SceneAnalyzedEventArgs {}
unsafe impl ::core::marker::Sync for SceneAnalyzedEventArgs {}
#[repr(transparent)]
pub struct SpeechCue(::windows_core::IUnknown);
impl SpeechCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpeechCue, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartPositionInInput(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPositionInInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetStartPositionInInput<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartPositionInInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndPositionInInput(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndPositionInInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetEndPositionInInput<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndPositionInInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SpeechCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechCue {}
impl ::core::fmt::Debug for SpeechCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechCue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeechCue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SpeechCue;{aee254dc-1725-4bad-8043-a98499b017a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpeechCue {
    type Vtable = ISpeechCue_Vtbl;
    const IID: ::windows_core::GUID = <ISpeechCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpeechCue {
    const NAME: &'static str = "Windows.Media.Core.SpeechCue";
}
impl ::core::convert::From<SpeechCue> for ::windows_core::IUnknown {
    fn from(value: SpeechCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechCue> for ::windows_core::IUnknown {
    fn from(value: &SpeechCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpeechCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpeechCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechCue> for ::windows_core::IInspectable {
    fn from(value: SpeechCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechCue> for ::windows_core::IInspectable {
    fn from(value: &SpeechCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpeechCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpeechCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechCue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechCue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for SpeechCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for &SpeechCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechCue {}
unsafe impl ::core::marker::Sync for SpeechCue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: Self = Self(0i32);
    pub const Chapter: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Data: Self = Self(3i32);
    pub const Description: Self = Self(4i32);
    pub const Subtitle: Self = Self(5i32);
    pub const ImageSubtitle: Self = Self(6i32);
    pub const Speech: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedMetadataKind {}
impl ::core::clone::Clone for TimedMetadataKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedMetadataKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedMetadataKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedMetadataKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TimedMetadataStreamDescriptor(::windows_core::IUnknown);
impl TimedMetadataStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::TimedMetadataEncodingProperties> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::TimedMetadataEncodingProperties>(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<TimedMetadataStreamDescriptor> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataStreamDescriptor>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::TimedMetadataEncodingProperties>>(encodingproperties: Param0) -> ::windows_core::Result<TimedMetadataStreamDescriptor> {
        Self::ITimedMetadataStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedMetadataStreamDescriptor>(result__)
        })
    }
    pub fn ITimedMetadataStreamDescriptorFactory<R, F: FnOnce(&ITimedMetadataStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedMetadataStreamDescriptor, ITimedMetadataStreamDescriptorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedMetadataStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataStreamDescriptor {}
impl ::core::fmt::Debug for TimedMetadataStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataStreamDescriptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataStreamDescriptor;{80f16e6e-92f7-451e-97d2-afd80742da70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataStreamDescriptor {
    type Vtable = IMediaStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamDescriptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataStreamDescriptor";
}
impl ::core::convert::From<TimedMetadataStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: TimedMetadataStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: &TimedMetadataStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: TimedMetadataStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: &TimedMetadataStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedMetadataStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: TimedMetadataStreamDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimedMetadataStreamDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for &TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<TimedMetadataStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows_core::Error;
    fn try_from(value: TimedMetadataStreamDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimedMetadataStreamDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor2> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor2> for &TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor2> {
        ::core::convert::TryInto::<IMediaStreamDescriptor2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedMetadataStreamDescriptor {}
unsafe impl ::core::marker::Sync for TimedMetadataStreamDescriptor {}
#[repr(transparent)]
pub struct TimedMetadataTrack(::windows_core::IUnknown);
impl TimedMetadataTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaTrackKind>::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaTrackKind>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CueEntered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CueEntered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCueEntered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCueEntered)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CueExited<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CueExited)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCueExited<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCueExited)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TrackFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TrackFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTrackFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTrackFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Cues(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Cues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IMediaCue>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ActiveCues(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveCues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IMediaCue>>(result__)
        }
    }
    pub fn TimedMetadataKind(&self) -> ::windows_core::Result<TimedMetadataKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedMetadataKind>::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataKind>(result__)
        }
    }
    pub fn DispatchType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DispatchType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AddCue<'a, Param0: ::windows_core::IntoParam<'a, IMediaCue>>(&self, cue: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddCue)(::windows_core::Interface::as_raw(this), cue.into_param().abi()).ok() }
    }
    pub fn RemoveCue<'a, Param0: ::windows_core::IntoParam<'a, IMediaCue>>(&self, cue: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCue)(::windows_core::Interface::as_raw(this), cue.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0, language: Param1, kind: TimedMetadataKind) -> ::windows_core::Result<TimedMetadataTrack> {
        Self::ITimedMetadataTrackFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), id.into_param().abi(), language.into_param().abi(), kind, result__.as_mut_ptr()).from_abi::<TimedMetadataTrack>(result__)
        })
    }
    pub fn ITimedMetadataTrackFactory<R, F: FnOnce(&ITimedMetadataTrackFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedMetadataTrack, ITimedMetadataTrackFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedMetadataTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrack {}
impl ::core::fmt::Debug for TimedMetadataTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataTrack {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrack;{9e6aed9e-f67a-49a9-b330-cf03b0e9cf07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataTrack {
    type Vtable = ITimedMetadataTrack_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataTrack as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrack";
}
impl ::core::convert::From<TimedMetadataTrack> for ::windows_core::IUnknown {
    fn from(value: TimedMetadataTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrack> for ::windows_core::IUnknown {
    fn from(value: &TimedMetadataTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedMetadataTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedMetadataTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataTrack> for ::windows_core::IInspectable {
    fn from(value: TimedMetadataTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrack> for ::windows_core::IInspectable {
    fn from(value: &TimedMetadataTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedMetadataTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedMetadataTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedMetadataTrack> for IMediaTrack {
    type Error = ::windows_core::Error;
    fn try_from(value: TimedMetadataTrack) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataTrack> for IMediaTrack {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimedMetadataTrack) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaTrack> for TimedMetadataTrack {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaTrack> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaTrack> for &TimedMetadataTrack {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaTrack> {
        ::core::convert::TryInto::<IMediaTrack>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedMetadataTrack {}
unsafe impl ::core::marker::Sync for TimedMetadataTrack {}
#[repr(transparent)]
pub struct TimedMetadataTrackError(::windows_core::IUnknown);
impl TimedMetadataTrackError {
    pub fn ErrorCode(&self) -> ::windows_core::Result<TimedMetadataTrackErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedMetadataTrackErrorCode>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataTrackErrorCode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedMetadataTrackError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrackError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackError {}
impl ::core::fmt::Debug for TimedMetadataTrackError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataTrackError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrackError;{b3767915-4114-4819-b9d9-dd76089e72f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackError_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataTrackError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrackError {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackError";
}
impl ::core::convert::From<TimedMetadataTrackError> for ::windows_core::IUnknown {
    fn from(value: TimedMetadataTrackError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackError> for ::windows_core::IUnknown {
    fn from(value: &TimedMetadataTrackError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedMetadataTrackError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedMetadataTrackError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataTrackError> for ::windows_core::IInspectable {
    fn from(value: TimedMetadataTrackError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackError> for ::windows_core::IInspectable {
    fn from(value: &TimedMetadataTrackError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedMetadataTrackError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedMetadataTrackError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedMetadataTrackError {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackError {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackErrorCode {}
impl ::core::clone::Clone for TimedMetadataTrackErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedMetadataTrackErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedMetadataTrackErrorCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedMetadataTrackErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackErrorCode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataTrackErrorCode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataTrackErrorCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(::windows_core::IUnknown);
impl TimedMetadataTrackFailedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataTrackError>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedMetadataTrackFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrackFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackFailedEventArgs {}
impl ::core::fmt::Debug for TimedMetadataTrackFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataTrackFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrackFailedEventArgs;{a57fc9d1-6789-4d4d-b07f-84b4f31acb70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataTrackFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackFailedEventArgs";
}
impl ::core::convert::From<TimedMetadataTrackFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TimedMetadataTrackFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TimedMetadataTrackFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataTrackFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TimedMetadataTrackFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TimedMetadataTrackFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedMetadataTrackFailedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackFailedEventArgs {}
#[repr(transparent)]
pub struct TimedTextBouten(::windows_core::IUnknown);
impl TimedTextBouten {
    pub fn Type(&self) -> ::windows_core::Result<TimedTextBoutenType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextBoutenType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextBoutenType>(result__)
        }
    }
    pub fn SetType(&self, value: TimedTextBoutenType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Color(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextBoutenPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextBoutenPosition>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextBoutenPosition>(result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextBoutenPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextBouten {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextBouten {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextBouten {}
impl ::core::fmt::Debug for TimedTextBouten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBouten").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextBouten {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextBouten;{d9062783-5597-5092-820c-8f738e0f774a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextBouten {
    type Vtable = ITimedTextBouten_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextBouten as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextBouten {
    const NAME: &'static str = "Windows.Media.Core.TimedTextBouten";
}
impl ::core::convert::From<TimedTextBouten> for ::windows_core::IUnknown {
    fn from(value: TimedTextBouten) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextBouten> for ::windows_core::IUnknown {
    fn from(value: &TimedTextBouten) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextBouten {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextBouten {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextBouten> for ::windows_core::IInspectable {
    fn from(value: TimedTextBouten) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextBouten> for ::windows_core::IInspectable {
    fn from(value: &TimedTextBouten) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextBouten {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextBouten {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextBouten {}
unsafe impl ::core::marker::Sync for TimedTextBouten {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextBoutenPosition {}
impl ::core::clone::Clone for TimedTextBoutenPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextBoutenPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextBoutenPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextBoutenPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextBoutenPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const FilledCircle: Self = Self(2i32);
    pub const OpenCircle: Self = Self(3i32);
    pub const FilledDot: Self = Self(4i32);
    pub const OpenDot: Self = Self(5i32);
    pub const FilledSesame: Self = Self(6i32);
    pub const OpenSesame: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedTextBoutenType {}
impl ::core::clone::Clone for TimedTextBoutenType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextBoutenType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextBoutenType {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextBoutenType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextBoutenType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TimedTextCue(::windows_core::IUnknown);
impl TimedTextCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextCue, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CueRegion(&self) -> ::windows_core::Result<TimedTextRegion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CueRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextRegion>(result__)
        }
    }
    pub fn SetCueRegion<'a, Param0: ::windows_core::IntoParam<'a, TimedTextRegion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCueRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CueStyle(&self) -> ::windows_core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CueStyle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextStyle>(result__)
        }
    }
    pub fn SetCueStyle<'a, Param0: ::windows_core::IntoParam<'a, TimedTextStyle>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCueStyle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lines(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<TimedTextLine>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Lines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<TimedTextLine>>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedTextCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextCue {}
impl ::core::fmt::Debug for TimedTextCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextCue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextCue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextCue;{51c79e51-3b86-494d-b359-bb2ea7aca9a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextCue {
    type Vtable = ITimedTextCue_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextCue {
    const NAME: &'static str = "Windows.Media.Core.TimedTextCue";
}
impl ::core::convert::From<TimedTextCue> for ::windows_core::IUnknown {
    fn from(value: TimedTextCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextCue> for ::windows_core::IUnknown {
    fn from(value: &TimedTextCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextCue> for ::windows_core::IInspectable {
    fn from(value: TimedTextCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextCue> for ::windows_core::IInspectable {
    fn from(value: &TimedTextCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextCue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedTextCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: TimedTextCue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedTextCue> for IMediaCue {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimedTextCue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for TimedTextCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaCue> for &TimedTextCue {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedTextCue {}
unsafe impl ::core::marker::Sync for TimedTextCue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextDisplayAlignment {}
impl ::core::clone::Clone for TimedTextDisplayAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextDisplayAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextDisplayAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextDisplayAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextDisplayAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextDisplayAlignment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextDisplayAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct TimedTextDouble {
    pub Value: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextDouble {}
impl ::core::clone::Clone for TimedTextDouble {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextDouble").field("Value", &self.Value).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows_core::Abi for TimedTextDouble {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for TimedTextDouble {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextDouble;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TimedTextDouble {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextDouble>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextDouble {}
impl ::core::default::Default for TimedTextDouble {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextFlowDirection {}
impl ::core::clone::Clone for TimedTextFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextFlowDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextFlowDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextFlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFlowDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextFlowDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFlowDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextFontStyle {}
impl ::core::clone::Clone for TimedTextFontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextFontStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextFontStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextFontStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFontStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextFontStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFontStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TimedTextLine(::windows_core::IUnknown);
impl TimedTextLine {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextLine, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Subformats(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<TimedTextSubformat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Subformats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<TimedTextSubformat>>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedTextLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextLine {}
impl ::core::fmt::Debug for TimedTextLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLine").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextLine {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextLine;{978d7ce2-7308-4c66-be50-65777289f5df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextLine {
    type Vtable = ITimedTextLine_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextLine as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextLine {
    const NAME: &'static str = "Windows.Media.Core.TimedTextLine";
}
impl ::core::convert::From<TimedTextLine> for ::windows_core::IUnknown {
    fn from(value: TimedTextLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextLine> for ::windows_core::IUnknown {
    fn from(value: &TimedTextLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextLine> for ::windows_core::IInspectable {
    fn from(value: TimedTextLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextLine> for ::windows_core::IInspectable {
    fn from(value: &TimedTextLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextLine {}
unsafe impl ::core::marker::Sync for TimedTextLine {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextLineAlignment {}
impl ::core::clone::Clone for TimedTextLineAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextLineAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextLineAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextLineAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLineAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextLineAlignment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextLineAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct TimedTextPadding {
    pub Before: f64,
    pub After: f64,
    pub Start: f64,
    pub End: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPadding {}
impl ::core::clone::Clone for TimedTextPadding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPadding").field("Before", &self.Before).field("After", &self.After).field("Start", &self.Start).field("End", &self.End).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows_core::Abi for TimedTextPadding {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for TimedTextPadding {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPadding;f8;f8;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TimedTextPadding {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextPadding>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextPadding {}
impl ::core::default::Default for TimedTextPadding {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TimedTextPoint {
    pub X: f64,
    pub Y: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPoint {}
impl ::core::clone::Clone for TimedTextPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPoint").field("X", &self.X).field("Y", &self.Y).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows_core::Abi for TimedTextPoint {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for TimedTextPoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPoint;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TimedTextPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextPoint {}
impl ::core::default::Default for TimedTextPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct TimedTextRegion(::windows_core::IUnknown);
impl TimedTextRegion {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextRegion, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextPoint>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextPoint>(result__)
        }
    }
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, TimedTextPoint>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Extent(&self) -> ::windows_core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextSize>::zeroed();
            (::windows_core::Interface::vtable(this).Extent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextSize>(result__)
        }
    }
    pub fn SetExtent<'a, Param0: ::windows_core::IntoParam<'a, TimedTextSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Background(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Background)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn WritingMode(&self) -> ::windows_core::Result<TimedTextWritingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextWritingMode>::zeroed();
            (::windows_core::Interface::vtable(this).WritingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextWritingMode>(result__)
        }
    }
    pub fn SetWritingMode(&self, value: TimedTextWritingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWritingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayAlignment(&self) -> ::windows_core::Result<TimedTextDisplayAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextDisplayAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAlignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextDisplayAlignment>(result__)
        }
    }
    pub fn SetDisplayAlignment(&self, value: TimedTextDisplayAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextDouble>::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextDouble>(result__)
        }
    }
    pub fn SetLineHeight<'a, Param0: ::windows_core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsOverflowClipped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverflowClipped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOverflowClipped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverflowClipped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> ::windows_core::Result<TimedTextPadding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextPadding>::zeroed();
            (::windows_core::Interface::vtable(this).Padding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextPadding>(result__)
        }
    }
    pub fn SetPadding<'a, Param0: ::windows_core::IntoParam<'a, TimedTextPadding>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TextWrapping(&self) -> ::windows_core::Result<TimedTextWrapping> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextWrapping>::zeroed();
            (::windows_core::Interface::vtable(this).TextWrapping)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextWrapping>(result__)
        }
    }
    pub fn SetTextWrapping(&self, value: TimedTextWrapping) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextWrapping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScrollMode(&self) -> ::windows_core::Result<TimedTextScrollMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextScrollMode>::zeroed();
            (::windows_core::Interface::vtable(this).ScrollMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextScrollMode>(result__)
        }
    }
    pub fn SetScrollMode(&self, value: TimedTextScrollMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScrollMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRegion {}
impl ::core::fmt::Debug for TimedTextRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRegion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextRegion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextRegion;{1ed0881f-8a06-4222-9f59-b21bf40124b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextRegion {
    type Vtable = ITimedTextRegion_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextRegion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextRegion {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRegion";
}
impl ::core::convert::From<TimedTextRegion> for ::windows_core::IUnknown {
    fn from(value: TimedTextRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRegion> for ::windows_core::IUnknown {
    fn from(value: &TimedTextRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextRegion> for ::windows_core::IInspectable {
    fn from(value: TimedTextRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRegion> for ::windows_core::IInspectable {
    fn from(value: &TimedTextRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextRegion {}
unsafe impl ::core::marker::Sync for TimedTextRegion {}
#[repr(transparent)]
pub struct TimedTextRuby(::windows_core::IUnknown);
impl TimedTextRuby {
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextRubyPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextRubyPosition>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextRubyPosition>(result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextRubyPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Align(&self) -> ::windows_core::Result<TimedTextRubyAlign> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextRubyAlign>::zeroed();
            (::windows_core::Interface::vtable(this).Align)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextRubyAlign>(result__)
        }
    }
    pub fn SetAlign(&self, value: TimedTextRubyAlign) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlign)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Reserve(&self) -> ::windows_core::Result<TimedTextRubyReserve> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextRubyReserve>::zeroed();
            (::windows_core::Interface::vtable(this).Reserve)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextRubyReserve>(result__)
        }
    }
    pub fn SetReserve(&self, value: TimedTextRubyReserve) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReserve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextRuby {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextRuby {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRuby {}
impl ::core::fmt::Debug for TimedTextRuby {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRuby").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextRuby {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextRuby;{10335c29-5b3c-5693-9959-d05a0bd24628})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextRuby {
    type Vtable = ITimedTextRuby_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextRuby as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextRuby {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRuby";
}
impl ::core::convert::From<TimedTextRuby> for ::windows_core::IUnknown {
    fn from(value: TimedTextRuby) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRuby> for ::windows_core::IUnknown {
    fn from(value: &TimedTextRuby) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextRuby {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextRuby {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextRuby> for ::windows_core::IInspectable {
    fn from(value: TimedTextRuby) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRuby> for ::windows_core::IInspectable {
    fn from(value: &TimedTextRuby) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextRuby {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextRuby {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextRuby {}
unsafe impl ::core::marker::Sync for TimedTextRuby {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
    pub const SpaceAround: Self = Self(3i32);
    pub const SpaceBetween: Self = Self(4i32);
    pub const WithBase: Self = Self(5i32);
}
impl ::core::marker::Copy for TimedTextRubyAlign {}
impl ::core::clone::Clone for TimedTextRubyAlign {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextRubyAlign {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextRubyAlign {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextRubyAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyAlign").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextRubyAlign {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyAlign;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextRubyPosition {}
impl ::core::clone::Clone for TimedTextRubyPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextRubyPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextRubyPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextRubyPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextRubyPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: Self = Self(0i32);
    pub const Before: Self = Self(1i32);
    pub const After: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const Outside: Self = Self(4i32);
}
impl ::core::marker::Copy for TimedTextRubyReserve {}
impl ::core::clone::Clone for TimedTextRubyReserve {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextRubyReserve {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextRubyReserve {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextRubyReserve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyReserve").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextRubyReserve {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyReserve;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextScrollMode {}
impl ::core::clone::Clone for TimedTextScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextScrollMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextScrollMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextScrollMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextScrollMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextScrollMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextScrollMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct TimedTextSize {
    pub Height: f64,
    pub Width: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextSize {}
impl ::core::clone::Clone for TimedTextSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextSize").field("Height", &self.Height).field("Width", &self.Width).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows_core::Abi for TimedTextSize {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for TimedTextSize {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextSize;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TimedTextSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextSize>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextSize {}
impl ::core::default::Default for TimedTextSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct TimedTextSource(::windows_core::IUnknown);
impl TimedTextSource {
    pub fn Resolved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Resolved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveResolved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResolved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    pub fn CreateFromUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStreamWithLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(stream: Param0, defaultlanguage: Param1) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithLanguage)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), defaultlanguage.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    pub fn CreateFromUriWithLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uri: Param0, defaultlanguage: Param1) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithLanguage)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), defaultlanguage.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStreamWithIndex<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(stream: Param0, indexstream: Param1) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithIndex)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), indexstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    pub fn CreateFromUriWithIndex<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0, indexuri: Param1) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithIndex)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), indexuri.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromStreamWithIndexAndLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(stream: Param0, indexstream: Param1, defaultlanguage: Param2) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithIndexAndLanguage)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), indexstream.into_param().abi(), defaultlanguage.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    pub fn CreateFromUriWithIndexAndLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uri: Param0, indexuri: Param1, defaultlanguage: Param2) -> ::windows_core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithIndexAndLanguage)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), indexuri.into_param().abi(), defaultlanguage.into_param().abi(), result__.as_mut_ptr()).from_abi::<TimedTextSource>(result__)
        })
    }
    pub fn ITimedTextSourceStatics<R, F: FnOnce(&ITimedTextSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextSource, ITimedTextSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITimedTextSourceStatics2<R, F: FnOnce(&ITimedTextSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextSource, ITimedTextSourceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedTextSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSource {}
impl ::core::fmt::Debug for TimedTextSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSource;{c4ed9ba6-101f-404d-a949-82f33fcd93b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextSource {
    type Vtable = ITimedTextSource_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSource";
}
impl ::core::convert::From<TimedTextSource> for ::windows_core::IUnknown {
    fn from(value: TimedTextSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSource> for ::windows_core::IUnknown {
    fn from(value: &TimedTextSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextSource> for ::windows_core::IInspectable {
    fn from(value: TimedTextSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSource> for ::windows_core::IInspectable {
    fn from(value: &TimedTextSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextSource {}
unsafe impl ::core::marker::Sync for TimedTextSource {}
#[repr(transparent)]
pub struct TimedTextSourceResolveResultEventArgs(::windows_core::IUnknown);
impl TimedTextSourceResolveResultEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataTrackError>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Tracks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Tracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TimedMetadataTrack>>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedTextSourceResolveResultEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextSourceResolveResultEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSourceResolveResultEventArgs {}
impl ::core::fmt::Debug for TimedTextSourceResolveResultEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSourceResolveResultEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextSourceResolveResultEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSourceResolveResultEventArgs;{48907c9c-dcd8-4c33-9ad3-6cdce7b1c566})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextSourceResolveResultEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSourceResolveResultEventArgs";
}
impl ::core::convert::From<TimedTextSourceResolveResultEventArgs> for ::windows_core::IUnknown {
    fn from(value: TimedTextSourceResolveResultEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSourceResolveResultEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TimedTextSourceResolveResultEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextSourceResolveResultEventArgs> for ::windows_core::IInspectable {
    fn from(value: TimedTextSourceResolveResultEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSourceResolveResultEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TimedTextSourceResolveResultEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextSourceResolveResultEventArgs {}
unsafe impl ::core::marker::Sync for TimedTextSourceResolveResultEventArgs {}
#[repr(transparent)]
pub struct TimedTextStyle(::windows_core::IUnknown);
impl TimedTextStyle {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextStyle, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FontFamily(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetFontFamily<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontFamily)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextDouble>::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextDouble>(result__)
        }
    }
    pub fn SetFontSize<'a, Param0: ::windows_core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> ::windows_core::Result<TimedTextWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextWeight>::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextWeight>(result__)
        }
    }
    pub fn SetFontWeight(&self, value: TimedTextWeight) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontWeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Foreground(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetForeground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Background(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Background)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsBackgroundAlwaysShown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBackgroundAlwaysShown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsBackgroundAlwaysShown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBackgroundAlwaysShown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FlowDirection(&self) -> ::windows_core::Result<TimedTextFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextFlowDirection>::zeroed();
            (::windows_core::Interface::vtable(this).FlowDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextFlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: TimedTextFlowDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFlowDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineAlignment(&self) -> ::windows_core::Result<TimedTextLineAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextLineAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).LineAlignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextLineAlignment>(result__)
        }
    }
    pub fn SetLineAlignment(&self, value: TimedTextLineAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn OutlineColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).OutlineColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetOutlineColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OutlineThickness(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextDouble>::zeroed();
            (::windows_core::Interface::vtable(this).OutlineThickness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextDouble>(result__)
        }
    }
    pub fn SetOutlineThickness<'a, Param0: ::windows_core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineThickness)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OutlineRadius(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextDouble>::zeroed();
            (::windows_core::Interface::vtable(this).OutlineRadius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextDouble>(result__)
        }
    }
    pub fn SetOutlineRadius<'a, Param0: ::windows_core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineRadius)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FontStyle(&self) -> ::windows_core::Result<TimedTextFontStyle> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedTextFontStyle>::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextFontStyle>(result__)
        }
    }
    pub fn SetFontStyle(&self, value: TimedTextFontStyle) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFontStyle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsUnderlineEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsUnderlineEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsUnderlineEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLineThroughEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLineThroughEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLineThroughEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLineThroughEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOverlineEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverlineEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOverlineEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverlineEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ruby(&self) -> ::windows_core::Result<TimedTextRuby> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Ruby)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextRuby>(result__)
        }
    }
    pub fn Bouten(&self) -> ::windows_core::Result<TimedTextBouten> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Bouten)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextBouten>(result__)
        }
    }
    pub fn IsTextCombined(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTextCombined)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextCombined(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsTextCombined)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontAngleInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FontAngleInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFontAngleInDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFontAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextStyle {}
impl ::core::fmt::Debug for TimedTextStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextStyle;{1bb2384d-a825-40c2-a7f5-281eaedf3b55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextStyle {
    type Vtable = ITimedTextStyle_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextStyle as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextStyle {
    const NAME: &'static str = "Windows.Media.Core.TimedTextStyle";
}
impl ::core::convert::From<TimedTextStyle> for ::windows_core::IUnknown {
    fn from(value: TimedTextStyle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextStyle> for ::windows_core::IUnknown {
    fn from(value: &TimedTextStyle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextStyle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextStyle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextStyle> for ::windows_core::IInspectable {
    fn from(value: TimedTextStyle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextStyle> for ::windows_core::IInspectable {
    fn from(value: &TimedTextStyle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextStyle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextStyle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextStyle {}
unsafe impl ::core::marker::Sync for TimedTextStyle {}
#[repr(transparent)]
pub struct TimedTextSubformat(::windows_core::IUnknown);
impl TimedTextSubformat {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimedTextSubformat, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StartIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).StartIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetStartIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetLength(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubformatStyle(&self) -> ::windows_core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubformatStyle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedTextStyle>(result__)
        }
    }
    pub fn SetSubformatStyle<'a, Param0: ::windows_core::IntoParam<'a, TimedTextStyle>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubformatStyle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for TimedTextSubformat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextSubformat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSubformat {}
impl ::core::fmt::Debug for TimedTextSubformat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSubformat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextSubformat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSubformat;{d713502f-3261-4722-a0c2-b937b2390f14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedTextSubformat {
    type Vtable = ITimedTextSubformat_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextSubformat as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSubformat {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSubformat";
}
impl ::core::convert::From<TimedTextSubformat> for ::windows_core::IUnknown {
    fn from(value: TimedTextSubformat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSubformat> for ::windows_core::IUnknown {
    fn from(value: &TimedTextSubformat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedTextSubformat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedTextSubformat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextSubformat> for ::windows_core::IInspectable {
    fn from(value: TimedTextSubformat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSubformat> for ::windows_core::IInspectable {
    fn from(value: &TimedTextSubformat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedTextSubformat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedTextSubformat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextSubformat {}
unsafe impl ::core::marker::Sync for TimedTextSubformat {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextUnit {}
impl ::core::clone::Clone for TimedTextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
impl ::core::marker::Copy for TimedTextWeight {}
impl ::core::clone::Clone for TimedTextWeight {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextWeight {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextWeight {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextWeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWeight").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextWeight {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWeight;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextWrapping {}
impl ::core::clone::Clone for TimedTextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextWrapping {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextWrapping {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextWrapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWrapping").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextWrapping {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWrapping;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: Self = Self(0i32);
    pub const RightLeftTopBottom: Self = Self(1i32);
    pub const TopBottomRightLeft: Self = Self(2i32);
    pub const TopBottomLeftRight: Self = Self(3i32);
    pub const LeftRight: Self = Self(4i32);
    pub const RightLeft: Self = Self(5i32);
    pub const TopBottom: Self = Self(6i32);
}
impl ::core::marker::Copy for TimedTextWritingMode {}
impl ::core::clone::Clone for TimedTextWritingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextWritingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedTextWritingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedTextWritingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWritingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedTextWritingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWritingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct VideoStabilizationEffect(::windows_core::IUnknown);
impl VideoStabilizationEffect {
    #[cfg(feature = "winrt-foundation")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnabledChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-media", feature = "winrt-media"))]
    pub fn GetRecommendedStreamConfiguration<'a, Param0: ::windows_core::IntoParam<'a, super::Devices::VideoDeviceController>, Param1: ::windows_core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>>(&self, controller: Param0, desiredproperties: Param1) -> ::windows_core::Result<super::Capture::VideoStreamConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRecommendedStreamConfiguration)(::windows_core::Interface::as_raw(this), controller.into_param().abi(), desiredproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Capture::VideoStreamConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoStabilizationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStabilizationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffect {}
impl ::core::fmt::Debug for VideoStabilizationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoStabilizationEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffect;{0808a650-9698-4e57-877b-bd7cb2ee0f8a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffect_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStabilizationEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStabilizationEffect {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffect";
}
impl ::core::convert::From<VideoStabilizationEffect> for ::windows_core::IUnknown {
    fn from(value: VideoStabilizationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffect> for ::windows_core::IUnknown {
    fn from(value: &VideoStabilizationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoStabilizationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoStabilizationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStabilizationEffect> for ::windows_core::IInspectable {
    fn from(value: VideoStabilizationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffect> for ::windows_core::IInspectable {
    fn from(value: &VideoStabilizationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoStabilizationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoStabilizationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoStabilizationEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoStabilizationEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoStabilizationEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoStabilizationEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for VideoStabilizationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for &VideoStabilizationEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoStabilizationEffect {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffect {}
#[cfg(feature = "winrt-media")]
#[repr(transparent)]
pub struct VideoStabilizationEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "winrt-media")]
impl VideoStabilizationEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoStabilizationEffectDefinition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::clone::Clone for VideoStabilizationEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::cmp::PartialEq for VideoStabilizationEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::cmp::Eq for VideoStabilizationEffectDefinition {}
#[cfg(feature = "winrt-media")]
impl ::core::fmt::Debug for VideoStabilizationEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::windows_core::RuntimeType for VideoStabilizationEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::windows_core::Interface for VideoStabilizationEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-media")]
impl ::windows_core::RuntimeName for VideoStabilizationEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectDefinition";
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<VideoStabilizationEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: VideoStabilizationEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<&VideoStabilizationEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &VideoStabilizationEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<VideoStabilizationEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: VideoStabilizationEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::From<&VideoStabilizationEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &VideoStabilizationEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<VideoStabilizationEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoStabilizationEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&VideoStabilizationEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoStabilizationEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for &VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IVideoEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-media")]
unsafe impl ::core::marker::Send for VideoStabilizationEffectDefinition {}
#[cfg(feature = "winrt-media")]
unsafe impl ::core::marker::Sync for VideoStabilizationEffectDefinition {}
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(::windows_core::IUnknown);
impl VideoStabilizationEffectEnabledChangedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<VideoStabilizationEffectEnabledChangedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VideoStabilizationEffectEnabledChangedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoStabilizationEffectEnabledChangedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStabilizationEffectEnabledChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffectEnabledChangedEventArgs {}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoStabilizationEffectEnabledChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs;{187eff28-67bb-4713-b900-4168da164529})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStabilizationEffectEnabledChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStabilizationEffectEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs";
}
impl ::core::convert::From<VideoStabilizationEffectEnabledChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffectEnabledChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStabilizationEffectEnabledChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffectEnabledChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoStabilizationEffectEnabledChangedEventArgs {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffectEnabledChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoStabilizationEffectEnabledChangedReason {}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoStabilizationEffectEnabledChangedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VideoStabilizationEffectEnabledChangedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoStabilizationEffectEnabledChangedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.VideoStabilizationEffectEnabledChangedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct VideoStreamDescriptor(::windows_core::IUnknown);
impl VideoStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<VideoStreamDescriptor> {
        let this = &::windows_core::Interface::cast::<IVideoStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoStreamDescriptor>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>>(encodingproperties: Param0) -> ::windows_core::Result<VideoStreamDescriptor> {
        Self::IVideoStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoStreamDescriptor>(result__)
        })
    }
    pub fn IVideoStreamDescriptorFactory<R, F: FnOnce(&IVideoStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoStreamDescriptor, IVideoStreamDescriptorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStreamDescriptor {}
impl ::core::fmt::Debug for VideoStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoStreamDescriptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStreamDescriptor;{12ee0d55-9c2b-4440-8057-2c7a90f0cbec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStreamDescriptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.VideoStreamDescriptor";
}
impl ::core::convert::From<VideoStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: VideoStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamDescriptor> for ::windows_core::IUnknown {
    fn from(value: &VideoStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: VideoStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamDescriptor> for ::windows_core::IInspectable {
    fn from(value: &VideoStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoStreamDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoStreamDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor> for &VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<VideoStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoStreamDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoStreamDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor2> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaStreamDescriptor2> for &VideoStreamDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaStreamDescriptor2> {
        ::core::convert::TryInto::<IMediaStreamDescriptor2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoStreamDescriptor {}
unsafe impl ::core::marker::Sync for VideoStreamDescriptor {}
#[repr(transparent)]
pub struct VideoTrack(::windows_core::IUnknown);
impl VideoTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaTrackKind>::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaTrackKind>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn OpenFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpenFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn GetEncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetEncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SupportInfo(&self) -> ::windows_core::Result<VideoTrackSupportInfo> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoTrackSupportInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrack {}
impl ::core::fmt::Debug for VideoTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoTrack {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoTrack {
    type Vtable = IMediaTrack_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTrack as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrack {
    const NAME: &'static str = "Windows.Media.Core.VideoTrack";
}
impl ::core::convert::From<VideoTrack> for ::windows_core::IUnknown {
    fn from(value: VideoTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrack> for ::windows_core::IUnknown {
    fn from(value: &VideoTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTrack> for ::windows_core::IInspectable {
    fn from(value: VideoTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrack> for ::windows_core::IInspectable {
    fn from(value: &VideoTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoTrack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoTrack> for IMediaTrack {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoTrack) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoTrack> for IMediaTrack {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoTrack) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaTrack> for VideoTrack {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaTrack> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaTrack> for &VideoTrack {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaTrack> {
        ::core::convert::TryInto::<IMediaTrack>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoTrack {}
unsafe impl ::core::marker::Sync for VideoTrack {}
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(::windows_core::IUnknown);
impl VideoTrackOpenFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTrackOpenFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrackOpenFailedEventArgs {}
impl ::core::fmt::Debug for VideoTrackOpenFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrackOpenFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrackOpenFailedEventArgs;{7679e231-04f9-4c82-a4ee-8602c8bb4754})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IVideoTrackOpenFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackOpenFailedEventArgs";
}
impl ::core::convert::From<VideoTrackOpenFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: VideoTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackOpenFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &VideoTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTrackOpenFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: VideoTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackOpenFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &VideoTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for VideoTrackOpenFailedEventArgs {}
#[repr(transparent)]
pub struct VideoTrackSupportInfo(::windows_core::IUnknown);
impl VideoTrackSupportInfo {
    pub fn DecoderStatus(&self) -> ::windows_core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaDecoderStatus>::zeroed();
            (::windows_core::Interface::vtable(this).DecoderStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDecoderStatus>(result__)
        }
    }
    pub fn MediaSourceStatus(&self) -> ::windows_core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaSourceStatus>::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTrackSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrackSupportInfo {}
impl ::core::fmt::Debug for VideoTrackSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrackSupportInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoTrackSupportInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrackSupportInfo;{4bb534a0-fc5f-450d-8ff0-778d590486de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = <IVideoTrackSupportInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackSupportInfo";
}
impl ::core::convert::From<VideoTrackSupportInfo> for ::windows_core::IUnknown {
    fn from(value: VideoTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackSupportInfo> for ::windows_core::IUnknown {
    fn from(value: &VideoTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTrackSupportInfo> for ::windows_core::IInspectable {
    fn from(value: VideoTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackSupportInfo> for ::windows_core::IInspectable {
    fn from(value: &VideoTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoTrackSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoTrackSupportInfo {}
unsafe impl ::core::marker::Sync for VideoTrackSupportInfo {}
