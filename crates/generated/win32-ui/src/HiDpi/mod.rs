#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(lprect: *mut ::win32_foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: Param2, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdjustWindowRectExForDpi(lprect: *mut ::win32_foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: ::win32_foundation::BOOL, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AdjustWindowRectExForDpi(::core::mem::transmute(lprect), ::core::mem::transmute(dwstyle), bmenu.into_param().abi(), ::core::mem::transmute(dwexstyle), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual<'a, Param0: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>, Param1: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(dpicontexta: Param0, dpicontextb: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreDpiAwarenessContextsEqual(dpicontexta: DPI_AWARENESS_CONTEXT, dpicontextb: DPI_AWARENESS_CONTEXT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AreDpiAwarenessContextsEqual(dpicontexta.into_param().abi(), dpicontextb.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub u32);
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(0u32);
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(1u32);
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(2u32);
impl ::core::marker::Copy for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {}
impl ::core::clone::Clone for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub u32);
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(0u32);
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(1u32);
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(2u32);
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(4u32);
impl ::core::marker::Copy for DIALOG_DPI_CHANGE_BEHAVIORS {}
impl ::core::clone::Clone for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIALOG_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DPI_AWARENESS(pub i32);
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = DPI_AWARENESS(-1i32);
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = DPI_AWARENESS(0i32);
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = DPI_AWARENESS(1i32);
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = DPI_AWARENESS(2i32);
impl ::core::marker::Copy for DPI_AWARENESS {}
impl ::core::clone::Clone for DPI_AWARENESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DPI_AWARENESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DPI_AWARENESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DPI_AWARENESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_AWARENESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DPI_AWARENESS_CONTEXT(pub isize);
impl DPI_AWARENESS_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for DPI_AWARENESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for DPI_AWARENESS_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for DPI_AWARENESS_CONTEXT {}
impl ::core::fmt::Debug for DPI_AWARENESS_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_AWARENESS_CONTEXT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for DPI_AWARENESS_CONTEXT {
    type Abi = Self;
}
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3i32 as _);
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4i32 as _);
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2i32 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1i32 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5i32 as _);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DPI_HOSTING_BEHAVIOR(pub i32);
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(-1i32);
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(0i32);
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(1i32);
impl ::core::marker::Copy for DPI_HOSTING_BEHAVIOR {}
impl ::core::clone::Clone for DPI_HOSTING_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DPI_HOSTING_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DPI_HOSTING_BEHAVIOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DPI_HOSTING_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_HOSTING_BEHAVIOR").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn EnableNonClientDpiScaling<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableNonClientDpiScaling(hwnd: ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnableNonClientDpiScaling(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext<'a, Param0: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> DPI_AWARENESS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAwarenessFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS;
        }
        ::core::mem::transmute(GetAwarenessFromDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDialogControlDpiChangeBehavior(hwnd: ::win32_foundation::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
        }
        ::core::mem::transmute(GetDialogControlDpiChangeBehavior(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0) -> DIALOG_DPI_CHANGE_BEHAVIORS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDialogDpiChangeBehavior(hdlg: ::win32_foundation::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS;
        }
        ::core::mem::transmute(GetDialogDpiChangeBehavior(hdlg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hprocess: Param0) -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiAwarenessContextForProcess(hprocess: ::win32_foundation::HANDLE) -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(GetDpiAwarenessContextForProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn GetDpiForMonitor<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HMONITOR>>(hmonitor: Param0, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiForMonitor(hmonitor: ::win32_graphics::Gdi::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows_core::HRESULT;
        }
        GetDpiForMonitor(hmonitor.into_param().abi(), ::core::mem::transmute(dpitype), ::core::mem::transmute(dpix), ::core::mem::transmute(dpiy)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiForSystem() -> u32;
        }
        ::core::mem::transmute(GetDpiForSystem())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDpiForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiForWindow(hwnd: ::win32_foundation::HWND) -> u32;
        }
        ::core::mem::transmute(GetDpiForWindow(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext<'a, Param0: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> u32;
        }
        ::core::mem::transmute(GetDpiFromDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetProcessDpiAwareness<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hprocess: Param0) -> ::windows_core::Result<PROCESS_DPI_AWARENESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDpiAwareness(hprocess: ::win32_foundation::HANDLE, value: *mut PROCESS_DPI_AWARENESS) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<PROCESS_DPI_AWARENESS>::zeroed();
        GetProcessDpiAwareness(hprocess.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROCESS_DPI_AWARENESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetSystemDpiForProcess<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hprocess: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDpiForProcess(hprocess: ::win32_foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetSystemDpiForProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32;
        }
        ::core::mem::transmute(GetSystemMetricsForDpi(::core::mem::transmute(nindex), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(GetThreadDpiAwarenessContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
        }
        ::core::mem::transmute(GetThreadDpiHostingBehavior())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowDpiAwarenessContext(hwnd: ::win32_foundation::HWND) -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(GetWindowDpiAwarenessContext(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowDpiHostingBehavior(hwnd: ::win32_foundation::HWND) -> DPI_HOSTING_BEHAVIOR;
        }
        ::core::mem::transmute(GetWindowDpiHostingBehavior(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsValidDpiAwarenessContext<'a, Param0: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsValidDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, lppoint: *mut ::win32_foundation::POINT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: ::win32_foundation::HWND, lppoint: *mut ::win32_foundation::POINT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(LogicalToPhysicalPointForPerMonitorDPI(hwnd.into_param().abi(), ::core::mem::transmute(lppoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MONITOR_DPI_TYPE(pub i32);
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(1i32);
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(2i32);
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
impl ::core::marker::Copy for MONITOR_DPI_TYPE {}
impl ::core::clone::Clone for MONITOR_DPI_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MONITOR_DPI_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MONITOR_DPI_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MONITOR_DPI_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_DPI_TYPE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn OpenThemeDataForDpi<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwnd: Param0, pszclasslist: Param1, dpi: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThemeDataForDpi(hwnd: ::win32_foundation::HWND, pszclasslist: ::windows_core::PCWSTR, dpi: u32) -> isize;
        }
        ::core::mem::transmute(OpenThemeDataForDpi(hwnd.into_param().abi(), pszclasslist.into_param().abi(), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_DPI_AWARENESS(pub i32);
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(0i32);
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(1i32);
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(2i32);
impl ::core::marker::Copy for PROCESS_DPI_AWARENESS {}
impl ::core::clone::Clone for PROCESS_DPI_AWARENESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_DPI_AWARENESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROCESS_DPI_AWARENESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_DPI_AWARENESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DPI_AWARENESS").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, lppoint: *mut ::win32_foundation::POINT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: ::win32_foundation::HWND, lppoint: *mut ::win32_foundation::POINT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(PhysicalToLogicalPointForPerMonitorDPI(hwnd.into_param().abi(), ::core::mem::transmute(lppoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDialogControlDpiChangeBehavior(hwnd: ::win32_foundation::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetDialogControlDpiChangeBehavior(hwnd.into_param().abi(), ::core::mem::transmute(mask), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDialogDpiChangeBehavior(hdlg: ::win32_foundation::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetDialogDpiChangeBehavior(hdlg.into_param().abi(), ::core::mem::transmute(mask), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows_core::HRESULT;
        }
        SetProcessDpiAwareness(::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext<'a, Param0: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext<'a, Param0: ::windows_core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(dpicontext: Param0) -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDpiAwarenessContext(dpicontext: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(SetThreadDpiAwarenessContext(dpicontext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR;
        }
        ::core::mem::transmute(SetThreadDpiHostingBehavior(::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SystemParametersInfoForDpi(::core::mem::transmute(uiaction), ::core::mem::transmute(uiparam), ::core::mem::transmute(pvparam), ::core::mem::transmute(fwinini), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
