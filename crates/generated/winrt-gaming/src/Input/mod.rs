#[cfg(feature = "Custom")]
pub mod Custom;
#[cfg(feature = "ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Preview")]
pub mod Preview;
#[repr(transparent)]
pub struct ArcadeStick(::windows_core::IUnknown);
impl ArcadeStick {
    pub fn GetButtonLabel(&self, button: ArcadeStickButtons) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetButtonLabel)(::windows_core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<ArcadeStickReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ArcadeStickReading>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ArcadeStickReading>(result__)
        }
    }
    pub fn ArcadeStickAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ArcadeStick>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ArcadeStickAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveArcadeStickAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveArcadeStickAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ArcadeStickRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<ArcadeStick>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ArcadeStickRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveArcadeStickRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveArcadeStickRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ArcadeSticks() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ArcadeStick>> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ArcadeSticks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ArcadeStick>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows_core::Result<ArcadeStick> {
        Self::IArcadeStickStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromGameController)(::windows_core::Interface::as_raw(this), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<ArcadeStick>(result__)
        })
    }
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn IArcadeStickStatics<R, F: FnOnce(&IArcadeStickStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ArcadeStick, IArcadeStickStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IArcadeStickStatics2<R, F: FnOnce(&IArcadeStickStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ArcadeStick, IArcadeStickStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ArcadeStick {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ArcadeStick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ArcadeStick {}
impl ::core::fmt::Debug for ArcadeStick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ArcadeStick").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ArcadeStick {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ArcadeStick;{b14a539d-befb-4c81-8051-15ecf3b13036})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ArcadeStick {
    type Vtable = IArcadeStick_Vtbl;
    const IID: ::windows_core::GUID = <IArcadeStick as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.ArcadeStick";
}
impl ::core::convert::From<ArcadeStick> for ::windows_core::IUnknown {
    fn from(value: ArcadeStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ArcadeStick> for ::windows_core::IUnknown {
    fn from(value: &ArcadeStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ArcadeStick> for ::windows_core::IInspectable {
    fn from(value: ArcadeStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ArcadeStick> for ::windows_core::IInspectable {
    fn from(value: &ArcadeStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ArcadeStick> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: ArcadeStick) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ArcadeStick> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: &ArcadeStick) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for &ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ArcadeStick> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: ArcadeStick) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ArcadeStick> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &ArcadeStick) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &ArcadeStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ArcadeStick {}
unsafe impl ::core::marker::Sync for ArcadeStick {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ArcadeStickButtons(pub u32);
impl ArcadeStickButtons {
    pub const None: Self = Self(0u32);
    pub const StickUp: Self = Self(1u32);
    pub const StickDown: Self = Self(2u32);
    pub const StickLeft: Self = Self(4u32);
    pub const StickRight: Self = Self(8u32);
    pub const Action1: Self = Self(16u32);
    pub const Action2: Self = Self(32u32);
    pub const Action3: Self = Self(64u32);
    pub const Action4: Self = Self(128u32);
    pub const Action5: Self = Self(256u32);
    pub const Action6: Self = Self(512u32);
    pub const Special1: Self = Self(1024u32);
    pub const Special2: Self = Self(2048u32);
}
impl ::core::marker::Copy for ArcadeStickButtons {}
impl ::core::clone::Clone for ArcadeStickButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ArcadeStickButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ArcadeStickButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for ArcadeStickButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ArcadeStickButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ArcadeStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ArcadeStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ArcadeStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ArcadeStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ArcadeStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for ArcadeStickButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ArcadeStickButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct ArcadeStickReading {
    pub Timestamp: u64,
    pub Buttons: ArcadeStickButtons,
}
impl ::core::marker::Copy for ArcadeStickReading {}
impl ::core::clone::Clone for ArcadeStickReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ArcadeStickReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ArcadeStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).finish()
    }
}
unsafe impl ::windows_core::Abi for ArcadeStickReading {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for ArcadeStickReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.ArcadeStickReading;u8;enum(Windows.Gaming.Input.ArcadeStickButtons;u4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ArcadeStickReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ArcadeStickReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for ArcadeStickReading {}
impl ::core::default::Default for ArcadeStickReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct FlightStick(::windows_core::IUnknown);
impl FlightStick {
    pub fn HatSwitchKind(&self) -> ::windows_core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerSwitchKind>::zeroed();
            (::windows_core::Interface::vtable(this).HatSwitchKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: FlightStickButtons) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetButtonLabel)(::windows_core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<FlightStickReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FlightStickReading>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FlightStickReading>(result__)
        }
    }
    pub fn FlightStickAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<FlightStick>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FlightStickAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveFlightStickAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveFlightStickAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn FlightStickRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<FlightStick>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FlightStickRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveFlightStickRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveFlightStickRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FlightSticks() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<FlightStick>> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlightSticks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<FlightStick>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows_core::Result<FlightStick> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromGameController)(::windows_core::Interface::as_raw(this), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<FlightStick>(result__)
        })
    }
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn IFlightStickStatics<R, F: FnOnce(&IFlightStickStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlightStick, IFlightStickStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FlightStick {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FlightStick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlightStick {}
impl ::core::fmt::Debug for FlightStick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlightStick").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FlightStick {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.FlightStick;{b4a2c01c-b83b-4459-a1a9-97b03c33da7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FlightStick {
    type Vtable = IFlightStick_Vtbl;
    const IID: ::windows_core::GUID = <IFlightStick as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.FlightStick";
}
impl ::core::convert::From<FlightStick> for ::windows_core::IUnknown {
    fn from(value: FlightStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlightStick> for ::windows_core::IUnknown {
    fn from(value: &FlightStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FlightStick> for ::windows_core::IInspectable {
    fn from(value: FlightStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlightStick> for ::windows_core::IInspectable {
    fn from(value: &FlightStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FlightStick> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: FlightStick) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FlightStick> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: &FlightStick) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for &FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FlightStick> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: FlightStick) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FlightStick> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &FlightStick) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &FlightStick {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FlightStick {}
unsafe impl ::core::marker::Sync for FlightStick {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: Self = Self(0u32);
    pub const FirePrimary: Self = Self(1u32);
    pub const FireSecondary: Self = Self(2u32);
}
impl ::core::marker::Copy for FlightStickButtons {}
impl ::core::clone::Clone for FlightStickButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlightStickButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FlightStickButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for FlightStickButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlightStickButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FlightStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FlightStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FlightStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FlightStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FlightStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for FlightStickButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.FlightStickButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct FlightStickReading {
    pub Timestamp: u64,
    pub Buttons: FlightStickButtons,
    pub HatSwitch: GameControllerSwitchPosition,
    pub Roll: f64,
    pub Pitch: f64,
    pub Yaw: f64,
    pub Throttle: f64,
}
impl ::core::marker::Copy for FlightStickReading {}
impl ::core::clone::Clone for FlightStickReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FlightStickReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FlightStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("HatSwitch", &self.HatSwitch).field("Roll", &self.Roll).field("Pitch", &self.Pitch).field("Yaw", &self.Yaw).field("Throttle", &self.Throttle).finish()
    }
}
unsafe impl ::windows_core::Abi for FlightStickReading {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for FlightStickReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.FlightStickReading;u8;enum(Windows.Gaming.Input.FlightStickButtons;u4);enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4);f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for FlightStickReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FlightStickReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for FlightStickReading {}
impl ::core::default::Default for FlightStickReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameControllerButtonLabel(pub i32);
impl GameControllerButtonLabel {
    pub const None: Self = Self(0i32);
    pub const XboxBack: Self = Self(1i32);
    pub const XboxStart: Self = Self(2i32);
    pub const XboxMenu: Self = Self(3i32);
    pub const XboxView: Self = Self(4i32);
    pub const XboxUp: Self = Self(5i32);
    pub const XboxDown: Self = Self(6i32);
    pub const XboxLeft: Self = Self(7i32);
    pub const XboxRight: Self = Self(8i32);
    pub const XboxA: Self = Self(9i32);
    pub const XboxB: Self = Self(10i32);
    pub const XboxX: Self = Self(11i32);
    pub const XboxY: Self = Self(12i32);
    pub const XboxLeftBumper: Self = Self(13i32);
    pub const XboxLeftTrigger: Self = Self(14i32);
    pub const XboxLeftStickButton: Self = Self(15i32);
    pub const XboxRightBumper: Self = Self(16i32);
    pub const XboxRightTrigger: Self = Self(17i32);
    pub const XboxRightStickButton: Self = Self(18i32);
    pub const XboxPaddle1: Self = Self(19i32);
    pub const XboxPaddle2: Self = Self(20i32);
    pub const XboxPaddle3: Self = Self(21i32);
    pub const XboxPaddle4: Self = Self(22i32);
    pub const Mode: Self = Self(23i32);
    pub const Select: Self = Self(24i32);
    pub const Menu: Self = Self(25i32);
    pub const View: Self = Self(26i32);
    pub const Back: Self = Self(27i32);
    pub const Start: Self = Self(28i32);
    pub const Options: Self = Self(29i32);
    pub const Share: Self = Self(30i32);
    pub const Up: Self = Self(31i32);
    pub const Down: Self = Self(32i32);
    pub const Left: Self = Self(33i32);
    pub const Right: Self = Self(34i32);
    pub const LetterA: Self = Self(35i32);
    pub const LetterB: Self = Self(36i32);
    pub const LetterC: Self = Self(37i32);
    pub const LetterL: Self = Self(38i32);
    pub const LetterR: Self = Self(39i32);
    pub const LetterX: Self = Self(40i32);
    pub const LetterY: Self = Self(41i32);
    pub const LetterZ: Self = Self(42i32);
    pub const Cross: Self = Self(43i32);
    pub const Circle: Self = Self(44i32);
    pub const Square: Self = Self(45i32);
    pub const Triangle: Self = Self(46i32);
    pub const LeftBumper: Self = Self(47i32);
    pub const LeftTrigger: Self = Self(48i32);
    pub const LeftStickButton: Self = Self(49i32);
    pub const Left1: Self = Self(50i32);
    pub const Left2: Self = Self(51i32);
    pub const Left3: Self = Self(52i32);
    pub const RightBumper: Self = Self(53i32);
    pub const RightTrigger: Self = Self(54i32);
    pub const RightStickButton: Self = Self(55i32);
    pub const Right1: Self = Self(56i32);
    pub const Right2: Self = Self(57i32);
    pub const Right3: Self = Self(58i32);
    pub const Paddle1: Self = Self(59i32);
    pub const Paddle2: Self = Self(60i32);
    pub const Paddle3: Self = Self(61i32);
    pub const Paddle4: Self = Self(62i32);
    pub const Plus: Self = Self(63i32);
    pub const Minus: Self = Self(64i32);
    pub const DownLeftArrow: Self = Self(65i32);
    pub const DialLeft: Self = Self(66i32);
    pub const DialRight: Self = Self(67i32);
    pub const Suspension: Self = Self(68i32);
}
impl ::core::marker::Copy for GameControllerButtonLabel {}
impl ::core::clone::Clone for GameControllerButtonLabel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameControllerButtonLabel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameControllerButtonLabel {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameControllerButtonLabel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameControllerButtonLabel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameControllerButtonLabel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerButtonLabel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: Self = Self(0i32);
    pub const FourWay: Self = Self(1i32);
    pub const EightWay: Self = Self(2i32);
}
impl ::core::marker::Copy for GameControllerSwitchKind {}
impl ::core::clone::Clone for GameControllerSwitchKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameControllerSwitchKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameControllerSwitchKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameControllerSwitchKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameControllerSwitchKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameControllerSwitchKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameControllerSwitchPosition(pub i32);
impl GameControllerSwitchPosition {
    pub const Center: Self = Self(0i32);
    pub const Up: Self = Self(1i32);
    pub const UpRight: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const DownRight: Self = Self(4i32);
    pub const Down: Self = Self(5i32);
    pub const DownLeft: Self = Self(6i32);
    pub const Left: Self = Self(7i32);
    pub const UpLeft: Self = Self(8i32);
}
impl ::core::marker::Copy for GameControllerSwitchPosition {}
impl ::core::clone::Clone for GameControllerSwitchPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameControllerSwitchPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameControllerSwitchPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameControllerSwitchPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameControllerSwitchPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameControllerSwitchPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct Gamepad(::windows_core::IUnknown);
impl Gamepad {
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn Vibration(&self) -> ::windows_core::Result<GamepadVibration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GamepadVibration>::zeroed();
            (::windows_core::Interface::vtable(this).Vibration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GamepadVibration>(result__)
        }
    }
    pub fn SetVibration<'a, Param0: ::windows_core::IntoParam<'a, GamepadVibration>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVibration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<GamepadReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GamepadReading>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GamepadReading>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: GamepadButtons) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = &::windows_core::Interface::cast::<IGamepad2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetButtonLabel)(::windows_core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GamepadAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<Gamepad>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GamepadAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveGamepadAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveGamepadAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn GamepadRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<Gamepad>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GamepadRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveGamepadRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveGamepadRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Gamepads() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Gamepad>> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Gamepads)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Gamepad>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows_core::Result<Gamepad> {
        Self::IGamepadStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromGameController)(::windows_core::Interface::as_raw(this), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<Gamepad>(result__)
        })
    }
    pub fn IGamepadStatics<R, F: FnOnce(&IGamepadStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Gamepad, IGamepadStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGamepadStatics2<R, F: FnOnce(&IGamepadStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Gamepad, IGamepadStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Gamepad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Gamepad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Gamepad {}
impl ::core::fmt::Debug for Gamepad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Gamepad").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Gamepad {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Gamepad;{bc7bb43c-0a69-3903-9e9d-a50f86a45de5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Gamepad {
    type Vtable = IGamepad_Vtbl;
    const IID: ::windows_core::GUID = <IGamepad as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Gamepad {
    const NAME: &'static str = "Windows.Gaming.Input.Gamepad";
}
impl ::core::convert::From<Gamepad> for ::windows_core::IUnknown {
    fn from(value: Gamepad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Gamepad> for ::windows_core::IUnknown {
    fn from(value: &Gamepad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Gamepad> for ::windows_core::IInspectable {
    fn from(value: Gamepad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Gamepad> for ::windows_core::IInspectable {
    fn from(value: &Gamepad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Gamepad> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: Gamepad) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Gamepad> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: &Gamepad) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for &Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<Gamepad> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: Gamepad) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Gamepad> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &Gamepad) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &Gamepad {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Gamepad {}
unsafe impl ::core::marker::Sync for Gamepad {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GamepadButtons(pub u32);
impl GamepadButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const A: Self = Self(4u32);
    pub const B: Self = Self(8u32);
    pub const X: Self = Self(16u32);
    pub const Y: Self = Self(32u32);
    pub const DPadUp: Self = Self(64u32);
    pub const DPadDown: Self = Self(128u32);
    pub const DPadLeft: Self = Self(256u32);
    pub const DPadRight: Self = Self(512u32);
    pub const LeftShoulder: Self = Self(1024u32);
    pub const RightShoulder: Self = Self(2048u32);
    pub const LeftThumbstick: Self = Self(4096u32);
    pub const RightThumbstick: Self = Self(8192u32);
    pub const Paddle1: Self = Self(16384u32);
    pub const Paddle2: Self = Self(32768u32);
    pub const Paddle3: Self = Self(65536u32);
    pub const Paddle4: Self = Self(131072u32);
}
impl ::core::marker::Copy for GamepadButtons {}
impl ::core::clone::Clone for GamepadButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GamepadButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GamepadButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for GamepadButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GamepadButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GamepadButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GamepadButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GamepadButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GamepadButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GamepadButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for GamepadButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GamepadButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct GamepadReading {
    pub Timestamp: u64,
    pub Buttons: GamepadButtons,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
    pub LeftThumbstickX: f64,
    pub LeftThumbstickY: f64,
    pub RightThumbstickX: f64,
    pub RightThumbstickY: f64,
}
impl ::core::marker::Copy for GamepadReading {}
impl ::core::clone::Clone for GamepadReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamepadReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamepadReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("LeftTrigger", &self.LeftTrigger).field("RightTrigger", &self.RightTrigger).field("LeftThumbstickX", &self.LeftThumbstickX).field("LeftThumbstickY", &self.LeftThumbstickY).field("RightThumbstickX", &self.RightThumbstickX).field("RightThumbstickY", &self.RightThumbstickY).finish()
    }
}
unsafe impl ::windows_core::Abi for GamepadReading {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for GamepadReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadReading;u8;enum(Windows.Gaming.Input.GamepadButtons;u4);f8;f8;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GamepadReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamepadReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamepadReading {}
impl ::core::default::Default for GamepadReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GamepadVibration {
    pub LeftMotor: f64,
    pub RightMotor: f64,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
}
impl ::core::marker::Copy for GamepadVibration {}
impl ::core::clone::Clone for GamepadVibration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamepadVibration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamepadVibration").field("LeftMotor", &self.LeftMotor).field("RightMotor", &self.RightMotor).field("LeftTrigger", &self.LeftTrigger).field("RightTrigger", &self.RightTrigger).finish()
    }
}
unsafe impl ::windows_core::Abi for GamepadVibration {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for GamepadVibration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadVibration;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GamepadVibration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamepadVibration>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamepadVibration {}
impl ::core::default::Default for GamepadVibration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct Headset(::windows_core::IUnknown);
impl Headset {
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn CaptureDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RenderDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RenderDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for Headset {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Headset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Headset {}
impl ::core::fmt::Debug for Headset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Headset").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Headset {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Headset;{3fd156ef-6925-3fa8-9181-029c5223ae3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Headset {
    type Vtable = IHeadset_Vtbl;
    const IID: ::windows_core::GUID = <IHeadset as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Headset {
    const NAME: &'static str = "Windows.Gaming.Input.Headset";
}
impl ::core::convert::From<Headset> for ::windows_core::IUnknown {
    fn from(value: Headset) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Headset> for ::windows_core::IUnknown {
    fn from(value: &Headset) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Headset {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Headset {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Headset> for ::windows_core::IInspectable {
    fn from(value: Headset) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Headset> for ::windows_core::IInspectable {
    fn from(value: &Headset) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Headset {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Headset {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Headset> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: Headset) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Headset> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &Headset) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for Headset {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &Headset {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Headset {}
unsafe impl ::core::marker::Sync for Headset {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcadeStick(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcadeStick {
    type Vtable = IArcadeStick_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb14a539d_befb_4c81_8051_15ecf3b13036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStick_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ArcadeStickReading) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcadeStickStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcadeStickStatics {
    type Vtable = IArcadeStickStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c37b8c8_37b1_4ad8_9458_200f1a30018e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ArcadeStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveArcadeStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ArcadeStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveArcadeStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ArcadeSticks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ArcadeSticks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcadeStickStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcadeStickStatics2 {
    type Vtable = IArcadeStickStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52b5d744_bb86_445a_b59c_596f0e2a49df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlightStick(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlightStick {
    type Vtable = IFlightStick_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4a2c01c_b83b_4459_a1a9_97b03c33da7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStick_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HatSwitchKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerSwitchKind) -> ::windows_core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FlightStickReading) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlightStickStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlightStickStatics {
    type Vtable = IFlightStickStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5514924a_fecc_435e_83dc_5cec8a18a520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStickStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FlightStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFlightStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FlightStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFlightStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FlightSticks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FlightSticks: usize,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGameController(::windows_core::IUnknown);
impl IGameController {
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
}
impl ::core::convert::From<IGameController> for ::windows_core::IUnknown {
    fn from(value: IGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameController> for ::windows_core::IUnknown {
    fn from(value: &IGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGameController> for ::windows_core::IInspectable {
    fn from(value: IGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameController> for ::windows_core::IInspectable {
    fn from(value: &IGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameController {}
impl ::core::fmt::Debug for IGameController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IGameController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1baf6522-5f64-42c5-8267-b9fe2215bfbd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IGameController {
    type Vtable = IGameController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1baf6522_5f64_42c5_8267_b9fe2215bfbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeadsetConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHeadsetConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub HeadsetDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHeadsetDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub UserChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    UserChanged: usize,
    pub RemoveUserChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Headset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsWireless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
}
#[repr(transparent)]
pub struct IGameControllerBatteryInfo(::windows_core::IUnknown);
impl IGameControllerBatteryInfo {
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
}
impl ::core::convert::From<IGameControllerBatteryInfo> for ::windows_core::IUnknown {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerBatteryInfo> for ::windows_core::IUnknown {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGameControllerBatteryInfo> for ::windows_core::IInspectable {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerBatteryInfo> for ::windows_core::IInspectable {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameControllerBatteryInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameControllerBatteryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerBatteryInfo {}
impl ::core::fmt::Debug for IGameControllerBatteryInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerBatteryInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IGameControllerBatteryInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{dcecc681-3963-4da6-955d-553f3b6f6161}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IGameControllerBatteryInfo {
    type Vtable = IGameControllerBatteryInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcecc681_3963_4da6_955d_553f3b6f6161);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerBatteryInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub TryGetBatteryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    TryGetBatteryReport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepad(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGamepad {
    type Vtable = IGamepad_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc7bb43c_0a69_3903_9e9d_a50f86a45de5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Vibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GamepadVibration) -> ::windows_core::HRESULT,
    pub SetVibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GamepadVibration) -> ::windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GamepadReading) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepad2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGamepad2 {
    type Vtable = IGamepad2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c1689bd_5915_4245_b0c0_c89fae0308ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepadStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGamepadStatics {
    type Vtable = IGamepadStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bbce529_d49c_39e9_9560_e47dde96b7c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GamepadAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGamepadAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GamepadRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGamepadRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Gamepads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Gamepads: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepadStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGamepadStatics2 {
    type Vtable = IGamepadStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42676dc5_0856_47c4_9213_b395504c3a3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHeadset(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHeadset {
    type Vtable = IHeadset_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fd156ef_6925_3fa8_9181_029c5223ae3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadset_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CaptureDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RenderDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRacingWheel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRacingWheel {
    type Vtable = IRacingWheel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf546656f_e106_4c82_a90f_554012904b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasClutch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasHandbrake: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasPatternShifter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxPatternShifterGear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxWheelAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-gaming")]
    pub WheelMotor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-gaming"))]
    WheelMotor: usize,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RacingWheelReading) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRacingWheelStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRacingWheelStatics {
    type Vtable = IRacingWheelStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ac12cd5_581b_4936_9f94_69f1e6514c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RacingWheelAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRacingWheelAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RacingWheelRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRacingWheelRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RacingWheels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RacingWheels: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRacingWheelStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRacingWheelStatics2 {
    type Vtable = IRacingWheelStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe666bcaa_edfd_4323_a9f6_3c384048d1ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawGameController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawGameController {
    type Vtable = IRawGameController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cad6d91_a7e1_4f71_9a78_33e9c5dfea62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AxisCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ButtonCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-gaming"))]
    pub ForceFeedbackMotors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-gaming")))]
    ForceFeedbackMotors: usize,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SwitchCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows_core::HRESULT,
    pub GetSwitchKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawGameController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawGameController2 {
    type Vtable = IRawGameController2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43c0c035_bb73_4756_a787_3ed6bea617bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub SimpleHapticsControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-foundation")))]
    SimpleHapticsControllers: usize,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawGameControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawGameControllerStatics {
    type Vtable = IRawGameControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb8d0792_e95a_4b19_afc7_0a59f8bf759e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RawGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRawGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RawGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRawGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RawGameControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RawGameControllers: usize,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUINavigationController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUINavigationController {
    type Vtable = IUINavigationController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5aeefdd_f50e_4a55_8cdc_d33229548175);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UINavigationReading) -> ::windows_core::HRESULT,
    pub GetOptionalButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
    pub GetRequiredButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUINavigationControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUINavigationControllerStatics {
    type Vtable = IUINavigationControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f14930a_f6f8_4a48_8d89_94786cca0c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UINavigationControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUINavigationControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UINavigationControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUINavigationControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub UINavigationControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UINavigationControllers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUINavigationControllerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUINavigationControllerStatics2 {
    type Vtable = IUINavigationControllerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0cb28e3_b20b_4b0b_9ed4_f3d53cec0de4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OptionalUINavigationButtons(pub u32);
impl OptionalUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Context1: Self = Self(1u32);
    pub const Context2: Self = Self(2u32);
    pub const Context3: Self = Self(4u32);
    pub const Context4: Self = Self(8u32);
    pub const PageUp: Self = Self(16u32);
    pub const PageDown: Self = Self(32u32);
    pub const PageLeft: Self = Self(64u32);
    pub const PageRight: Self = Self(128u32);
    pub const ScrollUp: Self = Self(256u32);
    pub const ScrollDown: Self = Self(512u32);
    pub const ScrollLeft: Self = Self(1024u32);
    pub const ScrollRight: Self = Self(2048u32);
}
impl ::core::marker::Copy for OptionalUINavigationButtons {}
impl ::core::clone::Clone for OptionalUINavigationButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OptionalUINavigationButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for OptionalUINavigationButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for OptionalUINavigationButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OptionalUINavigationButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OptionalUINavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OptionalUINavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OptionalUINavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OptionalUINavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OptionalUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for OptionalUINavigationButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RacingWheel(::windows_core::IUnknown);
impl RacingWheel {
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn HasClutch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasClutch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasHandbrake(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasHandbrake)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasPatternShifter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasPatternShifter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPatternShifterGear(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPatternShifterGear)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxWheelAngle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MaxWheelAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-gaming")]
    pub fn WheelMotor(&self) -> ::windows_core::Result<ForceFeedback::ForceFeedbackMotor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WheelMotor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedback::ForceFeedbackMotor>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: RacingWheelButtons) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetButtonLabel)(::windows_core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<RacingWheelReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RacingWheelReading>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RacingWheelReading>(result__)
        }
    }
    pub fn RacingWheelAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<RacingWheel>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RacingWheelAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRacingWheelAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRacingWheelAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn RacingWheelRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<RacingWheel>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RacingWheelRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRacingWheelRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRacingWheelRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RacingWheels() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<RacingWheel>> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RacingWheels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<RacingWheel>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows_core::Result<RacingWheel> {
        Self::IRacingWheelStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromGameController)(::windows_core::Interface::as_raw(this), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<RacingWheel>(result__)
        })
    }
    pub fn IRacingWheelStatics<R, F: FnOnce(&IRacingWheelStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RacingWheel, IRacingWheelStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRacingWheelStatics2<R, F: FnOnce(&IRacingWheelStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RacingWheel, IRacingWheelStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RacingWheel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RacingWheel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RacingWheel {}
impl ::core::fmt::Debug for RacingWheel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RacingWheel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RacingWheel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RacingWheel;{f546656f-e106-4c82-a90f-554012904b85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RacingWheel {
    type Vtable = IRacingWheel_Vtbl;
    const IID: ::windows_core::GUID = <IRacingWheel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.RacingWheel";
}
impl ::core::convert::From<RacingWheel> for ::windows_core::IUnknown {
    fn from(value: RacingWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RacingWheel> for ::windows_core::IUnknown {
    fn from(value: &RacingWheel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RacingWheel> for ::windows_core::IInspectable {
    fn from(value: RacingWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RacingWheel> for ::windows_core::IInspectable {
    fn from(value: &RacingWheel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RacingWheel> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: RacingWheel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RacingWheel> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: &RacingWheel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for &RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<RacingWheel> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: RacingWheel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RacingWheel> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &RacingWheel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &RacingWheel {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RacingWheel {}
unsafe impl ::core::marker::Sync for RacingWheel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RacingWheelButtons(pub u32);
impl RacingWheelButtons {
    pub const None: Self = Self(0u32);
    pub const PreviousGear: Self = Self(1u32);
    pub const NextGear: Self = Self(2u32);
    pub const DPadUp: Self = Self(4u32);
    pub const DPadDown: Self = Self(8u32);
    pub const DPadLeft: Self = Self(16u32);
    pub const DPadRight: Self = Self(32u32);
    pub const Button1: Self = Self(64u32);
    pub const Button2: Self = Self(128u32);
    pub const Button3: Self = Self(256u32);
    pub const Button4: Self = Self(512u32);
    pub const Button5: Self = Self(1024u32);
    pub const Button6: Self = Self(2048u32);
    pub const Button7: Self = Self(4096u32);
    pub const Button8: Self = Self(8192u32);
    pub const Button9: Self = Self(16384u32);
    pub const Button10: Self = Self(32768u32);
    pub const Button11: Self = Self(65536u32);
    pub const Button12: Self = Self(131072u32);
    pub const Button13: Self = Self(262144u32);
    pub const Button14: Self = Self(524288u32);
    pub const Button15: Self = Self(1048576u32);
    pub const Button16: Self = Self(2097152u32);
}
impl ::core::marker::Copy for RacingWheelButtons {}
impl ::core::clone::Clone for RacingWheelButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RacingWheelButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RacingWheelButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for RacingWheelButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RacingWheelButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RacingWheelButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RacingWheelButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RacingWheelButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RacingWheelButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RacingWheelButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for RacingWheelButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RacingWheelButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct RacingWheelReading {
    pub Timestamp: u64,
    pub Buttons: RacingWheelButtons,
    pub PatternShifterGear: i32,
    pub Wheel: f64,
    pub Throttle: f64,
    pub Brake: f64,
    pub Clutch: f64,
    pub Handbrake: f64,
}
impl ::core::marker::Copy for RacingWheelReading {}
impl ::core::clone::Clone for RacingWheelReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RacingWheelReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RacingWheelReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("PatternShifterGear", &self.PatternShifterGear).field("Wheel", &self.Wheel).field("Throttle", &self.Throttle).field("Brake", &self.Brake).field("Clutch", &self.Clutch).field("Handbrake", &self.Handbrake).finish()
    }
}
unsafe impl ::windows_core::Abi for RacingWheelReading {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for RacingWheelReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.RacingWheelReading;u8;enum(Windows.Gaming.Input.RacingWheelButtons;u4);i4;f8;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for RacingWheelReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RacingWheelReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for RacingWheelReading {}
impl ::core::default::Default for RacingWheelReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct RawGameController(::windows_core::IUnknown);
impl RawGameController {
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn AxisCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).AxisCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ButtonCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-gaming"))]
    pub fn ForceFeedbackMotors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ForceFeedbackMotors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SwitchCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SwitchCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GetButtonLabel(&self, buttonindex: i32) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetButtonLabel)(::windows_core::Interface::as_raw(this), buttonindex, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self, buttonarray: &mut [bool], switcharray: &mut [GameControllerSwitchPosition], axisarray: &mut [f64]) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), buttonarray.len() as u32, ::core::mem::transmute_copy(&buttonarray), switcharray.len() as u32, ::core::mem::transmute_copy(&switcharray), axisarray.len() as u32, ::core::mem::transmute_copy(&axisarray), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn GetSwitchKind(&self, switchindex: i32) -> ::windows_core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerSwitchKind>::zeroed();
            (::windows_core::Interface::vtable(this).GetSwitchKind)(::windows_core::Interface::as_raw(this), switchindex, result__.as_mut_ptr()).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub fn SimpleHapticsControllers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_devices::Haptics::SimpleHapticsController>> {
        let this = &::windows_core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsControllers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_devices::Haptics::SimpleHapticsController>>(result__)
        }
    }
    pub fn NonRoamableId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NonRoamableId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RawGameControllerAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<RawGameController>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RawGameControllerAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRawGameControllerAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRawGameControllerAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn RawGameControllerRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<RawGameController>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RawGameControllerRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRawGameControllerRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRawGameControllerRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RawGameControllers() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<RawGameController>> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RawGameControllers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<RawGameController>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows_core::Result<RawGameController> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromGameController)(::windows_core::Interface::as_raw(this), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<RawGameController>(result__)
        })
    }
    pub fn IRawGameControllerStatics<R, F: FnOnce(&IRawGameControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RawGameController, IRawGameControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RawGameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RawGameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RawGameController {}
impl ::core::fmt::Debug for RawGameController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RawGameController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RawGameController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RawGameController;{7cad6d91-a7e1-4f71-9a78-33e9c5dfea62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RawGameController {
    type Vtable = IRawGameController_Vtbl;
    const IID: ::windows_core::GUID = <IRawGameController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.RawGameController";
}
impl ::core::convert::From<RawGameController> for ::windows_core::IUnknown {
    fn from(value: RawGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawGameController> for ::windows_core::IUnknown {
    fn from(value: &RawGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RawGameController> for ::windows_core::IInspectable {
    fn from(value: RawGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawGameController> for ::windows_core::IInspectable {
    fn from(value: &RawGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RawGameController> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: RawGameController) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RawGameController> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: &RawGameController) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for &RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<RawGameController> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: RawGameController) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RawGameController> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &RawGameController) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &RawGameController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RawGameController {}
unsafe impl ::core::marker::Sync for RawGameController {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RequiredUINavigationButtons(pub u32);
impl RequiredUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const Accept: Self = Self(4u32);
    pub const Cancel: Self = Self(8u32);
    pub const Up: Self = Self(16u32);
    pub const Down: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const Right: Self = Self(128u32);
}
impl ::core::marker::Copy for RequiredUINavigationButtons {}
impl ::core::clone::Clone for RequiredUINavigationButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RequiredUINavigationButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RequiredUINavigationButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for RequiredUINavigationButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RequiredUINavigationButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RequiredUINavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RequiredUINavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RequiredUINavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RequiredUINavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RequiredUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for RequiredUINavigationButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UINavigationController(::windows_core::IUnknown);
impl UINavigationController {
    pub fn HeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetConnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetConnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeadsetDisconnected)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn UserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IGameController, ::winrt_system::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows_core::Result<Headset> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWireless)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<::winrt_devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Power::BatteryReport>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<UINavigationReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UINavigationReading>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UINavigationReading>(result__)
        }
    }
    pub fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetOptionalButtonLabel)(::windows_core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> ::windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerButtonLabel>::zeroed();
            (::windows_core::Interface::vtable(this).GetRequiredButtonLabel)(::windows_core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn UINavigationControllerAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<UINavigationController>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UINavigationControllerAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveUINavigationControllerAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveUINavigationControllerAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn UINavigationControllerRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<UINavigationController>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UINavigationControllerRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveUINavigationControllerRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveUINavigationControllerRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UINavigationControllers() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<UINavigationController>> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UINavigationControllers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<UINavigationController>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows_core::Result<UINavigationController> {
        Self::IUINavigationControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromGameController)(::windows_core::Interface::as_raw(this), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<UINavigationController>(result__)
        })
    }
    pub fn IUINavigationControllerStatics<R, F: FnOnce(&IUINavigationControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UINavigationController, IUINavigationControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUINavigationControllerStatics2<R, F: FnOnce(&IUINavigationControllerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UINavigationController, IUINavigationControllerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UINavigationController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UINavigationController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UINavigationController {}
impl ::core::fmt::Debug for UINavigationController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UINavigationController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UINavigationController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.UINavigationController;{e5aeefdd-f50e-4a55-8cdc-d33229548175})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UINavigationController {
    type Vtable = IUINavigationController_Vtbl;
    const IID: ::windows_core::GUID = <IUINavigationController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.UINavigationController";
}
impl ::core::convert::From<UINavigationController> for ::windows_core::IUnknown {
    fn from(value: UINavigationController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UINavigationController> for ::windows_core::IUnknown {
    fn from(value: &UINavigationController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UINavigationController> for ::windows_core::IInspectable {
    fn from(value: UINavigationController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UINavigationController> for ::windows_core::IInspectable {
    fn from(value: &UINavigationController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UINavigationController> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: UINavigationController) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UINavigationController> for IGameController {
    type Error = ::windows_core::Error;
    fn try_from(value: &UINavigationController) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameController> for &UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<UINavigationController> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: UINavigationController) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UINavigationController> for IGameControllerBatteryInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &UINavigationController) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerBatteryInfo> for &UINavigationController {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UINavigationController {}
unsafe impl ::core::marker::Sync for UINavigationController {}
#[repr(C)]
pub struct UINavigationReading {
    pub Timestamp: u64,
    pub RequiredButtons: RequiredUINavigationButtons,
    pub OptionalButtons: OptionalUINavigationButtons,
}
impl ::core::marker::Copy for UINavigationReading {}
impl ::core::clone::Clone for UINavigationReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UINavigationReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UINavigationReading").field("Timestamp", &self.Timestamp).field("RequiredButtons", &self.RequiredButtons).field("OptionalButtons", &self.OptionalButtons).finish()
    }
}
unsafe impl ::windows_core::Abi for UINavigationReading {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for UINavigationReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.UINavigationReading;u8;enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4);enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for UINavigationReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UINavigationReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for UINavigationReading {}
impl ::core::default::Default for UINavigationReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
