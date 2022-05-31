#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ALT_BREAKS(pub i32);
pub const ALT_BREAKS_SAME: ALT_BREAKS = ALT_BREAKS(0i32);
pub const ALT_BREAKS_UNIQUE: ALT_BREAKS = ALT_BREAKS(1i32);
pub const ALT_BREAKS_FULL: ALT_BREAKS = ALT_BREAKS(2i32);
impl ::core::marker::Copy for ALT_BREAKS {}
impl ::core::clone::Clone for ALT_BREAKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ALT_BREAKS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ALT_BREAKS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ALT_BREAKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ALT_BREAKS").field(&self.0).finish()
    }
}
pub const ASYNC_RECO_ADDSTROKE_FAILED: u32 = 4u32;
pub const ASYNC_RECO_INTERRUPTED: u32 = 1u32;
pub const ASYNC_RECO_PROCESS_FAILED: u32 = 2u32;
pub const ASYNC_RECO_RESETCONTEXT_FAILED: u32 = 16u32;
pub const ASYNC_RECO_SETCACMODE_FAILED: u32 = 8u32;
pub const ASYNC_RECO_SETFACTOID_FAILED: u32 = 128u32;
pub const ASYNC_RECO_SETFLAGS_FAILED: u32 = 64u32;
pub const ASYNC_RECO_SETGUIDE_FAILED: u32 = 32u32;
pub const ASYNC_RECO_SETTEXTCONTEXT_FAILED: u32 = 256u32;
pub const ASYNC_RECO_SETWORDLIST_FAILED: u32 = 512u32;
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn AddStroke<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const ::win32_graphics::Gdi::XFORM) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddStroke(hrc: HRECOCONTEXT, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const ::win32_graphics::Gdi::XFORM) -> ::windows_core::HRESULT;
        }
        AddStroke(hrc.into_param().abi(), ::core::mem::transmute(ppacketdesc), ::core::mem::transmute(cbpacket), ::core::mem::transmute(ppacket), ::core::mem::transmute(pxform)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AddWordsToWordList<'a, Param0: ::windows_core::IntoParam<'a, HRECOWORDLIST>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwl: Param0, pwcwords: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddWordsToWordList(hwl: HRECOWORDLIST, pwcwords: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        AddWordsToWordList(hwl.into_param().abi(), pwcwords.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AdviseInkChange<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hrc: Param0, bnewstroke: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdviseInkChange(hrc: HRECOCONTEXT, bnewstroke: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        AdviseInkChange(hrc.into_param().abi(), bnewstroke.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppearanceConstants(pub i32);
pub const rtfFlat: AppearanceConstants = AppearanceConstants(0i32);
pub const rtfThreeD: AppearanceConstants = AppearanceConstants(1i32);
impl ::core::marker::Copy for AppearanceConstants {}
impl ::core::clone::Clone for AppearanceConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppearanceConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppearanceConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppearanceConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppearanceConstants").field(&self.0).finish()
    }
}
pub const BEST_COMPLETE: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BorderStyleConstants(pub i32);
pub const rtfNoBorder: BorderStyleConstants = BorderStyleConstants(0i32);
pub const rtfFixedSingle: BorderStyleConstants = BorderStyleConstants(1i32);
impl ::core::marker::Copy for BorderStyleConstants {}
impl ::core::clone::Clone for BorderStyleConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BorderStyleConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BorderStyleConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for BorderStyleConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BorderStyleConstants").field(&self.0).finish()
    }
}
pub const CAC_FULL: u32 = 0u32;
pub const CAC_PREFIX: u32 = 1u32;
pub const CAC_RANDOM: u32 = 2u32;
#[repr(C)]
pub struct CHARACTER_RANGE {
    pub wcLow: u16,
    pub cChars: u16,
}
impl ::core::marker::Copy for CHARACTER_RANGE {}
impl ::core::clone::Clone for CHARACTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHARACTER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARACTER_RANGE").field("wcLow", &self.wcLow).field("cChars", &self.cChars).finish()
    }
}
unsafe impl ::windows_core::Abi for CHARACTER_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHARACTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARACTER_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHARACTER_RANGE {}
impl ::core::default::Default for CHARACTER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONFIDENCE_LEVEL(pub i32);
pub const CFL_STRONG: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(0i32);
pub const CFL_INTERMEDIATE: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(1i32);
pub const CFL_POOR: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(2i32);
impl ::core::marker::Copy for CONFIDENCE_LEVEL {}
impl ::core::clone::Clone for CONFIDENCE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONFIDENCE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CONFIDENCE_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONFIDENCE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONFIDENCE_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CorrectionMode(pub i32);
pub const CorrectionMode_NotVisible: CorrectionMode = CorrectionMode(0i32);
pub const CorrectionMode_PreInsertion: CorrectionMode = CorrectionMode(1i32);
pub const CorrectionMode_PostInsertionCollapsed: CorrectionMode = CorrectionMode(2i32);
pub const CorrectionMode_PostInsertionExpanded: CorrectionMode = CorrectionMode(3i32);
impl ::core::marker::Copy for CorrectionMode {}
impl ::core::clone::Clone for CorrectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorrectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CorrectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CorrectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorrectionMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CorrectionPosition(pub i32);
pub const CorrectionPosition_Auto: CorrectionPosition = CorrectionPosition(0i32);
pub const CorrectionPosition_Bottom: CorrectionPosition = CorrectionPosition(1i32);
pub const CorrectionPosition_Top: CorrectionPosition = CorrectionPosition(2i32);
impl ::core::marker::Copy for CorrectionPosition {}
impl ::core::clone::Clone for CorrectionPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorrectionPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CorrectionPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for CorrectionPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorrectionPosition").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn CreateContext<'a, Param0: ::windows_core::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, phrc: *mut HRECOCONTEXT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateContext(hrec: HRECOGNIZER, phrc: *mut HRECOCONTEXT) -> ::windows_core::HRESULT;
        }
        CreateContext(hrec.into_param().abi(), ::core::mem::transmute(phrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateRecognizer(pclsid: *mut ::windows_core::GUID, phrec: *mut HRECOGNIZER) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRecognizer(pclsid: *mut ::windows_core::GUID, phrec: *mut HRECOGNIZER) -> ::windows_core::HRESULT;
        }
        CreateRecognizer(::core::mem::transmute(pclsid), ::core::mem::transmute(phrec)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_Ink(pub i32);
pub const DISPID_IStrokes: DISPID_Ink = DISPID_Ink(1i32);
pub const DISPID_IExtendedProperties: DISPID_Ink = DISPID_Ink(2i32);
pub const DISPID_IGetBoundingBox: DISPID_Ink = DISPID_Ink(3i32);
pub const DISPID_IDeleteStrokes: DISPID_Ink = DISPID_Ink(4i32);
pub const DISPID_IDeleteStroke: DISPID_Ink = DISPID_Ink(5i32);
pub const DISPID_IExtractStrokes: DISPID_Ink = DISPID_Ink(6i32);
pub const DISPID_IExtractWithRectangle: DISPID_Ink = DISPID_Ink(7i32);
pub const DISPID_IDirty: DISPID_Ink = DISPID_Ink(8i32);
pub const DISPID_ICustomStrokes: DISPID_Ink = DISPID_Ink(9i32);
pub const DISPID_IClone: DISPID_Ink = DISPID_Ink(10i32);
pub const DISPID_IHitTestCircle: DISPID_Ink = DISPID_Ink(11i32);
pub const DISPID_IHitTestWithRectangle: DISPID_Ink = DISPID_Ink(12i32);
pub const DISPID_IHitTestWithLasso: DISPID_Ink = DISPID_Ink(13i32);
pub const DISPID_INearestPoint: DISPID_Ink = DISPID_Ink(14i32);
pub const DISPID_ICreateStrokes: DISPID_Ink = DISPID_Ink(15i32);
pub const DISPID_ICreateStroke: DISPID_Ink = DISPID_Ink(16i32);
pub const DISPID_IAddStrokesAtRectangle: DISPID_Ink = DISPID_Ink(17i32);
pub const DISPID_IClip: DISPID_Ink = DISPID_Ink(18i32);
pub const DISPID_ISave: DISPID_Ink = DISPID_Ink(19i32);
pub const DISPID_ILoad: DISPID_Ink = DISPID_Ink(20i32);
pub const DISPID_ICreateStrokeFromPoints: DISPID_Ink = DISPID_Ink(21i32);
pub const DISPID_IClipboardCopyWithRectangle: DISPID_Ink = DISPID_Ink(22i32);
pub const DISPID_IClipboardCopy: DISPID_Ink = DISPID_Ink(23i32);
pub const DISPID_ICanPaste: DISPID_Ink = DISPID_Ink(24i32);
pub const DISPID_IClipboardPaste: DISPID_Ink = DISPID_Ink(25i32);
impl ::core::marker::Copy for DISPID_Ink {}
impl ::core::clone::Clone for DISPID_Ink {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_Ink {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_Ink {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_Ink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_Ink").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCollector(pub i32);
pub const DISPID_ICEnabled: DISPID_InkCollector = DISPID_InkCollector(1i32);
pub const DISPID_ICHwnd: DISPID_InkCollector = DISPID_InkCollector(2i32);
pub const DISPID_ICPaint: DISPID_InkCollector = DISPID_InkCollector(3i32);
pub const DISPID_ICText: DISPID_InkCollector = DISPID_InkCollector(4i32);
pub const DISPID_ICDefaultDrawingAttributes: DISPID_InkCollector = DISPID_InkCollector(5i32);
pub const DISPID_ICRenderer: DISPID_InkCollector = DISPID_InkCollector(6i32);
pub const DISPID_ICInk: DISPID_InkCollector = DISPID_InkCollector(7i32);
pub const DISPID_ICAutoRedraw: DISPID_InkCollector = DISPID_InkCollector(8i32);
pub const DISPID_ICCollectingInk: DISPID_InkCollector = DISPID_InkCollector(9i32);
pub const DISPID_ICSetEventInterest: DISPID_InkCollector = DISPID_InkCollector(10i32);
pub const DISPID_ICGetEventInterest: DISPID_InkCollector = DISPID_InkCollector(11i32);
pub const DISPID_IOEditingMode: DISPID_InkCollector = DISPID_InkCollector(12i32);
pub const DISPID_IOSelection: DISPID_InkCollector = DISPID_InkCollector(13i32);
pub const DISPID_IOAttachMode: DISPID_InkCollector = DISPID_InkCollector(14i32);
pub const DISPID_IOHitTestSelection: DISPID_InkCollector = DISPID_InkCollector(15i32);
pub const DISPID_IODraw: DISPID_InkCollector = DISPID_InkCollector(16i32);
pub const DISPID_IPPicture: DISPID_InkCollector = DISPID_InkCollector(17i32);
pub const DISPID_IPSizeMode: DISPID_InkCollector = DISPID_InkCollector(18i32);
pub const DISPID_IPBackColor: DISPID_InkCollector = DISPID_InkCollector(19i32);
pub const DISPID_ICCursors: DISPID_InkCollector = DISPID_InkCollector(20i32);
pub const DISPID_ICMarginX: DISPID_InkCollector = DISPID_InkCollector(21i32);
pub const DISPID_ICMarginY: DISPID_InkCollector = DISPID_InkCollector(22i32);
pub const DISPID_ICSetWindowInputRectangle: DISPID_InkCollector = DISPID_InkCollector(23i32);
pub const DISPID_ICGetWindowInputRectangle: DISPID_InkCollector = DISPID_InkCollector(24i32);
pub const DISPID_ICTablet: DISPID_InkCollector = DISPID_InkCollector(25i32);
pub const DISPID_ICSetAllTabletsMode: DISPID_InkCollector = DISPID_InkCollector(26i32);
pub const DISPID_ICSetSingleTabletIntegratedMode: DISPID_InkCollector = DISPID_InkCollector(27i32);
pub const DISPID_ICCollectionMode: DISPID_InkCollector = DISPID_InkCollector(28i32);
pub const DISPID_ICSetGestureStatus: DISPID_InkCollector = DISPID_InkCollector(29i32);
pub const DISPID_ICGetGestureStatus: DISPID_InkCollector = DISPID_InkCollector(30i32);
pub const DISPID_ICDynamicRendering: DISPID_InkCollector = DISPID_InkCollector(31i32);
pub const DISPID_ICDesiredPacketDescription: DISPID_InkCollector = DISPID_InkCollector(32i32);
pub const DISPID_IOEraserMode: DISPID_InkCollector = DISPID_InkCollector(33i32);
pub const DISPID_IOEraserWidth: DISPID_InkCollector = DISPID_InkCollector(34i32);
pub const DISPID_ICMouseIcon: DISPID_InkCollector = DISPID_InkCollector(35i32);
pub const DISPID_ICMousePointer: DISPID_InkCollector = DISPID_InkCollector(36i32);
pub const DISPID_IPInkEnabled: DISPID_InkCollector = DISPID_InkCollector(37i32);
pub const DISPID_ICSupportHighContrastInk: DISPID_InkCollector = DISPID_InkCollector(38i32);
pub const DISPID_IOSupportHighContrastSelectionUI: DISPID_InkCollector = DISPID_InkCollector(39i32);
impl ::core::marker::Copy for DISPID_InkCollector {}
impl ::core::clone::Clone for DISPID_InkCollector {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCollector {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCollector {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCollector").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCollectorEvent(pub i32);
pub const DISPID_ICEStroke: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(1i32);
pub const DISPID_ICECursorDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(2i32);
pub const DISPID_ICENewPackets: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(3i32);
pub const DISPID_ICENewInAirPackets: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(4i32);
pub const DISPID_ICECursorButtonDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(5i32);
pub const DISPID_ICECursorButtonUp: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(6i32);
pub const DISPID_ICECursorInRange: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(7i32);
pub const DISPID_ICECursorOutOfRange: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(8i32);
pub const DISPID_ICESystemGesture: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(9i32);
pub const DISPID_ICEGesture: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(10i32);
pub const DISPID_ICETabletAdded: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(11i32);
pub const DISPID_ICETabletRemoved: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(12i32);
pub const DISPID_IOEPainting: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(13i32);
pub const DISPID_IOEPainted: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(14i32);
pub const DISPID_IOESelectionChanging: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(15i32);
pub const DISPID_IOESelectionChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(16i32);
pub const DISPID_IOESelectionMoving: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(17i32);
pub const DISPID_IOESelectionMoved: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(18i32);
pub const DISPID_IOESelectionResizing: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(19i32);
pub const DISPID_IOESelectionResized: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(20i32);
pub const DISPID_IOEStrokesDeleting: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(21i32);
pub const DISPID_IOEStrokesDeleted: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(22i32);
pub const DISPID_IPEChangeUICues: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(23i32);
pub const DISPID_IPEClick: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(24i32);
pub const DISPID_IPEDblClick: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(25i32);
pub const DISPID_IPEInvalidated: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(26i32);
pub const DISPID_IPEMouseDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(27i32);
pub const DISPID_IPEMouseEnter: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(28i32);
pub const DISPID_IPEMouseHover: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(29i32);
pub const DISPID_IPEMouseLeave: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(30i32);
pub const DISPID_IPEMouseMove: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(31i32);
pub const DISPID_IPEMouseUp: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(32i32);
pub const DISPID_IPEMouseWheel: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(33i32);
pub const DISPID_IPESizeModeChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(34i32);
pub const DISPID_IPEStyleChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(35i32);
pub const DISPID_IPESystemColorsChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(36i32);
pub const DISPID_IPEKeyDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(37i32);
pub const DISPID_IPEKeyPress: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(38i32);
pub const DISPID_IPEKeyUp: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(39i32);
pub const DISPID_IPEResize: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(40i32);
pub const DISPID_IPESizeChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(41i32);
impl ::core::marker::Copy for DISPID_InkCollectorEvent {}
impl ::core::clone::Clone for DISPID_InkCollectorEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCollectorEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCollectorEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCollectorEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCollectorEvent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCursor(pub i32);
pub const DISPID_ICsrName: DISPID_InkCursor = DISPID_InkCursor(0i32);
pub const DISPID_ICsrId: DISPID_InkCursor = DISPID_InkCursor(1i32);
pub const DISPID_ICsrDrawingAttributes: DISPID_InkCursor = DISPID_InkCursor(2i32);
pub const DISPID_ICsrButtons: DISPID_InkCursor = DISPID_InkCursor(3i32);
pub const DISPID_ICsrInverted: DISPID_InkCursor = DISPID_InkCursor(4i32);
pub const DISPID_ICsrTablet: DISPID_InkCursor = DISPID_InkCursor(5i32);
impl ::core::marker::Copy for DISPID_InkCursor {}
impl ::core::clone::Clone for DISPID_InkCursor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCursor {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCursor {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursor").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCursorButton(pub i32);
pub const DISPID_ICBName: DISPID_InkCursorButton = DISPID_InkCursorButton(0i32);
pub const DISPID_ICBId: DISPID_InkCursorButton = DISPID_InkCursorButton(1i32);
pub const DISPID_ICBState: DISPID_InkCursorButton = DISPID_InkCursorButton(2i32);
impl ::core::marker::Copy for DISPID_InkCursorButton {}
impl ::core::clone::Clone for DISPID_InkCursorButton {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCursorButton {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCursorButton {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCursorButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursorButton").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCursorButtons(pub i32);
pub const DISPID_ICBs_NewEnum: DISPID_InkCursorButtons = DISPID_InkCursorButtons(-4i32);
pub const DISPID_ICBsItem: DISPID_InkCursorButtons = DISPID_InkCursorButtons(0i32);
pub const DISPID_ICBsCount: DISPID_InkCursorButtons = DISPID_InkCursorButtons(1i32);
impl ::core::marker::Copy for DISPID_InkCursorButtons {}
impl ::core::clone::Clone for DISPID_InkCursorButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCursorButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCursorButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCursorButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursorButtons").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCursors(pub i32);
pub const DISPID_ICs_NewEnum: DISPID_InkCursors = DISPID_InkCursors(-4i32);
pub const DISPID_ICsItem: DISPID_InkCursors = DISPID_InkCursors(0i32);
pub const DISPID_ICsCount: DISPID_InkCursors = DISPID_InkCursors(1i32);
impl ::core::marker::Copy for DISPID_InkCursors {}
impl ::core::clone::Clone for DISPID_InkCursors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCursors {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCursors {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCursors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCursors").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkCustomStrokes(pub i32);
pub const DISPID_ICSs_NewEnum: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(-4i32);
pub const DISPID_ICSsItem: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(0i32);
pub const DISPID_ICSsCount: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(1i32);
pub const DISPID_ICSsAdd: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(2i32);
pub const DISPID_ICSsRemove: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(3i32);
pub const DISPID_ICSsClear: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(4i32);
impl ::core::marker::Copy for DISPID_InkCustomStrokes {}
impl ::core::clone::Clone for DISPID_InkCustomStrokes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkCustomStrokes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkCustomStrokes {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkCustomStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkCustomStrokes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkDivider(pub i32);
pub const DISPID_IInkDivider_Strokes: DISPID_InkDivider = DISPID_InkDivider(1i32);
pub const DISPID_IInkDivider_RecognizerContext: DISPID_InkDivider = DISPID_InkDivider(2i32);
pub const DISPID_IInkDivider_LineHeight: DISPID_InkDivider = DISPID_InkDivider(3i32);
pub const DISPID_IInkDivider_Divide: DISPID_InkDivider = DISPID_InkDivider(4i32);
impl ::core::marker::Copy for DISPID_InkDivider {}
impl ::core::clone::Clone for DISPID_InkDivider {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkDivider {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkDivider {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkDivider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivider").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkDivisionResult(pub i32);
pub const DISPID_IInkDivisionResult_Strokes: DISPID_InkDivisionResult = DISPID_InkDivisionResult(1i32);
pub const DISPID_IInkDivisionResult_ResultByType: DISPID_InkDivisionResult = DISPID_InkDivisionResult(2i32);
impl ::core::marker::Copy for DISPID_InkDivisionResult {}
impl ::core::clone::Clone for DISPID_InkDivisionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkDivisionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkDivisionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkDivisionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivisionResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkDivisionUnit(pub i32);
pub const DISPID_IInkDivisionUnit_Strokes: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(1i32);
pub const DISPID_IInkDivisionUnit_DivisionType: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(2i32);
pub const DISPID_IInkDivisionUnit_RecognizedString: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(3i32);
pub const DISPID_IInkDivisionUnit_RotationTransform: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(4i32);
impl ::core::marker::Copy for DISPID_InkDivisionUnit {}
impl ::core::clone::Clone for DISPID_InkDivisionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkDivisionUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkDivisionUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkDivisionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivisionUnit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkDivisionUnits(pub i32);
pub const DISPID_IInkDivisionUnits_NewEnum: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(-4i32);
pub const DISPID_IInkDivisionUnits_Item: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(0i32);
pub const DISPID_IInkDivisionUnits_Count: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(1i32);
impl ::core::marker::Copy for DISPID_InkDivisionUnits {}
impl ::core::clone::Clone for DISPID_InkDivisionUnits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkDivisionUnits {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkDivisionUnits {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkDivisionUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDivisionUnits").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkDrawingAttributes(pub i32);
pub const DISPID_DAHeight: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(1i32);
pub const DISPID_DAColor: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(2i32);
pub const DISPID_DAWidth: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(3i32);
pub const DISPID_DAFitToCurve: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(4i32);
pub const DISPID_DAIgnorePressure: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(5i32);
pub const DISPID_DAAntiAliased: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(6i32);
pub const DISPID_DATransparency: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(7i32);
pub const DISPID_DARasterOperation: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(8i32);
pub const DISPID_DAPenTip: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(9i32);
pub const DISPID_DAClone: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(10i32);
pub const DISPID_DAExtendedProperties: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(11i32);
impl ::core::marker::Copy for DISPID_InkDrawingAttributes {}
impl ::core::clone::Clone for DISPID_InkDrawingAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkDrawingAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkDrawingAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkDrawingAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkDrawingAttributes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkEdit(pub i32);
pub const DISPID_Text: DISPID_InkEdit = DISPID_InkEdit(0i32);
pub const DISPID_TextRTF: DISPID_InkEdit = DISPID_InkEdit(1i32);
pub const DISPID_Hwnd: DISPID_InkEdit = DISPID_InkEdit(2i32);
pub const DISPID_DisableNoScroll: DISPID_InkEdit = DISPID_InkEdit(3i32);
pub const DISPID_Locked: DISPID_InkEdit = DISPID_InkEdit(4i32);
pub const DISPID_Enabled: DISPID_InkEdit = DISPID_InkEdit(5i32);
pub const DISPID_MaxLength: DISPID_InkEdit = DISPID_InkEdit(6i32);
pub const DISPID_MultiLine: DISPID_InkEdit = DISPID_InkEdit(7i32);
pub const DISPID_ScrollBars: DISPID_InkEdit = DISPID_InkEdit(8i32);
pub const DISPID_RTSelStart: DISPID_InkEdit = DISPID_InkEdit(9i32);
pub const DISPID_RTSelLength: DISPID_InkEdit = DISPID_InkEdit(10i32);
pub const DISPID_RTSelText: DISPID_InkEdit = DISPID_InkEdit(11i32);
pub const DISPID_SelAlignment: DISPID_InkEdit = DISPID_InkEdit(12i32);
pub const DISPID_SelBold: DISPID_InkEdit = DISPID_InkEdit(13i32);
pub const DISPID_SelCharOffset: DISPID_InkEdit = DISPID_InkEdit(14i32);
pub const DISPID_SelColor: DISPID_InkEdit = DISPID_InkEdit(15i32);
pub const DISPID_SelFontName: DISPID_InkEdit = DISPID_InkEdit(16i32);
pub const DISPID_SelFontSize: DISPID_InkEdit = DISPID_InkEdit(17i32);
pub const DISPID_SelItalic: DISPID_InkEdit = DISPID_InkEdit(18i32);
pub const DISPID_SelRTF: DISPID_InkEdit = DISPID_InkEdit(19i32);
pub const DISPID_SelUnderline: DISPID_InkEdit = DISPID_InkEdit(20i32);
pub const DISPID_DragIcon: DISPID_InkEdit = DISPID_InkEdit(21i32);
pub const DISPID_Status: DISPID_InkEdit = DISPID_InkEdit(22i32);
pub const DISPID_UseMouseForInput: DISPID_InkEdit = DISPID_InkEdit(23i32);
pub const DISPID_InkMode: DISPID_InkEdit = DISPID_InkEdit(24i32);
pub const DISPID_InkInsertMode: DISPID_InkEdit = DISPID_InkEdit(25i32);
pub const DISPID_RecoTimeout: DISPID_InkEdit = DISPID_InkEdit(26i32);
pub const DISPID_DrawAttr: DISPID_InkEdit = DISPID_InkEdit(27i32);
pub const DISPID_Recognizer: DISPID_InkEdit = DISPID_InkEdit(28i32);
pub const DISPID_Factoid: DISPID_InkEdit = DISPID_InkEdit(29i32);
pub const DISPID_SelInk: DISPID_InkEdit = DISPID_InkEdit(30i32);
pub const DISPID_SelInksDisplayMode: DISPID_InkEdit = DISPID_InkEdit(31i32);
pub const DISPID_Recognize: DISPID_InkEdit = DISPID_InkEdit(32i32);
pub const DISPID_GetGestStatus: DISPID_InkEdit = DISPID_InkEdit(33i32);
pub const DISPID_SetGestStatus: DISPID_InkEdit = DISPID_InkEdit(34i32);
pub const DISPID_Refresh: DISPID_InkEdit = DISPID_InkEdit(35i32);
impl ::core::marker::Copy for DISPID_InkEdit {}
impl ::core::clone::Clone for DISPID_InkEdit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkEdit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkEdit {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkEdit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkEdit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkEditEvents(pub i32);
pub const DISPID_IeeChange: DISPID_InkEditEvents = DISPID_InkEditEvents(1i32);
pub const DISPID_IeeSelChange: DISPID_InkEditEvents = DISPID_InkEditEvents(2i32);
pub const DISPID_IeeKeyDown: DISPID_InkEditEvents = DISPID_InkEditEvents(3i32);
pub const DISPID_IeeKeyUp: DISPID_InkEditEvents = DISPID_InkEditEvents(4i32);
pub const DISPID_IeeMouseUp: DISPID_InkEditEvents = DISPID_InkEditEvents(5i32);
pub const DISPID_IeeMouseDown: DISPID_InkEditEvents = DISPID_InkEditEvents(6i32);
pub const DISPID_IeeKeyPress: DISPID_InkEditEvents = DISPID_InkEditEvents(7i32);
pub const DISPID_IeeDblClick: DISPID_InkEditEvents = DISPID_InkEditEvents(8i32);
pub const DISPID_IeeClick: DISPID_InkEditEvents = DISPID_InkEditEvents(9i32);
pub const DISPID_IeeMouseMove: DISPID_InkEditEvents = DISPID_InkEditEvents(10i32);
pub const DISPID_IeeCursorDown: DISPID_InkEditEvents = DISPID_InkEditEvents(21i32);
pub const DISPID_IeeStroke: DISPID_InkEditEvents = DISPID_InkEditEvents(22i32);
pub const DISPID_IeeGesture: DISPID_InkEditEvents = DISPID_InkEditEvents(23i32);
pub const DISPID_IeeRecognitionResult: DISPID_InkEditEvents = DISPID_InkEditEvents(24i32);
impl ::core::marker::Copy for DISPID_InkEditEvents {}
impl ::core::clone::Clone for DISPID_InkEditEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkEditEvents {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkEditEvents {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkEditEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkEditEvents").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkEvent(pub i32);
pub const DISPID_IEInkAdded: DISPID_InkEvent = DISPID_InkEvent(1i32);
pub const DISPID_IEInkDeleted: DISPID_InkEvent = DISPID_InkEvent(2i32);
impl ::core::marker::Copy for DISPID_InkEvent {}
impl ::core::clone::Clone for DISPID_InkEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkEvent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkExtendedProperties(pub i32);
pub const DISPID_IEPs_NewEnum: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(-4i32);
pub const DISPID_IEPsItem: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(0i32);
pub const DISPID_IEPsCount: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(1i32);
pub const DISPID_IEPsAdd: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(2i32);
pub const DISPID_IEPsRemove: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(3i32);
pub const DISPID_IEPsClear: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(4i32);
pub const DISPID_IEPsDoesPropertyExist: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(5i32);
impl ::core::marker::Copy for DISPID_InkExtendedProperties {}
impl ::core::clone::Clone for DISPID_InkExtendedProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkExtendedProperties {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkExtendedProperties {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkExtendedProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkExtendedProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkExtendedProperty(pub i32);
pub const DISPID_IEPGuid: DISPID_InkExtendedProperty = DISPID_InkExtendedProperty(1i32);
pub const DISPID_IEPData: DISPID_InkExtendedProperty = DISPID_InkExtendedProperty(2i32);
impl ::core::marker::Copy for DISPID_InkExtendedProperty {}
impl ::core::clone::Clone for DISPID_InkExtendedProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkExtendedProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkExtendedProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkExtendedProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkGesture(pub i32);
pub const DISPID_IGId: DISPID_InkGesture = DISPID_InkGesture(0i32);
pub const DISPID_IGGetHotPoint: DISPID_InkGesture = DISPID_InkGesture(1i32);
pub const DISPID_IGConfidence: DISPID_InkGesture = DISPID_InkGesture(2i32);
impl ::core::marker::Copy for DISPID_InkGesture {}
impl ::core::clone::Clone for DISPID_InkGesture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkGesture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkGesture {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkGesture").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecoAlternate(pub i32);
pub const DISPID_InkRecoAlternate_String: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(1i32);
pub const DISPID_InkRecoAlternate_LineNumber: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(2i32);
pub const DISPID_InkRecoAlternate_Baseline: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(3i32);
pub const DISPID_InkRecoAlternate_Midline: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(4i32);
pub const DISPID_InkRecoAlternate_Ascender: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(5i32);
pub const DISPID_InkRecoAlternate_Descender: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(6i32);
pub const DISPID_InkRecoAlternate_Confidence: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(7i32);
pub const DISPID_InkRecoAlternate_Strokes: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(8i32);
pub const DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(9i32);
pub const DISPID_InkRecoAlternate_GetStrokesFromTextRange: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(10i32);
pub const DISPID_InkRecoAlternate_GetTextRangeFromStrokes: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(11i32);
pub const DISPID_InkRecoAlternate_GetPropertyValue: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(12i32);
pub const DISPID_InkRecoAlternate_LineAlternates: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(13i32);
pub const DISPID_InkRecoAlternate_ConfidenceAlternates: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(14i32);
pub const DISPID_InkRecoAlternate_AlternatesWithConstantPropertyValues: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(15i32);
impl ::core::marker::Copy for DISPID_InkRecoAlternate {}
impl ::core::clone::Clone for DISPID_InkRecoAlternate {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecoAlternate {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecoAlternate {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecoAlternate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecoAlternate").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecoContext(pub i32);
pub const DISPID_IRecoCtx_Strokes: DISPID_InkRecoContext = DISPID_InkRecoContext(1i32);
pub const DISPID_IRecoCtx_CharacterAutoCompletionMode: DISPID_InkRecoContext = DISPID_InkRecoContext(2i32);
pub const DISPID_IRecoCtx_Factoid: DISPID_InkRecoContext = DISPID_InkRecoContext(3i32);
pub const DISPID_IRecoCtx_WordList: DISPID_InkRecoContext = DISPID_InkRecoContext(4i32);
pub const DISPID_IRecoCtx_Recognizer: DISPID_InkRecoContext = DISPID_InkRecoContext(5i32);
pub const DISPID_IRecoCtx_Guide: DISPID_InkRecoContext = DISPID_InkRecoContext(6i32);
pub const DISPID_IRecoCtx_Flags: DISPID_InkRecoContext = DISPID_InkRecoContext(7i32);
pub const DISPID_IRecoCtx_PrefixText: DISPID_InkRecoContext = DISPID_InkRecoContext(8i32);
pub const DISPID_IRecoCtx_SuffixText: DISPID_InkRecoContext = DISPID_InkRecoContext(9i32);
pub const DISPID_IRecoCtx_StopRecognition: DISPID_InkRecoContext = DISPID_InkRecoContext(10i32);
pub const DISPID_IRecoCtx_Clone: DISPID_InkRecoContext = DISPID_InkRecoContext(11i32);
pub const DISPID_IRecoCtx_Recognize: DISPID_InkRecoContext = DISPID_InkRecoContext(12i32);
pub const DISPID_IRecoCtx_StopBackgroundRecognition: DISPID_InkRecoContext = DISPID_InkRecoContext(13i32);
pub const DISPID_IRecoCtx_EndInkInput: DISPID_InkRecoContext = DISPID_InkRecoContext(14i32);
pub const DISPID_IRecoCtx_BackgroundRecognize: DISPID_InkRecoContext = DISPID_InkRecoContext(15i32);
pub const DISPID_IRecoCtx_BackgroundRecognizeWithAlternates: DISPID_InkRecoContext = DISPID_InkRecoContext(16i32);
pub const DISPID_IRecoCtx_IsStringSupported: DISPID_InkRecoContext = DISPID_InkRecoContext(17i32);
impl ::core::marker::Copy for DISPID_InkRecoContext {}
impl ::core::clone::Clone for DISPID_InkRecoContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecoContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecoContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecoContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecoContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecoContext2(pub i32);
pub const DISPID_IRecoCtx2_EnabledUnicodeRanges: DISPID_InkRecoContext2 = DISPID_InkRecoContext2(0i32);
impl ::core::marker::Copy for DISPID_InkRecoContext2 {}
impl ::core::clone::Clone for DISPID_InkRecoContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecoContext2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecoContext2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecoContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecoContext2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognitionAlternates(pub i32);
pub const DISPID_InkRecognitionAlternates_NewEnum: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(-4i32);
pub const DISPID_InkRecognitionAlternates_Item: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(0i32);
pub const DISPID_InkRecognitionAlternates_Count: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(1i32);
pub const DISPID_InkRecognitionAlternates_Strokes: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(2i32);
impl ::core::marker::Copy for DISPID_InkRecognitionAlternates {}
impl ::core::clone::Clone for DISPID_InkRecognitionAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognitionAlternates {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognitionAlternates {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognitionAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognitionAlternates").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognitionEvent(pub i32);
pub const DISPID_IRERecognitionWithAlternates: DISPID_InkRecognitionEvent = DISPID_InkRecognitionEvent(1i32);
pub const DISPID_IRERecognition: DISPID_InkRecognitionEvent = DISPID_InkRecognitionEvent(2i32);
impl ::core::marker::Copy for DISPID_InkRecognitionEvent {}
impl ::core::clone::Clone for DISPID_InkRecognitionEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognitionEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognitionEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognitionEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognitionEvent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognitionResult(pub i32);
pub const DISPID_InkRecognitionResult_TopString: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(1i32);
pub const DISPID_InkRecognitionResult_TopAlternate: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(2i32);
pub const DISPID_InkRecognitionResult_Strokes: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(3i32);
pub const DISPID_InkRecognitionResult_TopConfidence: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(4i32);
pub const DISPID_InkRecognitionResult_AlternatesFromSelection: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(5i32);
pub const DISPID_InkRecognitionResult_ModifyTopAlternate: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(6i32);
pub const DISPID_InkRecognitionResult_SetResultOnStrokes: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(7i32);
impl ::core::marker::Copy for DISPID_InkRecognitionResult {}
impl ::core::clone::Clone for DISPID_InkRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognitionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognitionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognitionResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognizer(pub i32);
pub const DISPID_RecoClsid: DISPID_InkRecognizer = DISPID_InkRecognizer(1i32);
pub const DISPID_RecoName: DISPID_InkRecognizer = DISPID_InkRecognizer(2i32);
pub const DISPID_RecoVendor: DISPID_InkRecognizer = DISPID_InkRecognizer(3i32);
pub const DISPID_RecoCapabilities: DISPID_InkRecognizer = DISPID_InkRecognizer(4i32);
pub const DISPID_RecoLanguageID: DISPID_InkRecognizer = DISPID_InkRecognizer(5i32);
pub const DISPID_RecoPreferredPacketDescription: DISPID_InkRecognizer = DISPID_InkRecognizer(6i32);
pub const DISPID_RecoCreateRecognizerContext: DISPID_InkRecognizer = DISPID_InkRecognizer(7i32);
pub const DISPID_RecoSupportedProperties: DISPID_InkRecognizer = DISPID_InkRecognizer(8i32);
impl ::core::marker::Copy for DISPID_InkRecognizer {}
impl ::core::clone::Clone for DISPID_InkRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognizer {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognizer {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizer").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognizer2(pub i32);
pub const DISPID_RecoId: DISPID_InkRecognizer2 = DISPID_InkRecognizer2(0i32);
pub const DISPID_RecoUnicodeRanges: DISPID_InkRecognizer2 = DISPID_InkRecognizer2(1i32);
impl ::core::marker::Copy for DISPID_InkRecognizer2 {}
impl ::core::clone::Clone for DISPID_InkRecognizer2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognizer2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognizer2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognizer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizer2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognizerGuide(pub i32);
pub const DISPID_IRGWritingBox: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(1i32);
pub const DISPID_IRGDrawnBox: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(2i32);
pub const DISPID_IRGRows: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(3i32);
pub const DISPID_IRGColumns: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(4i32);
pub const DISPID_IRGMidline: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(5i32);
pub const DISPID_IRGGuideData: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(6i32);
impl ::core::marker::Copy for DISPID_InkRecognizerGuide {}
impl ::core::clone::Clone for DISPID_InkRecognizerGuide {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognizerGuide {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognizerGuide {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognizerGuide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizerGuide").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRecognizers(pub i32);
pub const DISPID_IRecos_NewEnum: DISPID_InkRecognizers = DISPID_InkRecognizers(-4i32);
pub const DISPID_IRecosItem: DISPID_InkRecognizers = DISPID_InkRecognizers(0i32);
pub const DISPID_IRecosCount: DISPID_InkRecognizers = DISPID_InkRecognizers(1i32);
pub const DISPID_IRecosGetDefaultRecognizer: DISPID_InkRecognizers = DISPID_InkRecognizers(2i32);
impl ::core::marker::Copy for DISPID_InkRecognizers {}
impl ::core::clone::Clone for DISPID_InkRecognizers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRecognizers {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRecognizers {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRecognizers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRecognizers").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRectangle(pub i32);
pub const DISPID_IRTop: DISPID_InkRectangle = DISPID_InkRectangle(1i32);
pub const DISPID_IRLeft: DISPID_InkRectangle = DISPID_InkRectangle(2i32);
pub const DISPID_IRBottom: DISPID_InkRectangle = DISPID_InkRectangle(3i32);
pub const DISPID_IRRight: DISPID_InkRectangle = DISPID_InkRectangle(4i32);
pub const DISPID_IRGetRectangle: DISPID_InkRectangle = DISPID_InkRectangle(5i32);
pub const DISPID_IRSetRectangle: DISPID_InkRectangle = DISPID_InkRectangle(6i32);
pub const DISPID_IRData: DISPID_InkRectangle = DISPID_InkRectangle(7i32);
impl ::core::marker::Copy for DISPID_InkRectangle {}
impl ::core::clone::Clone for DISPID_InkRectangle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRectangle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRectangle {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRectangle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkRenderer(pub i32);
pub const DISPID_IRGetViewTransform: DISPID_InkRenderer = DISPID_InkRenderer(1i32);
pub const DISPID_IRSetViewTransform: DISPID_InkRenderer = DISPID_InkRenderer(2i32);
pub const DISPID_IRGetObjectTransform: DISPID_InkRenderer = DISPID_InkRenderer(3i32);
pub const DISPID_IRSetObjectTransform: DISPID_InkRenderer = DISPID_InkRenderer(4i32);
pub const DISPID_IRDraw: DISPID_InkRenderer = DISPID_InkRenderer(5i32);
pub const DISPID_IRDrawStroke: DISPID_InkRenderer = DISPID_InkRenderer(6i32);
pub const DISPID_IRPixelToInkSpace: DISPID_InkRenderer = DISPID_InkRenderer(7i32);
pub const DISPID_IRInkSpaceToPixel: DISPID_InkRenderer = DISPID_InkRenderer(8i32);
pub const DISPID_IRPixelToInkSpaceFromPoints: DISPID_InkRenderer = DISPID_InkRenderer(9i32);
pub const DISPID_IRInkSpaceToPixelFromPoints: DISPID_InkRenderer = DISPID_InkRenderer(10i32);
pub const DISPID_IRMeasure: DISPID_InkRenderer = DISPID_InkRenderer(11i32);
pub const DISPID_IRMeasureStroke: DISPID_InkRenderer = DISPID_InkRenderer(12i32);
pub const DISPID_IRMove: DISPID_InkRenderer = DISPID_InkRenderer(13i32);
pub const DISPID_IRRotate: DISPID_InkRenderer = DISPID_InkRenderer(14i32);
pub const DISPID_IRScale: DISPID_InkRenderer = DISPID_InkRenderer(15i32);
impl ::core::marker::Copy for DISPID_InkRenderer {}
impl ::core::clone::Clone for DISPID_InkRenderer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkRenderer {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkRenderer {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkRenderer").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkStrokeDisp(pub i32);
pub const DISPID_ISDInkIndex: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(1i32);
pub const DISPID_ISDID: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(2i32);
pub const DISPID_ISDGetBoundingBox: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(3i32);
pub const DISPID_ISDDrawingAttributes: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(4i32);
pub const DISPID_ISDFindIntersections: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(5i32);
pub const DISPID_ISDGetRectangleIntersections: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(6i32);
pub const DISPID_ISDClip: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(7i32);
pub const DISPID_ISDHitTestCircle: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(8i32);
pub const DISPID_ISDNearestPoint: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(9i32);
pub const DISPID_ISDSplit: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(10i32);
pub const DISPID_ISDExtendedProperties: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(11i32);
pub const DISPID_ISDInk: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(12i32);
pub const DISPID_ISDBezierPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(13i32);
pub const DISPID_ISDPolylineCusps: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(14i32);
pub const DISPID_ISDBezierCusps: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(15i32);
pub const DISPID_ISDSelfIntersections: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(16i32);
pub const DISPID_ISDPacketCount: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(17i32);
pub const DISPID_ISDPacketSize: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(18i32);
pub const DISPID_ISDPacketDescription: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(19i32);
pub const DISPID_ISDDeleted: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(20i32);
pub const DISPID_ISDGetPacketDescriptionPropertyMetrics: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(21i32);
pub const DISPID_ISDGetPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(22i32);
pub const DISPID_ISDSetPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(23i32);
pub const DISPID_ISDGetPacketData: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(24i32);
pub const DISPID_ISDGetPacketValuesByProperty: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(25i32);
pub const DISPID_ISDSetPacketValuesByProperty: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(26i32);
pub const DISPID_ISDGetFlattenedBezierPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(27i32);
pub const DISPID_ISDScaleToRectangle: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(28i32);
pub const DISPID_ISDTransform: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(29i32);
pub const DISPID_ISDMove: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(30i32);
pub const DISPID_ISDRotate: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(31i32);
pub const DISPID_ISDShear: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(32i32);
pub const DISPID_ISDScale: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(33i32);
impl ::core::marker::Copy for DISPID_InkStrokeDisp {}
impl ::core::clone::Clone for DISPID_InkStrokeDisp {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkStrokeDisp {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkStrokeDisp {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkStrokeDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkStrokeDisp").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkStrokes(pub i32);
pub const DISPID_ISs_NewEnum: DISPID_InkStrokes = DISPID_InkStrokes(-4i32);
pub const DISPID_ISsItem: DISPID_InkStrokes = DISPID_InkStrokes(0i32);
pub const DISPID_ISsCount: DISPID_InkStrokes = DISPID_InkStrokes(1i32);
pub const DISPID_ISsValid: DISPID_InkStrokes = DISPID_InkStrokes(2i32);
pub const DISPID_ISsInk: DISPID_InkStrokes = DISPID_InkStrokes(3i32);
pub const DISPID_ISsAdd: DISPID_InkStrokes = DISPID_InkStrokes(4i32);
pub const DISPID_ISsAddStrokes: DISPID_InkStrokes = DISPID_InkStrokes(5i32);
pub const DISPID_ISsRemove: DISPID_InkStrokes = DISPID_InkStrokes(6i32);
pub const DISPID_ISsRemoveStrokes: DISPID_InkStrokes = DISPID_InkStrokes(7i32);
pub const DISPID_ISsToString: DISPID_InkStrokes = DISPID_InkStrokes(8i32);
pub const DISPID_ISsModifyDrawingAttributes: DISPID_InkStrokes = DISPID_InkStrokes(9i32);
pub const DISPID_ISsGetBoundingBox: DISPID_InkStrokes = DISPID_InkStrokes(10i32);
pub const DISPID_ISsScaleToRectangle: DISPID_InkStrokes = DISPID_InkStrokes(11i32);
pub const DISPID_ISsTransform: DISPID_InkStrokes = DISPID_InkStrokes(12i32);
pub const DISPID_ISsMove: DISPID_InkStrokes = DISPID_InkStrokes(13i32);
pub const DISPID_ISsRotate: DISPID_InkStrokes = DISPID_InkStrokes(14i32);
pub const DISPID_ISsShear: DISPID_InkStrokes = DISPID_InkStrokes(15i32);
pub const DISPID_ISsScale: DISPID_InkStrokes = DISPID_InkStrokes(16i32);
pub const DISPID_ISsClip: DISPID_InkStrokes = DISPID_InkStrokes(17i32);
pub const DISPID_ISsRecognitionResult: DISPID_InkStrokes = DISPID_InkStrokes(18i32);
pub const DISPID_ISsRemoveRecognitionResult: DISPID_InkStrokes = DISPID_InkStrokes(19i32);
impl ::core::marker::Copy for DISPID_InkStrokes {}
impl ::core::clone::Clone for DISPID_InkStrokes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkStrokes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkStrokes {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkStrokes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkTablet(pub i32);
pub const DISPID_ITName: DISPID_InkTablet = DISPID_InkTablet(0i32);
pub const DISPID_ITPlugAndPlayId: DISPID_InkTablet = DISPID_InkTablet(1i32);
pub const DISPID_ITPropertyMetrics: DISPID_InkTablet = DISPID_InkTablet(2i32);
pub const DISPID_ITIsPacketPropertySupported: DISPID_InkTablet = DISPID_InkTablet(3i32);
pub const DISPID_ITMaximumInputRectangle: DISPID_InkTablet = DISPID_InkTablet(4i32);
pub const DISPID_ITHardwareCapabilities: DISPID_InkTablet = DISPID_InkTablet(5i32);
impl ::core::marker::Copy for DISPID_InkTablet {}
impl ::core::clone::Clone for DISPID_InkTablet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkTablet {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkTablet {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkTablet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablet").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkTablet2(pub i32);
pub const DISPID_IT2DeviceKind: DISPID_InkTablet2 = DISPID_InkTablet2(0i32);
impl ::core::marker::Copy for DISPID_InkTablet2 {}
impl ::core::clone::Clone for DISPID_InkTablet2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkTablet2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkTablet2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkTablet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablet2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkTablet3(pub i32);
pub const DISPID_IT3IsMultiTouch: DISPID_InkTablet3 = DISPID_InkTablet3(0i32);
pub const DISPID_IT3MaximumCursors: DISPID_InkTablet3 = DISPID_InkTablet3(1i32);
impl ::core::marker::Copy for DISPID_InkTablet3 {}
impl ::core::clone::Clone for DISPID_InkTablet3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkTablet3 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkTablet3 {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkTablet3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablet3").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkTablets(pub i32);
pub const DISPID_ITs_NewEnum: DISPID_InkTablets = DISPID_InkTablets(-4i32);
pub const DISPID_ITsItem: DISPID_InkTablets = DISPID_InkTablets(0i32);
pub const DISPID_ITsDefaultTablet: DISPID_InkTablets = DISPID_InkTablets(1i32);
pub const DISPID_ITsCount: DISPID_InkTablets = DISPID_InkTablets(2i32);
pub const DISPID_ITsIsPacketPropertySupported: DISPID_InkTablets = DISPID_InkTablets(3i32);
impl ::core::marker::Copy for DISPID_InkTablets {}
impl ::core::clone::Clone for DISPID_InkTablets {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkTablets {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkTablets {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkTablets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTablets").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkTransform(pub i32);
pub const DISPID_ITReset: DISPID_InkTransform = DISPID_InkTransform(1i32);
pub const DISPID_ITTranslate: DISPID_InkTransform = DISPID_InkTransform(2i32);
pub const DISPID_ITRotate: DISPID_InkTransform = DISPID_InkTransform(3i32);
pub const DISPID_ITReflect: DISPID_InkTransform = DISPID_InkTransform(4i32);
pub const DISPID_ITShear: DISPID_InkTransform = DISPID_InkTransform(5i32);
pub const DISPID_ITScale: DISPID_InkTransform = DISPID_InkTransform(6i32);
pub const DISPID_ITeM11: DISPID_InkTransform = DISPID_InkTransform(7i32);
pub const DISPID_ITeM12: DISPID_InkTransform = DISPID_InkTransform(8i32);
pub const DISPID_ITeM21: DISPID_InkTransform = DISPID_InkTransform(9i32);
pub const DISPID_ITeM22: DISPID_InkTransform = DISPID_InkTransform(10i32);
pub const DISPID_ITeDx: DISPID_InkTransform = DISPID_InkTransform(11i32);
pub const DISPID_ITeDy: DISPID_InkTransform = DISPID_InkTransform(12i32);
pub const DISPID_ITGetTransform: DISPID_InkTransform = DISPID_InkTransform(13i32);
pub const DISPID_ITSetTransform: DISPID_InkTransform = DISPID_InkTransform(14i32);
pub const DISPID_ITData: DISPID_InkTransform = DISPID_InkTransform(15i32);
impl ::core::marker::Copy for DISPID_InkTransform {}
impl ::core::clone::Clone for DISPID_InkTransform {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkTransform {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkTransform {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkTransform").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkWordList(pub i32);
pub const DISPID_InkWordList_AddWord: DISPID_InkWordList = DISPID_InkWordList(0i32);
pub const DISPID_InkWordList_RemoveWord: DISPID_InkWordList = DISPID_InkWordList(1i32);
pub const DISPID_InkWordList_Merge: DISPID_InkWordList = DISPID_InkWordList(2i32);
impl ::core::marker::Copy for DISPID_InkWordList {}
impl ::core::clone::Clone for DISPID_InkWordList {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkWordList {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkWordList {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkWordList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkWordList").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_InkWordList2(pub i32);
pub const DISPID_InkWordList2_AddWords: DISPID_InkWordList2 = DISPID_InkWordList2(3i32);
impl ::core::marker::Copy for DISPID_InkWordList2 {}
impl ::core::clone::Clone for DISPID_InkWordList2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_InkWordList2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_InkWordList2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_InkWordList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_InkWordList2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_MathInputControlEvents(pub i32);
pub const DISPID_MICInsert: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(0i32);
pub const DISPID_MICClose: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(1i32);
pub const DISPID_MICPaint: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(2i32);
pub const DISPID_MICClear: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(3i32);
impl ::core::marker::Copy for DISPID_MathInputControlEvents {}
impl ::core::clone::Clone for DISPID_MathInputControlEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_MathInputControlEvents {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_MathInputControlEvents {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_MathInputControlEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_MathInputControlEvents").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_PenInputPanel(pub i32);
pub const DISPID_PIPAttachedEditWindow: DISPID_PenInputPanel = DISPID_PenInputPanel(0i32);
pub const DISPID_PIPFactoid: DISPID_PenInputPanel = DISPID_PenInputPanel(1i32);
pub const DISPID_PIPCurrentPanel: DISPID_PenInputPanel = DISPID_PenInputPanel(2i32);
pub const DISPID_PIPDefaultPanel: DISPID_PenInputPanel = DISPID_PenInputPanel(3i32);
pub const DISPID_PIPVisible: DISPID_PenInputPanel = DISPID_PenInputPanel(4i32);
pub const DISPID_PIPTop: DISPID_PenInputPanel = DISPID_PenInputPanel(5i32);
pub const DISPID_PIPLeft: DISPID_PenInputPanel = DISPID_PenInputPanel(6i32);
pub const DISPID_PIPWidth: DISPID_PenInputPanel = DISPID_PenInputPanel(7i32);
pub const DISPID_PIPHeight: DISPID_PenInputPanel = DISPID_PenInputPanel(8i32);
pub const DISPID_PIPMoveTo: DISPID_PenInputPanel = DISPID_PenInputPanel(9i32);
pub const DISPID_PIPCommitPendingInput: DISPID_PenInputPanel = DISPID_PenInputPanel(10i32);
pub const DISPID_PIPRefresh: DISPID_PenInputPanel = DISPID_PenInputPanel(11i32);
pub const DISPID_PIPBusy: DISPID_PenInputPanel = DISPID_PenInputPanel(12i32);
pub const DISPID_PIPVerticalOffset: DISPID_PenInputPanel = DISPID_PenInputPanel(13i32);
pub const DISPID_PIPHorizontalOffset: DISPID_PenInputPanel = DISPID_PenInputPanel(14i32);
pub const DISPID_PIPEnableTsf: DISPID_PenInputPanel = DISPID_PenInputPanel(15i32);
pub const DISPID_PIPAutoShow: DISPID_PenInputPanel = DISPID_PenInputPanel(16i32);
impl ::core::marker::Copy for DISPID_PenInputPanel {}
impl ::core::clone::Clone for DISPID_PenInputPanel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_PenInputPanel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_PenInputPanel {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_PenInputPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_PenInputPanel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_PenInputPanelEvents(pub i32);
pub const DISPID_PIPEVisibleChanged: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(0i32);
pub const DISPID_PIPEPanelChanged: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(1i32);
pub const DISPID_PIPEInputFailed: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(2i32);
pub const DISPID_PIPEPanelMoving: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(3i32);
impl ::core::marker::Copy for DISPID_PenInputPanelEvents {}
impl ::core::clone::Clone for DISPID_PenInputPanelEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_PenInputPanelEvents {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_PenInputPanelEvents {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_PenInputPanelEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_PenInputPanelEvents").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPID_StrokeEvent(pub i32);
pub const DISPID_SEStrokesAdded: DISPID_StrokeEvent = DISPID_StrokeEvent(1i32);
pub const DISPID_SEStrokesRemoved: DISPID_StrokeEvent = DISPID_StrokeEvent(2i32);
impl ::core::marker::Copy for DISPID_StrokeEvent {}
impl ::core::clone::Clone for DISPID_StrokeEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_StrokeEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPID_StrokeEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPID_StrokeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_StrokeEvent").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DYNAMIC_RENDERER_CACHED_DATA {
    pub strokeId: i32,
    pub dynamicRenderer: ::core::option::Option<IDynamicRenderer>,
}
impl ::core::clone::Clone for DYNAMIC_RENDERER_CACHED_DATA {
    fn clone(&self) -> Self {
        Self { strokeId: self.strokeId, dynamicRenderer: self.dynamicRenderer.clone() }
    }
}
impl ::core::fmt::Debug for DYNAMIC_RENDERER_CACHED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DYNAMIC_RENDERER_CACHED_DATA").field("strokeId", &self.strokeId).field("dynamicRenderer", &self.dynamicRenderer).finish()
    }
}
unsafe impl ::windows_core::Abi for DYNAMIC_RENDERER_CACHED_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DYNAMIC_RENDERER_CACHED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.strokeId == other.strokeId && self.dynamicRenderer == other.dynamicRenderer
    }
}
impl ::core::cmp::Eq for DYNAMIC_RENDERER_CACHED_DATA {}
impl ::core::default::Default for DYNAMIC_RENDERER_CACHED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn DestroyContext<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyContext(hrc: HRECOCONTEXT) -> ::windows_core::HRESULT;
        }
        DestroyContext(hrc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DestroyRecognizer<'a, Param0: ::windows_core::IntoParam<'a, HRECOGNIZER>>(hrec: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyRecognizer(hrec: HRECOGNIZER) -> ::windows_core::HRESULT;
        }
        DestroyRecognizer(hrec.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DestroyWordList<'a, Param0: ::windows_core::IntoParam<'a, HRECOWORDLIST>>(hwl: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyWordList(hwl: HRECOWORDLIST) -> ::windows_core::HRESULT;
        }
        DestroyWordList(hwl.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DynamicRenderer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecd32aea_746f_4dcb_bf68_082757faff18);
pub const EM_GETDRAWATTR: u32 = 1541u32;
pub const EM_GETFACTOID: u32 = 1549u32;
pub const EM_GETGESTURESTATUS: u32 = 1545u32;
pub const EM_GETINKINSERTMODE: u32 = 1539u32;
pub const EM_GETINKMODE: u32 = 1537u32;
pub const EM_GETMOUSEICON: u32 = 1553u32;
pub const EM_GETMOUSEPOINTER: u32 = 1555u32;
pub const EM_GETRECOGNIZER: u32 = 1547u32;
pub const EM_GETRECOTIMEOUT: u32 = 1543u32;
pub const EM_GETSELINK: u32 = 1551u32;
pub const EM_GETSELINKDISPLAYMODE: u32 = 1562u32;
pub const EM_GETSTATUS: u32 = 1557u32;
pub const EM_GETUSEMOUSEFORINPUT: u32 = 1559u32;
pub const EM_RECOGNIZE: u32 = 1558u32;
pub const EM_SETDRAWATTR: u32 = 1542u32;
pub const EM_SETFACTOID: u32 = 1550u32;
pub const EM_SETGESTURESTATUS: u32 = 1546u32;
pub const EM_SETINKINSERTMODE: u32 = 1540u32;
pub const EM_SETINKMODE: u32 = 1538u32;
pub const EM_SETMOUSEICON: u32 = 1554u32;
pub const EM_SETMOUSEPOINTER: u32 = 1556u32;
pub const EM_SETRECOGNIZER: u32 = 1548u32;
pub const EM_SETRECOTIMEOUT: u32 = 1544u32;
pub const EM_SETSELINK: u32 = 1552u32;
pub const EM_SETSELINKDISPLAYMODE: u32 = 1561u32;
pub const EM_SETUSEMOUSEFORINPUT: u32 = 1560u32;
#[inline]
pub unsafe fn EndInkInput<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndInkInput(hrc: HRECOCONTEXT) -> ::windows_core::HRESULT;
        }
        EndInkInput(hrc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EventMask(pub i32);
pub const EventMask_InPlaceStateChanging: EventMask = EventMask(1i32);
pub const EventMask_InPlaceStateChanged: EventMask = EventMask(2i32);
pub const EventMask_InPlaceSizeChanging: EventMask = EventMask(4i32);
pub const EventMask_InPlaceSizeChanged: EventMask = EventMask(8i32);
pub const EventMask_InputAreaChanging: EventMask = EventMask(16i32);
pub const EventMask_InputAreaChanged: EventMask = EventMask(32i32);
pub const EventMask_CorrectionModeChanging: EventMask = EventMask(64i32);
pub const EventMask_CorrectionModeChanged: EventMask = EventMask(128i32);
pub const EventMask_InPlaceVisibilityChanging: EventMask = EventMask(256i32);
pub const EventMask_InPlaceVisibilityChanged: EventMask = EventMask(512i32);
pub const EventMask_TextInserting: EventMask = EventMask(1024i32);
pub const EventMask_TextInserted: EventMask = EventMask(2048i32);
pub const EventMask_All: EventMask = EventMask(4095i32);
impl ::core::marker::Copy for EventMask {}
impl ::core::clone::Clone for EventMask {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EventMask {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EventMask {
    type Abi = Self;
}
impl ::core::fmt::Debug for EventMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventMask").field(&self.0).finish()
    }
}
pub const FACILITY_INK: u32 = 40u32;
pub const FACTOID_BOPOMOFO: &str = "BOPOMOFO";
pub const FACTOID_CHINESESIMPLECOMMON: &str = "CHS_COMMON";
pub const FACTOID_CHINESETRADITIONALCOMMON: &str = "CHT_COMMON";
pub const FACTOID_CURRENCY: &str = "CURRENCY";
pub const FACTOID_DATE: &str = "DATE";
pub const FACTOID_DEFAULT: &str = "DEFAULT";
pub const FACTOID_DIGIT: &str = "DIGIT";
pub const FACTOID_EMAIL: &str = "EMAIL";
pub const FACTOID_FILENAME: &str = "FILENAME";
pub const FACTOID_HANGULCOMMON: &str = "HANGUL_COMMON";
pub const FACTOID_HANGULRARE: &str = "HANGUL_RARE";
pub const FACTOID_HIRAGANA: &str = "HIRAGANA";
pub const FACTOID_JAMO: &str = "JAMO";
pub const FACTOID_JAPANESECOMMON: &str = "JPN_COMMON";
pub const FACTOID_KANJICOMMON: &str = "KANJI_COMMON";
pub const FACTOID_KANJIRARE: &str = "KANJI_RARE";
pub const FACTOID_KATAKANA: &str = "KATAKANA";
pub const FACTOID_KOREANCOMMON: &str = "KOR_COMMON";
pub const FACTOID_LOWERCHAR: &str = "LOWERCHAR";
pub const FACTOID_NONE: &str = "NONE";
pub const FACTOID_NUMBER: &str = "NUMBER";
pub const FACTOID_NUMBERSIMPLE: &str = "NUMSIMPLE";
pub const FACTOID_ONECHAR: &str = "ONECHAR";
pub const FACTOID_PERCENT: &str = "PERCENT";
pub const FACTOID_POSTALCODE: &str = "POSTALCODE";
pub const FACTOID_PUNCCHAR: &str = "PUNCCHAR";
pub const FACTOID_SYSTEMDICTIONARY: &str = "SYSDICT";
pub const FACTOID_TELEPHONE: &str = "TELEPHONE";
pub const FACTOID_TIME: &str = "TIME";
pub const FACTOID_UPPERCHAR: &str = "UPPERCHAR";
pub const FACTOID_WEB: &str = "WEB";
pub const FACTOID_WORDLIST: &str = "WORDLIST";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FLICKACTION_COMMANDCODE(pub i32);
pub const FLICKACTION_COMMANDCODE_NULL: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(0i32);
pub const FLICKACTION_COMMANDCODE_SCROLL: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(1i32);
pub const FLICKACTION_COMMANDCODE_APPCOMMAND: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(2i32);
pub const FLICKACTION_COMMANDCODE_CUSTOMKEY: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(3i32);
pub const FLICKACTION_COMMANDCODE_KEYMODIFIER: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(4i32);
impl ::core::marker::Copy for FLICKACTION_COMMANDCODE {}
impl ::core::clone::Clone for FLICKACTION_COMMANDCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FLICKACTION_COMMANDCODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FLICKACTION_COMMANDCODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FLICKACTION_COMMANDCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLICKACTION_COMMANDCODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FLICKDIRECTION(pub i32);
pub const FLICKDIRECTION_MIN: FLICKDIRECTION = FLICKDIRECTION(0i32);
pub const FLICKDIRECTION_RIGHT: FLICKDIRECTION = FLICKDIRECTION(0i32);
pub const FLICKDIRECTION_UPRIGHT: FLICKDIRECTION = FLICKDIRECTION(1i32);
pub const FLICKDIRECTION_UP: FLICKDIRECTION = FLICKDIRECTION(2i32);
pub const FLICKDIRECTION_UPLEFT: FLICKDIRECTION = FLICKDIRECTION(3i32);
pub const FLICKDIRECTION_LEFT: FLICKDIRECTION = FLICKDIRECTION(4i32);
pub const FLICKDIRECTION_DOWNLEFT: FLICKDIRECTION = FLICKDIRECTION(5i32);
pub const FLICKDIRECTION_DOWN: FLICKDIRECTION = FLICKDIRECTION(6i32);
pub const FLICKDIRECTION_DOWNRIGHT: FLICKDIRECTION = FLICKDIRECTION(7i32);
pub const FLICKDIRECTION_INVALID: FLICKDIRECTION = FLICKDIRECTION(8i32);
impl ::core::marker::Copy for FLICKDIRECTION {}
impl ::core::clone::Clone for FLICKDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FLICKDIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FLICKDIRECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FLICKDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLICKDIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FLICKMODE(pub i32);
pub const FLICKMODE_MIN: FLICKMODE = FLICKMODE(0i32);
pub const FLICKMODE_OFF: FLICKMODE = FLICKMODE(0i32);
pub const FLICKMODE_ON: FLICKMODE = FLICKMODE(1i32);
pub const FLICKMODE_LEARNING: FLICKMODE = FLICKMODE(2i32);
pub const FLICKMODE_MAX: FLICKMODE = FLICKMODE(2i32);
pub const FLICKMODE_DEFAULT: FLICKMODE = FLICKMODE(1i32);
impl ::core::marker::Copy for FLICKMODE {}
impl ::core::clone::Clone for FLICKMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FLICKMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FLICKMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FLICKMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLICKMODE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct FLICK_DATA {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for FLICK_DATA {}
impl ::core::clone::Clone for FLICK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLICK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLICK_DATA").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows_core::Abi for FLICK_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLICK_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLICK_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLICK_DATA {}
impl ::core::default::Default for FLICK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FLICK_POINT {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for FLICK_POINT {}
impl ::core::clone::Clone for FLICK_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLICK_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLICK_POINT").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows_core::Abi for FLICK_POINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLICK_POINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLICK_POINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLICK_POINT {}
impl ::core::default::Default for FLICK_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FLICK_WM_HANDLED_MASK: u32 = 1u32;
pub const GESTURE_ARROW_DOWN: u32 = 61497u32;
pub const GESTURE_ARROW_LEFT: u32 = 61498u32;
pub const GESTURE_ARROW_RIGHT: u32 = 61499u32;
pub const GESTURE_ARROW_UP: u32 = 61496u32;
pub const GESTURE_ASTERISK: u32 = 61608u32;
pub const GESTURE_BRACE_LEFT: u32 = 61674u32;
pub const GESTURE_BRACE_OVER: u32 = 61672u32;
pub const GESTURE_BRACE_RIGHT: u32 = 61675u32;
pub const GESTURE_BRACE_UNDER: u32 = 61673u32;
pub const GESTURE_BRACKET_LEFT: u32 = 61670u32;
pub const GESTURE_BRACKET_OVER: u32 = 61668u32;
pub const GESTURE_BRACKET_RIGHT: u32 = 61671u32;
pub const GESTURE_BRACKET_UNDER: u32 = 61669u32;
pub const GESTURE_BULLET: u32 = 61450u32;
pub const GESTURE_BULLET_CROSS: u32 = 61451u32;
pub const GESTURE_CHECK: u32 = 61445u32;
pub const GESTURE_CHEVRON_DOWN: u32 = 61489u32;
pub const GESTURE_CHEVRON_LEFT: u32 = 61490u32;
pub const GESTURE_CHEVRON_RIGHT: u32 = 61491u32;
pub const GESTURE_CHEVRON_UP: u32 = 61488u32;
pub const GESTURE_CIRCLE: u32 = 61472u32;
pub const GESTURE_CIRCLE_CIRCLE: u32 = 61475u32;
pub const GESTURE_CIRCLE_CROSS: u32 = 61477u32;
pub const GESTURE_CIRCLE_LINE_HORZ: u32 = 61479u32;
pub const GESTURE_CIRCLE_LINE_VERT: u32 = 61478u32;
pub const GESTURE_CIRCLE_TAP: u32 = 61474u32;
pub const GESTURE_CLOSEUP: u32 = 61455u32;
pub const GESTURE_CROSS: u32 = 61447u32;
pub const GESTURE_CURLICUE: u32 = 61456u32;
#[repr(C)]
pub struct GESTURE_DATA {
    pub gestureId: i32,
    pub recoConfidence: i32,
    pub strokeCount: i32,
}
impl ::core::marker::Copy for GESTURE_DATA {}
impl ::core::clone::Clone for GESTURE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GESTURE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTURE_DATA").field("gestureId", &self.gestureId).field("recoConfidence", &self.recoConfidence).field("strokeCount", &self.strokeCount).finish()
    }
}
unsafe impl ::windows_core::Abi for GESTURE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GESTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTURE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for GESTURE_DATA {}
impl ::core::default::Default for GESTURE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const GESTURE_DIAGONAL_LEFTDOWN: u32 = 61534u32;
pub const GESTURE_DIAGONAL_LEFTUP: u32 = 61532u32;
pub const GESTURE_DIAGONAL_RIGHTDOWN: u32 = 61535u32;
pub const GESTURE_DIAGONAL_RIGHTUP: u32 = 61533u32;
pub const GESTURE_DIGIT_0: u32 = 61594u32;
pub const GESTURE_DIGIT_1: u32 = 61595u32;
pub const GESTURE_DIGIT_2: u32 = 61596u32;
pub const GESTURE_DIGIT_3: u32 = 61597u32;
pub const GESTURE_DIGIT_4: u32 = 61598u32;
pub const GESTURE_DIGIT_5: u32 = 61599u32;
pub const GESTURE_DIGIT_6: u32 = 61600u32;
pub const GESTURE_DIGIT_7: u32 = 61601u32;
pub const GESTURE_DIGIT_8: u32 = 61602u32;
pub const GESTURE_DIGIT_9: u32 = 61603u32;
pub const GESTURE_DOLLAR: u32 = 61607u32;
pub const GESTURE_DOUBLE_ARROW_DOWN: u32 = 61501u32;
pub const GESTURE_DOUBLE_ARROW_LEFT: u32 = 61502u32;
pub const GESTURE_DOUBLE_ARROW_RIGHT: u32 = 61503u32;
pub const GESTURE_DOUBLE_ARROW_UP: u32 = 61500u32;
pub const GESTURE_DOUBLE_CIRCLE: u32 = 61473u32;
pub const GESTURE_DOUBLE_CURLICUE: u32 = 61457u32;
pub const GESTURE_DOUBLE_DOWN: u32 = 61625u32;
pub const GESTURE_DOUBLE_LEFT: u32 = 61626u32;
pub const GESTURE_DOUBLE_RIGHT: u32 = 61627u32;
pub const GESTURE_DOUBLE_TAP: u32 = 61681u32;
pub const GESTURE_DOUBLE_UP: u32 = 61624u32;
pub const GESTURE_DOWN: u32 = 61529u32;
pub const GESTURE_DOWN_ARROW_LEFT: u32 = 61506u32;
pub const GESTURE_DOWN_ARROW_RIGHT: u32 = 61507u32;
pub const GESTURE_DOWN_LEFT: u32 = 61546u32;
pub const GESTURE_DOWN_LEFT_LONG: u32 = 61542u32;
pub const GESTURE_DOWN_RIGHT: u32 = 61547u32;
pub const GESTURE_DOWN_RIGHT_LONG: u32 = 61543u32;
pub const GESTURE_DOWN_UP: u32 = 61537u32;
pub const GESTURE_EXCLAMATION: u32 = 61604u32;
pub const GESTURE_INFINITY: u32 = 61446u32;
pub const GESTURE_LEFT: u32 = 61530u32;
pub const GESTURE_LEFT_ARROW_DOWN: u32 = 61509u32;
pub const GESTURE_LEFT_ARROW_UP: u32 = 61508u32;
pub const GESTURE_LEFT_DOWN: u32 = 61549u32;
pub const GESTURE_LEFT_RIGHT: u32 = 61538u32;
pub const GESTURE_LEFT_UP: u32 = 61548u32;
pub const GESTURE_LETTER_A: u32 = 61568u32;
pub const GESTURE_LETTER_B: u32 = 61569u32;
pub const GESTURE_LETTER_C: u32 = 61570u32;
pub const GESTURE_LETTER_D: u32 = 61571u32;
pub const GESTURE_LETTER_E: u32 = 61572u32;
pub const GESTURE_LETTER_F: u32 = 61573u32;
pub const GESTURE_LETTER_G: u32 = 61574u32;
pub const GESTURE_LETTER_H: u32 = 61575u32;
pub const GESTURE_LETTER_I: u32 = 61576u32;
pub const GESTURE_LETTER_J: u32 = 61577u32;
pub const GESTURE_LETTER_K: u32 = 61578u32;
pub const GESTURE_LETTER_L: u32 = 61579u32;
pub const GESTURE_LETTER_M: u32 = 61580u32;
pub const GESTURE_LETTER_N: u32 = 61581u32;
pub const GESTURE_LETTER_O: u32 = 61582u32;
pub const GESTURE_LETTER_P: u32 = 61583u32;
pub const GESTURE_LETTER_Q: u32 = 61584u32;
pub const GESTURE_LETTER_R: u32 = 61585u32;
pub const GESTURE_LETTER_S: u32 = 61586u32;
pub const GESTURE_LETTER_T: u32 = 61587u32;
pub const GESTURE_LETTER_U: u32 = 61588u32;
pub const GESTURE_LETTER_V: u32 = 61589u32;
pub const GESTURE_LETTER_W: u32 = 61590u32;
pub const GESTURE_LETTER_X: u32 = 61591u32;
pub const GESTURE_LETTER_Y: u32 = 61592u32;
pub const GESTURE_LETTER_Z: u32 = 61593u32;
pub const GESTURE_NULL: u32 = 61440u32;
pub const GESTURE_OPENUP: u32 = 61454u32;
pub const GESTURE_PARAGRAPH: u32 = 61448u32;
pub const GESTURE_PLUS: u32 = 61609u32;
pub const GESTURE_QUAD_TAP: u32 = 61683u32;
pub const GESTURE_QUESTION: u32 = 61605u32;
pub const GESTURE_RECTANGLE: u32 = 61458u32;
pub const GESTURE_RIGHT: u32 = 61531u32;
pub const GESTURE_RIGHT_ARROW_DOWN: u32 = 61511u32;
pub const GESTURE_RIGHT_ARROW_UP: u32 = 61510u32;
pub const GESTURE_RIGHT_DOWN: u32 = 61551u32;
pub const GESTURE_RIGHT_LEFT: u32 = 61539u32;
pub const GESTURE_RIGHT_UP: u32 = 61550u32;
pub const GESTURE_SCRATCHOUT: u32 = 61441u32;
pub const GESTURE_SECTION: u32 = 61449u32;
pub const GESTURE_SEMICIRCLE_LEFT: u32 = 61480u32;
pub const GESTURE_SEMICIRCLE_RIGHT: u32 = 61481u32;
pub const GESTURE_SHARP: u32 = 61606u32;
pub const GESTURE_SQUARE: u32 = 61443u32;
pub const GESTURE_SQUIGGLE: u32 = 61452u32;
pub const GESTURE_STAR: u32 = 61444u32;
pub const GESTURE_SWAP: u32 = 61453u32;
pub const GESTURE_TAP: u32 = 61680u32;
pub const GESTURE_TRIANGLE: u32 = 61442u32;
pub const GESTURE_TRIPLE_DOWN: u32 = 61629u32;
pub const GESTURE_TRIPLE_LEFT: u32 = 61630u32;
pub const GESTURE_TRIPLE_RIGHT: u32 = 61631u32;
pub const GESTURE_TRIPLE_TAP: u32 = 61682u32;
pub const GESTURE_TRIPLE_UP: u32 = 61628u32;
pub const GESTURE_UP: u32 = 61528u32;
pub const GESTURE_UP_ARROW_LEFT: u32 = 61504u32;
pub const GESTURE_UP_ARROW_RIGHT: u32 = 61505u32;
pub const GESTURE_UP_DOWN: u32 = 61536u32;
pub const GESTURE_UP_LEFT: u32 = 61544u32;
pub const GESTURE_UP_LEFT_LONG: u32 = 61540u32;
pub const GESTURE_UP_RIGHT: u32 = 61545u32;
pub const GESTURE_UP_RIGHT_LONG: u32 = 61541u32;
pub const GUID_DYNAMIC_RENDERER_CACHED_DATA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf531b92_25bf_4a95_89ad_0e476b34b4f5);
pub const GUID_GESTURE_DATA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41e4ec0f_26aa_455a_9aa5_2cd36cf63fb9);
pub const GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82dec5c7_f6ba_4906_894f_66d68dfc456c);
pub const GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x029123b4_8828_410b_b250_a0536595e5dc);
pub const GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b7fefc4_96aa_4bfe_ac26_8a5f0be07bf5);
pub const GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02585b91_049b_4750_9615_df8948ab3c9c);
pub const GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe706c804_57f0_4f00_8a0c_853d57789be9);
pub const GUID_PACKETPROPERTY_GUID_HEIGHT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe61858d2_e447_4218_9d3f_18865c203df4);
pub const GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7307502d_f9f4_4e18_b3f2_2ce1b1a3610c);
pub const GUID_PACKETPROPERTY_GUID_PACKET_STATUS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e0e07bf_afe7_4cf7_87d1_af6446208418);
pub const GUID_PACKETPROPERTY_GUID_PITCH_ROTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f7e57b7_be37_4be1_a356_7a84160e1893);
pub const GUID_PACKETPROPERTY_GUID_ROLL_ROTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d5d5e56_6ba9_4c5b_9fb0_851c91714e56);
pub const GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78a81b56_0935_4493_baae_00541a8a16c4);
pub const GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6da4488b_5244_41ec_905b_32d89ab80809);
pub const GUID_PACKETPROPERTY_GUID_TIMER_TICK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x436510c5_fed3_45d1_8b76_71d3ea7a829d);
pub const GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d324960_13b2_41e4_ace6_7ae9d43d2d3b);
pub const GUID_PACKETPROPERTY_GUID_WIDTH: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbaabe94d_2712_48f5_be9d_8f8b5ea0711a);
pub const GUID_PACKETPROPERTY_GUID_X: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x598a6a8f_52c0_4ba0_93af_af357411a561);
pub const GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8d07b3a_8bf0_40b0_95a9_b80a6bb787bf);
pub const GUID_PACKETPROPERTY_GUID_Y: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb53f9f75_04e0_4498_a7ee_c30dbb5a9011);
pub const GUID_PACKETPROPERTY_GUID_YAW_ROTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a849980_7c3a_45b7_aa82_90a262950e89);
pub const GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e932389_1d77_43af_ac00_5b950d6d4b2d);
pub const GUID_PACKETPROPERTY_GUID_Z: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x735adb30_0ebb_4788_a0e4_0f316490055d);
pub const GestureRecognizer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea30c654_c62c_441f_ac00_95f9a196782c);
#[inline]
pub unsafe fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows_core::GUID, count: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows_core::GUID, count: *mut u32) -> ::windows_core::HRESULT;
        }
        GetAllRecognizers(::core::mem::transmute(recognizerclsids), ::core::mem::transmute(count)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetBestResultString<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pcsize: *mut u32, pwcbestresult: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBestResultString(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcbestresult: ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        GetBestResultString(hrc.into_param().abi(), ::core::mem::transmute(pcsize), ::core::mem::transmute(pwcbestresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetLatticePtr<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pplattice: *mut *mut RECO_LATTICE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLatticePtr(hrc: HRECOCONTEXT, pplattice: *mut *mut RECO_LATTICE) -> ::windows_core::HRESULT;
        }
        GetLatticePtr(hrc.into_param().abi(), ::core::mem::transmute(pplattice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetLeftSeparator<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pcsize: *mut u32, pwcleftseparator: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLeftSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcleftseparator: ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        GetLeftSeparator(hrc.into_param().abi(), ::core::mem::transmute(pcsize), ::core::mem::transmute(pwcleftseparator)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetRecoAttributes<'a, Param0: ::windows_core::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, precoattrs: *mut RECO_ATTRS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRecoAttributes(hrec: HRECOGNIZER, precoattrs: *mut RECO_ATTRS) -> ::windows_core::HRESULT;
        }
        GetRecoAttributes(hrec.into_param().abi(), ::core::mem::transmute(precoattrs)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetResultPropertyList<'a, Param0: ::windows_core::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, ppropertycount: *mut u32, ppropertyguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetResultPropertyList(hrec: HRECOGNIZER, ppropertycount: *mut u32, ppropertyguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        GetResultPropertyList(hrec.into_param().abi(), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(ppropertyguid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetRightSeparator<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pcsize: *mut u32, pwcrightseparator: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRightSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcrightseparator: ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        GetRightSeparator(hrc.into_param().abi(), ::core::mem::transmute(pcsize), ::core::mem::transmute(pwcrightseparator)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetUnicodeRanges<'a, Param0: ::windows_core::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUnicodeRanges(hrec: HRECOGNIZER, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows_core::HRESULT;
        }
        GetUnicodeRanges(hrec.into_param().abi(), ::core::mem::transmute(pcranges), ::core::mem::transmute(pcr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRECOALT(pub isize);
impl HRECOALT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRECOALT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRECOALT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRECOALT {}
impl ::core::fmt::Debug for HRECOALT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRECOALT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HRECOALT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRECOCONTEXT(pub isize);
impl HRECOCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRECOCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRECOCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRECOCONTEXT {}
impl ::core::fmt::Debug for HRECOCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRECOCONTEXT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HRECOCONTEXT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRECOGNIZER(pub isize);
impl HRECOGNIZER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRECOGNIZER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRECOGNIZER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRECOGNIZER {}
impl ::core::fmt::Debug for HRECOGNIZER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRECOGNIZER").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HRECOGNIZER {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRECOLATTICE(pub isize);
impl HRECOLATTICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRECOLATTICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRECOLATTICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRECOLATTICE {}
impl ::core::fmt::Debug for HRECOLATTICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRECOLATTICE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HRECOLATTICE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRECOWORDLIST(pub isize);
impl HRECOWORDLIST {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRECOWORDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRECOWORDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRECOWORDLIST {}
impl ::core::fmt::Debug for HRECOWORDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRECOWORDLIST").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HRECOWORDLIST {
    type Abi = Self;
}
pub const HandwrittenTextInsertion: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f074ee2_e6e9_4d8a_a047_eb5b5c3c55da);
#[repr(transparent)]
pub struct IDynamicRenderer(::windows_core::IUnknown);
impl IDynamicRenderer {
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, benabled: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    pub unsafe fn HWND(&self) -> ::windows_core::Result<::win32_foundation::HANDLE_PTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE_PTR>::zeroed();
        (::windows_core::Interface::vtable(self).HWND)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE_PTR>(result__)
    }
    pub unsafe fn SetHWND<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE_PTR>>(&self, hwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHWND)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn ClipRectangle(&self) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).ClipRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn SetClipRectangle(&self, prccliprect: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClipRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prccliprect)).ok()
    }
    pub unsafe fn ClipRegion(&self) -> ::windows_core::Result<::win32_foundation::HANDLE_PTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE_PTR>::zeroed();
        (::windows_core::Interface::vtable(self).ClipRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE_PTR>(result__)
    }
    pub unsafe fn SetClipRegion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE_PTR>>(&self, hcliprgn: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClipRegion)(::windows_core::Interface::as_raw(self), hcliprgn.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, pida: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DrawingAttributes)(::windows_core::Interface::as_raw(self), pida.into_param().abi()).ok()
    }
    pub unsafe fn DataCacheEnabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).DataCacheEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetDataCacheEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fcachedata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDataCacheEnabled)(::windows_core::Interface::as_raw(self), fcachedata.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseCachedData(&self, strokeid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseCachedData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(strokeid)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Draw<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE_PTR>>(&self, hdc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Draw)(::windows_core::Interface::as_raw(self), hdc.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDynamicRenderer> for ::windows_core::IUnknown {
    fn from(value: IDynamicRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDynamicRenderer> for ::windows_core::IUnknown {
    fn from(value: &IDynamicRenderer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDynamicRenderer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDynamicRenderer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDynamicRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDynamicRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicRenderer {}
impl ::core::fmt::Debug for IDynamicRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicRenderer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDynamicRenderer {
    type Vtable = IDynamicRenderer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa079468e_7165_46f9_b7af_98ad01a93009);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicRenderer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub HWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    pub SetHWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    pub ClipRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prccliprect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub SetClipRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prccliprect: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub ClipRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phcliprgn: *mut ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    pub SetClipRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcliprgn: ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppida: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pida: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DrawingAttributes: usize,
    pub DataCacheEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcachedata: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetDataCacheEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcachedata: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub ReleaseCachedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
}
pub const IECN_GESTURE: u32 = 2050u32;
pub const IECN_RECOGNITIONRESULT: u32 = 2051u32;
pub const IECN_STROKE: u32 = 2049u32;
pub const IECN__BASE: u32 = 2048u32;
#[repr(C)]
#[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
pub struct IEC_GESTUREINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: ::core::option::Option<IInkCursor>,
    pub Strokes: ::core::option::Option<IInkStrokes>,
    pub Gestures: ::win32_system::Com::VARIANT,
}
#[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
impl ::core::clone::Clone for IEC_GESTUREINFO {
    fn clone(&self) -> Self {
        Self { nmhdr: self.nmhdr, Cursor: self.Cursor.clone(), Strokes: self.Strokes.clone(), Gestures: self.Gestures.clone() }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
unsafe impl ::windows_core::Abi for IEC_GESTUREINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::PartialEq for IEC_GESTUREINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.Cursor == other.Cursor && self.Strokes == other.Strokes && self.Gestures == other.Gestures
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::Eq for IEC_GESTUREINFO {}
#[cfg(all(feature = "win32-system", feature = "win32-system", feature = "win32-ui"))]
impl ::core::default::Default for IEC_GESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
pub struct IEC_RECOGNITIONRESULTINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub RecognitionResult: ::core::option::Option<IInkRecognitionResult>,
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::clone::Clone for IEC_RECOGNITIONRESULTINFO {
    fn clone(&self) -> Self {
        Self { nmhdr: self.nmhdr, RecognitionResult: self.RecognitionResult.clone() }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::fmt::Debug for IEC_RECOGNITIONRESULTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IEC_RECOGNITIONRESULTINFO").field("nmhdr", &self.nmhdr).field("RecognitionResult", &self.RecognitionResult).finish()
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
unsafe impl ::windows_core::Abi for IEC_RECOGNITIONRESULTINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::PartialEq for IEC_RECOGNITIONRESULTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.RecognitionResult == other.RecognitionResult
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::Eq for IEC_RECOGNITIONRESULTINFO {}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::default::Default for IEC_RECOGNITIONRESULTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
pub struct IEC_STROKEINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: ::core::option::Option<IInkCursor>,
    pub Stroke: ::core::option::Option<IInkStrokeDisp>,
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::clone::Clone for IEC_STROKEINFO {
    fn clone(&self) -> Self {
        Self { nmhdr: self.nmhdr, Cursor: self.Cursor.clone(), Stroke: self.Stroke.clone() }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::fmt::Debug for IEC_STROKEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IEC_STROKEINFO").field("nmhdr", &self.nmhdr).field("Cursor", &self.Cursor).field("Stroke", &self.Stroke).finish()
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
unsafe impl ::windows_core::Abi for IEC_STROKEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::PartialEq for IEC_STROKEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.Cursor == other.Cursor && self.Stroke == other.Stroke
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::cmp::Eq for IEC_STROKEINFO {}
#[cfg(all(feature = "win32-system", feature = "win32-ui"))]
impl ::core::default::Default for IEC_STROKEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IEC__BASE: u32 = 1536u32;
#[repr(transparent)]
pub struct IGestureRecognizer(::windows_core::IUnknown);
impl IGestureRecognizer {
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenabled: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), fenabled.into_param().abi()).ok()
    }
    pub unsafe fn MaxStrokeCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxStrokeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxStrokeCount(&self, cstrokes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxStrokeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstrokes)).ok()
    }
    pub unsafe fn EnableGestures(&self, pgestures: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableGestures)(::windows_core::Interface::as_raw(self), pgestures.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pgestures))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IGestureRecognizer> for ::windows_core::IUnknown {
    fn from(value: IGestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGestureRecognizer> for ::windows_core::IUnknown {
    fn from(value: &IGestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGestureRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGestureRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGestureRecognizer {}
impl ::core::fmt::Debug for IGestureRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGestureRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae9ef86b_7054_45e3_ae22_3174dc8811b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub MaxStrokeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstrokes: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxStrokeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstrokes: i32) -> ::windows_core::HRESULT,
    pub EnableGestures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cgestures: u32, pgestures: *const i32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IHandwrittenTextInsertion(::windows_core::IUnknown);
impl IHandwrittenTextInsertion {
    #[cfg(feature = "win32-system")]
    pub unsafe fn InsertRecognitionResultsArray<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, psaalternates: *const ::win32_system::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertRecognitionResultsArray)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psaalternates), ::core::mem::transmute(locale), falternatecontainsautospacinginformation.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn InsertInkRecognitionResult<'a, Param0: ::windows_core::IntoParam<'a, IInkRecognitionResult>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, piinkrecoresult: Param0, locale: u32, falternatecontainsautospacinginformation: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertInkRecognitionResult)(::windows_core::Interface::as_raw(self), piinkrecoresult.into_param().abi(), ::core::mem::transmute(locale), falternatecontainsautospacinginformation.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IHandwrittenTextInsertion> for ::windows_core::IUnknown {
    fn from(value: IHandwrittenTextInsertion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHandwrittenTextInsertion> for ::windows_core::IUnknown {
    fn from(value: &IHandwrittenTextInsertion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHandwrittenTextInsertion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHandwrittenTextInsertion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHandwrittenTextInsertion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHandwrittenTextInsertion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHandwrittenTextInsertion {}
impl ::core::fmt::Debug for IHandwrittenTextInsertion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHandwrittenTextInsertion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHandwrittenTextInsertion {
    type Vtable = IHandwrittenTextInsertion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56fdea97_ecd6_43e7_aa3a_816be7785860);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandwrittenTextInsertion_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub InsertRecognitionResultsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psaalternates: *const ::win32_system::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    InsertRecognitionResultsArray: usize,
    #[cfg(feature = "win32-system")]
    pub InsertInkRecognitionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piinkrecoresult: ::windows_core::RawPtr, locale: u32, falternatecontainsautospacinginformation: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    InsertInkRecognitionResult: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInk(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInk {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInk> for ::windows_core::IUnknown {
    fn from(value: IInk) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInk> for ::windows_core::IUnknown {
    fn from(value: &IInk) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInk> for ::win32_system::Com::IDispatch {
    fn from(value: IInk) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInk> for ::win32_system::Com::IDispatch {
    fn from(value: &IInk) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInk {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInk").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInk {
    type Vtable = IInk_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03f8e511_43a1_11d3_8bb6_0080c7d6bad5);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInk_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkCollector(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkCollector {
    pub unsafe fn hWnd(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).hWnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn SethWnd(&self, newwindow: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SethWnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newwindow)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, collecting: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(collecting)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DefaultDrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultDrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DefaultDrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, newattributes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DefaultDrawingAttributes)(::windows_core::Interface::as_raw(self), newattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Renderer(&self) -> ::windows_core::Result<IInkRenderer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Renderer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRenderer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Renderer<'a, Param0: ::windows_core::IntoParam<'a, IInkRenderer>>(&self, newinkrenderer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Renderer)(::windows_core::Interface::as_raw(self), newinkrenderer.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Ink(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Ink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Ink<'a, Param0: ::windows_core::IntoParam<'a, IInkDisp>>(&self, newink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Ink)(::windows_core::Interface::as_raw(self), newink.into_param().abi()).ok()
    }
    pub unsafe fn AutoRedraw(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoRedraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoRedraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(autoredraw)).ok()
    }
    pub unsafe fn CollectingInk(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CollectingInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CollectionMode(&self) -> ::windows_core::Result<InkCollectionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkCollectionMode>::zeroed();
        (::windows_core::Interface::vtable(self).CollectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkCollectionMode>(result__)
    }
    pub unsafe fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCollectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn DynamicRendering(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DynamicRendering)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDynamicRendering(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDynamicRendering)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn DesiredPacketDescription(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DesiredPacketDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDesiredPacketDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, packetguids: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDesiredPacketDescription)(::windows_core::Interface::as_raw(self), packetguids.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn MouseIcon(&self) -> ::windows_core::Result<::win32_system::Ole::IPictureDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MouseIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IPictureDisp>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_MouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    pub unsafe fn MousePointer(&self) -> ::windows_core::Result<InkMousePointer> {
        let mut result__ = ::core::mem::MaybeUninit::<InkMousePointer>::zeroed();
        (::windows_core::Interface::vtable(self).MousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkMousePointer>(result__)
    }
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mousepointer)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Cursors(&self) -> ::windows_core::Result<IInkCursors> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Cursors)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursors>(result__)
    }
    pub unsafe fn MarginX(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MarginX)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMarginX(&self, marginx: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMarginX)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(marginx)).ok()
    }
    pub unsafe fn MarginY(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MarginY)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMarginY(&self, marginy: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMarginY)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(marginy)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Tablet(&self) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Tablet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    pub unsafe fn SupportHighContrastInk(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SupportHighContrastInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSupportHighContrastInk(&self, support: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSupportHighContrastInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(support)).ok()
    }
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(listen)).ok()
    }
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWindowInputRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windowinputrectangle)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWindowInputRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, windowinputrectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWindowInputRectangle)(::windows_core::Interface::as_raw(self), windowinputrectangle.into_param().abi()).ok()
    }
    pub unsafe fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllTabletsMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(usemouseforinput)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSingleTabletIntegratedMode<'a, Param0: ::windows_core::IntoParam<'a, IInkTablet>>(&self, tablet: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSingleTabletIntegratedMode)(::windows_core::Interface::as_raw(self), tablet.into_param().abi()).ok()
    }
    pub unsafe fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), ::core::mem::transmute(listen)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCollector> for ::windows_core::IUnknown {
    fn from(value: IInkCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCollector> for ::windows_core::IUnknown {
    fn from(value: &IInkCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCollector> for ::win32_system::Com::IDispatch {
    fn from(value: IInkCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCollector> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkCollector {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkCollector {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCollector").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkCollector {
    type Vtable = IInkCollector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0f060b5_8b1f_4a7c_89ec_880692588a4f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkCollector_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub hWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows_core::HRESULT,
    pub SethWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DefaultDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattributes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DefaultDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub Renderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Renderer: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Renderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Renderer: usize,
    #[cfg(feature = "win32-system")]
    pub Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Ink: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Ink: usize,
    pub AutoRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows_core::HRESULT,
    pub SetAutoRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows_core::HRESULT,
    pub CollectingInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows_core::HRESULT,
    pub CollectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows_core::HRESULT,
    pub SetCollectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows_core::HRESULT,
    pub DynamicRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetDynamicRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub DesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetguids: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    DesiredPacketDescription: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDesiredPacketDescription: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    MouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows_core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Cursors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cursors: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Cursors: usize,
    pub MarginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows_core::HRESULT,
    pub SetMarginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows_core::HRESULT,
    pub MarginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows_core::HRESULT,
    pub SetMarginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Tablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Tablet: usize,
    pub SupportHighContrastInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows_core::HRESULT,
    pub SetSupportHighContrastInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: i16) -> ::windows_core::HRESULT,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWindowInputRectangle: usize,
    #[cfg(feature = "win32-system")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetWindowInputRectangle: usize,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SetSingleTabletIntegratedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablet: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSingleTabletIntegratedMode: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows_core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkCursor(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkCursor {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Inverted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Inverted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, attributes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DrawingAttributes)(::windows_core::Interface::as_raw(self), attributes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Tablet(&self) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Tablet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Buttons(&self) -> ::windows_core::Result<IInkCursorButtons> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Buttons)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursorButtons>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursor> for ::windows_core::IUnknown {
    fn from(value: IInkCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursor> for ::windows_core::IUnknown {
    fn from(value: &IInkCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursor> for ::win32_system::Com::IDispatch {
    fn from(value: IInkCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursor> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkCursor {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkCursor {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursor").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkCursor {
    type Vtable = IInkCursor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad30c630_40c5_4350_8405_9c71012fc558);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursor_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT,
    pub Inverted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub Tablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Tablet: usize,
    #[cfg(feature = "win32-system")]
    pub Buttons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttons: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Buttons: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkCursorButton(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkCursorButton {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<InkCursorButtonState> {
        let mut result__ = ::core::mem::MaybeUninit::<InkCursorButtonState>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkCursorButtonState>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursorButton> for ::windows_core::IUnknown {
    fn from(value: IInkCursorButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursorButton> for ::windows_core::IUnknown {
    fn from(value: &IInkCursorButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkCursorButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkCursorButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursorButton> for ::win32_system::Com::IDispatch {
    fn from(value: IInkCursorButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursorButton> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkCursorButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkCursorButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkCursorButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkCursorButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkCursorButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkCursorButton {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkCursorButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursorButton").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkCursorButton {
    type Vtable = IInkCursorButton_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85ef9417_1d59_49b2_a13c_702c85430894);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursorButton_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentstate: *mut InkCursorButtonState) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkCursorButtons(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkCursorButtons {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifier: Param0) -> ::windows_core::Result<IInkCursorButton> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), identifier.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursorButton>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursorButtons> for ::windows_core::IUnknown {
    fn from(value: IInkCursorButtons) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursorButtons> for ::windows_core::IUnknown {
    fn from(value: &IInkCursorButtons) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkCursorButtons {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkCursorButtons {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursorButtons> for ::win32_system::Com::IDispatch {
    fn from(value: IInkCursorButtons) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursorButtons> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkCursorButtons) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkCursorButtons {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkCursorButtons {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkCursorButtons {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkCursorButtons {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkCursorButtons {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkCursorButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursorButtons").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkCursorButtons {
    type Vtable = IInkCursorButtons_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3671cc40_b624_4671_9fa0_db119d952d54);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursorButtons_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, button: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Item: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkCursors(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkCursors {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<IInkCursor> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursor>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursors> for ::windows_core::IUnknown {
    fn from(value: IInkCursors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursors> for ::windows_core::IUnknown {
    fn from(value: &IInkCursors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkCursors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkCursors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCursors> for ::win32_system::Com::IDispatch {
    fn from(value: IInkCursors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCursors> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkCursors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkCursors {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkCursors {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkCursors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkCursors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkCursors {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkCursors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCursors").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkCursors {
    type Vtable = IInkCursors_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa248c1ac_c698_4e06_9e5c_d57f77c7e647);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursors_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, cursor: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkCustomStrokes(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkCustomStrokes {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifier: Param0) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), identifier.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, name: Param0, strokes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), name.into_param().abi(), strokes.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifier: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), identifier.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCustomStrokes> for ::windows_core::IUnknown {
    fn from(value: IInkCustomStrokes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCustomStrokes> for ::windows_core::IUnknown {
    fn from(value: &IInkCustomStrokes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkCustomStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkCustomStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkCustomStrokes> for ::win32_system::Com::IDispatch {
    fn from(value: IInkCustomStrokes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkCustomStrokes> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkCustomStrokes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkCustomStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkCustomStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkCustomStrokes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkCustomStrokes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkCustomStrokes {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkCustomStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCustomStrokes").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkCustomStrokes {
    type Vtable = IInkCustomStrokes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e23a88f_c30e_420f_9bdb_28902543f0c1);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkCustomStrokes_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Item: usize,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, strokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkDisp(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkDisp {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExtendedProperties(&self) -> ::windows_core::Result<IInkExtendedProperties> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkExtendedProperties>(result__)
    }
    pub unsafe fn Dirty(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Dirty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDirty(&self, dirty: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDirty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dirty)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CustomStrokes(&self) -> ::windows_core::Result<IInkCustomStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CustomStrokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCustomStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBoundingBox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(boundingboxmode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteStrokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteStrokes)(::windows_core::Interface::as_raw(self), strokes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DeleteStroke<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokeDisp>>(&self, stroke: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteStroke)(::windows_core::Interface::as_raw(self), stroke.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExtractStrokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0, extractflags: InkExtractFlags) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExtractStrokes)(::windows_core::Interface::as_raw(self), strokes.into_param().abi(), ::core::mem::transmute(extractflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExtractWithRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0, extractflags: InkExtractFlags) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExtractWithRectangle)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi(), ::core::mem::transmute(extractflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clip<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clip)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).HitTestCircle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(radius), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn HitTestWithRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, selectionrectangle: Param0, intersectpercent: f32) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).HitTestWithRectangle)(::windows_core::Interface::as_raw(self), selectionrectangle.into_param().abi(), ::core::mem::transmute(intersectpercent), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn HitTestWithLasso<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, points: Param0, intersectpercent: f32, lassopoints: *mut ::win32_system::Com::VARIANT, strokes: *mut ::core::option::Option<IInkStrokes>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HitTestWithLasso)(::windows_core::Interface::as_raw(self), points.into_param().abi(), ::core::mem::transmute(intersectpercent), ::core::mem::transmute(lassopoints), ::core::mem::transmute(strokes)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NearestPoint(&self, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NearestPoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(pointonstroke), ::core::mem::transmute(distancefrompacket), ::core::mem::transmute(stroke)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateStrokes<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, strokeids: Param0) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateStrokes)(::windows_core::Interface::as_raw(self), strokeids.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AddStrokesAtRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>, Param1: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, sourcestrokes: Param0, targetrectangle: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStrokesAtRectangle)(::windows_core::Interface::as_raw(self), sourcestrokes.into_param().abi(), targetrectangle.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Save(&self, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(persistenceformat), ::core::mem::transmute(compressionmode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, data: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateStroke<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, packetdata: Param0, packetdescription: Param1) -> ::windows_core::Result<IInkStrokeDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateStroke)(::windows_core::Interface::as_raw(self), packetdata.into_param().abi(), packetdescription.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokeDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ClipboardCopyWithRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows_core::Result<::win32_system::Com::IDataObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ClipboardCopyWithRectangle)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi(), ::core::mem::transmute(clipboardformats), ::core::mem::transmute(clipboardmodes), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IDataObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ClipboardCopy<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows_core::Result<::win32_system::Com::IDataObject> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ClipboardCopy)(::windows_core::Interface::as_raw(self), strokes.into_param().abi(), ::core::mem::transmute(clipboardformats), ::core::mem::transmute(clipboardmodes), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IDataObject>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CanPaste<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IDataObject>>(&self, dataobject: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CanPaste)(::windows_core::Interface::as_raw(self), dataobject.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ClipboardPaste<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IDataObject>>(&self, x: i32, y: i32, dataobject: Param2) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ClipboardPaste)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), dataobject.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDisp> for ::windows_core::IUnknown {
    fn from(value: IInkDisp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDisp> for ::windows_core::IUnknown {
    fn from(value: &IInkDisp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDisp> for ::win32_system::Com::IDispatch {
    fn from(value: IInkDisp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDisp> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkDisp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkDisp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkDisp {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkDisp {
    type Vtable = IInkDisp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d398fa0_c4e2_4fcd_9973_975caaf47ea6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkDisp_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExtendedProperties: usize,
    pub Dirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dirty: *mut i16) -> ::windows_core::HRESULT,
    pub SetDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dirty: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CustomStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkinkcustomstrokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CustomStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub GetBoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetBoundingBox: usize,
    #[cfg(feature = "win32-system")]
    pub DeleteStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeleteStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub DeleteStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DeleteStroke: usize,
    #[cfg(feature = "win32-system")]
    pub ExtractStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExtractStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub ExtractWithRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExtractWithRectangle: usize,
    #[cfg(feature = "win32-system")]
    pub Clip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clip: usize,
    #[cfg(feature = "win32-system")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clone: usize,
    #[cfg(feature = "win32-system")]
    pub HitTestCircle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    HitTestCircle: usize,
    #[cfg(feature = "win32-system")]
    pub HitTestWithRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionrectangle: ::windows_core::RawPtr, intersectpercent: f32, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    HitTestWithRectangle: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub HitTestWithLasso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut ::win32_system::Com::VARIANT, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    HitTestWithLasso: usize,
    #[cfg(feature = "win32-system")]
    pub NearestPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NearestPoint: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeids: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub AddStrokesAtRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestrokes: ::windows_core::RawPtr, targetrectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AddStrokesAtRectangle: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Save: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Load: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetdata: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, packetdescription: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, stroke: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateStroke: usize,
    #[cfg(feature = "win32-system")]
    pub ClipboardCopyWithRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ClipboardCopyWithRectangle: usize,
    #[cfg(feature = "win32-system")]
    pub ClipboardCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ClipboardCopy: usize,
    #[cfg(feature = "win32-system")]
    pub CanPaste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataobject: ::windows_core::RawPtr, canpaste: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CanPaste: usize,
    #[cfg(feature = "win32-system")]
    pub ClipboardPaste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, dataobject: ::windows_core::RawPtr, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ClipboardPaste: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkDivider(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkDivider {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Strokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Strokes)(::windows_core::Interface::as_raw(self), strokes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RecognizerContext(&self) -> ::windows_core::Result<IInkRecognizerContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RecognizerContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizerContext>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_RecognizerContext<'a, Param0: ::windows_core::IntoParam<'a, IInkRecognizerContext>>(&self, recognizercontext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_RecognizerContext)(::windows_core::Interface::as_raw(self), recognizercontext.into_param().abi()).ok()
    }
    pub unsafe fn LineHeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LineHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLineHeight(&self, lineheight: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLineHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lineheight)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Divide(&self) -> ::windows_core::Result<IInkDivisionResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Divide)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDivisionResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivider> for ::windows_core::IUnknown {
    fn from(value: IInkDivider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivider> for ::windows_core::IUnknown {
    fn from(value: &IInkDivider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkDivider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkDivider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivider> for ::win32_system::Com::IDispatch {
    fn from(value: IInkDivider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivider> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkDivider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkDivider {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkDivider {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkDivider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkDivider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkDivider {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkDivider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivider").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkDivider {
    type Vtable = IInkDivider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5de00405_f9a4_4651_b0c5_c317defd58b9);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivider_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub RecognizerContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizercontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RecognizerContext: usize,
    #[cfg(feature = "win32-system")]
    pub putref_RecognizerContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizercontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_RecognizerContext: usize,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineheight: *mut i32) -> ::windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineheight: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Divide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkdivisionresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Divide: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkDivisionResult(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkDivisionResult {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ResultByType(&self, divisiontype: InkDivisionType) -> ::windows_core::Result<IInkDivisionUnits> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ResultByType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(divisiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDivisionUnits>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivisionResult> for ::windows_core::IUnknown {
    fn from(value: IInkDivisionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivisionResult> for ::windows_core::IUnknown {
    fn from(value: &IInkDivisionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkDivisionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkDivisionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivisionResult> for ::win32_system::Com::IDispatch {
    fn from(value: IInkDivisionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivisionResult> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkDivisionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkDivisionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkDivisionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkDivisionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkDivisionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkDivisionResult {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkDivisionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivisionResult").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkDivisionResult {
    type Vtable = IInkDivisionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dbec0a7_74c7_4b38_81eb_aa8ef0c24900);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivisionResult_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub ResultByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, divisiontype: InkDivisionType, inkdivisionunits: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ResultByType: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkDivisionUnit(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkDivisionUnit {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    pub unsafe fn DivisionType(&self) -> ::windows_core::Result<InkDivisionType> {
        let mut result__ = ::core::mem::MaybeUninit::<InkDivisionType>::zeroed();
        (::windows_core::Interface::vtable(self).DivisionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkDivisionType>(result__)
    }
    pub unsafe fn RecognizedString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RecognizedString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RotationTransform(&self) -> ::windows_core::Result<IInkTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RotationTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTransform>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivisionUnit> for ::windows_core::IUnknown {
    fn from(value: IInkDivisionUnit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivisionUnit> for ::windows_core::IUnknown {
    fn from(value: &IInkDivisionUnit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkDivisionUnit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkDivisionUnit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivisionUnit> for ::win32_system::Com::IDispatch {
    fn from(value: IInkDivisionUnit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivisionUnit> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkDivisionUnit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkDivisionUnit {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkDivisionUnit {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkDivisionUnit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkDivisionUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkDivisionUnit {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkDivisionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivisionUnit").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkDivisionUnit {
    type Vtable = IInkDivisionUnit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85aee342_48b0_4244_9dd5_1ed435410fab);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivisionUnit_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    pub DivisionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, divisiontype: *mut InkDivisionType) -> ::windows_core::HRESULT,
    pub RecognizedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recostring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub RotationTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotationtransform: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RotationTransform: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkDivisionUnits(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkDivisionUnits {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<IInkDivisionUnit> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDivisionUnit>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivisionUnits> for ::windows_core::IUnknown {
    fn from(value: IInkDivisionUnits) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivisionUnits> for ::windows_core::IUnknown {
    fn from(value: &IInkDivisionUnits) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkDivisionUnits {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkDivisionUnits {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDivisionUnits> for ::win32_system::Com::IDispatch {
    fn from(value: IInkDivisionUnits) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDivisionUnits> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkDivisionUnits) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkDivisionUnits {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkDivisionUnits {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkDivisionUnits {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkDivisionUnits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkDivisionUnits {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkDivisionUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDivisionUnits").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkDivisionUnits {
    type Vtable = IInkDivisionUnits_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb5ddc2_31cc_4135_ab82_2c66c9f00c41);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivisionUnits_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, inkdivisionunit: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkDrawingAttributes(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkDrawingAttributes {
    pub unsafe fn Color(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Color)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetColor(&self, newcolor: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newcolor)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).Width)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetWidth(&self, newwidth: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newwidth)).ok()
    }
    pub unsafe fn Height(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).Height)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetHeight(&self, newheight: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newheight)).ok()
    }
    pub unsafe fn FitToCurve(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).FitToCurve)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFitToCurve(&self, flag: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFitToCurve)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flag)).ok()
    }
    pub unsafe fn IgnorePressure(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IgnorePressure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIgnorePressure(&self, flag: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIgnorePressure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flag)).ok()
    }
    pub unsafe fn AntiAliased(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AntiAliased)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAntiAliased(&self, flag: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAntiAliased)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flag)).ok()
    }
    pub unsafe fn Transparency(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Transparency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTransparency(&self, newtransparency: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransparency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newtransparency)).ok()
    }
    pub unsafe fn RasterOperation(&self) -> ::windows_core::Result<InkRasterOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRasterOperation>::zeroed();
        (::windows_core::Interface::vtable(self).RasterOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRasterOperation>(result__)
    }
    pub unsafe fn SetRasterOperation(&self, newrasteroperation: InkRasterOperation) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRasterOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newrasteroperation)).ok()
    }
    pub unsafe fn PenTip(&self) -> ::windows_core::Result<InkPenTip> {
        let mut result__ = ::core::mem::MaybeUninit::<InkPenTip>::zeroed();
        (::windows_core::Interface::vtable(self).PenTip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkPenTip>(result__)
    }
    pub unsafe fn SetPenTip(&self, newpentip: InkPenTip) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPenTip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newpentip)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExtendedProperties(&self) -> ::windows_core::Result<IInkExtendedProperties> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkExtendedProperties>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDrawingAttributes> for ::windows_core::IUnknown {
    fn from(value: IInkDrawingAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDrawingAttributes> for ::windows_core::IUnknown {
    fn from(value: &IInkDrawingAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkDrawingAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkDrawingAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkDrawingAttributes> for ::win32_system::Com::IDispatch {
    fn from(value: IInkDrawingAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkDrawingAttributes> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkDrawingAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkDrawingAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkDrawingAttributes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkDrawingAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkDrawingAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkDrawingAttributes {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkDrawingAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDrawingAttributes").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf519b75_0a15_4623_adc9_c00d436a8092);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentcolor: *mut i32) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcolor: i32) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentwidth: *mut f32) -> ::windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newwidth: f32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentheight: *mut f32) -> ::windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newheight: f32) -> ::windows_core::HRESULT,
    pub FitToCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows_core::HRESULT,
    pub SetFitToCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows_core::HRESULT,
    pub IgnorePressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows_core::HRESULT,
    pub SetIgnorePressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows_core::HRESULT,
    pub AntiAliased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows_core::HRESULT,
    pub SetAntiAliased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows_core::HRESULT,
    pub Transparency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currenttransparency: *mut i32) -> ::windows_core::HRESULT,
    pub SetTransparency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newtransparency: i32) -> ::windows_core::HRESULT,
    pub RasterOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentrasteroperation: *mut InkRasterOperation) -> ::windows_core::HRESULT,
    pub SetRasterOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newrasteroperation: InkRasterOperation) -> ::windows_core::HRESULT,
    pub PenTip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpentip: *mut InkPenTip) -> ::windows_core::HRESULT,
    pub SetPenTip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newpentip: InkPenTip) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExtendedProperties: usize,
    #[cfg(feature = "win32-system")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clone: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkEdit(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkEdit {
    pub unsafe fn Status(&self) -> ::windows_core::Result<InkEditStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<InkEditStatus>::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkEditStatus>(result__)
    }
    pub unsafe fn UseMouseForInput(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).UseMouseForInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseMouseForInput(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUseMouseForInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn InkMode(&self) -> ::windows_core::Result<InkMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkMode>::zeroed();
        (::windows_core::Interface::vtable(self).InkMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkMode>(result__)
    }
    pub unsafe fn SetInkMode(&self, newval: InkMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInkMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn InkInsertMode(&self) -> ::windows_core::Result<InkInsertMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkInsertMode>::zeroed();
        (::windows_core::Interface::vtable(self).InkInsertMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkInsertMode>(result__)
    }
    pub unsafe fn SetInkInsertMode(&self, newval: InkInsertMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInkInsertMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DrawingAttributes)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn RecognitionTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RecognitionTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRecognitionTimeout(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRecognitionTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Recognizer(&self) -> ::windows_core::Result<IInkRecognizer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Recognizer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Recognizer<'a, Param0: ::windows_core::IntoParam<'a, IInkRecognizer>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Recognizer)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Factoid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Factoid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFactoid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFactoid)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelInks(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelInks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelInks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, selink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelInks)(::windows_core::Interface::as_raw(self), selink.into_param().abi()).ok()
    }
    pub unsafe fn SelInksDisplayMode(&self) -> ::windows_core::Result<InkDisplayMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkDisplayMode>::zeroed();
        (::windows_core::Interface::vtable(self).SelInksDisplayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkDisplayMode>(result__)
    }
    pub unsafe fn SetSelInksDisplayMode(&self, inkdisplaymode: InkDisplayMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelInksDisplayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(inkdisplaymode)).ok()
    }
    pub unsafe fn Recognize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Recognize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(listen)).ok()
    }
    pub unsafe fn SetBackColor(&self, clr: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clr)).ok()
    }
    pub unsafe fn BackColor(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).BackColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Appearance(&self) -> ::windows_core::Result<AppearanceConstants> {
        let mut result__ = ::core::mem::MaybeUninit::<AppearanceConstants>::zeroed();
        (::windows_core::Interface::vtable(self).Appearance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AppearanceConstants>(result__)
    }
    pub unsafe fn SetAppearance(&self, pappearance: AppearanceConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAppearance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pappearance)).ok()
    }
    pub unsafe fn BorderStyle(&self) -> ::windows_core::Result<BorderStyleConstants> {
        let mut result__ = ::core::mem::MaybeUninit::<BorderStyleConstants>::zeroed();
        (::windows_core::Interface::vtable(self).BorderStyle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BorderStyleConstants>(result__)
    }
    pub unsafe fn SetBorderStyle(&self, pborderstyle: BorderStyleConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBorderStyle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pborderstyle)).ok()
    }
    pub unsafe fn Hwnd(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Hwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Font(&self) -> ::windows_core::Result<::win32_system::Ole::IFontDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Font)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IFontDisp>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn putref_Font<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IFontDisp>>(&self, ppfont: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Font)(::windows_core::Interface::as_raw(self), ppfont.into_param().abi()).ok()
    }
    pub unsafe fn Text(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Text)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, pbstrtext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetText)(::windows_core::Interface::as_raw(self), pbstrtext.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn MouseIcon(&self) -> ::windows_core::Result<::win32_system::Ole::IPictureDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MouseIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IPictureDisp>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_MouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    pub unsafe fn MousePointer(&self) -> ::windows_core::Result<InkMousePointer> {
        let mut result__ = ::core::mem::MaybeUninit::<InkMousePointer>::zeroed();
        (::windows_core::Interface::vtable(self).MousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkMousePointer>(result__)
    }
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mousepointer)).ok()
    }
    pub unsafe fn Locked(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Locked)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLocked(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocked)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn MaxLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxLength(&self, lmaxlength: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmaxlength)).ok()
    }
    pub unsafe fn MultiLine(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MultiLine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMultiLine(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultiLine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn ScrollBars(&self) -> ::windows_core::Result<ScrollBarsConstants> {
        let mut result__ = ::core::mem::MaybeUninit::<ScrollBarsConstants>::zeroed();
        (::windows_core::Interface::vtable(self).ScrollBars)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ScrollBarsConstants>(result__)
    }
    pub unsafe fn SetScrollBars(&self, newval: ScrollBarsConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScrollBars)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn DisableNoScroll(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DisableNoScroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisableNoScroll(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisableNoScroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelAlignment(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelAlignment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelAlignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselalignment: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelAlignment)(::windows_core::Interface::as_raw(self), pvarselalignment.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelBold(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelBold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelBold<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselbold: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelBold)(::windows_core::Interface::as_raw(self), pvarselbold.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelItalic(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelItalic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelItalic<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselitalic: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelItalic)(::windows_core::Interface::as_raw(self), pvarselitalic.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelUnderline(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelUnderline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelUnderline<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselunderline: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelUnderline)(::windows_core::Interface::as_raw(self), pvarselunderline.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelColor(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelColor<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselcolor: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelColor)(::windows_core::Interface::as_raw(self), pvarselcolor.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelFontName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelFontName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelFontName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselfontname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelFontName)(::windows_core::Interface::as_raw(self), pvarselfontname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelFontSize(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelFontSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelFontSize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselfontsize: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelFontSize)(::windows_core::Interface::as_raw(self), pvarselfontsize.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelCharOffset(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelCharOffset)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetSelCharOffset<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, pvarselcharoffset: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelCharOffset)(::windows_core::Interface::as_raw(self), pvarselcharoffset.into_param().abi()).ok()
    }
    pub unsafe fn TextRTF(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TextRTF)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetTextRTF<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, pbstrtextrtf: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTextRTF)(::windows_core::Interface::as_raw(self), pbstrtextrtf.into_param().abi()).ok()
    }
    pub unsafe fn SelStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SelStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSelStart(&self, plselstart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plselstart)).ok()
    }
    pub unsafe fn SelLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SelLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSelLength(&self, plsellength: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plsellength)).ok()
    }
    pub unsafe fn SelText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SelText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSelText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, pbstrseltext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelText)(::windows_core::Interface::as_raw(self), pbstrseltext.into_param().abi()).ok()
    }
    pub unsafe fn SelRTF(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SelRTF)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSelRTF<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, pbstrselrtf: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelRTF)(::windows_core::Interface::as_raw(self), pbstrselrtf.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkEdit> for ::windows_core::IUnknown {
    fn from(value: IInkEdit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkEdit> for ::windows_core::IUnknown {
    fn from(value: &IInkEdit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkEdit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkEdit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkEdit> for ::win32_system::Com::IDispatch {
    fn from(value: IInkEdit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkEdit> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkEdit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkEdit {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkEdit {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkEdit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkEdit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkEdit {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkEdit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkEdit").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkEdit {
    type Vtable = IInkEdit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2127a19_fbfb_4aed_8464_3f36d78cfefb);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkEdit_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut InkEditStatus) -> ::windows_core::HRESULT,
    pub UseMouseForInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetUseMouseForInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub InkMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut InkMode) -> ::windows_core::HRESULT,
    pub SetInkMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: InkMode) -> ::windows_core::HRESULT,
    pub InkInsertMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut InkInsertMode) -> ::windows_core::HRESULT,
    pub SetInkInsertMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: InkInsertMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DrawingAttributes: usize,
    pub RecognitionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetRecognitionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Recognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Recognizer: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Recognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Recognizer: usize,
    pub Factoid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFactoid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelInks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pselink: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelInks: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelInks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selink: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelInks: usize,
    pub SelInksDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinkdisplaymode: *mut InkDisplayMode) -> ::windows_core::HRESULT,
    pub SetSelInksDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkdisplaymode: InkDisplayMode) -> ::windows_core::HRESULT,
    pub Recognize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows_core::HRESULT,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows_core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows_core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappearance: *mut AppearanceConstants) -> ::windows_core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappearance: AppearanceConstants) -> ::windows_core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pborderstyle: *mut BorderStyleConstants) -> ::windows_core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pborderstyle: BorderStyleConstants) -> ::windows_core::HRESULT,
    pub Hwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pohhwnd: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Font: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub putref_Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    putref_Font: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    MouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows_core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows_core::HRESULT,
    pub Locked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxlength: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxlength: i32) -> ::windows_core::HRESULT,
    pub MultiLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetMultiLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub ScrollBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ScrollBarsConstants) -> ::windows_core::HRESULT,
    pub SetScrollBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ScrollBarsConstants) -> ::windows_core::HRESULT,
    pub DisableNoScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetDisableNoScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselalignment: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelAlignment: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselalignment: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelAlignment: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselbold: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelBold: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselbold: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelBold: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselitalic: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelItalic: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselitalic: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelItalic: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselunderline: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelUnderline: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselunderline: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelUnderline: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselcolor: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelColor: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselcolor: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelColor: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselfontname: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelFontName: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselfontname: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelFontName: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselfontsize: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelFontSize: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselfontsize: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelFontSize: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelCharOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselcharoffset: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelCharOffset: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetSelCharOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselcharoffset: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetSelCharOffset: usize,
    pub TextRTF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtextrtf: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetTextRTF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtextrtf: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SelStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plselstart: *mut i32) -> ::windows_core::HRESULT,
    pub SetSelStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plselstart: i32) -> ::windows_core::HRESULT,
    pub SelLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsellength: *mut i32) -> ::windows_core::HRESULT,
    pub SetSelLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsellength: i32) -> ::windows_core::HRESULT,
    pub SelText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrseltext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSelText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrseltext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SelRTF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrselrtf: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSelRTF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrselrtf: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkExtendedProperties(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkExtendedProperties {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifier: Param0) -> ::windows_core::Result<IInkExtendedProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), identifier.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkExtendedProperty>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, guid: Param0, data: Param1) -> ::windows_core::Result<IInkExtendedProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), guid.into_param().abi(), data.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkExtendedProperty>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifier: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), identifier.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DoesPropertyExist<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, guid: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DoesPropertyExist)(::windows_core::Interface::as_raw(self), guid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkExtendedProperties> for ::windows_core::IUnknown {
    fn from(value: IInkExtendedProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkExtendedProperties> for ::windows_core::IUnknown {
    fn from(value: &IInkExtendedProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkExtendedProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkExtendedProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkExtendedProperties> for ::win32_system::Com::IDispatch {
    fn from(value: IInkExtendedProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkExtendedProperties> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkExtendedProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkExtendedProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkExtendedProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkExtendedProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkExtendedProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkExtendedProperties {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkExtendedProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkExtendedProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkExtendedProperties {
    type Vtable = IInkExtendedProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89f2a8be_95a9_4530_8b8f_88e971e3e25f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkExtendedProperties_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, item: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Item: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, data: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, inkextendedproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Add: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DoesPropertyExist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkExtendedProperty(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkExtendedProperty {
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Guid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, data: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), data.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkExtendedProperty> for ::windows_core::IUnknown {
    fn from(value: IInkExtendedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkExtendedProperty> for ::windows_core::IUnknown {
    fn from(value: &IInkExtendedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkExtendedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkExtendedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkExtendedProperty> for ::win32_system::Com::IDispatch {
    fn from(value: IInkExtendedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkExtendedProperty> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkExtendedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkExtendedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkExtendedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkExtendedProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkExtendedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkExtendedProperty {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkExtendedProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkExtendedProperty {
    type Vtable = IInkExtendedProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb489209_b7c3_411d_90f6_1548cfff271e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkExtendedProperty_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Data: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetData: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkGesture(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkGesture {
    pub unsafe fn Confidence(&self) -> ::windows_core::Result<InkRecognitionConfidence> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecognitionConfidence>::zeroed();
        (::windows_core::Interface::vtable(self).Confidence)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecognitionConfidence>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<InkApplicationGesture> {
        let mut result__ = ::core::mem::MaybeUninit::<InkApplicationGesture>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkApplicationGesture>(result__)
    }
    pub unsafe fn GetHotPoint(&self, x: *mut i32, y: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHotPoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkGesture> for ::windows_core::IUnknown {
    fn from(value: IInkGesture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkGesture> for ::windows_core::IUnknown {
    fn from(value: &IInkGesture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkGesture> for ::win32_system::Com::IDispatch {
    fn from(value: IInkGesture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkGesture> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkGesture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkGesture {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkGesture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkGesture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkGesture {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkGesture").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkGesture {
    type Vtable = IInkGesture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bdc0a97_04e5_4e26_b813_18f052d41def);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkGesture_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut InkApplicationGesture) -> ::windows_core::HRESULT,
    pub GetHotPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut i32, y: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInkLineInfo(::windows_core::IUnknown);
impl IInkLineInfo {
    pub unsafe fn SetFormat(&self, pim: *const INKMETRIC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pim)).ok()
    }
    pub unsafe fn GetFormat(&self, pim: *const INKMETRIC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pim)).ok()
    }
    pub unsafe fn GetInkExtent(&self, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInkExtent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pim), ::core::mem::transmute(pnwidth)).ok()
    }
    pub unsafe fn GetCandidate<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ncandidatenum: u32, pwcrecogword: Param1, pcwcrecogword: *const u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncandidatenum), pwcrecogword.into_param().abi(), ::core::mem::transmute(pcwcrecogword), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn SetCandidate<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ncandidatenum: u32, strrecogword: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCandidate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncandidatenum), strrecogword.into_param().abi()).ok()
    }
    pub unsafe fn Recognize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Recognize)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IInkLineInfo> for ::windows_core::IUnknown {
    fn from(value: IInkLineInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkLineInfo> for ::windows_core::IUnknown {
    fn from(value: &IInkLineInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkLineInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkLineInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkLineInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkLineInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkLineInfo {}
impl ::core::fmt::Debug for IInkLineInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkLineInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInkLineInfo {
    type Vtable = IInkLineInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c1c5ad6_f22f_4de4_b453_a2cc482e7c33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkLineInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows_core::HRESULT,
    pub GetInkExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows_core::HRESULT,
    pub GetCandidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncandidatenum: u32, pwcrecogword: ::windows_core::PCWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows_core::HRESULT,
    pub SetCandidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncandidatenum: u32, strrecogword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Recognize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkOverlay(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkOverlay {
    pub unsafe fn hWnd(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).hWnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn SethWnd(&self, newwindow: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SethWnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newwindow)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, collecting: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(collecting)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DefaultDrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultDrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DefaultDrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, newattributes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DefaultDrawingAttributes)(::windows_core::Interface::as_raw(self), newattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Renderer(&self) -> ::windows_core::Result<IInkRenderer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Renderer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRenderer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Renderer<'a, Param0: ::windows_core::IntoParam<'a, IInkRenderer>>(&self, newinkrenderer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Renderer)(::windows_core::Interface::as_raw(self), newinkrenderer.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Ink(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Ink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Ink<'a, Param0: ::windows_core::IntoParam<'a, IInkDisp>>(&self, newink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Ink)(::windows_core::Interface::as_raw(self), newink.into_param().abi()).ok()
    }
    pub unsafe fn AutoRedraw(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoRedraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoRedraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(autoredraw)).ok()
    }
    pub unsafe fn CollectingInk(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CollectingInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CollectionMode(&self) -> ::windows_core::Result<InkCollectionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkCollectionMode>::zeroed();
        (::windows_core::Interface::vtable(self).CollectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkCollectionMode>(result__)
    }
    pub unsafe fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCollectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn DynamicRendering(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DynamicRendering)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDynamicRendering(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDynamicRendering)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn DesiredPacketDescription(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DesiredPacketDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDesiredPacketDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, packetguids: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDesiredPacketDescription)(::windows_core::Interface::as_raw(self), packetguids.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn MouseIcon(&self) -> ::windows_core::Result<::win32_system::Ole::IPictureDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MouseIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IPictureDisp>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_MouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    pub unsafe fn MousePointer(&self) -> ::windows_core::Result<InkMousePointer> {
        let mut result__ = ::core::mem::MaybeUninit::<InkMousePointer>::zeroed();
        (::windows_core::Interface::vtable(self).MousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkMousePointer>(result__)
    }
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mousepointer)).ok()
    }
    pub unsafe fn EditingMode(&self) -> ::windows_core::Result<InkOverlayEditingMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkOverlayEditingMode>::zeroed();
        (::windows_core::Interface::vtable(self).EditingMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkOverlayEditingMode>(result__)
    }
    pub unsafe fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEditingMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(editingmode)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Selection(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Selection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSelection<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, selection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelection)(::windows_core::Interface::as_raw(self), selection.into_param().abi()).ok()
    }
    pub unsafe fn EraserMode(&self) -> ::windows_core::Result<InkOverlayEraserMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkOverlayEraserMode>::zeroed();
        (::windows_core::Interface::vtable(self).EraserMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkOverlayEraserMode>(result__)
    }
    pub unsafe fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEraserMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(erasermode)).ok()
    }
    pub unsafe fn EraserWidth(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EraserWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEraserWidth(&self, neweraserwidth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEraserWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(neweraserwidth)).ok()
    }
    pub unsafe fn AttachMode(&self) -> ::windows_core::Result<InkOverlayAttachMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkOverlayAttachMode>::zeroed();
        (::windows_core::Interface::vtable(self).AttachMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkOverlayAttachMode>(result__)
    }
    pub unsafe fn SetAttachMode(&self, attachmode: InkOverlayAttachMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(attachmode)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Cursors(&self) -> ::windows_core::Result<IInkCursors> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Cursors)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursors>(result__)
    }
    pub unsafe fn MarginX(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MarginX)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMarginX(&self, marginx: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMarginX)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(marginx)).ok()
    }
    pub unsafe fn MarginY(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MarginY)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMarginY(&self, marginy: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMarginY)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(marginy)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Tablet(&self) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Tablet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    pub unsafe fn SupportHighContrastInk(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SupportHighContrastInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSupportHighContrastInk(&self, support: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSupportHighContrastInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(support)).ok()
    }
    pub unsafe fn SupportHighContrastSelectionUI(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SupportHighContrastSelectionUI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSupportHighContrastSelectionUI(&self, support: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSupportHighContrastSelectionUI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(support)).ok()
    }
    pub unsafe fn HitTestSelection(&self, x: i32, y: i32) -> ::windows_core::Result<SelectionHitResult> {
        let mut result__ = ::core::mem::MaybeUninit::<SelectionHitResult>::zeroed();
        (::windows_core::Interface::vtable(self).HitTestSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SelectionHitResult>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Draw<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Draw)(::windows_core::Interface::as_raw(self), rect.into_param().abi()).ok()
    }
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(listen)).ok()
    }
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWindowInputRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windowinputrectangle)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWindowInputRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, windowinputrectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWindowInputRectangle)(::windows_core::Interface::as_raw(self), windowinputrectangle.into_param().abi()).ok()
    }
    pub unsafe fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllTabletsMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(usemouseforinput)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSingleTabletIntegratedMode<'a, Param0: ::windows_core::IntoParam<'a, IInkTablet>>(&self, tablet: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSingleTabletIntegratedMode)(::windows_core::Interface::as_raw(self), tablet.into_param().abi()).ok()
    }
    pub unsafe fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), ::core::mem::transmute(listen)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkOverlay> for ::windows_core::IUnknown {
    fn from(value: IInkOverlay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkOverlay> for ::windows_core::IUnknown {
    fn from(value: &IInkOverlay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkOverlay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkOverlay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkOverlay> for ::win32_system::Com::IDispatch {
    fn from(value: IInkOverlay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkOverlay> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkOverlay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkOverlay {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkOverlay {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkOverlay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkOverlay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkOverlay {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkOverlay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkOverlay").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkOverlay {
    type Vtable = IInkOverlay_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb82a463b_c1c5_45a3_997c_deab5651b67a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkOverlay_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub hWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows_core::HRESULT,
    pub SethWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DefaultDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattributes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DefaultDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub Renderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Renderer: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Renderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Renderer: usize,
    #[cfg(feature = "win32-system")]
    pub Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Ink: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Ink: usize,
    pub AutoRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows_core::HRESULT,
    pub SetAutoRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows_core::HRESULT,
    pub CollectingInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows_core::HRESULT,
    pub CollectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows_core::HRESULT,
    pub SetCollectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows_core::HRESULT,
    pub DynamicRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetDynamicRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub DesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetguids: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    DesiredPacketDescription: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDesiredPacketDescription: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    MouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows_core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows_core::HRESULT,
    pub EditingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows_core::HRESULT,
    pub SetEditingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Selection: usize,
    #[cfg(feature = "win32-system")]
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSelection: usize,
    pub EraserMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows_core::HRESULT,
    pub SetEraserMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows_core::HRESULT,
    pub EraserWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows_core::HRESULT,
    pub SetEraserWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows_core::HRESULT,
    pub AttachMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmode: *mut InkOverlayAttachMode) -> ::windows_core::HRESULT,
    pub SetAttachMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmode: InkOverlayAttachMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Cursors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cursors: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Cursors: usize,
    pub MarginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows_core::HRESULT,
    pub SetMarginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows_core::HRESULT,
    pub MarginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows_core::HRESULT,
    pub SetMarginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Tablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Tablet: usize,
    pub SupportHighContrastInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows_core::HRESULT,
    pub SetSupportHighContrastInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: i16) -> ::windows_core::HRESULT,
    pub SupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows_core::HRESULT,
    pub SetSupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: i16) -> ::windows_core::HRESULT,
    pub HitTestSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Draw: usize,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWindowInputRectangle: usize,
    #[cfg(feature = "win32-system")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetWindowInputRectangle: usize,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SetSingleTabletIntegratedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablet: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSingleTabletIntegratedMode: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows_core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkPicture(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkPicture {
    pub unsafe fn hWnd(&self) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).hWnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DefaultDrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultDrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DefaultDrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, newattributes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DefaultDrawingAttributes)(::windows_core::Interface::as_raw(self), newattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Renderer(&self) -> ::windows_core::Result<IInkRenderer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Renderer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRenderer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Renderer<'a, Param0: ::windows_core::IntoParam<'a, IInkRenderer>>(&self, newinkrenderer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Renderer)(::windows_core::Interface::as_raw(self), newinkrenderer.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Ink(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Ink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Ink<'a, Param0: ::windows_core::IntoParam<'a, IInkDisp>>(&self, newink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Ink)(::windows_core::Interface::as_raw(self), newink.into_param().abi()).ok()
    }
    pub unsafe fn AutoRedraw(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoRedraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoRedraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(autoredraw)).ok()
    }
    pub unsafe fn CollectingInk(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CollectingInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CollectionMode(&self) -> ::windows_core::Result<InkCollectionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkCollectionMode>::zeroed();
        (::windows_core::Interface::vtable(self).CollectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkCollectionMode>(result__)
    }
    pub unsafe fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCollectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn DynamicRendering(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DynamicRendering)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDynamicRendering(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDynamicRendering)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn DesiredPacketDescription(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DesiredPacketDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDesiredPacketDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, packetguids: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDesiredPacketDescription)(::windows_core::Interface::as_raw(self), packetguids.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn MouseIcon(&self) -> ::windows_core::Result<::win32_system::Ole::IPictureDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MouseIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IPictureDisp>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_MouseIcon)(::windows_core::Interface::as_raw(self), mouseicon.into_param().abi()).ok()
    }
    pub unsafe fn MousePointer(&self) -> ::windows_core::Result<InkMousePointer> {
        let mut result__ = ::core::mem::MaybeUninit::<InkMousePointer>::zeroed();
        (::windows_core::Interface::vtable(self).MousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkMousePointer>(result__)
    }
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMousePointer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mousepointer)).ok()
    }
    pub unsafe fn EditingMode(&self) -> ::windows_core::Result<InkOverlayEditingMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkOverlayEditingMode>::zeroed();
        (::windows_core::Interface::vtable(self).EditingMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkOverlayEditingMode>(result__)
    }
    pub unsafe fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEditingMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(editingmode)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Selection(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Selection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSelection<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, selection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSelection)(::windows_core::Interface::as_raw(self), selection.into_param().abi()).ok()
    }
    pub unsafe fn EraserMode(&self) -> ::windows_core::Result<InkOverlayEraserMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkOverlayEraserMode>::zeroed();
        (::windows_core::Interface::vtable(self).EraserMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkOverlayEraserMode>(result__)
    }
    pub unsafe fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEraserMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(erasermode)).ok()
    }
    pub unsafe fn EraserWidth(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).EraserWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEraserWidth(&self, neweraserwidth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEraserWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(neweraserwidth)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn putref_Picture<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, ppicture: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Picture)(::windows_core::Interface::as_raw(self), ppicture.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPicture<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Ole::IPictureDisp>>(&self, ppicture: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPicture)(::windows_core::Interface::as_raw(self), ppicture.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Picture(&self) -> ::windows_core::Result<::win32_system::Ole::IPictureDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Picture)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IPictureDisp>(result__)
    }
    pub unsafe fn SetSizeMode(&self, smnewsizemode: InkPictureSizeMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSizeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(smnewsizemode)).ok()
    }
    pub unsafe fn SizeMode(&self) -> ::windows_core::Result<InkPictureSizeMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkPictureSizeMode>::zeroed();
        (::windows_core::Interface::vtable(self).SizeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkPictureSizeMode>(result__)
    }
    pub unsafe fn SetBackColor(&self, newcolor: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newcolor)).ok()
    }
    pub unsafe fn BackColor(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).BackColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Cursors(&self) -> ::windows_core::Result<IInkCursors> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Cursors)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursors>(result__)
    }
    pub unsafe fn MarginX(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MarginX)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMarginX(&self, marginx: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMarginX)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(marginx)).ok()
    }
    pub unsafe fn MarginY(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MarginY)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMarginY(&self, marginy: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMarginY)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(marginy)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Tablet(&self) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Tablet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    pub unsafe fn SupportHighContrastInk(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SupportHighContrastInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSupportHighContrastInk(&self, support: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSupportHighContrastInk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(support)).ok()
    }
    pub unsafe fn SupportHighContrastSelectionUI(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).SupportHighContrastSelectionUI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSupportHighContrastSelectionUI(&self, support: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSupportHighContrastSelectionUI)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(support)).ok()
    }
    pub unsafe fn HitTestSelection(&self, x: i32, y: i32) -> ::windows_core::Result<SelectionHitResult> {
        let mut result__ = ::core::mem::MaybeUninit::<SelectionHitResult>::zeroed();
        (::windows_core::Interface::vtable(self).HitTestSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SelectionHitResult>(result__)
    }
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(listen)).ok()
    }
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetGestureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gesture), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWindowInputRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windowinputrectangle)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWindowInputRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, windowinputrectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWindowInputRectangle)(::windows_core::Interface::as_raw(self), windowinputrectangle.into_param().abi()).ok()
    }
    pub unsafe fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllTabletsMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(usemouseforinput)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSingleTabletIntegratedMode<'a, Param0: ::windows_core::IntoParam<'a, IInkTablet>>(&self, tablet: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSingleTabletIntegratedMode)(::windows_core::Interface::as_raw(self), tablet.into_param().abi()).ok()
    }
    pub unsafe fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), ::core::mem::transmute(listen)).ok()
    }
    pub unsafe fn InkEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).InkEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetInkEnabled(&self, collecting: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInkEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(collecting)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, vbool: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbool)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkPicture> for ::windows_core::IUnknown {
    fn from(value: IInkPicture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkPicture> for ::windows_core::IUnknown {
    fn from(value: &IInkPicture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkPicture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkPicture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkPicture> for ::win32_system::Com::IDispatch {
    fn from(value: IInkPicture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkPicture> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkPicture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkPicture {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkPicture {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkPicture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkPicture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkPicture {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkPicture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPicture").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkPicture {
    type Vtable = IInkPicture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe85662e0_379a_40d7_9b5c_757d233f9923);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkPicture_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub hWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DefaultDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newattributes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DefaultDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub Renderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Renderer: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Renderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Renderer: usize,
    #[cfg(feature = "win32-system")]
    pub Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Ink: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Ink: usize,
    pub AutoRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows_core::HRESULT,
    pub SetAutoRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows_core::HRESULT,
    pub CollectingInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows_core::HRESULT,
    pub CollectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows_core::HRESULT,
    pub SetCollectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows_core::HRESULT,
    pub DynamicRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetDynamicRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub DesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetguids: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    DesiredPacketDescription: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDesiredPacketDescription: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    MouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mouseicon: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows_core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows_core::HRESULT,
    pub EditingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows_core::HRESULT,
    pub SetEditingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Selection: usize,
    #[cfg(feature = "win32-system")]
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSelection: usize,
    pub EraserMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows_core::HRESULT,
    pub SetEraserMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows_core::HRESULT,
    pub EraserWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows_core::HRESULT,
    pub SetEraserWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub putref_Picture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicture: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    putref_Picture: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicture: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPicture: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Picture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppicture: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Picture: usize,
    pub SetSizeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smnewsizemode: InkPictureSizeMode) -> ::windows_core::HRESULT,
    pub SizeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smsizemode: *mut InkPictureSizeMode) -> ::windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcolor: u32) -> ::windows_core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Cursors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cursors: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Cursors: usize,
    pub MarginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows_core::HRESULT,
    pub SetMarginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows_core::HRESULT,
    pub MarginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows_core::HRESULT,
    pub SetMarginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Tablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Tablet: usize,
    pub SupportHighContrastInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows_core::HRESULT,
    pub SetSupportHighContrastInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: i16) -> ::windows_core::HRESULT,
    pub SupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows_core::HRESULT,
    pub SetSupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, support: i16) -> ::windows_core::HRESULT,
    pub HitTestSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows_core::HRESULT,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows_core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWindowInputRectangle: usize,
    #[cfg(feature = "win32-system")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetWindowInputRectangle: usize,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SetSingleTabletIntegratedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablet: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSingleTabletIntegratedMode: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows_core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_core::HRESULT,
    pub InkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows_core::HRESULT,
    pub SetInkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbool: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognitionAlternate(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognitionAlternate {
    pub unsafe fn String(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).String)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Confidence(&self) -> ::windows_core::Result<InkRecognitionConfidence> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecognitionConfidence>::zeroed();
        (::windows_core::Interface::vtable(self).Confidence)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecognitionConfidence>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Baseline(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Baseline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Midline(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Midline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Ascender(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Ascender)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Descender(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Descender)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn LineNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LineNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn LineAlternates(&self) -> ::windows_core::Result<IInkRecognitionAlternates> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LineAlternates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ConfidenceAlternates(&self) -> ::windows_core::Result<IInkRecognitionAlternates> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ConfidenceAlternates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetStrokesFromStrokeRanges<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokesFromStrokeRanges)(::windows_core::Interface::as_raw(self), strokes.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetStrokesFromTextRange(&self, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::core::option::Option<IInkStrokes>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStrokesFromTextRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(selectionstart), ::core::mem::transmute(selectionlength), ::core::mem::transmute(getstrokesfromtextrange)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTextRangeFromStrokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTextRangeFromStrokes)(::windows_core::Interface::as_raw(self), strokes.into_param().abi(), ::core::mem::transmute(selectionstart), ::core::mem::transmute(selectionlength)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AlternatesWithConstantPropertyValues<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertytype: Param0) -> ::windows_core::Result<IInkRecognitionAlternates> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AlternatesWithConstantPropertyValues)(::windows_core::Interface::as_raw(self), propertytype.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetPropertyValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertytype: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyValue)(::windows_core::Interface::as_raw(self), propertytype.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognitionAlternate> for ::windows_core::IUnknown {
    fn from(value: IInkRecognitionAlternate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognitionAlternate> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognitionAlternate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognitionAlternate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognitionAlternate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognitionAlternate> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognitionAlternate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognitionAlternate> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognitionAlternate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognitionAlternate {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognitionAlternate {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognitionAlternate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognitionAlternate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognitionAlternate {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognitionAlternate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognitionAlternate").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognitionAlternate {
    type Vtable = IInkRecognitionAlternate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7e660ad_77e4_429b_adda_873780d1fc4a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionAlternate_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recostring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Baseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseline: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Baseline: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Midline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, midline: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Midline: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Ascender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ascender: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Ascender: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Descender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descender: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Descender: usize,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub LineAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linealternates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    LineAlternates: usize,
    #[cfg(feature = "win32-system")]
    pub ConfidenceAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confidencealternates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ConfidenceAlternates: usize,
    #[cfg(feature = "win32-system")]
    pub GetStrokesFromStrokeRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr, getstrokesfromstrokeranges: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetStrokesFromStrokeRanges: usize,
    #[cfg(feature = "win32-system")]
    pub GetStrokesFromTextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetStrokesFromTextRange: usize,
    #[cfg(feature = "win32-system")]
    pub GetTextRangeFromStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTextRangeFromStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub AlternatesWithConstantPropertyValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, alternateswithconstantpropertyvalues: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AlternatesWithConstantPropertyValues: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetPropertyValue: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognitionAlternates(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognitionAlternates {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<IInkRecognitionAlternate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionAlternate>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognitionAlternates> for ::windows_core::IUnknown {
    fn from(value: IInkRecognitionAlternates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognitionAlternates> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognitionAlternates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognitionAlternates {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognitionAlternates {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognitionAlternates> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognitionAlternates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognitionAlternates> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognitionAlternates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognitionAlternates {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognitionAlternates {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognitionAlternates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognitionAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognitionAlternates {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognitionAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognitionAlternates").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognitionAlternates {
    type Vtable = IInkRecognitionAlternates_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x286a167f_9f19_4c61_9d53_4f07be622b84);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionAlternates_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, inkrecoalternate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognitionResult(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognitionResult {
    pub unsafe fn TopString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TopString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TopAlternate(&self) -> ::windows_core::Result<IInkRecognitionAlternate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).TopAlternate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionAlternate>(result__)
    }
    pub unsafe fn TopConfidence(&self) -> ::windows_core::Result<InkRecognitionConfidence> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecognitionConfidence>::zeroed();
        (::windows_core::Interface::vtable(self).TopConfidence)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecognitionConfidence>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AlternatesFromSelection(&self, selectionstart: i32, selectionlength: i32, maximumalternates: i32) -> ::windows_core::Result<IInkRecognitionAlternates> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AlternatesFromSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(selectionstart), ::core::mem::transmute(selectionlength), ::core::mem::transmute(maximumalternates), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ModifyTopAlternate<'a, Param0: ::windows_core::IntoParam<'a, IInkRecognitionAlternate>>(&self, alternate: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifyTopAlternate)(::windows_core::Interface::as_raw(self), alternate.into_param().abi()).ok()
    }
    pub unsafe fn SetResultOnStrokes(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetResultOnStrokes)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognitionResult> for ::windows_core::IUnknown {
    fn from(value: IInkRecognitionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognitionResult> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognitionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognitionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognitionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognitionResult> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognitionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognitionResult> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognitionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognitionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognitionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognitionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognitionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognitionResult {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognitionResult").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognitionResult {
    type Vtable = IInkRecognitionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bc129a8_86cd_45ad_bde8_e0d32d61c16d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionResult_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub TopString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topstring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub TopAlternate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topalternate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TopAlternate: usize,
    pub TopConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topconfidence: *mut InkRecognitionConfidence) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub AlternatesFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AlternatesFromSelection: usize,
    #[cfg(feature = "win32-system")]
    pub ModifyTopAlternate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alternate: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ModifyTopAlternate: usize,
    pub SetResultOnStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognizer(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognizer {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Vendor(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Vendor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Capabilities(&self) -> ::windows_core::Result<InkRecognizerCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecognizerCapabilities>::zeroed();
        (::windows_core::Interface::vtable(self).Capabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecognizerCapabilities>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SupportedProperties(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SupportedProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PreferredPacketDescription(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PreferredPacketDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateRecognizerContext(&self) -> ::windows_core::Result<IInkRecognizerContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRecognizerContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizerContext>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizer> for ::windows_core::IUnknown {
    fn from(value: IInkRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizer> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizer> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizer> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognizer {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizer").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognizer {
    type Vtable = IInkRecognizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x782bf7cf_034b_4396_8a32_3a1833cf6b56);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Vendor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendor: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Languages: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SupportedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedproperties: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SupportedProperties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PreferredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredpacketdescription: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PreferredPacketDescription: usize,
    #[cfg(feature = "win32-system")]
    pub CreateRecognizerContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateRecognizerContext: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognizer2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognizer2 {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn UnicodeRanges(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).UnicodeRanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizer2> for ::windows_core::IUnknown {
    fn from(value: IInkRecognizer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizer2> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognizer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognizer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognizer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizer2> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognizer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizer2> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognizer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognizer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognizer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognizer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognizer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognizer2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognizer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizer2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognizer2 {
    type Vtable = IInkRecognizer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6110118a_3a75_4ad6_b2aa_04b2b72bbe65);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer2_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub UnicodeRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicoderanges: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    UnicodeRanges: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognizerContext(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognizerContext {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Strokes(&self) -> ::windows_core::Result<IInkStrokes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Strokes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Strokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Strokes)(::windows_core::Interface::as_raw(self), strokes.into_param().abi()).ok()
    }
    pub unsafe fn CharacterAutoCompletionMode(&self) -> ::windows_core::Result<InkRecognizerCharacterAutoCompletionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecognizerCharacterAutoCompletionMode>::zeroed();
        (::windows_core::Interface::vtable(self).CharacterAutoCompletionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecognizerCharacterAutoCompletionMode>(result__)
    }
    pub unsafe fn SetCharacterAutoCompletionMode(&self, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCharacterAutoCompletionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn Factoid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Factoid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFactoid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, factoid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFactoid)(::windows_core::Interface::as_raw(self), factoid.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Guide(&self) -> ::windows_core::Result<IInkRecognizerGuide> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Guide)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizerGuide>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Guide<'a, Param0: ::windows_core::IntoParam<'a, IInkRecognizerGuide>>(&self, recognizerguide: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Guide)(::windows_core::Interface::as_raw(self), recognizerguide.into_param().abi()).ok()
    }
    pub unsafe fn PrefixText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PrefixText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPrefixText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, prefix: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrefixText)(::windows_core::Interface::as_raw(self), prefix.into_param().abi()).ok()
    }
    pub unsafe fn SuffixText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SuffixText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSuffixText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, suffix: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSuffixText)(::windows_core::Interface::as_raw(self), suffix.into_param().abi()).ok()
    }
    pub unsafe fn RecognitionFlags(&self) -> ::windows_core::Result<InkRecognitionModes> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecognitionModes>::zeroed();
        (::windows_core::Interface::vtable(self).RecognitionFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecognitionModes>(result__)
    }
    pub unsafe fn SetRecognitionFlags(&self, modes: InkRecognitionModes) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRecognitionFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(modes)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn WordList(&self) -> ::windows_core::Result<IInkWordList> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WordList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkWordList>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_WordList<'a, Param0: ::windows_core::IntoParam<'a, IInkWordList>>(&self, wordlist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_WordList)(::windows_core::Interface::as_raw(self), wordlist.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Recognizer(&self) -> ::windows_core::Result<IInkRecognizer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Recognizer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Recognize(&self, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::core::option::Option<IInkRecognitionResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Recognize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(recognitionstatus), ::core::mem::transmute(recognitionresult)).ok()
    }
    pub unsafe fn StopBackgroundRecognition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopBackgroundRecognition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndInkInput(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndInkInput)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BackgroundRecognize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, customdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundRecognize)(::windows_core::Interface::as_raw(self), customdata.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BackgroundRecognizeWithAlternates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, customdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundRecognizeWithAlternates)(::windows_core::Interface::as_raw(self), customdata.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IInkRecognizerContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizerContext>(result__)
    }
    pub unsafe fn IsStringSupported<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, string: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsStringSupported)(::windows_core::Interface::as_raw(self), string.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizerContext> for ::windows_core::IUnknown {
    fn from(value: IInkRecognizerContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizerContext> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognizerContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognizerContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognizerContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizerContext> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognizerContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizerContext> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognizerContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognizerContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognizerContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognizerContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognizerContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognizerContext {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognizerContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerContext").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognizerContext {
    type Vtable = IInkRecognizerContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc68f52f9_32a3_4625_906c_44fc23b40958);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContext_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Strokes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Strokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Strokes: usize,
    pub CharacterAutoCompletionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows_core::HRESULT,
    pub SetCharacterAutoCompletionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows_core::HRESULT,
    pub Factoid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factoid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFactoid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Guide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizerguide: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Guide: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Guide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizerguide: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Guide: usize,
    pub PrefixText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPrefixText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SuffixText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suffix: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSuffixText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suffix: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RecognitionFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modes: *mut InkRecognitionModes) -> ::windows_core::HRESULT,
    pub SetRecognitionFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modes: InkRecognitionModes) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub WordList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wordlist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    WordList: usize,
    #[cfg(feature = "win32-system")]
    pub putref_WordList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wordlist: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_WordList: usize,
    #[cfg(feature = "win32-system")]
    pub Recognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Recognizer: usize,
    #[cfg(feature = "win32-system")]
    pub Recognize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Recognize: usize,
    pub StopBackgroundRecognition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndInkInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BackgroundRecognize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BackgroundRecognize: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BackgroundRecognizeWithAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BackgroundRecognizeWithAlternates: usize,
    #[cfg(feature = "win32-system")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clone: usize,
    pub IsStringSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, supported: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognizerContext2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognizerContext2 {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn EnabledUnicodeRanges(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).EnabledUnicodeRanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetEnabledUnicodeRanges<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, unicoderanges: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabledUnicodeRanges)(::windows_core::Interface::as_raw(self), unicoderanges.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizerContext2> for ::windows_core::IUnknown {
    fn from(value: IInkRecognizerContext2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizerContext2> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognizerContext2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognizerContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognizerContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizerContext2> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognizerContext2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizerContext2> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognizerContext2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognizerContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognizerContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognizerContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognizerContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognizerContext2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognizerContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerContext2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognizerContext2 {
    type Vtable = IInkRecognizerContext2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6f0e32f_73d8_408e_8e9f_5fea592c363f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContext2_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub EnabledUnicodeRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicoderanges: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    EnabledUnicodeRanges: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetEnabledUnicodeRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicoderanges: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetEnabledUnicodeRanges: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognizerGuide(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognizerGuide {
    #[cfg(feature = "win32-system")]
    pub unsafe fn WritingBox(&self) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WritingBox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWritingBox<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWritingBox)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DrawnBox(&self) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DrawnBox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetDrawnBox<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDrawnBox)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    pub unsafe fn Rows(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Rows)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRows(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRows)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn Columns(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Columns)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetColumns(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColumns)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn Midline(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Midline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMidline(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMidline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn GuideData(&self) -> ::windows_core::Result<InkRecoGuide> {
        let mut result__ = ::core::mem::MaybeUninit::<InkRecoGuide>::zeroed();
        (::windows_core::Interface::vtable(self).GuideData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InkRecoGuide>(result__)
    }
    pub unsafe fn SetGuideData<'a, Param0: ::windows_core::IntoParam<'a, InkRecoGuide>>(&self, recoguide: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGuideData)(::windows_core::Interface::as_raw(self), recoguide.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizerGuide> for ::windows_core::IUnknown {
    fn from(value: IInkRecognizerGuide) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizerGuide> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognizerGuide) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognizerGuide {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognizerGuide {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizerGuide> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognizerGuide) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizerGuide> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognizerGuide) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognizerGuide {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognizerGuide {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognizerGuide {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognizerGuide {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognizerGuide {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognizerGuide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerGuide").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognizerGuide {
    type Vtable = IInkRecognizerGuide_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd934be07_7b84_4208_9136_83c20994e905);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerGuide_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub WritingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    WritingBox: usize,
    #[cfg(feature = "win32-system")]
    pub SetWritingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetWritingBox: usize,
    #[cfg(feature = "win32-system")]
    pub DrawnBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DrawnBox: usize,
    #[cfg(feature = "win32-system")]
    pub SetDrawnBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetDrawnBox: usize,
    pub Rows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetRows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub Columns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetColumns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub Midline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetMidline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub GuideData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precoguide: *mut InkRecoGuide) -> ::windows_core::HRESULT,
    pub SetGuideData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recoguide: InkRecoGuide) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRecognizers(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRecognizers {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetDefaultRecognizer(&self, lcid: i32) -> ::windows_core::Result<IInkRecognizer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultRecognizer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<IInkRecognizer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognizer>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizers> for ::windows_core::IUnknown {
    fn from(value: IInkRecognizers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizers> for ::windows_core::IUnknown {
    fn from(value: &IInkRecognizers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRecognizers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRecognizers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRecognizers> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRecognizers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRecognizers> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRecognizers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRecognizers {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRecognizers {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRecognizers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRecognizers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRecognizers {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRecognizers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizers").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRecognizers {
    type Vtable = IInkRecognizers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ccc4f12_b0b7_4a8b_bf58_4aeca4e8cefd);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizers_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetDefaultRecognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: i32, defaultrecognizer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetDefaultRecognizer: usize,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, inkrecognizer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRectangle(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRectangle {
    pub unsafe fn Top(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Top)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTop(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTop)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn Left(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Left)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLeft(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLeft)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn Bottom(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Bottom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBottom(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBottom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn Right(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Right)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRight(&self, units: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(units)).ok()
    }
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(&self, rect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), rect.into_param().abi()).ok()
    }
    pub unsafe fn GetRectangle(&self, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(top), ::core::mem::transmute(left), ::core::mem::transmute(bottom), ::core::mem::transmute(right)).ok()
    }
    pub unsafe fn SetRectangle(&self, top: i32, left: i32, bottom: i32, right: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(top), ::core::mem::transmute(left), ::core::mem::transmute(bottom), ::core::mem::transmute(right)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRectangle> for ::windows_core::IUnknown {
    fn from(value: IInkRectangle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRectangle> for ::windows_core::IUnknown {
    fn from(value: &IInkRectangle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRectangle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRectangle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRectangle> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRectangle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRectangle> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRectangle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRectangle {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRectangle {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRectangle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRectangle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRectangle {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRectangle").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRectangle {
    type Vtable = IInkRectangle_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9794ff82_6071_4717_8a8b_6ac7c64a686e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRectangle_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Top: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub Bottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub Right: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows_core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, units: i32) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub GetRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows_core::HRESULT,
    pub SetRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: i32, left: i32, bottom: i32, right: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkRenderer(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkRenderer {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetViewTransform<'a, Param0: ::windows_core::IntoParam<'a, IInkTransform>>(&self, viewtransform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetViewTransform)(::windows_core::Interface::as_raw(self), viewtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetViewTransform<'a, Param0: ::windows_core::IntoParam<'a, IInkTransform>>(&self, viewtransform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetViewTransform)(::windows_core::Interface::as_raw(self), viewtransform.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetObjectTransform<'a, Param0: ::windows_core::IntoParam<'a, IInkTransform>>(&self, objecttransform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectTransform)(::windows_core::Interface::as_raw(self), objecttransform.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetObjectTransform<'a, Param0: ::windows_core::IntoParam<'a, IInkTransform>>(&self, objecttransform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetObjectTransform)(::windows_core::Interface::as_raw(self), objecttransform.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Draw<'a, Param1: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, hdc: isize, strokes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Draw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hdc), strokes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DrawStroke<'a, Param1: ::windows_core::IntoParam<'a, IInkStrokeDisp>, Param2: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, hdc: isize, stroke: Param1, drawingattributes: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DrawStroke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hdc), stroke.into_param().abi(), drawingattributes.into_param().abi()).ok()
    }
    pub unsafe fn PixelToInkSpace(&self, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PixelToInkSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hdc), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn InkSpaceToPixel(&self, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InkSpaceToPixel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hdcdisplay), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PixelToInkSpaceFromPoints(&self, hdc: isize, points: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PixelToInkSpaceFromPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hdc), ::core::mem::transmute(points)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn InkSpaceToPixelFromPoints(&self, hdc: isize, points: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InkSpaceToPixelFromPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hdc), ::core::mem::transmute(points)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Measure<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Measure)(::windows_core::Interface::as_raw(self), strokes.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn MeasureStroke<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokeDisp>, Param1: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, stroke: Param0, drawingattributes: Param1) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MeasureStroke)(::windows_core::Interface::as_raw(self), stroke.into_param().abi(), drawingattributes.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    pub unsafe fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalcomponent), ::core::mem::transmute(verticalcomponent)).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Rotate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(degrees), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalmultiplier), ::core::mem::transmute(verticalmultiplier), ::core::mem::transmute(applyonpenwidth)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRenderer> for ::windows_core::IUnknown {
    fn from(value: IInkRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRenderer> for ::windows_core::IUnknown {
    fn from(value: &IInkRenderer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkRenderer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkRenderer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkRenderer> for ::win32_system::Com::IDispatch {
    fn from(value: IInkRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkRenderer> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkRenderer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkRenderer {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkRenderer {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkRenderer {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRenderer").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkRenderer {
    type Vtable = IInkRenderer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6257a9c_b511_4f4c_a8b0_a7dbc9506b83);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkRenderer_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetViewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewtransform: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetViewTransform: usize,
    #[cfg(feature = "win32-system")]
    pub SetViewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewtransform: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetViewTransform: usize,
    #[cfg(feature = "win32-system")]
    pub GetObjectTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objecttransform: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetObjectTransform: usize,
    #[cfg(feature = "win32-system")]
    pub SetObjectTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objecttransform: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetObjectTransform: usize,
    #[cfg(feature = "win32-system")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, strokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Draw: usize,
    #[cfg(feature = "win32-system")]
    pub DrawStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, stroke: ::windows_core::RawPtr, drawingattributes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DrawStroke: usize,
    pub PixelToInkSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows_core::HRESULT,
    pub InkSpaceToPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PixelToInkSpaceFromPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PixelToInkSpaceFromPoints: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub InkSpaceToPixelFromPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    InkSpaceToPixelFromPoints: usize,
    #[cfg(feature = "win32-system")]
    pub Measure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Measure: usize,
    #[cfg(feature = "win32-system")]
    pub MeasureStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: ::windows_core::RawPtr, drawingattributes: ::windows_core::RawPtr, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    MeasureStroke: usize,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows_core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkStrokeDisp(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkStrokeDisp {
    pub unsafe fn ID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BezierPoints(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).BezierPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DrawingAttributes(&self) -> ::windows_core::Result<IInkDrawingAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DrawingAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, drawattrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_DrawingAttributes)(::windows_core::Interface::as_raw(self), drawattrs.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Ink(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Ink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExtendedProperties(&self) -> ::windows_core::Result<IInkExtendedProperties> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ExtendedProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkExtendedProperties>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PolylineCusps(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolylineCusps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn BezierCusps(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).BezierCusps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SelfIntersections(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SelfIntersections)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn PacketCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PacketCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PacketSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PacketDescription(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PacketDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Deleted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Deleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBoundingBox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(boundingboxmode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn FindIntersections<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).FindIntersections)(::windows_core::Interface::as_raw(self), strokes.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetRectangleIntersections<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetRectangleIntersections)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clip<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clip)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    pub unsafe fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).HitTestCircle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(radius), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn NearestPoint(&self, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NearestPoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(distance), ::core::mem::transmute(point)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Split(&self, splitat: f32) -> ::windows_core::Result<IInkStrokeDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Split)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(splitat), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokeDisp>(result__)
    }
    pub unsafe fn GetPacketDescriptionPropertyMetrics<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertyname: Param0, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPacketDescriptionPropertyMetrics)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), ::core::mem::transmute(minimum), ::core::mem::transmute(maximum), ::core::mem::transmute(units), ::core::mem::transmute(resolution)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetPoints(&self, index: i32, count: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPoints<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, points: Param0, index: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SetPoints)(::windows_core::Interface::as_raw(self), points.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetPacketData(&self, index: i32, count: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPacketData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetPacketValuesByProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertyname: Param0, index: i32, count: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPacketValuesByProperty)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPacketValuesByProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrpropertyname: Param0, packetvalues: Param1, index: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SetPacketValuesByProperty)(::windows_core::Interface::as_raw(self), bstrpropertyname.into_param().abi(), packetvalues.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetFlattenedBezierPoints(&self, fittingerror: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlattenedBezierPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fittingerror), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Transform<'a, Param0: ::windows_core::IntoParam<'a, IInkTransform>>(&self, transform: Param0, applyonpenwidth: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Transform)(::windows_core::Interface::as_raw(self), transform.into_param().abi(), ::core::mem::transmute(applyonpenwidth)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ScaleToRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScaleToRectangle)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    pub unsafe fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalcomponent), ::core::mem::transmute(verticalcomponent)).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Rotate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(degrees), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalmultiplier), ::core::mem::transmute(verticalmultiplier)).ok()
    }
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalmultiplier), ::core::mem::transmute(verticalmultiplier)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkStrokeDisp> for ::windows_core::IUnknown {
    fn from(value: IInkStrokeDisp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkStrokeDisp> for ::windows_core::IUnknown {
    fn from(value: &IInkStrokeDisp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkStrokeDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkStrokeDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkStrokeDisp> for ::win32_system::Com::IDispatch {
    fn from(value: IInkStrokeDisp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkStrokeDisp> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkStrokeDisp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkStrokeDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkStrokeDisp {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkStrokeDisp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkStrokeDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkStrokeDisp {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkStrokeDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkStrokeDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkStrokeDisp {
    type Vtable = IInkStrokeDisp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43242fea_91d1_4a72_963e_fbb91829cfa2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeDisp_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BezierPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BezierPoints: usize,
    #[cfg(feature = "win32-system")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawattrs: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawattrs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_DrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Ink: usize,
    #[cfg(feature = "win32-system")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExtendedProperties: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PolylineCusps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cusps: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PolylineCusps: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub BezierCusps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cusps: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    BezierCusps: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SelfIntersections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intersections: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SelfIntersections: usize,
    pub PacketCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub PacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetdescription: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PacketDescription: usize,
    pub Deleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deleted: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetBoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetBoundingBox: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub FindIntersections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr, intersections: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    FindIntersections: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetRectangleIntersections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr, intersections: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetRectangleIntersections: usize,
    #[cfg(feature = "win32-system")]
    pub Clip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clip: usize,
    pub HitTestCircle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows_core::HRESULT,
    pub NearestPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Split: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, splitat: f32, newstroke: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Split: usize,
    pub GetPacketDescriptionPropertyMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, count: i32, points: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetPoints: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPoints: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetPacketData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, count: i32, packetdata: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetPacketData: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetPacketValuesByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, index: i32, count: i32, packetvalues: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetPacketValuesByProperty: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPacketValuesByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, packetvalues: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPacketValuesByProperty: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetFlattenedBezierPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fittingerror: i32, flattenedbezierpoints: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetFlattenedBezierPoints: usize,
    #[cfg(feature = "win32-system")]
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows_core::RawPtr, applyonpenwidth: i16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Transform: usize,
    #[cfg(feature = "win32-system")]
    pub ScaleToRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ScaleToRectangle: usize,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows_core::HRESULT,
    pub Shear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkStrokes(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkStrokes {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Ink(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Ink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RecognitionResult(&self) -> ::windows_core::Result<IInkRecognitionResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RecognitionResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRecognitionResult>(result__)
    }
    pub unsafe fn ToString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ToString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<IInkStrokeDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkStrokeDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokeDisp>>(&self, inkstroke: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), inkstroke.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AddStrokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, inkstrokes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStrokes)(::windows_core::Interface::as_raw(self), inkstrokes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokeDisp>>(&self, inkstroke: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), inkstroke.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn RemoveStrokes<'a, Param0: ::windows_core::IntoParam<'a, IInkStrokes>>(&self, inkstrokes: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStrokes)(::windows_core::Interface::as_raw(self), inkstrokes.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ModifyDrawingAttributes<'a, Param0: ::windows_core::IntoParam<'a, IInkDrawingAttributes>>(&self, drawattrs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifyDrawingAttributes)(::windows_core::Interface::as_raw(self), drawattrs.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBoundingBox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(boundingboxmode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Transform<'a, Param0: ::windows_core::IntoParam<'a, IInkTransform>>(&self, transform: Param0, applyonpenwidth: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Transform)(::windows_core::Interface::as_raw(self), transform.into_param().abi(), ::core::mem::transmute(applyonpenwidth)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ScaleToRectangle<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScaleToRectangle)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    pub unsafe fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalcomponent), ::core::mem::transmute(verticalcomponent)).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Rotate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(degrees), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalmultiplier), ::core::mem::transmute(verticalmultiplier)).ok()
    }
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalmultiplier), ::core::mem::transmute(verticalmultiplier)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clip<'a, Param0: ::windows_core::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clip)(::windows_core::Interface::as_raw(self), rectangle.into_param().abi()).ok()
    }
    pub unsafe fn RemoveRecognitionResult(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveRecognitionResult)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkStrokes> for ::windows_core::IUnknown {
    fn from(value: IInkStrokes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkStrokes> for ::windows_core::IUnknown {
    fn from(value: &IInkStrokes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkStrokes> for ::win32_system::Com::IDispatch {
    fn from(value: IInkStrokes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkStrokes> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkStrokes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkStrokes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkStrokes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkStrokes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkStrokes {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkStrokes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkStrokes").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkStrokes {
    type Vtable = IInkStrokes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1f4c9d8_590a_4963_b3ae_1935671bb6f3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokes_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Ink: usize,
    #[cfg(feature = "win32-system")]
    pub RecognitionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognitionresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RecognitionResult: usize,
    pub ToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tostring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, stroke: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstroke: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    #[cfg(feature = "win32-system")]
    pub AddStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstrokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AddStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstroke: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Remove: usize,
    #[cfg(feature = "win32-system")]
    pub RemoveStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkstrokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    RemoveStrokes: usize,
    #[cfg(feature = "win32-system")]
    pub ModifyDrawingAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawattrs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ModifyDrawingAttributes: usize,
    #[cfg(feature = "win32-system")]
    pub GetBoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetBoundingBox: usize,
    #[cfg(feature = "win32-system")]
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows_core::RawPtr, applyonpenwidth: i16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Transform: usize,
    #[cfg(feature = "win32-system")]
    pub ScaleToRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ScaleToRectangle: usize,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows_core::HRESULT,
    pub Shear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Clip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clip: usize,
    pub RemoveRecognitionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkTablet(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkTablet {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn PlugAndPlayId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlugAndPlayId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn MaximumInputRectangle(&self) -> ::windows_core::Result<IInkRectangle> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MaximumInputRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkRectangle>(result__)
    }
    pub unsafe fn HardwareCapabilities(&self) -> ::windows_core::Result<TabletHardwareCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::<TabletHardwareCapabilities>::zeroed();
        (::windows_core::Interface::vtable(self).HardwareCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TabletHardwareCapabilities>(result__)
    }
    pub unsafe fn IsPacketPropertySupported<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, packetpropertyname: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPacketPropertySupported)(::windows_core::Interface::as_raw(self), packetpropertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetPropertyMetrics<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertyname: Param0, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyMetrics)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), ::core::mem::transmute(minimum), ::core::mem::transmute(maximum), ::core::mem::transmute(units), ::core::mem::transmute(resolution)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablet> for ::windows_core::IUnknown {
    fn from(value: IInkTablet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablet> for ::windows_core::IUnknown {
    fn from(value: &IInkTablet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkTablet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkTablet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablet> for ::win32_system::Com::IDispatch {
    fn from(value: IInkTablet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablet> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkTablet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkTablet {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkTablet {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkTablet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkTablet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkTablet {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkTablet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablet").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkTablet {
    type Vtable = IInkTablet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2de25eaa_6ef8_42d5_aee9_185bc81b912d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablet_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub PlugAndPlayId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub MaximumInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    MaximumInputRectangle: usize,
    pub HardwareCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: *mut TabletHardwareCapabilities) -> ::windows_core::HRESULT,
    pub IsPacketPropertySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, supported: *mut i16) -> ::windows_core::HRESULT,
    pub GetPropertyMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkTablet2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkTablet2 {
    pub unsafe fn DeviceKind(&self) -> ::windows_core::Result<TabletDeviceKind> {
        let mut result__ = ::core::mem::MaybeUninit::<TabletDeviceKind>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceKind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TabletDeviceKind>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablet2> for ::windows_core::IUnknown {
    fn from(value: IInkTablet2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablet2> for ::windows_core::IUnknown {
    fn from(value: &IInkTablet2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkTablet2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkTablet2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablet2> for ::win32_system::Com::IDispatch {
    fn from(value: IInkTablet2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablet2> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkTablet2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkTablet2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkTablet2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkTablet2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkTablet2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkTablet2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkTablet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablet2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkTablet2 {
    type Vtable = IInkTablet2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90c91ad2_fa36_49d6_9516_ce8d570f6f85);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablet2_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: *mut TabletDeviceKind) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkTablet3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkTablet3 {
    pub unsafe fn IsMultiTouch(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsMultiTouch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MaximumCursors(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).MaximumCursors)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablet3> for ::windows_core::IUnknown {
    fn from(value: IInkTablet3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablet3> for ::windows_core::IUnknown {
    fn from(value: &IInkTablet3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkTablet3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkTablet3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablet3> for ::win32_system::Com::IDispatch {
    fn from(value: IInkTablet3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablet3> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkTablet3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkTablet3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkTablet3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkTablet3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkTablet3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkTablet3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkTablet3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablet3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkTablet3 {
    type Vtable = IInkTablet3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e313997_1327_41dd_8ca9_79f24be17250);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablet3_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub IsMultiTouch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pismultitouch: *mut i16) -> ::windows_core::HRESULT,
    pub MaximumCursors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaximumcursors: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkTablets(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkTablets {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn DefaultTablet(&self) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultTablet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    pub unsafe fn IsPacketPropertySupported<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, packetpropertyname: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPacketPropertySupported)(::windows_core::Interface::as_raw(self), packetpropertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablets> for ::windows_core::IUnknown {
    fn from(value: IInkTablets) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablets> for ::windows_core::IUnknown {
    fn from(value: &IInkTablets) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkTablets {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkTablets {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTablets> for ::win32_system::Com::IDispatch {
    fn from(value: IInkTablets) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTablets> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkTablets) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkTablets {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkTablets {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkTablets {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkTablets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkTablets {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkTablets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTablets").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkTablets {
    type Vtable = IInkTablets_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x112086d9_7779_4535_a699_862b43ac1863);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablets_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub DefaultTablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaulttablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DefaultTablet: usize,
    #[cfg(feature = "win32-system")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, tablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Item: usize,
    pub IsPacketPropertySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, supported: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkTransform(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkTransform {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Translate(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Translate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalcomponent), ::core::mem::transmute(verticalcomponent)).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Rotate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(degrees), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn Reflect(&self, horizontally: i16, vertically: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reflect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontally), ::core::mem::transmute(vertically)).ok()
    }
    pub unsafe fn Shear(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shear)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalcomponent), ::core::mem::transmute(verticalcomponent)).ok()
    }
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScaleTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontalmultiplier), ::core::mem::transmute(verticalmultiplier)).ok()
    }
    pub unsafe fn GetTransform(&self, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(em11), ::core::mem::transmute(em12), ::core::mem::transmute(em21), ::core::mem::transmute(em22), ::core::mem::transmute(edx), ::core::mem::transmute(edy)).ok()
    }
    pub unsafe fn SetTransform(&self, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(em11), ::core::mem::transmute(em12), ::core::mem::transmute(em21), ::core::mem::transmute(em22), ::core::mem::transmute(edx), ::core::mem::transmute(edy)).ok()
    }
    pub unsafe fn eM11(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).eM11)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SeteM11(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeteM11)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn eM12(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).eM12)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SeteM12(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeteM12)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn eM21(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).eM21)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SeteM21(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeteM21)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn eM22(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).eM22)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SeteM22(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeteM22)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn eDx(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).eDx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SeteDx(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeteDx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn eDy(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).eDy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SeteDy(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeteDy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn Data(&self) -> ::windows_core::Result<::win32_graphics::Gdi::XFORM> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::XFORM>::zeroed();
        (::windows_core::Interface::vtable(self).Data)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::XFORM>(result__)
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::XFORM>>(&self, xform: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), xform.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTransform> for ::windows_core::IUnknown {
    fn from(value: IInkTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTransform> for ::windows_core::IUnknown {
    fn from(value: &IInkTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkTransform> for ::win32_system::Com::IDispatch {
    fn from(value: IInkTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkTransform> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkTransform {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkTransform").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkTransform {
    type Vtable = IInkTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x615f1d43_8703_4565_88e2_8201d2ecd7b7);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkTransform_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Translate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows_core::HRESULT,
    pub Reflect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontally: i16, vertically: i16) -> ::windows_core::HRESULT,
    pub Shear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows_core::HRESULT,
    pub eM11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows_core::HRESULT,
    pub SeteM11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub eM12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows_core::HRESULT,
    pub SeteM12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub eM21: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows_core::HRESULT,
    pub SeteM21: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub eM22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows_core::HRESULT,
    pub SeteM22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub eDx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows_core::HRESULT,
    pub SeteDx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub eDy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows_core::HRESULT,
    pub SeteDy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-graphics")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xform: *mut ::win32_graphics::Gdi::XFORM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-graphics"))]
    Data: usize,
    #[cfg(feature = "win32-graphics")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xform: ::win32_graphics::Gdi::XFORM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-graphics"))]
    SetData: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkWordList(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkWordList {
    pub unsafe fn AddWord<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newword: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddWord)(::windows_core::Interface::as_raw(self), newword.into_param().abi()).ok()
    }
    pub unsafe fn RemoveWord<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, removeword: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveWord)(::windows_core::Interface::as_raw(self), removeword.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Merge<'a, Param0: ::windows_core::IntoParam<'a, IInkWordList>>(&self, mergewordlist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Merge)(::windows_core::Interface::as_raw(self), mergewordlist.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkWordList> for ::windows_core::IUnknown {
    fn from(value: IInkWordList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkWordList> for ::windows_core::IUnknown {
    fn from(value: &IInkWordList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkWordList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkWordList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkWordList> for ::win32_system::Com::IDispatch {
    fn from(value: IInkWordList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkWordList> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkWordList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkWordList {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkWordList {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkWordList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkWordList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkWordList {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkWordList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkWordList").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkWordList {
    type Vtable = IInkWordList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76ba3491_cb2f_406b_9961_0e0c4cdaaef2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkWordList_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub AddWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, removeword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mergewordlist: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Merge: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IInkWordList2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IInkWordList2 {
    pub unsafe fn AddWords<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newwords: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddWords)(::windows_core::Interface::as_raw(self), newwords.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkWordList2> for ::windows_core::IUnknown {
    fn from(value: IInkWordList2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkWordList2> for ::windows_core::IUnknown {
    fn from(value: &IInkWordList2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkWordList2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkWordList2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IInkWordList2> for ::win32_system::Com::IDispatch {
    fn from(value: IInkWordList2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IInkWordList2> for ::win32_system::Com::IDispatch {
    fn from(value: &IInkWordList2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IInkWordList2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IInkWordList2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IInkWordList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IInkWordList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IInkWordList2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IInkWordList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkWordList2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IInkWordList2 {
    type Vtable = IInkWordList2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14542586_11bf_4f5f_b6e7_49d0744aab6e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IInkWordList2_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub AddWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newwords: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInputPanelWindowHandle(::windows_core::IUnknown);
impl IInputPanelWindowHandle {
    pub unsafe fn AttachedEditWindow32(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AttachedEditWindow32)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAttachedEditWindow32(&self, attachededitwindow: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachedEditWindow32)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(attachededitwindow)).ok()
    }
    pub unsafe fn AttachedEditWindow64(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        (::windows_core::Interface::vtable(self).AttachedEditWindow64)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn SetAttachedEditWindow64(&self, attachededitwindow: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachedEditWindow64)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(attachededitwindow)).ok()
    }
}
impl ::core::convert::From<IInputPanelWindowHandle> for ::windows_core::IUnknown {
    fn from(value: IInputPanelWindowHandle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInputPanelWindowHandle> for ::windows_core::IUnknown {
    fn from(value: &IInputPanelWindowHandle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInputPanelWindowHandle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInputPanelWindowHandle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInputPanelWindowHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInputPanelWindowHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPanelWindowHandle {}
impl ::core::fmt::Debug for IInputPanelWindowHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPanelWindowHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInputPanelWindowHandle {
    type Vtable = IInputPanelWindowHandle_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4af81847_fdc4_4fc3_ad0b_422479c1b935);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPanelWindowHandle_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AttachedEditWindow32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows_core::HRESULT,
    pub SetAttachedEditWindow32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows_core::HRESULT,
    pub AttachedEditWindow64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i64) -> ::windows_core::HRESULT,
    pub SetAttachedEditWindow64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: i64) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IMathInputControl(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IMathInputControl {
    pub unsafe fn Show(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Show)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Hide(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Hide)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsVisible(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsVisible)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetPosition(&self, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom)).ok()
    }
    pub unsafe fn SetPosition(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCustomPaint(&self, element: i32, paint: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCustomPaint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(element), ::core::mem::transmute(paint)).ok()
    }
    pub unsafe fn SetCaptionText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, captiontext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCaptionText)(::windows_core::Interface::as_raw(self), captiontext.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn LoadInk<'a, Param0: ::windows_core::IntoParam<'a, IInkDisp>>(&self, ink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadInk)(::windows_core::Interface::as_raw(self), ink.into_param().abi()).ok()
    }
    pub unsafe fn SetOwnerWindow(&self, ownerwindow: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOwnerWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ownerwindow)).ok()
    }
    pub unsafe fn EnableExtendedButtons(&self, extended: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableExtendedButtons)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(extended)).ok()
    }
    pub unsafe fn GetPreviewHeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPreviewHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreviewHeight(&self, height: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreviewHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(height)).ok()
    }
    pub unsafe fn EnableAutoGrow(&self, autogrow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableAutoGrow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(autogrow)).ok()
    }
    pub unsafe fn AddFunctionName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, functionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddFunctionName)(::windows_core::Interface::as_raw(self), functionname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveFunctionName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, functionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveFunctionName)(::windows_core::Interface::as_raw(self), functionname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetHoverIcon(&self) -> ::windows_core::Result<::win32_system::Ole::IPictureDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetHoverIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Ole::IPictureDisp>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMathInputControl> for ::windows_core::IUnknown {
    fn from(value: IMathInputControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMathInputControl> for ::windows_core::IUnknown {
    fn from(value: &IMathInputControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMathInputControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMathInputControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IMathInputControl> for ::win32_system::Com::IDispatch {
    fn from(value: IMathInputControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IMathInputControl> for ::win32_system::Com::IDispatch {
    fn from(value: &IMathInputControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IMathInputControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IMathInputControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IMathInputControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IMathInputControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IMathInputControl {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IMathInputControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMathInputControl").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IMathInputControl {
    type Vtable = IMathInputControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba615aa_fac6_4738_ba5f_ff09e9fe473e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IMathInputControl_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbshown: *mut i16) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCustomPaint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: i32, paint: i16) -> ::windows_core::HRESULT,
    pub SetCaptionText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captiontext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub LoadInk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    LoadInk: usize,
    pub SetOwnerWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownerwindow: isize) -> ::windows_core::HRESULT,
    pub EnableExtendedButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extended: i16) -> ::windows_core::HRESULT,
    pub GetPreviewHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows_core::HRESULT,
    pub SetPreviewHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, height: i32) -> ::windows_core::HRESULT,
    pub EnableAutoGrow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autogrow: i16) -> ::windows_core::HRESULT,
    pub AddFunctionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveFunctionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetHoverIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hoverimage: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetHoverIcon: usize,
}
pub const INKEDIT_CLASS: &str = "INKEDIT";
pub const INKEDIT_CLASSW: &str = "INKEDIT";
#[repr(C)]
pub struct INKMETRIC {
    pub iHeight: i32,
    pub iFontAscent: i32,
    pub iFontDescent: i32,
    pub dwFlags: u32,
    pub color: u32,
}
impl ::core::marker::Copy for INKMETRIC {}
impl ::core::clone::Clone for INKMETRIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INKMETRIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INKMETRIC").field("iHeight", &self.iHeight).field("iFontAscent", &self.iFontAscent).field("iFontDescent", &self.iFontDescent).field("dwFlags", &self.dwFlags).field("color", &self.color).finish()
    }
}
unsafe impl ::windows_core::Abi for INKMETRIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INKMETRIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INKMETRIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for INKMETRIC {}
impl ::core::default::Default for INKMETRIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const INKRECOGNITIONPROPERTY_BOXNUMBER: &str = "{2C243E3A-F733-4EB6-B1F8-B5DC5C2C4CDA}";
pub const INKRECOGNITIONPROPERTY_CONFIDENCELEVEL: &str = "{7DFE11A7-FB5D-4958-8765-154ADF0D833F}";
pub const INKRECOGNITIONPROPERTY_HOTPOINT: &str = "{CA6F40DC-5292-452a-91FB-2181C0BEC0DE}";
pub const INKRECOGNITIONPROPERTY_LINEMETRICS: &str = "{8CC24B27-30A9-4b96-9056-2D3A90DA0727}";
pub const INKRECOGNITIONPROPERTY_LINENUMBER: &str = "{DBF29F2C-5289-4BE8-B3D8-6EF63246253E}";
pub const INKRECOGNITIONPROPERTY_MAXIMUMSTROKECOUNT: &str = "{BF0EEC4E-4B7D-47a9-8CFA-234DD24BD22A}";
pub const INKRECOGNITIONPROPERTY_POINTSPERINCH: &str = "{7ED16B76-889C-468e-8276-0021B770187E}";
pub const INKRECOGNITIONPROPERTY_SEGMENTATION: &str = "{B3C0FE6C-FB51-4164-BA2F-844AF8F983DA}";
pub const INK_SERIALIZED_FORMAT: &str = "Ink Serialized Format";
pub const IP_CURSOR_DOWN: u32 = 1u32;
pub const IP_INVERTED: u32 = 2u32;
pub const IP_MARGIN: u32 = 4u32;
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IPenInputPanel(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IPenInputPanel {
    pub unsafe fn Busy(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Busy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Factoid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Factoid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFactoid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, factoid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFactoid)(::windows_core::Interface::as_raw(self), factoid.into_param().abi()).ok()
    }
    pub unsafe fn AttachedEditWindow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AttachedEditWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAttachedEditWindow(&self, attachededitwindow: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachedEditWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(attachededitwindow)).ok()
    }
    pub unsafe fn CurrentPanel(&self) -> ::windows_core::Result<PanelType> {
        let mut result__ = ::core::mem::MaybeUninit::<PanelType>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPanel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PanelType>(result__)
    }
    pub unsafe fn SetCurrentPanel(&self, currentpanel: PanelType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCurrentPanel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(currentpanel)).ok()
    }
    pub unsafe fn DefaultPanel(&self) -> ::windows_core::Result<PanelType> {
        let mut result__ = ::core::mem::MaybeUninit::<PanelType>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultPanel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PanelType>(result__)
    }
    pub unsafe fn SetDefaultPanel(&self, defaultpanel: PanelType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultPanel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(defaultpanel)).ok()
    }
    pub unsafe fn Visible(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Visible)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetVisible(&self, visible: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVisible)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(visible)).ok()
    }
    pub unsafe fn Top(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Top)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Left(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Left)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Width(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Width)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Height(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Height)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn VerticalOffset(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).VerticalOffset)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetVerticalOffset(&self, verticaloffset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVerticalOffset)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(verticaloffset)).ok()
    }
    pub unsafe fn HorizontalOffset(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HorizontalOffset)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHorizontalOffset(&self, horizontaloffset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHorizontalOffset)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(horizontaloffset)).ok()
    }
    pub unsafe fn AutoShow(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoShow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAutoShow(&self, autoshow: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoShow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(autoshow)).ok()
    }
    pub unsafe fn MoveTo(&self, left: i32, top: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(left), ::core::mem::transmute(top)).ok()
    }
    pub unsafe fn CommitPendingInput(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitPendingInput)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableTsf(&self, enable: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableTsf)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enable)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPenInputPanel> for ::windows_core::IUnknown {
    fn from(value: IPenInputPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPenInputPanel> for ::windows_core::IUnknown {
    fn from(value: &IPenInputPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPenInputPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPenInputPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IPenInputPanel> for ::win32_system::Com::IDispatch {
    fn from(value: IPenInputPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IPenInputPanel> for ::win32_system::Com::IDispatch {
    fn from(value: &IPenInputPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IPenInputPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IPenInputPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IPenInputPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IPenInputPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IPenInputPanel {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IPenInputPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPenInputPanel").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IPenInputPanel {
    type Vtable = IPenInputPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa7a4083_5747_4040_a182_0b0e9fd4fac7);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IPenInputPanel_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Busy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busy: *mut i16) -> ::windows_core::HRESULT,
    pub Factoid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factoid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFactoid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AttachedEditWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows_core::HRESULT,
    pub SetAttachedEditWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows_core::HRESULT,
    pub CurrentPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpanel: *mut PanelType) -> ::windows_core::HRESULT,
    pub SetCurrentPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpanel: PanelType) -> ::windows_core::HRESULT,
    pub DefaultPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefaultpanel: *mut PanelType) -> ::windows_core::HRESULT,
    pub SetDefaultPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaultpanel: PanelType) -> ::windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: *mut i16) -> ::windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: i16) -> ::windows_core::HRESULT,
    pub Top: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, verticaloffset: *mut i32) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, verticaloffset: i32) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontaloffset: *mut i32) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontaloffset: i32) -> ::windows_core::HRESULT,
    pub AutoShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pautoshow: *mut i16) -> ::windows_core::HRESULT,
    pub SetAutoShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoshow: i16) -> ::windows_core::HRESULT,
    pub MoveTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: i32, top: i32) -> ::windows_core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableTsf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRealTimeStylus(::windows_core::IUnknown);
impl IRealTimeStylus {
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn HWND(&self) -> ::windows_core::Result<::win32_foundation::HANDLE_PTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE_PTR>::zeroed();
        (::windows_core::Interface::vtable(self).HWND)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE_PTR>(result__)
    }
    pub unsafe fn SetHWND<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE_PTR>>(&self, hwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHWND)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn WindowInputRectangle(&self) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).WindowInputRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn SetWindowInputRectangle(&self, prcwndinputrect: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWindowInputRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prcwndinputrect)).ok()
    }
    pub unsafe fn AddStylusSyncPlugin<'a, Param1: ::windows_core::IntoParam<'a, IStylusSyncPlugin>>(&self, iindex: u32, piplugin: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStylusSyncPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), piplugin.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStylusSyncPlugin(&self, iindex: u32, ppiplugin: *mut ::core::option::Option<IStylusSyncPlugin>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStylusSyncPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ::core::mem::transmute(ppiplugin)).ok()
    }
    pub unsafe fn RemoveAllStylusSyncPlugins(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllStylusSyncPlugins)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetStylusSyncPlugin(&self, iindex: u32) -> ::windows_core::Result<IStylusSyncPlugin> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStylusSyncPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStylusSyncPlugin>(result__)
    }
    pub unsafe fn GetStylusSyncPluginCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetStylusSyncPluginCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn AddStylusAsyncPlugin<'a, Param1: ::windows_core::IntoParam<'a, IStylusAsyncPlugin>>(&self, iindex: u32, piplugin: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStylusAsyncPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), piplugin.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStylusAsyncPlugin(&self, iindex: u32, ppiplugin: *mut ::core::option::Option<IStylusAsyncPlugin>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStylusAsyncPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ::core::mem::transmute(ppiplugin)).ok()
    }
    pub unsafe fn RemoveAllStylusAsyncPlugins(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllStylusAsyncPlugins)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetStylusAsyncPlugin(&self, iindex: u32) -> ::windows_core::Result<IStylusAsyncPlugin> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStylusAsyncPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStylusAsyncPlugin>(result__)
    }
    pub unsafe fn GetStylusAsyncPluginCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetStylusAsyncPluginCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ChildRealTimeStylusPlugin(&self) -> ::windows_core::Result<IRealTimeStylus> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ChildRealTimeStylusPlugin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRealTimeStylus>(result__)
    }
    pub unsafe fn putref_ChildRealTimeStylusPlugin<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirts: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_ChildRealTimeStylusPlugin)(::windows_core::Interface::as_raw(self), pirts.into_param().abi()).ok()
    }
    pub unsafe fn AddCustomStylusDataToQueue(&self, sq: StylusQueue, pguidid: *const ::windows_core::GUID, pbdata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddCustomStylusDataToQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sq), ::core::mem::transmute(pguidid), pbdata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbdata))).ok()
    }
    pub unsafe fn ClearStylusQueues(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearStylusQueues)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAllTabletsMode<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fusemouseforinput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllTabletsMode)(::windows_core::Interface::as_raw(self), fusemouseforinput.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSingleTabletMode<'a, Param0: ::windows_core::IntoParam<'a, IInkTablet>>(&self, pitablet: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSingleTabletMode)(::windows_core::Interface::as_raw(self), pitablet.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTablet(&self) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTablet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTabletContextIdFromTablet<'a, Param0: ::windows_core::IntoParam<'a, IInkTablet>>(&self, pitablet: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetTabletContextIdFromTablet)(::windows_core::Interface::as_raw(self), pitablet.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTabletFromTabletContextId(&self, tcid: u32) -> ::windows_core::Result<IInkTablet> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTabletFromTabletContextId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(tcid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkTablet>(result__)
    }
    pub unsafe fn GetAllTabletContextIds(&self, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAllTabletContextIds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pctcidcount), ::core::mem::transmute(pptcids)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetStyluses(&self) -> ::windows_core::Result<IInkCursors> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStyluses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursors>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetStylusForId(&self, sid: u32) -> ::windows_core::Result<IInkCursor> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStylusForId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkCursor>(result__)
    }
    pub unsafe fn SetDesiredPacketDescription(&self, ppropertyguids: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDesiredPacketDescription)(::windows_core::Interface::as_raw(self), ppropertyguids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppropertyguids))).ok()
    }
    pub unsafe fn GetDesiredPacketDescription(&self, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDesiredPacketDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcproperties), ::core::mem::transmute(pppropertyguids)).ok()
    }
    pub unsafe fn GetPacketDescriptionData(&self, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPacketDescriptionData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(tcid), ::core::mem::transmute(pfinktodevicescalex), ::core::mem::transmute(pfinktodevicescaley), ::core::mem::transmute(pcpacketproperties), ::core::mem::transmute(pppacketproperties)).ok()
    }
}
impl ::core::convert::From<IRealTimeStylus> for ::windows_core::IUnknown {
    fn from(value: IRealTimeStylus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRealTimeStylus> for ::windows_core::IUnknown {
    fn from(value: &IRealTimeStylus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRealTimeStylus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRealTimeStylus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRealTimeStylus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylus {}
impl ::core::fmt::Debug for IRealTimeStylus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRealTimeStylus {
    type Vtable = IRealTimeStylus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8bb5d22_3144_4a7b_93cd_f34a16be513a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylus_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenable: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub HWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    pub SetHWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    pub WindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcwndinputrect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcwndinputrect: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub AddStylusSyncPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveStylusSyncPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAllStylusSyncPlugins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStylusSyncPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStylusSyncPluginCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows_core::HRESULT,
    pub AddStylusAsyncPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveStylusAsyncPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAllStylusAsyncPlugins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStylusAsyncPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStylusAsyncPluginCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows_core::HRESULT,
    pub ChildRealTimeStylusPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppirts: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub putref_ChildRealTimeStylusPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirts: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddCustomStylusDataToQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sq: StylusQueue, pguidid: *const ::windows_core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows_core::HRESULT,
    pub ClearStylusQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fusemouseforinput: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SetSingleTabletMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitablet: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSingleTabletMode: usize,
    #[cfg(feature = "win32-system")]
    pub GetTablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppisingletablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTablet: usize,
    #[cfg(feature = "win32-system")]
    pub GetTabletContextIdFromTablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitablet: ::windows_core::RawPtr, ptcid: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTabletContextIdFromTablet: usize,
    #[cfg(feature = "win32-system")]
    pub GetTabletFromTabletContextId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tcid: u32, ppitablet: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTabletFromTabletContextId: usize,
    pub GetAllTabletContextIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetStyluses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiinkcursors: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetStyluses: usize,
    #[cfg(feature = "win32-system")]
    pub GetStylusForId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sid: u32, ppiinkcursor: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetStylusForId: usize,
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropertyguids: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetDesiredPacketDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetPacketDescriptionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRealTimeStylus2(::windows_core::IUnknown);
impl IRealTimeStylus2 {
    pub unsafe fn FlicksEnabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).FlicksEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetFlicksEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlicksEnabled)(::windows_core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRealTimeStylus2> for ::windows_core::IUnknown {
    fn from(value: IRealTimeStylus2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRealTimeStylus2> for ::windows_core::IUnknown {
    fn from(value: &IRealTimeStylus2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRealTimeStylus2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRealTimeStylus2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRealTimeStylus2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylus2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylus2 {}
impl ::core::fmt::Debug for IRealTimeStylus2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylus2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRealTimeStylus2 {
    type Vtable = IRealTimeStylus2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5f2a6cd_3179_4a3e_b9c4_bb5865962be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylus2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub FlicksEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenable: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetFlicksEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRealTimeStylus3(::windows_core::IUnknown);
impl IRealTimeStylus3 {
    pub unsafe fn MultiTouchEnabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).MultiTouchEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetMultiTouchEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultiTouchEnabled)(::windows_core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRealTimeStylus3> for ::windows_core::IUnknown {
    fn from(value: IRealTimeStylus3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRealTimeStylus3> for ::windows_core::IUnknown {
    fn from(value: &IRealTimeStylus3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRealTimeStylus3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRealTimeStylus3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRealTimeStylus3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylus3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylus3 {}
impl ::core::fmt::Debug for IRealTimeStylus3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylus3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRealTimeStylus3 {
    type Vtable = IRealTimeStylus3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd70230a3_6986_4051_b57a_1cf69f4d9db5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylus3_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub MultiTouchEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenable: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetMultiTouchEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRealTimeStylusSynchronization(::windows_core::IUnknown);
impl IRealTimeStylusSynchronization {
    pub unsafe fn AcquireLock(&self, lock: RealTimeStylusLockType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireLock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lock)).ok()
    }
    pub unsafe fn ReleaseLock(&self, lock: RealTimeStylusLockType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseLock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lock)).ok()
    }
}
impl ::core::convert::From<IRealTimeStylusSynchronization> for ::windows_core::IUnknown {
    fn from(value: IRealTimeStylusSynchronization) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRealTimeStylusSynchronization> for ::windows_core::IUnknown {
    fn from(value: &IRealTimeStylusSynchronization) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRealTimeStylusSynchronization {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRealTimeStylusSynchronization {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRealTimeStylusSynchronization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRealTimeStylusSynchronization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRealTimeStylusSynchronization {}
impl ::core::fmt::Debug for IRealTimeStylusSynchronization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRealTimeStylusSynchronization").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRealTimeStylusSynchronization {
    type Vtable = IRealTimeStylusSynchronization_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa87eab8_ab4a_4cea_b5cb_46d84c6a2509);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylusSynchronization_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AcquireLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows_core::HRESULT,
    pub ReleaseLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct ISketchInk(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl ISketchInk {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISketchInk> for ::windows_core::IUnknown {
    fn from(value: ISketchInk) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISketchInk> for ::windows_core::IUnknown {
    fn from(value: &ISketchInk) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISketchInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISketchInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<ISketchInk> for ::win32_system::Com::IDispatch {
    fn from(value: ISketchInk) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&ISketchInk> for ::win32_system::Com::IDispatch {
    fn from(value: &ISketchInk) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for ISketchInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a ISketchInk {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for ISketchInk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for ISketchInk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for ISketchInk {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for ISketchInk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISketchInk").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for ISketchInk {
    type Vtable = ISketchInk_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4563688_98eb_4646_b279_44da14d45748);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct ISketchInk_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[repr(transparent)]
pub struct IStrokeBuilder(::windows_core::IUnknown);
impl IStrokeBuilder {
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateStroke(&self, ppackets: &[i32], ppacketproperties: &[PACKET_PROPERTY], finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateStroke)(::windows_core::Interface::as_raw(self), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ppacketproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacketproperties)), ::core::mem::transmute(finktodevicescalex), ::core::mem::transmute(finktodevicescaley), ::core::mem::transmute(ppiinkstroke)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn BeginStroke(&self, tcid: u32, sid: u32, ppacket: *const i32, ppacketproperties: &[PACKET_PROPERTY], finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginStroke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(tcid), ::core::mem::transmute(sid), ::core::mem::transmute(ppacket), ppacketproperties.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacketproperties)), ::core::mem::transmute(finktodevicescalex), ::core::mem::transmute(finktodevicescaley), ::core::mem::transmute(ppiinkstroke)).ok()
    }
    pub unsafe fn AppendPackets(&self, tcid: u32, sid: u32, ppackets: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AppendPackets)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(tcid), ::core::mem::transmute(sid), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets))).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EndStroke(&self, tcid: u32, sid: u32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>, pdirtyrect: *mut ::win32_foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndStroke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(tcid), ::core::mem::transmute(sid), ::core::mem::transmute(ppiinkstroke), ::core::mem::transmute(pdirtyrect)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Ink(&self) -> ::windows_core::Result<IInkDisp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Ink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInkDisp>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn putref_Ink<'a, Param0: ::windows_core::IntoParam<'a, IInkDisp>>(&self, piinkobj: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).putref_Ink)(::windows_core::Interface::as_raw(self), piinkobj.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IStrokeBuilder> for ::windows_core::IUnknown {
    fn from(value: IStrokeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStrokeBuilder> for ::windows_core::IUnknown {
    fn from(value: &IStrokeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStrokeBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStrokeBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStrokeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStrokeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStrokeBuilder {}
impl ::core::fmt::Debug for IStrokeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStrokeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStrokeBuilder {
    type Vtable = IStrokeBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5fd4e2d_c44b_4092_9177_260905eb672b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStrokeBuilder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-system")]
    pub CreateStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateStroke: usize,
    #[cfg(feature = "win32-system")]
    pub BeginStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BeginStroke: usize,
    pub AppendPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub EndStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppiinkstroke: *mut ::windows_core::RawPtr, pdirtyrect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EndStroke: usize,
    #[cfg(feature = "win32-system")]
    pub Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiinkobj: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Ink: usize,
    #[cfg(feature = "win32-system")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piinkobj: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    putref_Ink: usize,
}
#[repr(transparent)]
pub struct IStylusAsyncPlugin(::windows_core::IUnknown);
impl IStylusAsyncPlugin {
    pub unsafe fn RealTimeStylusEnabled<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ptcids: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RealTimeStylusEnabled)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ptcids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ptcids))).ok()
    }
    pub unsafe fn RealTimeStylusDisabled<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ptcids: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RealTimeStylusDisabled)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ptcids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ptcids))).ok()
    }
    pub unsafe fn StylusInRange<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusInRange)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid)).ok()
    }
    pub unsafe fn StylusOutOfRange<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusOutOfRange)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid)).ok()
    }
    pub unsafe fn StylusDown<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusDown)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ppacket.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacket)), ::core::mem::transmute(ppinoutpkt)).ok()
    }
    pub unsafe fn StylusUp<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusUp)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ppacket.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacket)), ::core::mem::transmute(ppinoutpkt)).ok()
    }
    pub unsafe fn StylusButtonDown<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusButtonDown)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(sid), ::core::mem::transmute(pguidstylusbutton), ::core::mem::transmute(pstyluspos)).ok()
    }
    pub unsafe fn StylusButtonUp<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusButtonUp)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(sid), ::core::mem::transmute(pguidstylusbutton), ::core::mem::transmute(pstyluspos)).ok()
    }
    pub unsafe fn InAirPackets<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InAirPackets)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ::core::mem::transmute(cpktcount), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ::core::mem::transmute(pcinoutpkts), ::core::mem::transmute(ppinoutpkts)).ok()
    }
    pub unsafe fn Packets<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Packets)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ::core::mem::transmute(cpktcount), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ::core::mem::transmute(pcinoutpkts), ::core::mem::transmute(ppinoutpkts)).ok()
    }
    pub unsafe fn CustomStylusDataAdded<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pguidid: *const ::windows_core::GUID, pbdata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CustomStylusDataAdded)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pguidid), pbdata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbdata))).ok()
    }
    pub unsafe fn SystemEvent<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param4: ::windows_core::IntoParam<'a, SYSTEM_EVENT_DATA>>(&self, pirtssrc: Param0, tcid: u32, sid: u32, event: u16, eventdata: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SystemEvent)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid), ::core::mem::transmute(event), eventdata.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TabletAdded<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param1: ::windows_core::IntoParam<'a, IInkTablet>>(&self, pirtssrc: Param0, pitablet: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TabletAdded)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), pitablet.into_param().abi()).ok()
    }
    pub unsafe fn TabletRemoved<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, itabletindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TabletRemoved)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(itabletindex)).ok()
    }
    pub unsafe fn Error<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param1: ::windows_core::IntoParam<'a, IStylusPlugin>>(&self, pirtssrc: Param0, piplugin: Param1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows_core::HRESULT, lptrkey: *mut isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Error)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), piplugin.into_param().abi(), ::core::mem::transmute(datainterest), ::core::mem::transmute(hrerrorcode), ::core::mem::transmute(lptrkey)).ok()
    }
    pub unsafe fn UpdateMapping<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UpdateMapping)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi()).ok()
    }
    pub unsafe fn DataInterest(&self) -> ::windows_core::Result<RealTimeStylusDataInterest> {
        let mut result__ = ::core::mem::MaybeUninit::<RealTimeStylusDataInterest>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DataInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RealTimeStylusDataInterest>(result__)
    }
}
impl ::core::convert::From<IStylusAsyncPlugin> for ::windows_core::IUnknown {
    fn from(value: IStylusAsyncPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylusAsyncPlugin> for ::windows_core::IUnknown {
    fn from(value: &IStylusAsyncPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStylusAsyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStylusAsyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStylusAsyncPlugin> for IStylusPlugin {
    fn from(value: IStylusAsyncPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylusAsyncPlugin> for IStylusPlugin {
    fn from(value: &IStylusAsyncPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStylusPlugin> for IStylusAsyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, IStylusPlugin> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStylusPlugin> for &'a IStylusAsyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, IStylusPlugin> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStylusAsyncPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStylusAsyncPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylusAsyncPlugin {}
impl ::core::fmt::Debug for IStylusAsyncPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylusAsyncPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStylusAsyncPlugin {
    type Vtable = IStylusAsyncPlugin_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7cca85a_31bc_4cd2_aadc_3289a3af11c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylusAsyncPlugin_Vtbl {
    pub base__: IStylusPlugin_Vtbl,
}
#[repr(transparent)]
pub struct IStylusPlugin(::windows_core::IUnknown);
impl IStylusPlugin {
    pub unsafe fn RealTimeStylusEnabled<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ptcids: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RealTimeStylusEnabled)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ptcids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ptcids))).ok()
    }
    pub unsafe fn RealTimeStylusDisabled<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ptcids: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RealTimeStylusDisabled)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ptcids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ptcids))).ok()
    }
    pub unsafe fn StylusInRange<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StylusInRange)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid)).ok()
    }
    pub unsafe fn StylusOutOfRange<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StylusOutOfRange)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid)).ok()
    }
    pub unsafe fn StylusDown<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StylusDown)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ppacket.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacket)), ::core::mem::transmute(ppinoutpkt)).ok()
    }
    pub unsafe fn StylusUp<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StylusUp)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ppacket.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacket)), ::core::mem::transmute(ppinoutpkt)).ok()
    }
    pub unsafe fn StylusButtonDown<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StylusButtonDown)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(sid), ::core::mem::transmute(pguidstylusbutton), ::core::mem::transmute(pstyluspos)).ok()
    }
    pub unsafe fn StylusButtonUp<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StylusButtonUp)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(sid), ::core::mem::transmute(pguidstylusbutton), ::core::mem::transmute(pstyluspos)).ok()
    }
    pub unsafe fn InAirPackets<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InAirPackets)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ::core::mem::transmute(cpktcount), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ::core::mem::transmute(pcinoutpkts), ::core::mem::transmute(ppinoutpkts)).ok()
    }
    pub unsafe fn Packets<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Packets)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ::core::mem::transmute(cpktcount), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ::core::mem::transmute(pcinoutpkts), ::core::mem::transmute(ppinoutpkts)).ok()
    }
    pub unsafe fn CustomStylusDataAdded<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pguidid: *const ::windows_core::GUID, pbdata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CustomStylusDataAdded)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pguidid), pbdata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbdata))).ok()
    }
    pub unsafe fn SystemEvent<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param4: ::windows_core::IntoParam<'a, SYSTEM_EVENT_DATA>>(&self, pirtssrc: Param0, tcid: u32, sid: u32, event: u16, eventdata: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SystemEvent)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid), ::core::mem::transmute(event), eventdata.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TabletAdded<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param1: ::windows_core::IntoParam<'a, IInkTablet>>(&self, pirtssrc: Param0, pitablet: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TabletAdded)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), pitablet.into_param().abi()).ok()
    }
    pub unsafe fn TabletRemoved<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, itabletindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TabletRemoved)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(itabletindex)).ok()
    }
    pub unsafe fn Error<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param1: ::windows_core::IntoParam<'a, IStylusPlugin>>(&self, pirtssrc: Param0, piplugin: Param1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows_core::HRESULT, lptrkey: *mut isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Error)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), piplugin.into_param().abi(), ::core::mem::transmute(datainterest), ::core::mem::transmute(hrerrorcode), ::core::mem::transmute(lptrkey)).ok()
    }
    pub unsafe fn UpdateMapping<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateMapping)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi()).ok()
    }
    pub unsafe fn DataInterest(&self) -> ::windows_core::Result<RealTimeStylusDataInterest> {
        let mut result__ = ::core::mem::MaybeUninit::<RealTimeStylusDataInterest>::zeroed();
        (::windows_core::Interface::vtable(self).DataInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RealTimeStylusDataInterest>(result__)
    }
}
impl ::core::convert::From<IStylusPlugin> for ::windows_core::IUnknown {
    fn from(value: IStylusPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylusPlugin> for ::windows_core::IUnknown {
    fn from(value: &IStylusPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStylusPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStylusPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStylusPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStylusPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylusPlugin {}
impl ::core::fmt::Debug for IStylusPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylusPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStylusPlugin {
    type Vtable = IStylusPlugin_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa81436d8_4757_4fd1_a185_133f97c6c545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylusPlugin_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RealTimeStylusEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows_core::HRESULT,
    pub RealTimeStylusDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows_core::HRESULT,
    pub StylusInRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, tcid: u32, sid: u32) -> ::windows_core::HRESULT,
    pub StylusOutOfRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, tcid: u32, sid: u32) -> ::windows_core::HRESULT,
    pub StylusDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows_core::HRESULT,
    pub StylusUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows_core::HRESULT,
    pub StylusButtonDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT,
    pub StylusButtonUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT,
    pub InAirPackets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::HRESULT,
    pub Packets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::HRESULT,
    pub CustomStylusDataAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, pguidid: *const ::windows_core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows_core::HRESULT,
    pub SystemEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub TabletAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, pitablet: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TabletAdded: usize,
    pub TabletRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, itabletindex: i32) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr, piplugin: ::windows_core::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows_core::HRESULT, lptrkey: *mut isize) -> ::windows_core::HRESULT,
    pub UpdateMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirtssrc: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DataInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IStylusSyncPlugin(::windows_core::IUnknown);
impl IStylusSyncPlugin {
    pub unsafe fn RealTimeStylusEnabled<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ptcids: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RealTimeStylusEnabled)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ptcids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ptcids))).ok()
    }
    pub unsafe fn RealTimeStylusDisabled<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ptcids: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RealTimeStylusDisabled)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ptcids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ptcids))).ok()
    }
    pub unsafe fn StylusInRange<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusInRange)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid)).ok()
    }
    pub unsafe fn StylusOutOfRange<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusOutOfRange)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid)).ok()
    }
    pub unsafe fn StylusDown<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusDown)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ppacket.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacket)), ::core::mem::transmute(ppinoutpkt)).ok()
    }
    pub unsafe fn StylusUp<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, ppacket: &[i32], ppinoutpkt: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusUp)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ppacket.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppacket)), ::core::mem::transmute(ppinoutpkt)).ok()
    }
    pub unsafe fn StylusButtonDown<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusButtonDown)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(sid), ::core::mem::transmute(pguidstylusbutton), ::core::mem::transmute(pstyluspos)).ok()
    }
    pub unsafe fn StylusButtonUp<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows_core::GUID, pstyluspos: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StylusButtonUp)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(sid), ::core::mem::transmute(pguidstylusbutton), ::core::mem::transmute(pstyluspos)).ok()
    }
    pub unsafe fn InAirPackets<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InAirPackets)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ::core::mem::transmute(cpktcount), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ::core::mem::transmute(pcinoutpkts), ::core::mem::transmute(ppinoutpkts)).ok()
    }
    pub unsafe fn Packets<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, ppackets: &[i32], pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Packets)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pstylusinfo), ::core::mem::transmute(cpktcount), ppackets.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(ppackets)), ::core::mem::transmute(pcinoutpkts), ::core::mem::transmute(ppinoutpkts)).ok()
    }
    pub unsafe fn CustomStylusDataAdded<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pguidid: *const ::windows_core::GUID, pbdata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CustomStylusDataAdded)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(pguidid), pbdata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbdata))).ok()
    }
    pub unsafe fn SystemEvent<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param4: ::windows_core::IntoParam<'a, SYSTEM_EVENT_DATA>>(&self, pirtssrc: Param0, tcid: u32, sid: u32, event: u16, eventdata: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SystemEvent)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(tcid), ::core::mem::transmute(sid), ::core::mem::transmute(event), eventdata.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TabletAdded<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param1: ::windows_core::IntoParam<'a, IInkTablet>>(&self, pirtssrc: Param0, pitablet: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TabletAdded)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), pitablet.into_param().abi()).ok()
    }
    pub unsafe fn TabletRemoved<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, itabletindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TabletRemoved)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), ::core::mem::transmute(itabletindex)).ok()
    }
    pub unsafe fn Error<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>, Param1: ::windows_core::IntoParam<'a, IStylusPlugin>>(&self, pirtssrc: Param0, piplugin: Param1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows_core::HRESULT, lptrkey: *mut isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Error)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi(), piplugin.into_param().abi(), ::core::mem::transmute(datainterest), ::core::mem::transmute(hrerrorcode), ::core::mem::transmute(lptrkey)).ok()
    }
    pub unsafe fn UpdateMapping<'a, Param0: ::windows_core::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UpdateMapping)(::windows_core::Interface::as_raw(self), pirtssrc.into_param().abi()).ok()
    }
    pub unsafe fn DataInterest(&self) -> ::windows_core::Result<RealTimeStylusDataInterest> {
        let mut result__ = ::core::mem::MaybeUninit::<RealTimeStylusDataInterest>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DataInterest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RealTimeStylusDataInterest>(result__)
    }
}
impl ::core::convert::From<IStylusSyncPlugin> for ::windows_core::IUnknown {
    fn from(value: IStylusSyncPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylusSyncPlugin> for ::windows_core::IUnknown {
    fn from(value: &IStylusSyncPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStylusSyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStylusSyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStylusSyncPlugin> for IStylusPlugin {
    fn from(value: IStylusSyncPlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylusSyncPlugin> for IStylusPlugin {
    fn from(value: &IStylusSyncPlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStylusPlugin> for IStylusSyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, IStylusPlugin> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStylusPlugin> for &'a IStylusSyncPlugin {
    fn into_param(self) -> ::windows_core::Param<'a, IStylusPlugin> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStylusSyncPlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStylusSyncPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylusSyncPlugin {}
impl ::core::fmt::Debug for IStylusSyncPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylusSyncPlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStylusSyncPlugin {
    type Vtable = IStylusSyncPlugin_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa157b174_482f_4d71_a3f6_3a41ddd11be9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylusSyncPlugin_Vtbl {
    pub base__: IStylusPlugin_Vtbl,
}
#[repr(transparent)]
pub struct ITextInputPanel(::windows_core::IUnknown);
impl ITextInputPanel {
    pub unsafe fn AttachedEditWindow(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).AttachedEditWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
    pub unsafe fn SetAttachedEditWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, attachededitwindow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachedEditWindow)(::windows_core::Interface::as_raw(self), attachededitwindow.into_param().abi()).ok()
    }
    pub unsafe fn CurrentInteractionMode(&self) -> ::windows_core::Result<InteractionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<InteractionMode>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentInteractionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InteractionMode>(result__)
    }
    pub unsafe fn DefaultInPlaceState(&self) -> ::windows_core::Result<InPlaceState> {
        let mut result__ = ::core::mem::MaybeUninit::<InPlaceState>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultInPlaceState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InPlaceState>(result__)
    }
    pub unsafe fn SetDefaultInPlaceState(&self, state: InPlaceState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultInPlaceState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(state)).ok()
    }
    pub unsafe fn CurrentInPlaceState(&self) -> ::windows_core::Result<InPlaceState> {
        let mut result__ = ::core::mem::MaybeUninit::<InPlaceState>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentInPlaceState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InPlaceState>(result__)
    }
    pub unsafe fn DefaultInputArea(&self) -> ::windows_core::Result<PanelInputArea> {
        let mut result__ = ::core::mem::MaybeUninit::<PanelInputArea>::zeroed();
        (::windows_core::Interface::vtable(self).DefaultInputArea)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PanelInputArea>(result__)
    }
    pub unsafe fn SetDefaultInputArea(&self, area: PanelInputArea) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultInputArea)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(area)).ok()
    }
    pub unsafe fn CurrentInputArea(&self) -> ::windows_core::Result<PanelInputArea> {
        let mut result__ = ::core::mem::MaybeUninit::<PanelInputArea>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentInputArea)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PanelInputArea>(result__)
    }
    pub unsafe fn CurrentCorrectionMode(&self) -> ::windows_core::Result<CorrectionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<CorrectionMode>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentCorrectionMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CorrectionMode>(result__)
    }
    pub unsafe fn PreferredInPlaceDirection(&self) -> ::windows_core::Result<InPlaceDirection> {
        let mut result__ = ::core::mem::MaybeUninit::<InPlaceDirection>::zeroed();
        (::windows_core::Interface::vtable(self).PreferredInPlaceDirection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InPlaceDirection>(result__)
    }
    pub unsafe fn SetPreferredInPlaceDirection(&self, direction: InPlaceDirection) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreferredInPlaceDirection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(direction)).ok()
    }
    pub unsafe fn ExpandPostInsertionCorrection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).ExpandPostInsertionCorrection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetExpandPostInsertionCorrection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, expand: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExpandPostInsertionCorrection)(::windows_core::Interface::as_raw(self), expand.into_param().abi()).ok()
    }
    pub unsafe fn InPlaceVisibleOnFocus(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).InPlaceVisibleOnFocus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetInPlaceVisibleOnFocus<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, visible: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInPlaceVisibleOnFocus)(::windows_core::Interface::as_raw(self), visible.into_param().abi()).ok()
    }
    pub unsafe fn InPlaceBoundingRectangle(&self) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).InPlaceBoundingRectangle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn PopUpCorrectionHeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PopUpCorrectionHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PopDownCorrectionHeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PopDownCorrectionHeight)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CommitPendingInput(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitPendingInput)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInPlaceVisibility<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, visible: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInPlaceVisibility)(::windows_core::Interface::as_raw(self), visible.into_param().abi()).ok()
    }
    pub unsafe fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInPlacePosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xposition), ::core::mem::transmute(yposition), ::core::mem::transmute(position)).ok()
    }
    pub unsafe fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInPlaceHoverTargetPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xposition), ::core::mem::transmute(yposition)).ok()
    }
    pub unsafe fn Advise<'a, Param0: ::windows_core::IntoParam<'a, ITextInputPanelEventSink>>(&self, eventsink: Param0, eventmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), eventsink.into_param().abi(), ::core::mem::transmute(eventmask)).ok()
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows_core::IntoParam<'a, ITextInputPanelEventSink>>(&self, eventsink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), eventsink.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITextInputPanel> for ::windows_core::IUnknown {
    fn from(value: ITextInputPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextInputPanel> for ::windows_core::IUnknown {
    fn from(value: &ITextInputPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITextInputPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITextInputPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextInputPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextInputPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextInputPanel {}
impl ::core::fmt::Debug for ITextInputPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextInputPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextInputPanel {
    type Vtable = ITextInputPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b6a65a5_6af3_46c2_b6ea_56cd1f80df71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanel_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AttachedEditWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub SetAttachedEditWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachededitwindow: ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub CurrentInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> ::windows_core::HRESULT,
    pub DefaultInPlaceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows_core::HRESULT,
    pub SetDefaultInPlaceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: InPlaceState) -> ::windows_core::HRESULT,
    pub CurrentInPlaceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows_core::HRESULT,
    pub DefaultInputArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows_core::HRESULT,
    pub SetDefaultInputArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, area: PanelInputArea) -> ::windows_core::HRESULT,
    pub CurrentInputArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows_core::HRESULT,
    pub CurrentCorrectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut CorrectionMode) -> ::windows_core::HRESULT,
    pub PreferredInPlaceDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: *mut InPlaceDirection) -> ::windows_core::HRESULT,
    pub SetPreferredInPlaceDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: InPlaceDirection) -> ::windows_core::HRESULT,
    pub ExpandPostInsertionCorrection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expand: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetExpandPostInsertionCorrection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expand: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub InPlaceVisibleOnFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetInPlaceVisibleOnFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub InPlaceBoundingRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundingrectangle: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub PopUpCorrectionHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows_core::HRESULT,
    pub PopDownCorrectionHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows_core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetInPlaceVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetInPlacePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows_core::HRESULT,
    pub SetInPlaceHoverTargetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32) -> ::windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventsink: ::windows_core::RawPtr, eventmask: u32) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventsink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITextInputPanelEventSink(::windows_core::IUnknown);
impl ITextInputPanelEventSink {
    pub unsafe fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceStateChanging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(oldinplacestate), ::core::mem::transmute(newinplacestate)).ok()
    }
    pub unsafe fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceStateChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(oldinplacestate), ::core::mem::transmute(newinplacestate)).ok()
    }
    pub unsafe fn InPlaceSizeChanging<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(&self, oldboundingrectangle: Param0, newboundingrectangle: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceSizeChanging)(::windows_core::Interface::as_raw(self), oldboundingrectangle.into_param().abi(), newboundingrectangle.into_param().abi()).ok()
    }
    pub unsafe fn InPlaceSizeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(&self, oldboundingrectangle: Param0, newboundingrectangle: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceSizeChanged)(::windows_core::Interface::as_raw(self), oldboundingrectangle.into_param().abi(), newboundingrectangle.into_param().abi()).ok()
    }
    pub unsafe fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InputAreaChanging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(oldinputarea), ::core::mem::transmute(newinputarea)).ok()
    }
    pub unsafe fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InputAreaChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(oldinputarea), ::core::mem::transmute(newinputarea)).ok()
    }
    pub unsafe fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CorrectionModeChanging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(oldcorrectionmode), ::core::mem::transmute(newcorrectionmode)).ok()
    }
    pub unsafe fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CorrectionModeChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(oldcorrectionmode), ::core::mem::transmute(newcorrectionmode)).ok()
    }
    pub unsafe fn InPlaceVisibilityChanging<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, oldvisible: Param0, newvisible: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceVisibilityChanging)(::windows_core::Interface::as_raw(self), oldvisible.into_param().abi(), newvisible.into_param().abi()).ok()
    }
    pub unsafe fn InPlaceVisibilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, oldvisible: Param0, newvisible: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceVisibilityChanged)(::windows_core::Interface::as_raw(self), oldvisible.into_param().abi(), newvisible.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TextInserting(&self, ink: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TextInserting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ink)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn TextInserted(&self, ink: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TextInserted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ink)).ok()
    }
}
impl ::core::convert::From<ITextInputPanelEventSink> for ::windows_core::IUnknown {
    fn from(value: ITextInputPanelEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextInputPanelEventSink> for ::windows_core::IUnknown {
    fn from(value: &ITextInputPanelEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITextInputPanelEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITextInputPanelEventSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextInputPanelEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextInputPanelEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextInputPanelEventSink {}
impl ::core::fmt::Debug for ITextInputPanelEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextInputPanelEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextInputPanelEventSink {
    type Vtable = ITextInputPanelEventSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27560408_8e64_4fe1_804e_421201584b31);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelEventSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InPlaceStateChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows_core::HRESULT,
    pub InPlaceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows_core::HRESULT,
    pub InPlaceSizeChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldboundingrectangle: ::win32_foundation::RECT, newboundingrectangle: ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub InPlaceSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldboundingrectangle: ::win32_foundation::RECT, newboundingrectangle: ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub InputAreaChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows_core::HRESULT,
    pub InputAreaChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows_core::HRESULT,
    pub CorrectionModeChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows_core::HRESULT,
    pub CorrectionModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows_core::HRESULT,
    pub InPlaceVisibilityChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldvisible: ::win32_foundation::BOOL, newvisible: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub InPlaceVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldvisible: ::win32_foundation::BOOL, newvisible: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub TextInserting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TextInserting: usize,
    #[cfg(feature = "win32-system")]
    pub TextInserted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ink: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    TextInserted: usize,
}
#[repr(transparent)]
pub struct ITextInputPanelRunInfo(::windows_core::IUnknown);
impl ITextInputPanelRunInfo {
    pub unsafe fn IsTipRunning(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsTipRunning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITextInputPanelRunInfo> for ::windows_core::IUnknown {
    fn from(value: ITextInputPanelRunInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextInputPanelRunInfo> for ::windows_core::IUnknown {
    fn from(value: &ITextInputPanelRunInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITextInputPanelRunInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITextInputPanelRunInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextInputPanelRunInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextInputPanelRunInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextInputPanelRunInfo {}
impl ::core::fmt::Debug for ITextInputPanelRunInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextInputPanelRunInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextInputPanelRunInfo {
    type Vtable = ITextInputPanelRunInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f424568_1920_48cc_9811_a993cbf5adba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelRunInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsTipRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrunning: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITipAutoCompleteClient(::windows_core::IUnknown);
impl ITipAutoCompleteClient {
    pub unsafe fn AdviseProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ITipAutoCompleteProvider>>(&self, hwndfield: Param0, piprovider: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AdviseProvider)(::windows_core::Interface::as_raw(self), hwndfield.into_param().abi(), piprovider.into_param().abi()).ok()
    }
    pub unsafe fn UnadviseProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ITipAutoCompleteProvider>>(&self, hwndfield: Param0, piprovider: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnadviseProvider)(::windows_core::Interface::as_raw(self), hwndfield.into_param().abi(), piprovider.into_param().abi()).ok()
    }
    pub unsafe fn UserSelection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UserSelection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PreferredRects(&self, prcaclist: *const ::win32_foundation::RECT, prcfield: *const ::win32_foundation::RECT, prcmodifiedaclist: *mut ::win32_foundation::RECT, pfshownabovetip: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreferredRects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prcaclist), ::core::mem::transmute(prcfield), ::core::mem::transmute(prcmodifiedaclist), ::core::mem::transmute(pfshownabovetip)).ok()
    }
    pub unsafe fn RequestShowUI<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndlist: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).RequestShowUI)(::windows_core::Interface::as_raw(self), hwndlist.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITipAutoCompleteClient> for ::windows_core::IUnknown {
    fn from(value: ITipAutoCompleteClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipAutoCompleteClient> for ::windows_core::IUnknown {
    fn from(value: &ITipAutoCompleteClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITipAutoCompleteClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITipAutoCompleteClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITipAutoCompleteClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipAutoCompleteClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipAutoCompleteClient {}
impl ::core::fmt::Debug for ITipAutoCompleteClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipAutoCompleteClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITipAutoCompleteClient {
    type Vtable = ITipAutoCompleteClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e078e03_8265_4bbe_9487_d242edbef910);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipAutoCompleteClient_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AdviseProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndfield: ::win32_foundation::HWND, piprovider: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnadviseProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndfield: ::win32_foundation::HWND, piprovider: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UserSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreferredRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcaclist: *const ::win32_foundation::RECT, prcfield: *const ::win32_foundation::RECT, prcmodifiedaclist: *mut ::win32_foundation::RECT, pfshownabovetip: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub RequestShowUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndlist: ::win32_foundation::HWND, pfallowshowing: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITipAutoCompleteProvider(::windows_core::IUnknown);
impl ITipAutoCompleteProvider {
    pub unsafe fn UpdatePendingText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpendingtext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdatePendingText)(::windows_core::Interface::as_raw(self), bstrpendingtext.into_param().abi()).ok()
    }
    pub unsafe fn Show<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fshow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Show)(::windows_core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITipAutoCompleteProvider> for ::windows_core::IUnknown {
    fn from(value: ITipAutoCompleteProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipAutoCompleteProvider> for ::windows_core::IUnknown {
    fn from(value: &ITipAutoCompleteProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITipAutoCompleteProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITipAutoCompleteProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITipAutoCompleteProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipAutoCompleteProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipAutoCompleteProvider {}
impl ::core::fmt::Debug for ITipAutoCompleteProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipAutoCompleteProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITipAutoCompleteProvider {
    type Vtable = ITipAutoCompleteProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c6cf46d_8404_46b9_ad33_f5b6036d4007);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipAutoCompleteProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub UpdatePendingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpendingtext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InPlaceDirection(pub i32);
pub const InPlaceDirection_Auto: InPlaceDirection = InPlaceDirection(0i32);
pub const InPlaceDirection_Bottom: InPlaceDirection = InPlaceDirection(1i32);
pub const InPlaceDirection_Top: InPlaceDirection = InPlaceDirection(2i32);
impl ::core::marker::Copy for InPlaceDirection {}
impl ::core::clone::Clone for InPlaceDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InPlaceDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InPlaceDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for InPlaceDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InPlaceDirection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InPlaceState(pub i32);
pub const InPlaceState_Auto: InPlaceState = InPlaceState(0i32);
pub const InPlaceState_HoverTarget: InPlaceState = InPlaceState(1i32);
pub const InPlaceState_Expanded: InPlaceState = InPlaceState(2i32);
impl ::core::marker::Copy for InPlaceState {}
impl ::core::clone::Clone for InPlaceState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InPlaceState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InPlaceState {
    type Abi = Self;
}
impl ::core::fmt::Debug for InPlaceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InPlaceState").field(&self.0).finish()
    }
}
pub const Ink: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13de4a42_8d21_4c8e_bf9c_8f69cb068fca);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkApplicationGesture(pub i32);
pub const IAG_AllGestures: InkApplicationGesture = InkApplicationGesture(0i32);
pub const IAG_NoGesture: InkApplicationGesture = InkApplicationGesture(61440i32);
pub const IAG_Scratchout: InkApplicationGesture = InkApplicationGesture(61441i32);
pub const IAG_Triangle: InkApplicationGesture = InkApplicationGesture(61442i32);
pub const IAG_Square: InkApplicationGesture = InkApplicationGesture(61443i32);
pub const IAG_Star: InkApplicationGesture = InkApplicationGesture(61444i32);
pub const IAG_Check: InkApplicationGesture = InkApplicationGesture(61445i32);
pub const IAG_Curlicue: InkApplicationGesture = InkApplicationGesture(61456i32);
pub const IAG_DoubleCurlicue: InkApplicationGesture = InkApplicationGesture(61457i32);
pub const IAG_Circle: InkApplicationGesture = InkApplicationGesture(61472i32);
pub const IAG_DoubleCircle: InkApplicationGesture = InkApplicationGesture(61473i32);
pub const IAG_SemiCircleLeft: InkApplicationGesture = InkApplicationGesture(61480i32);
pub const IAG_SemiCircleRight: InkApplicationGesture = InkApplicationGesture(61481i32);
pub const IAG_ChevronUp: InkApplicationGesture = InkApplicationGesture(61488i32);
pub const IAG_ChevronDown: InkApplicationGesture = InkApplicationGesture(61489i32);
pub const IAG_ChevronLeft: InkApplicationGesture = InkApplicationGesture(61490i32);
pub const IAG_ChevronRight: InkApplicationGesture = InkApplicationGesture(61491i32);
pub const IAG_ArrowUp: InkApplicationGesture = InkApplicationGesture(61496i32);
pub const IAG_ArrowDown: InkApplicationGesture = InkApplicationGesture(61497i32);
pub const IAG_ArrowLeft: InkApplicationGesture = InkApplicationGesture(61498i32);
pub const IAG_ArrowRight: InkApplicationGesture = InkApplicationGesture(61499i32);
pub const IAG_Up: InkApplicationGesture = InkApplicationGesture(61528i32);
pub const IAG_Down: InkApplicationGesture = InkApplicationGesture(61529i32);
pub const IAG_Left: InkApplicationGesture = InkApplicationGesture(61530i32);
pub const IAG_Right: InkApplicationGesture = InkApplicationGesture(61531i32);
pub const IAG_UpDown: InkApplicationGesture = InkApplicationGesture(61536i32);
pub const IAG_DownUp: InkApplicationGesture = InkApplicationGesture(61537i32);
pub const IAG_LeftRight: InkApplicationGesture = InkApplicationGesture(61538i32);
pub const IAG_RightLeft: InkApplicationGesture = InkApplicationGesture(61539i32);
pub const IAG_UpLeftLong: InkApplicationGesture = InkApplicationGesture(61540i32);
pub const IAG_UpRightLong: InkApplicationGesture = InkApplicationGesture(61541i32);
pub const IAG_DownLeftLong: InkApplicationGesture = InkApplicationGesture(61542i32);
pub const IAG_DownRightLong: InkApplicationGesture = InkApplicationGesture(61543i32);
pub const IAG_UpLeft: InkApplicationGesture = InkApplicationGesture(61544i32);
pub const IAG_UpRight: InkApplicationGesture = InkApplicationGesture(61545i32);
pub const IAG_DownLeft: InkApplicationGesture = InkApplicationGesture(61546i32);
pub const IAG_DownRight: InkApplicationGesture = InkApplicationGesture(61547i32);
pub const IAG_LeftUp: InkApplicationGesture = InkApplicationGesture(61548i32);
pub const IAG_LeftDown: InkApplicationGesture = InkApplicationGesture(61549i32);
pub const IAG_RightUp: InkApplicationGesture = InkApplicationGesture(61550i32);
pub const IAG_RightDown: InkApplicationGesture = InkApplicationGesture(61551i32);
pub const IAG_Exclamation: InkApplicationGesture = InkApplicationGesture(61604i32);
pub const IAG_Tap: InkApplicationGesture = InkApplicationGesture(61680i32);
pub const IAG_DoubleTap: InkApplicationGesture = InkApplicationGesture(61681i32);
impl ::core::marker::Copy for InkApplicationGesture {}
impl ::core::clone::Clone for InkApplicationGesture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkApplicationGesture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkApplicationGesture {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkApplicationGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkApplicationGesture").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkBoundingBoxMode(pub i32);
pub const IBBM_Default: InkBoundingBoxMode = InkBoundingBoxMode(0i32);
pub const IBBM_NoCurveFit: InkBoundingBoxMode = InkBoundingBoxMode(1i32);
pub const IBBM_CurveFit: InkBoundingBoxMode = InkBoundingBoxMode(2i32);
pub const IBBM_PointsOnly: InkBoundingBoxMode = InkBoundingBoxMode(3i32);
pub const IBBM_Union: InkBoundingBoxMode = InkBoundingBoxMode(4i32);
impl ::core::marker::Copy for InkBoundingBoxMode {}
impl ::core::clone::Clone for InkBoundingBoxMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkBoundingBoxMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkBoundingBoxMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkBoundingBoxMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkBoundingBoxMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkClipboardFormats(pub i32);
pub const ICF_None: InkClipboardFormats = InkClipboardFormats(0i32);
pub const ICF_InkSerializedFormat: InkClipboardFormats = InkClipboardFormats(1i32);
pub const ICF_SketchInk: InkClipboardFormats = InkClipboardFormats(2i32);
pub const ICF_TextInk: InkClipboardFormats = InkClipboardFormats(6i32);
pub const ICF_EnhancedMetafile: InkClipboardFormats = InkClipboardFormats(8i32);
pub const ICF_Metafile: InkClipboardFormats = InkClipboardFormats(32i32);
pub const ICF_Bitmap: InkClipboardFormats = InkClipboardFormats(64i32);
pub const ICF_PasteMask: InkClipboardFormats = InkClipboardFormats(7i32);
pub const ICF_CopyMask: InkClipboardFormats = InkClipboardFormats(127i32);
pub const ICF_Default: InkClipboardFormats = InkClipboardFormats(127i32);
impl ::core::marker::Copy for InkClipboardFormats {}
impl ::core::clone::Clone for InkClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkClipboardFormats {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkClipboardFormats {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkClipboardFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkClipboardFormats").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkClipboardModes(pub i32);
pub const ICB_Copy: InkClipboardModes = InkClipboardModes(0i32);
pub const ICB_Cut: InkClipboardModes = InkClipboardModes(1i32);
pub const ICB_ExtractOnly: InkClipboardModes = InkClipboardModes(48i32);
pub const ICB_DelayedCopy: InkClipboardModes = InkClipboardModes(32i32);
pub const ICB_Default: InkClipboardModes = InkClipboardModes(0i32);
impl ::core::marker::Copy for InkClipboardModes {}
impl ::core::clone::Clone for InkClipboardModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkClipboardModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkClipboardModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkClipboardModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkClipboardModes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkCollectionMode(pub i32);
pub const ICM_InkOnly: InkCollectionMode = InkCollectionMode(0i32);
pub const ICM_GestureOnly: InkCollectionMode = InkCollectionMode(1i32);
pub const ICM_InkAndGesture: InkCollectionMode = InkCollectionMode(2i32);
impl ::core::marker::Copy for InkCollectionMode {}
impl ::core::clone::Clone for InkCollectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkCollectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkCollectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkCollectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkCollectionMode").field(&self.0).finish()
    }
}
pub const InkCollector: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43fb1553_ad74_4ee8_88e4_3e6daac915db);
pub const InkCollectorClipInkToMargin: i32 = 0i32;
pub const InkCollectorDefaultMargin: i32 = -2147483648i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkCollectorEventInterest(pub i32);
pub const ICEI_DefaultEvents: InkCollectorEventInterest = InkCollectorEventInterest(-1i32);
pub const ICEI_CursorDown: InkCollectorEventInterest = InkCollectorEventInterest(0i32);
pub const ICEI_Stroke: InkCollectorEventInterest = InkCollectorEventInterest(1i32);
pub const ICEI_NewPackets: InkCollectorEventInterest = InkCollectorEventInterest(2i32);
pub const ICEI_NewInAirPackets: InkCollectorEventInterest = InkCollectorEventInterest(3i32);
pub const ICEI_CursorButtonDown: InkCollectorEventInterest = InkCollectorEventInterest(4i32);
pub const ICEI_CursorButtonUp: InkCollectorEventInterest = InkCollectorEventInterest(5i32);
pub const ICEI_CursorInRange: InkCollectorEventInterest = InkCollectorEventInterest(6i32);
pub const ICEI_CursorOutOfRange: InkCollectorEventInterest = InkCollectorEventInterest(7i32);
pub const ICEI_SystemGesture: InkCollectorEventInterest = InkCollectorEventInterest(8i32);
pub const ICEI_TabletAdded: InkCollectorEventInterest = InkCollectorEventInterest(9i32);
pub const ICEI_TabletRemoved: InkCollectorEventInterest = InkCollectorEventInterest(10i32);
pub const ICEI_MouseDown: InkCollectorEventInterest = InkCollectorEventInterest(11i32);
pub const ICEI_MouseMove: InkCollectorEventInterest = InkCollectorEventInterest(12i32);
pub const ICEI_MouseUp: InkCollectorEventInterest = InkCollectorEventInterest(13i32);
pub const ICEI_MouseWheel: InkCollectorEventInterest = InkCollectorEventInterest(14i32);
pub const ICEI_DblClick: InkCollectorEventInterest = InkCollectorEventInterest(15i32);
pub const ICEI_AllEvents: InkCollectorEventInterest = InkCollectorEventInterest(16i32);
impl ::core::marker::Copy for InkCollectorEventInterest {}
impl ::core::clone::Clone for InkCollectorEventInterest {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkCollectorEventInterest {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkCollectorEventInterest {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkCollectorEventInterest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkCollectorEventInterest").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkCursorButtonState(pub i32);
pub const ICBS_Unavailable: InkCursorButtonState = InkCursorButtonState(0i32);
pub const ICBS_Up: InkCursorButtonState = InkCursorButtonState(1i32);
pub const ICBS_Down: InkCursorButtonState = InkCursorButtonState(2i32);
impl ::core::marker::Copy for InkCursorButtonState {}
impl ::core::clone::Clone for InkCursorButtonState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkCursorButtonState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkCursorButtonState {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkCursorButtonState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkCursorButtonState").field(&self.0).finish()
    }
}
pub const InkDisp: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x937c1a34_151d_4610_9ca6_a8cc9bdb5d83);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkDisplayMode(pub i32);
pub const IDM_Ink: InkDisplayMode = InkDisplayMode(0i32);
pub const IDM_Text: InkDisplayMode = InkDisplayMode(1i32);
impl ::core::marker::Copy for InkDisplayMode {}
impl ::core::clone::Clone for InkDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkDisplayMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkDisplayMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDisplayMode").field(&self.0).finish()
    }
}
pub const InkDivider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8854f6a0_4683_4ae7_9191_752fe64612c3);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkDivisionType(pub i32);
pub const IDT_Segment: InkDivisionType = InkDivisionType(0i32);
pub const IDT_Line: InkDivisionType = InkDivisionType(1i32);
pub const IDT_Paragraph: InkDivisionType = InkDivisionType(2i32);
pub const IDT_Drawing: InkDivisionType = InkDivisionType(3i32);
impl ::core::marker::Copy for InkDivisionType {}
impl ::core::clone::Clone for InkDivisionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkDivisionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkDivisionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkDivisionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDivisionType").field(&self.0).finish()
    }
}
pub const InkDrawingAttributes: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8bf32a2_05a5_44c3_b3aa_5e80ac7d2576);
pub const InkEdit: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5ca59f5_57c4_4dd8_9bd6_1deeedd27af4);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkEditStatus(pub i32);
pub const IES_Idle: InkEditStatus = InkEditStatus(0i32);
pub const IES_Collecting: InkEditStatus = InkEditStatus(1i32);
pub const IES_Recognizing: InkEditStatus = InkEditStatus(2i32);
impl ::core::marker::Copy for InkEditStatus {}
impl ::core::clone::Clone for InkEditStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkEditStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkEditStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkEditStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkEditStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkExtractFlags(pub i32);
pub const IEF_CopyFromOriginal: InkExtractFlags = InkExtractFlags(0i32);
pub const IEF_RemoveFromOriginal: InkExtractFlags = InkExtractFlags(1i32);
pub const IEF_Default: InkExtractFlags = InkExtractFlags(1i32);
impl ::core::marker::Copy for InkExtractFlags {}
impl ::core::clone::Clone for InkExtractFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkExtractFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkExtractFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkExtractFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkExtractFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkInsertMode(pub i32);
pub const IEM_InsertText: InkInsertMode = InkInsertMode(0i32);
pub const IEM_InsertInk: InkInsertMode = InkInsertMode(1i32);
impl ::core::marker::Copy for InkInsertMode {}
impl ::core::clone::Clone for InkInsertMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkInsertMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkInsertMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkInsertMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInsertMode").field(&self.0).finish()
    }
}
pub const InkMaxTransparencyValue: i32 = 255i32;
pub const InkMinTransparencyValue: i32 = 0i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkMode(pub i32);
pub const IEM_Disabled: InkMode = InkMode(0i32);
pub const IEM_Ink: InkMode = InkMode(1i32);
pub const IEM_InkAndGesture: InkMode = InkMode(2i32);
impl ::core::marker::Copy for InkMode {}
impl ::core::clone::Clone for InkMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkMouseButton(pub i32);
pub const IMF_Left: InkMouseButton = InkMouseButton(1i32);
pub const IMF_Right: InkMouseButton = InkMouseButton(2i32);
pub const IMF_Middle: InkMouseButton = InkMouseButton(4i32);
impl ::core::marker::Copy for InkMouseButton {}
impl ::core::clone::Clone for InkMouseButton {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkMouseButton {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkMouseButton {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkMouseButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkMouseButton").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkMousePointer(pub i32);
pub const IMP_Default: InkMousePointer = InkMousePointer(0i32);
pub const IMP_Arrow: InkMousePointer = InkMousePointer(1i32);
pub const IMP_Crosshair: InkMousePointer = InkMousePointer(2i32);
pub const IMP_Ibeam: InkMousePointer = InkMousePointer(3i32);
pub const IMP_SizeNESW: InkMousePointer = InkMousePointer(4i32);
pub const IMP_SizeNS: InkMousePointer = InkMousePointer(5i32);
pub const IMP_SizeNWSE: InkMousePointer = InkMousePointer(6i32);
pub const IMP_SizeWE: InkMousePointer = InkMousePointer(7i32);
pub const IMP_UpArrow: InkMousePointer = InkMousePointer(8i32);
pub const IMP_Hourglass: InkMousePointer = InkMousePointer(9i32);
pub const IMP_NoDrop: InkMousePointer = InkMousePointer(10i32);
pub const IMP_ArrowHourglass: InkMousePointer = InkMousePointer(11i32);
pub const IMP_ArrowQuestion: InkMousePointer = InkMousePointer(12i32);
pub const IMP_SizeAll: InkMousePointer = InkMousePointer(13i32);
pub const IMP_Hand: InkMousePointer = InkMousePointer(14i32);
pub const IMP_Custom: InkMousePointer = InkMousePointer(99i32);
impl ::core::marker::Copy for InkMousePointer {}
impl ::core::clone::Clone for InkMousePointer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkMousePointer {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkMousePointer {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkMousePointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkMousePointer").field(&self.0).finish()
    }
}
pub const InkOverlay: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65d00646_cde3_4a88_9163_6769f0f1a97d);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkOverlayAttachMode(pub i32);
pub const IOAM_Behind: InkOverlayAttachMode = InkOverlayAttachMode(0i32);
pub const IOAM_InFront: InkOverlayAttachMode = InkOverlayAttachMode(1i32);
impl ::core::marker::Copy for InkOverlayAttachMode {}
impl ::core::clone::Clone for InkOverlayAttachMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkOverlayAttachMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkOverlayAttachMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkOverlayAttachMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkOverlayAttachMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkOverlayEditingMode(pub i32);
pub const IOEM_Ink: InkOverlayEditingMode = InkOverlayEditingMode(0i32);
pub const IOEM_Delete: InkOverlayEditingMode = InkOverlayEditingMode(1i32);
pub const IOEM_Select: InkOverlayEditingMode = InkOverlayEditingMode(2i32);
impl ::core::marker::Copy for InkOverlayEditingMode {}
impl ::core::clone::Clone for InkOverlayEditingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkOverlayEditingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkOverlayEditingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkOverlayEditingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkOverlayEditingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkOverlayEraserMode(pub i32);
pub const IOERM_StrokeErase: InkOverlayEraserMode = InkOverlayEraserMode(0i32);
pub const IOERM_PointErase: InkOverlayEraserMode = InkOverlayEraserMode(1i32);
impl ::core::marker::Copy for InkOverlayEraserMode {}
impl ::core::clone::Clone for InkOverlayEraserMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkOverlayEraserMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkOverlayEraserMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkOverlayEraserMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkOverlayEraserMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPenTip(pub i32);
pub const IPT_Ball: InkPenTip = InkPenTip(0i32);
pub const IPT_Rectangle: InkPenTip = InkPenTip(1i32);
impl ::core::marker::Copy for InkPenTip {}
impl ::core::clone::Clone for InkPenTip {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPenTip {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkPenTip {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPenTip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPenTip").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPersistenceCompressionMode(pub i32);
pub const IPCM_Default: InkPersistenceCompressionMode = InkPersistenceCompressionMode(0i32);
pub const IPCM_MaximumCompression: InkPersistenceCompressionMode = InkPersistenceCompressionMode(1i32);
pub const IPCM_NoCompression: InkPersistenceCompressionMode = InkPersistenceCompressionMode(2i32);
impl ::core::marker::Copy for InkPersistenceCompressionMode {}
impl ::core::clone::Clone for InkPersistenceCompressionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPersistenceCompressionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkPersistenceCompressionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPersistenceCompressionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPersistenceCompressionMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPersistenceFormat(pub i32);
pub const IPF_InkSerializedFormat: InkPersistenceFormat = InkPersistenceFormat(0i32);
pub const IPF_Base64InkSerializedFormat: InkPersistenceFormat = InkPersistenceFormat(1i32);
pub const IPF_GIF: InkPersistenceFormat = InkPersistenceFormat(2i32);
pub const IPF_Base64GIF: InkPersistenceFormat = InkPersistenceFormat(3i32);
impl ::core::marker::Copy for InkPersistenceFormat {}
impl ::core::clone::Clone for InkPersistenceFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPersistenceFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkPersistenceFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPersistenceFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPersistenceFormat").field(&self.0).finish()
    }
}
pub const InkPicture: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04a1e553_fe36_4fde_865e_344194e69424);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPictureSizeMode(pub i32);
pub const IPSM_AutoSize: InkPictureSizeMode = InkPictureSizeMode(0i32);
pub const IPSM_CenterImage: InkPictureSizeMode = InkPictureSizeMode(1i32);
pub const IPSM_Normal: InkPictureSizeMode = InkPictureSizeMode(2i32);
pub const IPSM_StretchImage: InkPictureSizeMode = InkPictureSizeMode(3i32);
impl ::core::marker::Copy for InkPictureSizeMode {}
impl ::core::clone::Clone for InkPictureSizeMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkPictureSizeMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkPictureSizeMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPictureSizeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPictureSizeMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRasterOperation(pub i32);
pub const IRO_Black: InkRasterOperation = InkRasterOperation(1i32);
pub const IRO_NotMergePen: InkRasterOperation = InkRasterOperation(2i32);
pub const IRO_MaskNotPen: InkRasterOperation = InkRasterOperation(3i32);
pub const IRO_NotCopyPen: InkRasterOperation = InkRasterOperation(4i32);
pub const IRO_MaskPenNot: InkRasterOperation = InkRasterOperation(5i32);
pub const IRO_Not: InkRasterOperation = InkRasterOperation(6i32);
pub const IRO_XOrPen: InkRasterOperation = InkRasterOperation(7i32);
pub const IRO_NotMaskPen: InkRasterOperation = InkRasterOperation(8i32);
pub const IRO_MaskPen: InkRasterOperation = InkRasterOperation(9i32);
pub const IRO_NotXOrPen: InkRasterOperation = InkRasterOperation(10i32);
pub const IRO_NoOperation: InkRasterOperation = InkRasterOperation(11i32);
pub const IRO_MergeNotPen: InkRasterOperation = InkRasterOperation(12i32);
pub const IRO_CopyPen: InkRasterOperation = InkRasterOperation(13i32);
pub const IRO_MergePenNot: InkRasterOperation = InkRasterOperation(14i32);
pub const IRO_MergePen: InkRasterOperation = InkRasterOperation(15i32);
pub const IRO_White: InkRasterOperation = InkRasterOperation(16i32);
impl ::core::marker::Copy for InkRasterOperation {}
impl ::core::clone::Clone for InkRasterOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRasterOperation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRasterOperation {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRasterOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRasterOperation").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct InkRecoGuide {
    pub rectWritingBox: ::win32_foundation::RECT,
    pub rectDrawnBox: ::win32_foundation::RECT,
    pub cRows: i32,
    pub cColumns: i32,
    pub midline: i32,
}
impl ::core::marker::Copy for InkRecoGuide {}
impl ::core::clone::Clone for InkRecoGuide {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InkRecoGuide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InkRecoGuide").field("rectWritingBox", &self.rectWritingBox).field("rectDrawnBox", &self.rectDrawnBox).field("cRows", &self.cRows).field("cColumns", &self.cColumns).field("midline", &self.midline).finish()
    }
}
unsafe impl ::windows_core::Abi for InkRecoGuide {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for InkRecoGuide {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InkRecoGuide>()) == 0 }
    }
}
impl ::core::cmp::Eq for InkRecoGuide {}
impl ::core::default::Default for InkRecoGuide {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognitionAlternatesSelection(pub i32);
pub const IRAS_Start: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(0i32);
pub const IRAS_DefaultCount: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(10i32);
pub const IRAS_All: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(-1i32);
impl ::core::marker::Copy for InkRecognitionAlternatesSelection {}
impl ::core::clone::Clone for InkRecognitionAlternatesSelection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognitionAlternatesSelection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRecognitionAlternatesSelection {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognitionAlternatesSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionAlternatesSelection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognitionConfidence(pub i32);
pub const IRC_Strong: InkRecognitionConfidence = InkRecognitionConfidence(0i32);
pub const IRC_Intermediate: InkRecognitionConfidence = InkRecognitionConfidence(1i32);
pub const IRC_Poor: InkRecognitionConfidence = InkRecognitionConfidence(2i32);
impl ::core::marker::Copy for InkRecognitionConfidence {}
impl ::core::clone::Clone for InkRecognitionConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognitionConfidence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRecognitionConfidence {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognitionConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionConfidence").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognitionModes(pub i32);
pub const IRM_None: InkRecognitionModes = InkRecognitionModes(0i32);
pub const IRM_WordModeOnly: InkRecognitionModes = InkRecognitionModes(1i32);
pub const IRM_Coerce: InkRecognitionModes = InkRecognitionModes(2i32);
pub const IRM_TopInkBreaksOnly: InkRecognitionModes = InkRecognitionModes(4i32);
pub const IRM_PrefixOk: InkRecognitionModes = InkRecognitionModes(8i32);
pub const IRM_LineMode: InkRecognitionModes = InkRecognitionModes(16i32);
pub const IRM_DisablePersonalization: InkRecognitionModes = InkRecognitionModes(32i32);
pub const IRM_AutoSpace: InkRecognitionModes = InkRecognitionModes(64i32);
pub const IRM_Max: InkRecognitionModes = InkRecognitionModes(128i32);
impl ::core::marker::Copy for InkRecognitionModes {}
impl ::core::clone::Clone for InkRecognitionModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognitionModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRecognitionModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognitionModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionModes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognitionStatus(pub i32);
pub const IRS_NoError: InkRecognitionStatus = InkRecognitionStatus(0i32);
pub const IRS_Interrupted: InkRecognitionStatus = InkRecognitionStatus(1i32);
pub const IRS_ProcessFailed: InkRecognitionStatus = InkRecognitionStatus(2i32);
pub const IRS_InkAddedFailed: InkRecognitionStatus = InkRecognitionStatus(4i32);
pub const IRS_SetAutoCompletionModeFailed: InkRecognitionStatus = InkRecognitionStatus(8i32);
pub const IRS_SetStrokesFailed: InkRecognitionStatus = InkRecognitionStatus(16i32);
pub const IRS_SetGuideFailed: InkRecognitionStatus = InkRecognitionStatus(32i32);
pub const IRS_SetFlagsFailed: InkRecognitionStatus = InkRecognitionStatus(64i32);
pub const IRS_SetFactoidFailed: InkRecognitionStatus = InkRecognitionStatus(128i32);
pub const IRS_SetPrefixSuffixFailed: InkRecognitionStatus = InkRecognitionStatus(256i32);
pub const IRS_SetWordListFailed: InkRecognitionStatus = InkRecognitionStatus(512i32);
impl ::core::marker::Copy for InkRecognitionStatus {}
impl ::core::clone::Clone for InkRecognitionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognitionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRecognitionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognitionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognizerCapabilities(pub i32);
pub const IRC_DontCare: InkRecognizerCapabilities = InkRecognizerCapabilities(1i32);
pub const IRC_Object: InkRecognizerCapabilities = InkRecognizerCapabilities(2i32);
pub const IRC_FreeInput: InkRecognizerCapabilities = InkRecognizerCapabilities(4i32);
pub const IRC_LinedInput: InkRecognizerCapabilities = InkRecognizerCapabilities(8i32);
pub const IRC_BoxedInput: InkRecognizerCapabilities = InkRecognizerCapabilities(16i32);
pub const IRC_CharacterAutoCompletionInput: InkRecognizerCapabilities = InkRecognizerCapabilities(32i32);
pub const IRC_RightAndDown: InkRecognizerCapabilities = InkRecognizerCapabilities(64i32);
pub const IRC_LeftAndDown: InkRecognizerCapabilities = InkRecognizerCapabilities(128i32);
pub const IRC_DownAndLeft: InkRecognizerCapabilities = InkRecognizerCapabilities(256i32);
pub const IRC_DownAndRight: InkRecognizerCapabilities = InkRecognizerCapabilities(512i32);
pub const IRC_ArbitraryAngle: InkRecognizerCapabilities = InkRecognizerCapabilities(1024i32);
pub const IRC_Lattice: InkRecognizerCapabilities = InkRecognizerCapabilities(2048i32);
pub const IRC_AdviseInkChange: InkRecognizerCapabilities = InkRecognizerCapabilities(4096i32);
pub const IRC_StrokeReorder: InkRecognizerCapabilities = InkRecognizerCapabilities(8192i32);
pub const IRC_Personalizable: InkRecognizerCapabilities = InkRecognizerCapabilities(16384i32);
pub const IRC_PrefersArbitraryAngle: InkRecognizerCapabilities = InkRecognizerCapabilities(32768i32);
pub const IRC_PrefersParagraphBreaking: InkRecognizerCapabilities = InkRecognizerCapabilities(65536i32);
pub const IRC_PrefersSegmentation: InkRecognizerCapabilities = InkRecognizerCapabilities(131072i32);
pub const IRC_Cursive: InkRecognizerCapabilities = InkRecognizerCapabilities(262144i32);
pub const IRC_TextPrediction: InkRecognizerCapabilities = InkRecognizerCapabilities(524288i32);
pub const IRC_Alpha: InkRecognizerCapabilities = InkRecognizerCapabilities(1048576i32);
pub const IRC_Beta: InkRecognizerCapabilities = InkRecognizerCapabilities(2097152i32);
impl ::core::marker::Copy for InkRecognizerCapabilities {}
impl ::core::clone::Clone for InkRecognizerCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognizerCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRecognizerCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognizerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizerCapabilities").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognizerCharacterAutoCompletionMode(pub i32);
pub const IRCACM_Full: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(0i32);
pub const IRCACM_Prefix: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(1i32);
pub const IRCACM_Random: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(2i32);
impl ::core::marker::Copy for InkRecognizerCharacterAutoCompletionMode {}
impl ::core::clone::Clone for InkRecognizerCharacterAutoCompletionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkRecognizerCharacterAutoCompletionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkRecognizerCharacterAutoCompletionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognizerCharacterAutoCompletionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizerCharacterAutoCompletionMode").field(&self.0).finish()
    }
}
pub const InkRecognizerContext: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaac46a37_9229_4fc0_8cce_4497569bf4d1);
pub const InkRecognizerGuide: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8770d941_a63a_4671_a375_2855a18eba73);
pub const InkRecognizers: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fd4e808_f6e6_4e65_98d3_aa39054c1255);
pub const InkRectangle: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43b07326_aae0_4b62_a83d_5fd768b7353c);
pub const InkRenderer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c1cc6e4_d7eb_4eeb_9091_15a7c8791ed9);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkSelectionConstants(pub i32);
pub const ISC_FirstElement: InkSelectionConstants = InkSelectionConstants(0i32);
pub const ISC_AllElements: InkSelectionConstants = InkSelectionConstants(-1i32);
impl ::core::marker::Copy for InkSelectionConstants {}
impl ::core::clone::Clone for InkSelectionConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkSelectionConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkSelectionConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkSelectionConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkSelectionConstants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkShiftKeyModifierFlags(pub i32);
pub const IKM_Shift: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(1i32);
pub const IKM_Control: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(2i32);
pub const IKM_Alt: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(4i32);
impl ::core::marker::Copy for InkShiftKeyModifierFlags {}
impl ::core::clone::Clone for InkShiftKeyModifierFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkShiftKeyModifierFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkShiftKeyModifierFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkShiftKeyModifierFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkShiftKeyModifierFlags").field(&self.0).finish()
    }
}
pub const InkStrokes: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48f491bc_240e_4860_b079_a1e94d3d2c86);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkSystemGesture(pub i32);
pub const ISG_Tap: InkSystemGesture = InkSystemGesture(16i32);
pub const ISG_DoubleTap: InkSystemGesture = InkSystemGesture(17i32);
pub const ISG_RightTap: InkSystemGesture = InkSystemGesture(18i32);
pub const ISG_Drag: InkSystemGesture = InkSystemGesture(19i32);
pub const ISG_RightDrag: InkSystemGesture = InkSystemGesture(20i32);
pub const ISG_HoldEnter: InkSystemGesture = InkSystemGesture(21i32);
pub const ISG_HoldLeave: InkSystemGesture = InkSystemGesture(22i32);
pub const ISG_HoverEnter: InkSystemGesture = InkSystemGesture(23i32);
pub const ISG_HoverLeave: InkSystemGesture = InkSystemGesture(24i32);
pub const ISG_Flick: InkSystemGesture = InkSystemGesture(31i32);
impl ::core::marker::Copy for InkSystemGesture {}
impl ::core::clone::Clone for InkSystemGesture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkSystemGesture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkSystemGesture {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkSystemGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkSystemGesture").field(&self.0).finish()
    }
}
pub const InkTablets: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e4fcb12_510a_4d40_9304_1da10ae9147c);
pub const InkTransform: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3d5d93c_1663_4a78_a1a7_22375dfebaee);
pub const InkWordList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9de85094_f71f_44f1_8471_15a2fa76fcf3);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionMode(pub i32);
pub const InteractionMode_InPlace: InteractionMode = InteractionMode(0i32);
pub const InteractionMode_Floating: InteractionMode = InteractionMode(1i32);
pub const InteractionMode_DockedTop: InteractionMode = InteractionMode(2i32);
pub const InteractionMode_DockedBottom: InteractionMode = InteractionMode(3i32);
impl ::core::marker::Copy for InteractionMode {}
impl ::core::clone::Clone for InteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionMode").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn IsStringSupported<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hrc: Param0, wcstring: u32, pwcstring: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsStringSupported(hrc: HRECOCONTEXT, wcstring: u32, pwcstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        IsStringSupported(hrc.into_param().abi(), ::core::mem::transmute(wcstring), pwcstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KEYMODIFIER(pub i32);
pub const KEYMODIFIER_CONTROL: KEYMODIFIER = KEYMODIFIER(1i32);
pub const KEYMODIFIER_MENU: KEYMODIFIER = KEYMODIFIER(2i32);
pub const KEYMODIFIER_SHIFT: KEYMODIFIER = KEYMODIFIER(4i32);
pub const KEYMODIFIER_WIN: KEYMODIFIER = KEYMODIFIER(8i32);
pub const KEYMODIFIER_ALTGR: KEYMODIFIER = KEYMODIFIER(16i32);
pub const KEYMODIFIER_EXT: KEYMODIFIER = KEYMODIFIER(32i32);
impl ::core::marker::Copy for KEYMODIFIER {}
impl ::core::clone::Clone for KEYMODIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEYMODIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KEYMODIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for KEYMODIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEYMODIFIER").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct LATTICE_METRICS {
    pub lsBaseline: LINE_SEGMENT,
    pub iMidlineOffset: i16,
}
impl ::core::marker::Copy for LATTICE_METRICS {}
impl ::core::clone::Clone for LATTICE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LATTICE_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LATTICE_METRICS").field("lsBaseline", &self.lsBaseline).field("iMidlineOffset", &self.iMidlineOffset).finish()
    }
}
unsafe impl ::windows_core::Abi for LATTICE_METRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LATTICE_METRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LATTICE_METRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for LATTICE_METRICS {}
impl ::core::default::Default for LATTICE_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LINE_METRICS(pub i32);
pub const LM_BASELINE: LINE_METRICS = LINE_METRICS(0i32);
pub const LM_MIDLINE: LINE_METRICS = LINE_METRICS(1i32);
pub const LM_ASCENDER: LINE_METRICS = LINE_METRICS(2i32);
pub const LM_DESCENDER: LINE_METRICS = LINE_METRICS(3i32);
impl ::core::marker::Copy for LINE_METRICS {}
impl ::core::clone::Clone for LINE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LINE_METRICS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LINE_METRICS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LINE_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LINE_METRICS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct LINE_SEGMENT {
    pub PtA: ::win32_foundation::POINT,
    pub PtB: ::win32_foundation::POINT,
}
impl ::core::marker::Copy for LINE_SEGMENT {}
impl ::core::clone::Clone for LINE_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LINE_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LINE_SEGMENT").field("PtA", &self.PtA).field("PtB", &self.PtB).finish()
    }
}
unsafe impl ::windows_core::Abi for LINE_SEGMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LINE_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LINE_SEGMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for LINE_SEGMENT {}
impl ::core::default::Default for LINE_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn LoadCachedAttributes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(clsid: Param0, precoattributes: *mut RECO_ATTRS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadCachedAttributes(clsid: ::windows_core::GUID, precoattributes: *mut RECO_ATTRS) -> ::windows_core::HRESULT;
        }
        LoadCachedAttributes(clsid.into_param().abi(), ::core::mem::transmute(precoattributes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_FRIENDLYNAME: u32 = 64u32;
pub const MAX_LANGUAGES: u32 = 64u32;
pub const MAX_PACKET_BUTTON_COUNT: u32 = 32u32;
pub const MAX_PACKET_PROPERTY_COUNT: u32 = 32u32;
pub const MAX_VENDORNAME: u32 = 32u32;
pub const MICROSOFT_PENINPUT_PANEL_PROPERTY_T: &str = "Microsoft PenInputPanel 1.5";
pub const MICROSOFT_TIP_COMBOBOXLIST_PROPERTY: &str = "Microsoft TIP ComboBox List Window Identifier";
pub const MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY: &str = "Microsoft TIP No Insert Option";
pub const MICROSOFT_TIP_OPENING_MSG: &str = "TabletInputPanelOpening";
pub const MICROSOFT_URL_EXPERIENCE_PROPERTY: &str = "Microsoft TIP URL Experience";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MICUIELEMENT(pub i32);
pub const MICUIELEMENT_BUTTON_WRITE: MICUIELEMENT = MICUIELEMENT(1i32);
pub const MICUIELEMENT_BUTTON_ERASE: MICUIELEMENT = MICUIELEMENT(2i32);
pub const MICUIELEMENT_BUTTON_CORRECT: MICUIELEMENT = MICUIELEMENT(4i32);
pub const MICUIELEMENT_BUTTON_CLEAR: MICUIELEMENT = MICUIELEMENT(8i32);
pub const MICUIELEMENT_BUTTON_UNDO: MICUIELEMENT = MICUIELEMENT(16i32);
pub const MICUIELEMENT_BUTTON_REDO: MICUIELEMENT = MICUIELEMENT(32i32);
pub const MICUIELEMENT_BUTTON_INSERT: MICUIELEMENT = MICUIELEMENT(64i32);
pub const MICUIELEMENT_BUTTON_CANCEL: MICUIELEMENT = MICUIELEMENT(128i32);
pub const MICUIELEMENT_INKPANEL_BACKGROUND: MICUIELEMENT = MICUIELEMENT(256i32);
pub const MICUIELEMENT_RESULTPANEL_BACKGROUND: MICUIELEMENT = MICUIELEMENT(512i32);
impl ::core::marker::Copy for MICUIELEMENT {}
impl ::core::clone::Clone for MICUIELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MICUIELEMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MICUIELEMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MICUIELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MICUIELEMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MICUIELEMENTSTATE(pub i32);
pub const MICUIELEMENTSTATE_NORMAL: MICUIELEMENTSTATE = MICUIELEMENTSTATE(1i32);
pub const MICUIELEMENTSTATE_HOT: MICUIELEMENTSTATE = MICUIELEMENTSTATE(2i32);
pub const MICUIELEMENTSTATE_PRESSED: MICUIELEMENTSTATE = MICUIELEMENTSTATE(3i32);
pub const MICUIELEMENTSTATE_DISABLED: MICUIELEMENTSTATE = MICUIELEMENTSTATE(4i32);
impl ::core::marker::Copy for MICUIELEMENTSTATE {}
impl ::core::clone::Clone for MICUIELEMENTSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MICUIELEMENTSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MICUIELEMENTSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MICUIELEMENTSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MICUIELEMENTSTATE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn MakeWordList<'a, Param0: ::windows_core::IntoParam<'a, HRECOGNIZER>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hrec: Param0, pbuffer: Param1, phwl: *mut HRECOWORDLIST) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeWordList(hrec: HRECOGNIZER, pbuffer: ::windows_core::PCWSTR, phwl: *mut HRECOWORDLIST) -> ::windows_core::HRESULT;
        }
        MakeWordList(hrec.into_param().abi(), pbuffer.into_param().abi(), ::core::mem::transmute(phwl)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MathInputControl: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc561816c_14d8_4090_830c_98d994b21c7b);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MouseButton(pub i32);
pub const NO_BUTTON: MouseButton = MouseButton(0i32);
pub const LEFT_BUTTON: MouseButton = MouseButton(1i32);
pub const RIGHT_BUTTON: MouseButton = MouseButton(2i32);
pub const MIDDLE_BUTTON: MouseButton = MouseButton(4i32);
impl ::core::marker::Copy for MouseButton {}
impl ::core::clone::Clone for MouseButton {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MouseButton {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MouseButton {
    type Abi = Self;
}
impl ::core::fmt::Debug for MouseButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseButton").field(&self.0).finish()
    }
}
pub const NUM_FLICK_DIRECTIONS: u32 = 8u32;
#[repr(C)]
pub struct PACKET_DESCRIPTION {
    pub cbPacketSize: u32,
    pub cPacketProperties: u32,
    pub pPacketProperties: *mut PACKET_PROPERTY,
    pub cButtons: u32,
    pub pguidButtons: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for PACKET_DESCRIPTION {}
impl ::core::clone::Clone for PACKET_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PACKET_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKET_DESCRIPTION").field("cbPacketSize", &self.cbPacketSize).field("cPacketProperties", &self.cPacketProperties).field("pPacketProperties", &self.pPacketProperties).field("cButtons", &self.cButtons).field("pguidButtons", &self.pguidButtons).finish()
    }
}
unsafe impl ::windows_core::Abi for PACKET_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PACKET_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PACKET_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PACKET_DESCRIPTION {}
impl ::core::default::Default for PACKET_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PACKET_PROPERTY {
    pub guid: ::windows_core::GUID,
    pub PropertyMetrics: PROPERTY_METRICS,
}
impl ::core::marker::Copy for PACKET_PROPERTY {}
impl ::core::clone::Clone for PACKET_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PACKET_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKET_PROPERTY").field("guid", &self.guid).field("PropertyMetrics", &self.PropertyMetrics).finish()
    }
}
unsafe impl ::windows_core::Abi for PACKET_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PACKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PACKET_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PACKET_PROPERTY {}
impl ::core::default::Default for PACKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PROPERTY_METRICS {
    pub nLogicalMin: i32,
    pub nLogicalMax: i32,
    pub Units: PROPERTY_UNITS,
    pub fResolution: f32,
}
impl ::core::marker::Copy for PROPERTY_METRICS {}
impl ::core::clone::Clone for PROPERTY_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPERTY_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPERTY_METRICS").field("nLogicalMin", &self.nLogicalMin).field("nLogicalMax", &self.nLogicalMax).field("Units", &self.Units).field("fResolution", &self.fResolution).finish()
    }
}
unsafe impl ::windows_core::Abi for PROPERTY_METRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROPERTY_METRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPERTY_METRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROPERTY_METRICS {}
impl ::core::default::Default for PROPERTY_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPERTY_UNITS(pub i32);
pub const PROPERTY_UNITS_DEFAULT: PROPERTY_UNITS = PROPERTY_UNITS(0i32);
pub const PROPERTY_UNITS_INCHES: PROPERTY_UNITS = PROPERTY_UNITS(1i32);
pub const PROPERTY_UNITS_CENTIMETERS: PROPERTY_UNITS = PROPERTY_UNITS(2i32);
pub const PROPERTY_UNITS_DEGREES: PROPERTY_UNITS = PROPERTY_UNITS(3i32);
pub const PROPERTY_UNITS_RADIANS: PROPERTY_UNITS = PROPERTY_UNITS(4i32);
pub const PROPERTY_UNITS_SECONDS: PROPERTY_UNITS = PROPERTY_UNITS(5i32);
pub const PROPERTY_UNITS_POUNDS: PROPERTY_UNITS = PROPERTY_UNITS(6i32);
pub const PROPERTY_UNITS_GRAMS: PROPERTY_UNITS = PROPERTY_UNITS(7i32);
pub const PROPERTY_UNITS_SILINEAR: PROPERTY_UNITS = PROPERTY_UNITS(8i32);
pub const PROPERTY_UNITS_SIROTATION: PROPERTY_UNITS = PROPERTY_UNITS(9i32);
pub const PROPERTY_UNITS_ENGLINEAR: PROPERTY_UNITS = PROPERTY_UNITS(10i32);
pub const PROPERTY_UNITS_ENGROTATION: PROPERTY_UNITS = PROPERTY_UNITS(11i32);
pub const PROPERTY_UNITS_SLUGS: PROPERTY_UNITS = PROPERTY_UNITS(12i32);
pub const PROPERTY_UNITS_KELVIN: PROPERTY_UNITS = PROPERTY_UNITS(13i32);
pub const PROPERTY_UNITS_FAHRENHEIT: PROPERTY_UNITS = PROPERTY_UNITS(14i32);
pub const PROPERTY_UNITS_AMPERE: PROPERTY_UNITS = PROPERTY_UNITS(15i32);
pub const PROPERTY_UNITS_CANDELA: PROPERTY_UNITS = PROPERTY_UNITS(16i32);
impl ::core::marker::Copy for PROPERTY_UNITS {}
impl ::core::clone::Clone for PROPERTY_UNITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTY_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPERTY_UNITS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPERTY_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTY_UNITS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PanelInputArea(pub i32);
pub const PanelInputArea_Auto: PanelInputArea = PanelInputArea(0i32);
pub const PanelInputArea_Keyboard: PanelInputArea = PanelInputArea(1i32);
pub const PanelInputArea_WritingPad: PanelInputArea = PanelInputArea(2i32);
pub const PanelInputArea_CharacterPad: PanelInputArea = PanelInputArea(3i32);
impl ::core::marker::Copy for PanelInputArea {}
impl ::core::clone::Clone for PanelInputArea {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PanelInputArea {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PanelInputArea {
    type Abi = Self;
}
impl ::core::fmt::Debug for PanelInputArea {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PanelInputArea").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PanelType(pub i32);
pub const PT_Default: PanelType = PanelType(0i32);
pub const PT_Inactive: PanelType = PanelType(1i32);
pub const PT_Handwriting: PanelType = PanelType(2i32);
pub const PT_Keyboard: PanelType = PanelType(3i32);
impl ::core::marker::Copy for PanelType {}
impl ::core::clone::Clone for PanelType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PanelType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PanelType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PanelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PanelType").field(&self.0).finish()
    }
}
pub const PenInputPanel: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf744e496_1b5a_489e_81dc_fbd7ac6298a8);
pub const PenInputPanel_Internal: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x802b1fb9_056b_4720_b0cc_80d23b71171e);
pub type PfnRecoCallback = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut u8, param2: HRECOCONTEXT) -> ::windows_core::HRESULT>;
#[inline]
pub unsafe fn Process<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pbpartialprocessing: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Process(hrc: HRECOCONTEXT, pbpartialprocessing: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        Process(hrc.into_param().abi(), ::core::mem::transmute(pbpartialprocessing)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RECOCONF_HIGHCONFIDENCE: u32 = 1u32;
pub const RECOCONF_LOWCONFIDENCE: i32 = -1i32;
pub const RECOCONF_MEDIUMCONFIDENCE: u32 = 0u32;
pub const RECOCONF_NOTSET: u32 = 128u32;
pub const RECOFLAG_AUTOSPACE: u32 = 64u32;
pub const RECOFLAG_COERCE: u32 = 2u32;
pub const RECOFLAG_DISABLEPERSONALIZATION: u32 = 32u32;
pub const RECOFLAG_LINEMODE: u32 = 16u32;
pub const RECOFLAG_PREFIXOK: u32 = 8u32;
pub const RECOFLAG_SINGLESEG: u32 = 4u32;
pub const RECOFLAG_WORDMODE: u32 = 1u32;
#[repr(C)]
pub struct RECO_ATTRS {
    pub dwRecoCapabilityFlags: u32,
    pub awcVendorName: [u16; 32],
    pub awcFriendlyName: [u16; 64],
    pub awLanguageId: [u16; 64],
}
impl ::core::marker::Copy for RECO_ATTRS {}
impl ::core::clone::Clone for RECO_ATTRS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_ATTRS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_ATTRS").field("dwRecoCapabilityFlags", &self.dwRecoCapabilityFlags).field("awcVendorName", &self.awcVendorName).field("awcFriendlyName", &self.awcFriendlyName).field("awLanguageId", &self.awLanguageId).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_ATTRS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_ATTRS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_ATTRS>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_ATTRS {}
impl ::core::default::Default for RECO_ATTRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_GUIDE {
    pub xOrigin: i32,
    pub yOrigin: i32,
    pub cxBox: i32,
    pub cyBox: i32,
    pub cxBase: i32,
    pub cyBase: i32,
    pub cHorzBox: i32,
    pub cVertBox: i32,
    pub cyMid: i32,
}
impl ::core::marker::Copy for RECO_GUIDE {}
impl ::core::clone::Clone for RECO_GUIDE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_GUIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_GUIDE").field("xOrigin", &self.xOrigin).field("yOrigin", &self.yOrigin).field("cxBox", &self.cxBox).field("cyBox", &self.cyBox).field("cxBase", &self.cxBase).field("cyBase", &self.cyBase).field("cHorzBox", &self.cHorzBox).field("cVertBox", &self.cVertBox).field("cyMid", &self.cyMid).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_GUIDE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_GUIDE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_GUIDE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_GUIDE {}
impl ::core::default::Default for RECO_GUIDE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_LATTICE {
    pub ulColumnCount: u32,
    pub pLatticeColumns: *mut RECO_LATTICE_COLUMN,
    pub ulPropertyCount: u32,
    pub pGuidProperties: *mut ::windows_core::GUID,
    pub ulBestResultColumnCount: u32,
    pub pulBestResultColumns: *mut u32,
    pub pulBestResultIndexes: *mut u32,
}
impl ::core::marker::Copy for RECO_LATTICE {}
impl ::core::clone::Clone for RECO_LATTICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_LATTICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE").field("ulColumnCount", &self.ulColumnCount).field("pLatticeColumns", &self.pLatticeColumns).field("ulPropertyCount", &self.ulPropertyCount).field("pGuidProperties", &self.pGuidProperties).field("ulBestResultColumnCount", &self.ulBestResultColumnCount).field("pulBestResultColumns", &self.pulBestResultColumns).field("pulBestResultIndexes", &self.pulBestResultIndexes).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_LATTICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_LATTICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_LATTICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_LATTICE {}
impl ::core::default::Default for RECO_LATTICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_LATTICE_COLUMN {
    pub key: u32,
    pub cpProp: RECO_LATTICE_PROPERTIES,
    pub cStrokes: u32,
    pub pStrokes: *mut u32,
    pub cLatticeElements: u32,
    pub pLatticeElements: *mut RECO_LATTICE_ELEMENT,
}
impl ::core::marker::Copy for RECO_LATTICE_COLUMN {}
impl ::core::clone::Clone for RECO_LATTICE_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_LATTICE_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_COLUMN").field("key", &self.key).field("cpProp", &self.cpProp).field("cStrokes", &self.cStrokes).field("pStrokes", &self.pStrokes).field("cLatticeElements", &self.cLatticeElements).field("pLatticeElements", &self.pLatticeElements).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_LATTICE_COLUMN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_LATTICE_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_LATTICE_COLUMN>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_COLUMN {}
impl ::core::default::Default for RECO_LATTICE_COLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_LATTICE_ELEMENT {
    pub score: i32,
    pub r#type: u16,
    pub pData: *mut u8,
    pub ulNextColumn: u32,
    pub ulStrokeNumber: u32,
    pub epProp: RECO_LATTICE_PROPERTIES,
}
impl ::core::marker::Copy for RECO_LATTICE_ELEMENT {}
impl ::core::clone::Clone for RECO_LATTICE_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_LATTICE_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_ELEMENT").field("score", &self.score).field("type", &self.r#type).field("pData", &self.pData).field("ulNextColumn", &self.ulNextColumn).field("ulStrokeNumber", &self.ulStrokeNumber).field("epProp", &self.epProp).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_LATTICE_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_LATTICE_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_LATTICE_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_ELEMENT {}
impl ::core::default::Default for RECO_LATTICE_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_LATTICE_PROPERTIES {
    pub cProperties: u32,
    pub apProps: *mut *mut RECO_LATTICE_PROPERTY,
}
impl ::core::marker::Copy for RECO_LATTICE_PROPERTIES {}
impl ::core::clone::Clone for RECO_LATTICE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_LATTICE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_PROPERTIES").field("cProperties", &self.cProperties).field("apProps", &self.apProps).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_LATTICE_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_LATTICE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_LATTICE_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_PROPERTIES {}
impl ::core::default::Default for RECO_LATTICE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_LATTICE_PROPERTY {
    pub guidProperty: ::windows_core::GUID,
    pub cbPropertyValue: u16,
    pub pPropertyValue: *mut u8,
}
impl ::core::marker::Copy for RECO_LATTICE_PROPERTY {}
impl ::core::clone::Clone for RECO_LATTICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_LATTICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_LATTICE_PROPERTY").field("guidProperty", &self.guidProperty).field("cbPropertyValue", &self.cbPropertyValue).field("pPropertyValue", &self.pPropertyValue).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_LATTICE_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_LATTICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_LATTICE_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_LATTICE_PROPERTY {}
impl ::core::default::Default for RECO_LATTICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECO_RANGE {
    pub iwcBegin: u32,
    pub cCount: u32,
}
impl ::core::marker::Copy for RECO_RANGE {}
impl ::core::clone::Clone for RECO_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECO_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECO_RANGE").field("iwcBegin", &self.iwcBegin).field("cCount", &self.cCount).finish()
    }
}
unsafe impl ::windows_core::Abi for RECO_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECO_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECO_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECO_RANGE {}
impl ::core::default::Default for RECO_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RF_ADVISEINKCHANGE: i32 = 4096i32;
pub const RF_ARBITRARY_ANGLE: i32 = 1024i32;
pub const RF_BOXED_INPUT: i32 = 16i32;
pub const RF_CAC_INPUT: i32 = 32i32;
pub const RF_DONTCARE: i32 = 1i32;
pub const RF_DOWN_AND_LEFT: i32 = 256i32;
pub const RF_DOWN_AND_RIGHT: i32 = 512i32;
pub const RF_FREE_INPUT: i32 = 4i32;
pub const RF_LATTICE: i32 = 2048i32;
pub const RF_LEFT_AND_DOWN: i32 = 128i32;
pub const RF_LINED_INPUT: i32 = 8i32;
pub const RF_OBJECT: i32 = 2i32;
pub const RF_PERFORMSLINEBREAKING: i32 = 65536i32;
pub const RF_PERSONALIZABLE: i32 = 16384i32;
pub const RF_REQUIRESSEGMENTATIONBREAKING: i32 = 131072i32;
pub const RF_RIGHT_AND_DOWN: i32 = 64i32;
pub const RF_STROKEREORDER: i32 = 8192i32;
pub const RealTimeStylus: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe26b366d_f998_43ce_836f_cb6d904432b0);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RealTimeStylusDataInterest(pub i32);
pub const RTSDI_AllData: RealTimeStylusDataInterest = RealTimeStylusDataInterest(-1i32);
pub const RTSDI_None: RealTimeStylusDataInterest = RealTimeStylusDataInterest(0i32);
pub const RTSDI_Error: RealTimeStylusDataInterest = RealTimeStylusDataInterest(1i32);
pub const RTSDI_RealTimeStylusEnabled: RealTimeStylusDataInterest = RealTimeStylusDataInterest(2i32);
pub const RTSDI_RealTimeStylusDisabled: RealTimeStylusDataInterest = RealTimeStylusDataInterest(4i32);
pub const RTSDI_StylusNew: RealTimeStylusDataInterest = RealTimeStylusDataInterest(8i32);
pub const RTSDI_StylusInRange: RealTimeStylusDataInterest = RealTimeStylusDataInterest(16i32);
pub const RTSDI_InAirPackets: RealTimeStylusDataInterest = RealTimeStylusDataInterest(32i32);
pub const RTSDI_StylusOutOfRange: RealTimeStylusDataInterest = RealTimeStylusDataInterest(64i32);
pub const RTSDI_StylusDown: RealTimeStylusDataInterest = RealTimeStylusDataInterest(128i32);
pub const RTSDI_Packets: RealTimeStylusDataInterest = RealTimeStylusDataInterest(256i32);
pub const RTSDI_StylusUp: RealTimeStylusDataInterest = RealTimeStylusDataInterest(512i32);
pub const RTSDI_StylusButtonUp: RealTimeStylusDataInterest = RealTimeStylusDataInterest(1024i32);
pub const RTSDI_StylusButtonDown: RealTimeStylusDataInterest = RealTimeStylusDataInterest(2048i32);
pub const RTSDI_SystemEvents: RealTimeStylusDataInterest = RealTimeStylusDataInterest(4096i32);
pub const RTSDI_TabletAdded: RealTimeStylusDataInterest = RealTimeStylusDataInterest(8192i32);
pub const RTSDI_TabletRemoved: RealTimeStylusDataInterest = RealTimeStylusDataInterest(16384i32);
pub const RTSDI_CustomStylusDataAdded: RealTimeStylusDataInterest = RealTimeStylusDataInterest(32768i32);
pub const RTSDI_UpdateMapping: RealTimeStylusDataInterest = RealTimeStylusDataInterest(65536i32);
pub const RTSDI_DefaultEvents: RealTimeStylusDataInterest = RealTimeStylusDataInterest(37766i32);
impl ::core::marker::Copy for RealTimeStylusDataInterest {}
impl ::core::clone::Clone for RealTimeStylusDataInterest {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RealTimeStylusDataInterest {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RealTimeStylusDataInterest {
    type Abi = Self;
}
impl ::core::fmt::Debug for RealTimeStylusDataInterest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RealTimeStylusDataInterest").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RealTimeStylusLockType(pub i32);
pub const RTSLT_ObjLock: RealTimeStylusLockType = RealTimeStylusLockType(1i32);
pub const RTSLT_SyncEventLock: RealTimeStylusLockType = RealTimeStylusLockType(2i32);
pub const RTSLT_AsyncEventLock: RealTimeStylusLockType = RealTimeStylusLockType(4i32);
pub const RTSLT_ExcludeCallback: RealTimeStylusLockType = RealTimeStylusLockType(8i32);
pub const RTSLT_SyncObjLock: RealTimeStylusLockType = RealTimeStylusLockType(11i32);
pub const RTSLT_AsyncObjLock: RealTimeStylusLockType = RealTimeStylusLockType(13i32);
impl ::core::marker::Copy for RealTimeStylusLockType {}
impl ::core::clone::Clone for RealTimeStylusLockType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RealTimeStylusLockType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RealTimeStylusLockType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RealTimeStylusLockType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RealTimeStylusLockType").field(&self.0).finish()
    }
}
pub const SAFE_PARTIAL: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCROLLDIRECTION(pub i32);
pub const SCROLLDIRECTION_UP: SCROLLDIRECTION = SCROLLDIRECTION(0i32);
pub const SCROLLDIRECTION_DOWN: SCROLLDIRECTION = SCROLLDIRECTION(1i32);
impl ::core::marker::Copy for SCROLLDIRECTION {}
impl ::core::clone::Clone for SCROLLDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCROLLDIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SCROLLDIRECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCROLLDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLDIRECTION").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct STROKE_RANGE {
    pub iStrokeBegin: u32,
    pub iStrokeEnd: u32,
}
impl ::core::marker::Copy for STROKE_RANGE {}
impl ::core::clone::Clone for STROKE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STROKE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STROKE_RANGE").field("iStrokeBegin", &self.iStrokeBegin).field("iStrokeEnd", &self.iStrokeEnd).finish()
    }
}
unsafe impl ::windows_core::Abi for STROKE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STROKE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STROKE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for STROKE_RANGE {}
impl ::core::default::Default for STROKE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const STR_GUID_ALTITUDEORIENTATION: &str = "{82DEC5C7-F6BA-4906-894F-66D68DFC456C}";
pub const STR_GUID_AZIMUTHORIENTATION: &str = "{029123B4-8828-410B-B250-A0536595E5DC}";
pub const STR_GUID_BUTTONPRESSURE: &str = "{8B7FEFC4-96AA-4BFE-AC26-8A5F0BE07BF5}";
pub const STR_GUID_DEVICE_CONTACT_ID: &str = "{02585B91-049B-4750-9615-DF8948AB3C9C}";
pub const STR_GUID_FINGERCONTACTCONFIDENCE: &str = "{E706C804-57F0-4F00-8A0C-853D57789BE9}";
pub const STR_GUID_HEIGHT: &str = "{E61858D2-E447-4218-9D3F-18865C203DF4}";
pub const STR_GUID_NORMALPRESSURE: &str = "{7307502D-F9F4-4E18-B3F2-2CE1B1A3610C}";
pub const STR_GUID_PAKETSTATUS: &str = "{6E0E07BF-AFE7-4CF7-87D1-AF6446208418}";
pub const STR_GUID_PITCHROTATION: &str = "{7F7E57B7-BE37-4BE1-A356-7A84160E1893}";
pub const STR_GUID_ROLLROTATION: &str = "{5D5D5E56-6BA9-4C5B-9FB0-851C91714E56}";
pub const STR_GUID_SERIALNUMBER: &str = "{78A81B56-0935-4493-BAAE-00541A8A16C4}";
pub const STR_GUID_TANGENTPRESSURE: &str = "{6DA4488B-5244-41EC-905B-32D89AB80809}";
pub const STR_GUID_TIMERTICK: &str = "{436510C5-FED3-45D1-8B76-71D3EA7A829D}";
pub const STR_GUID_TWISTORIENTATION: &str = "{0D324960-13B2-41E4-ACE6-7AE9D43D2D3B}";
pub const STR_GUID_WIDTH: &str = "{BAABE94D-2712-48F5-BE9D-8F8B5EA0711A}";
pub const STR_GUID_X: &str = "{598A6A8F-52C0-4BA0-93AF-AF357411A561}";
pub const STR_GUID_XTILTORIENTATION: &str = "{A8D07B3A-8BF0-40B0-95A9-B80A6BB787BF}";
pub const STR_GUID_Y: &str = "{B53F9F75-04E0-4498-A7EE-C30DBB5A9011}";
pub const STR_GUID_YAWROTATION: &str = "{6A849980-7C3A-45B7-AA82-90A262950E89}";
pub const STR_GUID_YTILTORIENTATION: &str = "{0E932389-1D77-43AF-AC00-5B950D6D4B2D}";
pub const STR_GUID_Z: &str = "{735ADB30-0EBB-4788-A0E4-0F316490055D}";
#[repr(C)]
pub struct SYSTEM_EVENT_DATA {
    pub bModifier: u8,
    pub wKey: u16,
    pub xPos: i32,
    pub yPos: i32,
    pub bCursorMode: u8,
    pub dwButtonState: u32,
}
impl ::core::marker::Copy for SYSTEM_EVENT_DATA {}
impl ::core::clone::Clone for SYSTEM_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_EVENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_EVENT_DATA").field("bModifier", &self.bModifier).field("wKey", &self.wKey).field("xPos", &self.xPos).field("yPos", &self.yPos).field("bCursorMode", &self.bCursorMode).field("dwButtonState", &self.dwButtonState).finish()
    }
}
unsafe impl ::windows_core::Abi for SYSTEM_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYSTEM_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYSTEM_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYSTEM_EVENT_DATA {}
impl ::core::default::Default for SYSTEM_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ScrollBarsConstants(pub i32);
pub const rtfNone: ScrollBarsConstants = ScrollBarsConstants(0i32);
pub const rtfHorizontal: ScrollBarsConstants = ScrollBarsConstants(1i32);
pub const rtfVertical: ScrollBarsConstants = ScrollBarsConstants(2i32);
pub const rtfBoth: ScrollBarsConstants = ScrollBarsConstants(3i32);
impl ::core::marker::Copy for ScrollBarsConstants {}
impl ::core::clone::Clone for ScrollBarsConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollBarsConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ScrollBarsConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for ScrollBarsConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollBarsConstants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SelAlignmentConstants(pub i32);
pub const rtfLeft: SelAlignmentConstants = SelAlignmentConstants(0i32);
pub const rtfRight: SelAlignmentConstants = SelAlignmentConstants(1i32);
pub const rtfCenter: SelAlignmentConstants = SelAlignmentConstants(2i32);
impl ::core::marker::Copy for SelAlignmentConstants {}
impl ::core::clone::Clone for SelAlignmentConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SelAlignmentConstants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SelAlignmentConstants {
    type Abi = Self;
}
impl ::core::fmt::Debug for SelAlignmentConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelAlignmentConstants").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SelectionHitResult(pub i32);
pub const SHR_None: SelectionHitResult = SelectionHitResult(0i32);
pub const SHR_NW: SelectionHitResult = SelectionHitResult(1i32);
pub const SHR_SE: SelectionHitResult = SelectionHitResult(2i32);
pub const SHR_NE: SelectionHitResult = SelectionHitResult(3i32);
pub const SHR_SW: SelectionHitResult = SelectionHitResult(4i32);
pub const SHR_E: SelectionHitResult = SelectionHitResult(5i32);
pub const SHR_W: SelectionHitResult = SelectionHitResult(6i32);
pub const SHR_N: SelectionHitResult = SelectionHitResult(7i32);
pub const SHR_S: SelectionHitResult = SelectionHitResult(8i32);
pub const SHR_Selection: SelectionHitResult = SelectionHitResult(9i32);
impl ::core::marker::Copy for SelectionHitResult {}
impl ::core::clone::Clone for SelectionHitResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SelectionHitResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SelectionHitResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for SelectionHitResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionHitResult").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn SetEnabledUnicodeRanges<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnabledUnicodeRanges(hrc: HRECOCONTEXT, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows_core::HRESULT;
        }
        SetEnabledUnicodeRanges(hrc.into_param().abi(), ::core::mem::transmute(cranges), ::core::mem::transmute(pcr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetFactoid<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hrc: Param0, cwcfactoid: u32, pwcfactoid: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFactoid(hrc: HRECOCONTEXT, cwcfactoid: u32, pwcfactoid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        SetFactoid(hrc.into_param().abi(), ::core::mem::transmute(cwcfactoid), pwcfactoid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetFlags<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, dwflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFlags(hrc: HRECOCONTEXT, dwflags: u32) -> ::windows_core::HRESULT;
        }
        SetFlags(hrc.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetGuide<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetGuide(hrc: HRECOCONTEXT, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows_core::HRESULT;
        }
        SetGuide(hrc.into_param().abi(), ::core::mem::transmute(pguide), ::core::mem::transmute(iindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetTextContext<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pwcbefore: &[u16], pwcafter: &[u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTextContext(hrc: HRECOCONTEXT, cwcbefore: u32, pwcbefore: ::windows_core::PCWSTR, cwcafter: u32, pwcafter: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        SetTextContext(hrc.into_param().abi(), pwcbefore.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pwcbefore)), pwcafter.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pwcafter))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetWordList<'a, Param0: ::windows_core::IntoParam<'a, HRECOCONTEXT>, Param1: ::windows_core::IntoParam<'a, HRECOWORDLIST>>(hrc: Param0, hwl: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWordList(hrc: HRECOCONTEXT, hwl: HRECOWORDLIST) -> ::windows_core::HRESULT;
        }
        SetWordList(hrc.into_param().abi(), hwl.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SketchInk: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0291081_e87c_4e07_97da_a0a03761e586);
pub const StrokeBuilder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe810cee7_6e51_4cb0_aa3a_0b985b70daf7);
#[repr(C)]
pub struct StylusInfo {
    pub tcid: u32,
    pub cid: u32,
    pub bIsInvertedCursor: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for StylusInfo {}
impl ::core::clone::Clone for StylusInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StylusInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StylusInfo").field("tcid", &self.tcid).field("cid", &self.cid).field("bIsInvertedCursor", &self.bIsInvertedCursor).finish()
    }
}
unsafe impl ::windows_core::Abi for StylusInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for StylusInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<StylusInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for StylusInfo {}
impl ::core::default::Default for StylusInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StylusQueue(pub i32);
pub const SyncStylusQueue: StylusQueue = StylusQueue(1i32);
pub const AsyncStylusQueueImmediate: StylusQueue = StylusQueue(2i32);
pub const AsyncStylusQueue: StylusQueue = StylusQueue(3i32);
impl ::core::marker::Copy for StylusQueue {}
impl ::core::clone::Clone for StylusQueue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StylusQueue {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StylusQueue {
    type Abi = Self;
}
impl ::core::fmt::Debug for StylusQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StylusQueue").field(&self.0).finish()
    }
}
pub const TABLET_DISABLE_FLICKFALLBACKKEYS: u32 = 1048576u32;
pub const TABLET_DISABLE_FLICKS: u32 = 65536u32;
pub const TABLET_DISABLE_PENBARRELFEEDBACK: u32 = 16u32;
pub const TABLET_DISABLE_PENTAPFEEDBACK: u32 = 8u32;
pub const TABLET_DISABLE_PRESSANDHOLD: u32 = 1u32;
pub const TABLET_DISABLE_SMOOTHSCROLLING: u32 = 524288u32;
pub const TABLET_DISABLE_TOUCHSWITCH: u32 = 32768u32;
pub const TABLET_DISABLE_TOUCHUIFORCEOFF: u32 = 512u32;
pub const TABLET_DISABLE_TOUCHUIFORCEON: u32 = 256u32;
pub const TABLET_ENABLE_FLICKLEARNINGMODE: u32 = 262144u32;
pub const TABLET_ENABLE_FLICKSONCONTEXT: u32 = 131072u32;
pub const TABLET_ENABLE_MULTITOUCHDATA: u32 = 16777216u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TabletDeviceKind(pub i32);
pub const TDK_Mouse: TabletDeviceKind = TabletDeviceKind(0i32);
pub const TDK_Pen: TabletDeviceKind = TabletDeviceKind(1i32);
pub const TDK_Touch: TabletDeviceKind = TabletDeviceKind(2i32);
impl ::core::marker::Copy for TabletDeviceKind {}
impl ::core::clone::Clone for TabletDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TabletDeviceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TabletDeviceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for TabletDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabletDeviceKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TabletHardwareCapabilities(pub i32);
pub const THWC_Integrated: TabletHardwareCapabilities = TabletHardwareCapabilities(1i32);
pub const THWC_CursorMustTouch: TabletHardwareCapabilities = TabletHardwareCapabilities(2i32);
pub const THWC_HardProximity: TabletHardwareCapabilities = TabletHardwareCapabilities(4i32);
pub const THWC_CursorsHavePhysicalIds: TabletHardwareCapabilities = TabletHardwareCapabilities(8i32);
impl ::core::marker::Copy for TabletHardwareCapabilities {}
impl ::core::clone::Clone for TabletHardwareCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TabletHardwareCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TabletHardwareCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for TabletHardwareCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabletHardwareCapabilities").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TabletPropertyMetricUnit(pub i32);
pub const TPMU_Default: TabletPropertyMetricUnit = TabletPropertyMetricUnit(0i32);
pub const TPMU_Inches: TabletPropertyMetricUnit = TabletPropertyMetricUnit(1i32);
pub const TPMU_Centimeters: TabletPropertyMetricUnit = TabletPropertyMetricUnit(2i32);
pub const TPMU_Degrees: TabletPropertyMetricUnit = TabletPropertyMetricUnit(3i32);
pub const TPMU_Radians: TabletPropertyMetricUnit = TabletPropertyMetricUnit(4i32);
pub const TPMU_Seconds: TabletPropertyMetricUnit = TabletPropertyMetricUnit(5i32);
pub const TPMU_Pounds: TabletPropertyMetricUnit = TabletPropertyMetricUnit(6i32);
pub const TPMU_Grams: TabletPropertyMetricUnit = TabletPropertyMetricUnit(7i32);
impl ::core::marker::Copy for TabletPropertyMetricUnit {}
impl ::core::clone::Clone for TabletPropertyMetricUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TabletPropertyMetricUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TabletPropertyMetricUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for TabletPropertyMetricUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabletPropertyMetricUnit").field(&self.0).finish()
    }
}
pub const TextInputPanel: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9b189d7_228b_4f2b_8650_b97f59e02c8c);
pub const TipAutoCompleteClient: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x807c1e6c_1d00_453f_b920_b61bb7cdd997);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VisualState(pub i32);
pub const InPlace: VisualState = VisualState(0i32);
pub const Floating: VisualState = VisualState(1i32);
pub const DockedTop: VisualState = VisualState(2i32);
pub const DockedBottom: VisualState = VisualState(3i32);
pub const Closed: VisualState = VisualState(4i32);
impl ::core::marker::Copy for VisualState {}
impl ::core::clone::Clone for VisualState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisualState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VisualState {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisualState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualState").field(&self.0).finish()
    }
}
pub const WM_TABLET_ADDED: u32 = 712u32;
pub const WM_TABLET_DEFBASE: u32 = 704u32;
pub const WM_TABLET_DELETED: u32 = 713u32;
pub const WM_TABLET_FLICK: u32 = 715u32;
pub const WM_TABLET_MAXOFFSET: u32 = 32u32;
pub const WM_TABLET_QUERYSYSTEMGESTURESTATUS: u32 = 716u32;
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkCollectorEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkCollectorEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkCollectorEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkCollectorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkCollectorEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkCollectorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkCollectorEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkCollectorEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkCollectorEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkCollectorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkCollectorEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkCollectorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkCollectorEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkCollectorEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkCollectorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkCollectorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkCollectorEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkCollectorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkCollectorEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkCollectorEvents {
    type Vtable = _IInkCollectorEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11a583f2_712d_4fea_abcf_ab4af38ea06b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkCollectorEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkEditEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkEditEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkEditEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkEditEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkEditEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkEditEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkEditEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkEditEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkEditEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkEditEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkEditEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkEditEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkEditEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkEditEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkEditEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkEditEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkEditEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkEditEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkEditEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkEditEvents {
    type Vtable = _IInkEditEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3b0b797_a72e_46db_a0d7_6c9eba8e9bbc);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkEditEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkEvents {
    type Vtable = _IInkEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x427b1865_ca3f_479a_83a9_0f420f2a0073);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkOverlayEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkOverlayEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkOverlayEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkOverlayEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkOverlayEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkOverlayEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkOverlayEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkOverlayEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkOverlayEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkOverlayEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkOverlayEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkOverlayEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkOverlayEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkOverlayEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkOverlayEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkOverlayEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkOverlayEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkOverlayEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkOverlayEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkOverlayEvents {
    type Vtable = _IInkOverlayEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31179b69_e563_489e_b16f_712f1e8a0651);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkOverlayEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkPictureEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkPictureEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkPictureEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkPictureEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkPictureEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkPictureEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkPictureEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkPictureEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkPictureEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkPictureEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkPictureEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkPictureEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkPictureEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkPictureEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkPictureEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkPictureEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkPictureEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkPictureEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkPictureEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkPictureEvents {
    type Vtable = _IInkPictureEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60ff4fee_22ff_4484_acc1_d308d9cd7ea3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkPictureEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkRecognitionEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkRecognitionEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkRecognitionEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkRecognitionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkRecognitionEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkRecognitionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkRecognitionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkRecognitionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkRecognitionEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkRecognitionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkRecognitionEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkRecognitionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkRecognitionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkRecognitionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkRecognitionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkRecognitionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkRecognitionEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkRecognitionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkRecognitionEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkRecognitionEvents {
    type Vtable = _IInkRecognitionEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17bce92f_2e21_47fd_9d33_3c6afbfd8c59);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkRecognitionEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IInkStrokesEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IInkStrokesEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkStrokesEvents> for ::windows_core::IUnknown {
    fn from(value: _IInkStrokesEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkStrokesEvents> for ::windows_core::IUnknown {
    fn from(value: &_IInkStrokesEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IInkStrokesEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IInkStrokesEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IInkStrokesEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IInkStrokesEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IInkStrokesEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IInkStrokesEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IInkStrokesEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IInkStrokesEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IInkStrokesEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IInkStrokesEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IInkStrokesEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IInkStrokesEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IInkStrokesEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IInkStrokesEvents {
    type Vtable = _IInkStrokesEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf33053ec_5d25_430a_928f_76a6491dde15);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IInkStrokesEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IMathInputControlEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IMathInputControlEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IMathInputControlEvents> for ::windows_core::IUnknown {
    fn from(value: _IMathInputControlEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IMathInputControlEvents> for ::windows_core::IUnknown {
    fn from(value: &_IMathInputControlEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IMathInputControlEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IMathInputControlEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IMathInputControlEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IMathInputControlEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IMathInputControlEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IMathInputControlEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IMathInputControlEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IMathInputControlEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IMathInputControlEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IMathInputControlEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IMathInputControlEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IMathInputControlEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IMathInputControlEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IMathInputControlEvents {
    type Vtable = _IMathInputControlEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683336b5_a47d_4358_96f9_875a472ae70a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IMathInputControlEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct _IPenInputPanelEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl _IPenInputPanelEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IPenInputPanelEvents> for ::windows_core::IUnknown {
    fn from(value: _IPenInputPanelEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IPenInputPanelEvents> for ::windows_core::IUnknown {
    fn from(value: &_IPenInputPanelEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for _IPenInputPanelEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a _IPenInputPanelEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<_IPenInputPanelEvents> for ::win32_system::Com::IDispatch {
    fn from(value: _IPenInputPanelEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&_IPenInputPanelEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &_IPenInputPanelEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for _IPenInputPanelEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a _IPenInputPanelEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for _IPenInputPanelEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for _IPenInputPanelEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for _IPenInputPanelEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for _IPenInputPanelEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IPenInputPanelEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for _IPenInputPanelEvents {
    type Vtable = _IPenInputPanelEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7e489da_3719_439f_848f_e7acbd820f17);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct _IPenInputPanelEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct enumGetCandidateFlags(pub i32);
pub const TCF_ALLOW_RECOGNITION: enumGetCandidateFlags = enumGetCandidateFlags(1i32);
pub const TCF_FORCE_RECOGNITION: enumGetCandidateFlags = enumGetCandidateFlags(2i32);
impl ::core::marker::Copy for enumGetCandidateFlags {}
impl ::core::clone::Clone for enumGetCandidateFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for enumGetCandidateFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for enumGetCandidateFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for enumGetCandidateFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("enumGetCandidateFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct enumINKMETRIC_FLAGS(pub i32);
pub const IMF_FONT_SELECTED_IN_HDC: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(1i32);
pub const IMF_ITALIC: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(2i32);
pub const IMF_BOLD: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(4i32);
impl ::core::marker::Copy for enumINKMETRIC_FLAGS {}
impl ::core::clone::Clone for enumINKMETRIC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for enumINKMETRIC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for enumINKMETRIC_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for enumINKMETRIC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("enumINKMETRIC_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct enumRECO_TYPE(pub i32);
pub const RECO_TYPE_WSTRING: enumRECO_TYPE = enumRECO_TYPE(0i32);
pub const RECO_TYPE_WCHAR: enumRECO_TYPE = enumRECO_TYPE(1i32);
impl ::core::marker::Copy for enumRECO_TYPE {}
impl ::core::clone::Clone for enumRECO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for enumRECO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for enumRECO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for enumRECO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("enumRECO_TYPE").field(&self.0).finish()
    }
}
