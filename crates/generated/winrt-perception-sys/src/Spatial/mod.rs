#[cfg(feature = "Preview")]
pub mod Preview;
#[cfg(feature = "Surfaces")]
pub mod Surfaces;
pub type SpatialAnchor = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: Self = Self(0i32);
    pub const Sharing: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAnchorExportPurpose {}
impl ::core::clone::Clone for SpatialAnchorExportPurpose {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialAnchorExportSufficiency = *mut ::core::ffi::c_void;
pub type SpatialAnchorExporter = *mut ::core::ffi::c_void;
pub type SpatialAnchorRawCoordinateSystemAdjustedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialAnchorStore = *mut ::core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "winrt-foundation-sys")]
pub struct SpatialBoundingBox {
    pub Center: ::winrt_foundation_sys::Numerics::Vector3,
    pub Extents: ::winrt_foundation_sys::Numerics::Vector3,
}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::marker::Copy for SpatialBoundingBox {}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::clone::Clone for SpatialBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "winrt-foundation-sys")]
pub struct SpatialBoundingFrustum {
    pub Near: ::winrt_foundation_sys::Numerics::Plane,
    pub Far: ::winrt_foundation_sys::Numerics::Plane,
    pub Right: ::winrt_foundation_sys::Numerics::Plane,
    pub Left: ::winrt_foundation_sys::Numerics::Plane,
    pub Top: ::winrt_foundation_sys::Numerics::Plane,
    pub Bottom: ::winrt_foundation_sys::Numerics::Plane,
}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::marker::Copy for SpatialBoundingFrustum {}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::clone::Clone for SpatialBoundingFrustum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "winrt-foundation-sys")]
pub struct SpatialBoundingOrientedBox {
    pub Center: ::winrt_foundation_sys::Numerics::Vector3,
    pub Extents: ::winrt_foundation_sys::Numerics::Vector3,
    pub Orientation: ::winrt_foundation_sys::Numerics::Quaternion,
}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::marker::Copy for SpatialBoundingOrientedBox {}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::clone::Clone for SpatialBoundingOrientedBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "winrt-foundation-sys")]
pub struct SpatialBoundingSphere {
    pub Center: ::winrt_foundation_sys::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::marker::Copy for SpatialBoundingSphere {}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::clone::Clone for SpatialBoundingSphere {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialBoundingVolume = *mut ::core::ffi::c_void;
pub type SpatialCoordinateSystem = *mut ::core::ffi::c_void;
pub type SpatialEntity = *mut ::core::ffi::c_void;
pub type SpatialEntityAddedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialEntityRemovedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialEntityStore = *mut ::core::ffi::c_void;
pub type SpatialEntityUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialEntityWatcher = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialEntityWatcherStatus {}
impl ::core::clone::Clone for SpatialEntityWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const PositionalTrackingActivating: Self = Self(2i32);
    pub const PositionalTrackingActive: Self = Self(3i32);
    pub const PositionalTrackingInhibited: Self = Self(4i32);
}
impl ::core::marker::Copy for SpatialLocatability {}
impl ::core::clone::Clone for SpatialLocatability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialLocation = *mut ::core::ffi::c_void;
pub type SpatialLocator = *mut ::core::ffi::c_void;
pub type SpatialLocatorAttachedFrameOfReference = *mut ::core::ffi::c_void;
pub type SpatialLocatorPositionalTrackingDeactivatingEventArgs = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: Self = Self(0i32);
    pub const Omnidirectional: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialLookDirectionRange {}
impl ::core::clone::Clone for SpatialLookDirectionRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: Self = Self(0i32);
    pub const Bounded: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialMovementRange {}
impl ::core::clone::Clone for SpatialMovementRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialPerceptionAccessStatus {}
impl ::core::clone::Clone for SpatialPerceptionAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "winrt-foundation-sys")]
pub struct SpatialRay {
    pub Origin: ::winrt_foundation_sys::Numerics::Vector3,
    pub Direction: ::winrt_foundation_sys::Numerics::Vector3,
}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::marker::Copy for SpatialRay {}
#[cfg(feature = "winrt-foundation-sys")]
impl ::core::clone::Clone for SpatialRay {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialStageFrameOfReference = *mut ::core::ffi::c_void;
pub type SpatialStationaryFrameOfReference = *mut ::core::ffi::c_void;
