#[cfg(feature = "winrt-")]
pub struct BackgroundEnergyDiagnostics;
#[cfg(feature = "winrt-")]
impl BackgroundEnergyDiagnostics {
    #[cfg(feature = "winrt-")]
    pub fn DeviceSpecificConversionFactor() -> ::windows_core::Result<f64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceSpecificConversionFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ComputeTotalEnergyUsage() -> ::windows_core::Result<u64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ComputeTotalEnergyUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ResetTotalEnergyUsage() -> ::windows_core::Result<()> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ResetTotalEnergyUsage)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn IBackgroundEnergyDiagnosticsStatics<R, F: FnOnce(&IBackgroundEnergyDiagnosticsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundEnergyDiagnostics, IBackgroundEnergyDiagnosticsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for BackgroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics";
}
#[cfg(feature = "winrt-")]
pub struct ForegroundEnergyDiagnostics;
#[cfg(feature = "winrt-")]
impl ForegroundEnergyDiagnostics {
    #[cfg(feature = "winrt-")]
    pub fn DeviceSpecificConversionFactor() -> ::windows_core::Result<f64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceSpecificConversionFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ComputeTotalEnergyUsage() -> ::windows_core::Result<u64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ComputeTotalEnergyUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ResetTotalEnergyUsage() -> ::windows_core::Result<()> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ResetTotalEnergyUsage)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn IForegroundEnergyDiagnosticsStatics<R, F: FnOnce(&IForegroundEnergyDiagnosticsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ForegroundEnergyDiagnostics, IForegroundEnergyDiagnosticsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for ForegroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics";
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IBackgroundEnergyDiagnosticsStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IBackgroundEnergyDiagnosticsStatics {
    type Vtable = IBackgroundEnergyDiagnosticsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7663702_d3a6_46e0_8f9b_50b95bb4f9c5);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundEnergyDiagnosticsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceSpecificConversionFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceSpecificConversionFactor: usize,
    #[cfg(feature = "winrt-")]
    pub ComputeTotalEnergyUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ComputeTotalEnergyUsage: usize,
    #[cfg(feature = "winrt-")]
    pub ResetTotalEnergyUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ResetTotalEnergyUsage: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IForegroundEnergyDiagnosticsStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IForegroundEnergyDiagnosticsStatics {
    type Vtable = IForegroundEnergyDiagnosticsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23ca0917_cd07_4609_be15_8fe894c5e41e);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundEnergyDiagnosticsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceSpecificConversionFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceSpecificConversionFactor: usize,
    #[cfg(feature = "winrt-")]
    pub ComputeTotalEnergyUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ComputeTotalEnergyUsage: usize,
    #[cfg(feature = "winrt-")]
    pub ResetTotalEnergyUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ResetTotalEnergyUsage: usize,
}
