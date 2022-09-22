#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoCaptureSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedPhotoCaptureSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f3863a_0018_445b_93d2_646d1c5ed05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCaptureSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AdvancedPhotoMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedPhotoControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b15486_9001_4682_9309_68eae0080eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController {
    type Vtable = IAdvancedVideoCaptureDeviceController_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde6ff4d3_2b96_4583_80ab_b5b01dc6a8d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetDeviceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController10(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController10 {
    type Vtable = IAdvancedVideoCaptureDeviceController10_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController10 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc621b82d_d6f0_5c1b_a388_a6e938407146);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController10_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CameraOcclusionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController2 {
    type Vtable = IAdvancedVideoCaptureDeviceController2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb94f8f_f11a_43db_b402_11930b80ae56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LowLagPhotoSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LowLagPhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SceneModeControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TorchControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FlashControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WhiteBalanceControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExposureControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegionsOfInterestControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrimaryUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CaptureUse) -> ::windows::core::HRESULT,
    pub SetPrimaryUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CaptureUse) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController3 {
    type Vtable = IAdvancedVideoCaptureDeviceController3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa98b8f34_ee0d_470c_b9f0_4229c4bbd089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices_Core")]
    pub VariablePhotoSequenceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    VariablePhotoSequenceController: usize,
    pub PhotoConfirmationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ZoomControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController4 {
    type Vtable = IAdvancedVideoCaptureDeviceController4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9fbfaf_d371_41c3_9a17_824a87ebdfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExposurePriorityVideoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DesiredOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureOptimization) -> ::windows::core::HRESULT,
    pub SetDesiredOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureOptimization) -> ::windows::core::HRESULT,
    pub HdrVideoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpticalImageStabilizationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdvancedPhotoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController5 {
    type Vtable = IAdvancedVideoCaptureDeviceController5_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33512b17_b9cb_4a23_b875_f9eaab535492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyById: usize,
    pub SetDevicePropertyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyByExtendedId: usize,
    pub SetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController6 {
    type Vtable = IAdvancedVideoCaptureDeviceController6_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6563a53_68a1_44b7_9f89_b5fa97ac0cbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VideoTemporalDenoisingControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController7(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController7 {
    type Vtable = IAdvancedVideoCaptureDeviceController7_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d2927f0_a054_50e7_b7df_7c04234d10f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController7_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InfraredTorchControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController8(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController8 {
    type Vtable = IAdvancedVideoCaptureDeviceController8_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd843f010_e7fb_595b_9a78_0e54c4532b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController8_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PanelBasedOptimizationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController9(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvancedVideoCaptureDeviceController9 {
    type Vtable = IAdvancedVideoCaptureDeviceController9_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController9 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdca95d_0255_51bc_a10d_5a169ec1625a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController9_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DigitalWindowControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioDeviceController {
    type Vtable = IAudioDeviceController_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioDeviceController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedd4a388_79c7_4f7c_90e8_ef934b21580a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Muted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetVolumePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub VolumePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModule(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioDeviceModule {
    type Vtable = IAudioDeviceModule_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioDeviceModule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86cfac36_47c1_4b33_9852_8773ec4be123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModule_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendCommandAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModuleNotificationEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioDeviceModuleNotificationEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3e3ccaf_224c_48be_956b_9a13134e96e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModuleNotificationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Module: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub NotificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    NotificationData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModulesManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioDeviceModulesManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aa40c4d_960a_4d1c_b318_0022604547ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ModuleNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveModuleNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllById: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModulesManagerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAudioDeviceModulesManagerFactory {
    type Vtable = IAudioDeviceModulesManagerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAudioDeviceModulesManagerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8db03670_e64d_4773_96c0_bc7ebf0e063f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManagerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICallControl {
    type Vtable = ICallControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ICallControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa520d0d6_ae8d_45db_8011_ca49d3b3e578);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IndicateNewIncomingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enableringer: bool, callerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IndicateNewOutgoingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IndicateActiveCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows::core::HRESULT,
    pub EndCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows::core::HRESULT,
    pub HasRinger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HangUpRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHangUpRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RedialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRedialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub KeypadPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeypadPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub AudioTransferRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioTransferRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioTransferRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioTransferRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallControlStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICallControlStatics {
    type Vtable = ICallControlStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICallControlStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03945ad5_85ab_40e1_af19_56c94303b019);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallControlStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOcclusionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ICameraOcclusionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6c4ad0_a84d_5db6_be58_a5da21cfe011);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOcclusionKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOcclusionState(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICameraOcclusionState {
    type Vtable = ICameraOcclusionState_Vtbl;
}
unsafe impl ::windows::core::Interface for ICameraOcclusionState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x430adeb8_6842_5e55_9bde_04b4ef3a8a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionState_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsOccluded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOcclusionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOcclusionStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICameraOcclusionStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8512d848_c0de_57ca_a1ca_fb2c3d23df55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct IDefaultAudioDeviceChangedEventArgs(::windows::core::IUnknown);
impl IDefaultAudioDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Role(&self) -> ::windows::core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Role)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceRole>(result__)
        }
    }
}
impl ::core::convert::From<IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDefaultAudioDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDefaultAudioDeviceChangedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDefaultAudioDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDefaultAudioDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDefaultAudioDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDefaultAudioDeviceChangedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IDefaultAudioDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDefaultAudioDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDefaultAudioDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDefaultAudioDeviceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultAudioDeviceChangedEventArgs {}
impl ::core::fmt::Debug for IDefaultAudioDeviceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultAudioDeviceChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDefaultAudioDeviceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{110f882f-1c05-4657-a18e-47c9b69f07ab}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IDefaultAudioDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDefaultAudioDeviceChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110f882f_1c05_4657_a18e_47c9b69f07ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultAudioDeviceChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceRole) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDialRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x037b929e_953c_4286_8866_4f0f376c855a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDigitalWindowBounds(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_Vtbl;
}
unsafe impl ::windows::core::Interface for IDigitalWindowBounds {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4f21dd_d173_5c6b_8c25_bdd26d5122b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowBounds_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NormalizedOriginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetNormalizedOriginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub NormalizedOriginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetNormalizedOriginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDigitalWindowCapability(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_Vtbl;
}
unsafe impl ::windows::core::Interface for IDigitalWindowCapability {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78bad2c_f721_5244_a196_b56ccbec606c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowCapability_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MinScaleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxScaleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinScaleValueWithoutUpsampling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedFieldOfViewLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedFieldOfViewLimit: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDigitalWindowControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDigitalWindowControl {
    type Vtable = IDigitalWindowControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IDigitalWindowControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23b69eff_65d2_53ea_8780_de582b48b544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows::core::HRESULT,
    pub CurrentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DigitalWindowMode) -> ::windows::core::HRESULT,
    pub GetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode) -> ::windows::core::HRESULT,
    pub ConfigureWithBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCapabilities: usize,
    pub GetCapabilityForSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExposureCompensationControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IExposureCompensationControl {
    type Vtable = IExposureCompensationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IExposureCompensationControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c8e834_dcec_4011_a610_1f3847e64aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposureCompensationControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExposureControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IExposureControl {
    type Vtable = IExposureControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IExposureControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e8cbe2_ad96_4f28_a0e0_96ed7e1b5fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposureControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutterduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExposurePriorityVideoControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IExposurePriorityVideoControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb240a3_5168_4271_9ea5_47621a98a352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposurePriorityVideoControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlashControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFlashControl {
    type Vtable = IFlashControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IFlashControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef41dbe_7d68_45e3_8c0f_be7bb32837d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlashControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlashControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFlashControl2 {
    type Vtable = IFlashControl2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFlashControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d29cc9e_75e1_4af7_bd7d_4e38e1c06cd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlashControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AssistantLightSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AssistantLightEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAssistantLightEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFocusControl {
    type Vtable = IFocusControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IFocusControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d889f6_5228_4453_b153_85606592b238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPresets: usize,
    pub Preset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusPreset) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: FocusPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPresetWithCompletionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: FocusPreset, completebeforefocus: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetWithCompletionOptionAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focus: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFocusControl2 {
    type Vtable = IFocusControl2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFocusControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7cff48_c534_4e9e_94c3_52ef2afd5d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FocusChangedSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub WaitForFocusSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusModes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusDistances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusDistances: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusRanges: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureFocusState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnlockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockAsync: usize,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFocusSettings {
    type Vtable = IFocusSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IFocusSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79958f6b_3263_4275_85d6_aeae891c96ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FocusMode) -> ::windows::core::HRESULT,
    pub AutoFocusRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoFocusRange) -> ::windows::core::HRESULT,
    pub SetAutoFocusRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoFocusRange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Foundation")]
    pub Distance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Distance: usize,
    #[cfg(feature = "Foundation")]
    pub SetDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDistance: usize,
    pub WaitForFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetWaitForFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisableDriverFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDisableDriverFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdrVideoControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHdrVideoControl {
    type Vtable = IHdrVideoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IHdrVideoControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55d8e2d0_30c0_43bf_9b9a_9799d70ced94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdrVideoControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdrVideoMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HdrVideoMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInfraredTorchControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInfraredTorchControl {
    type Vtable = IInfraredTorchControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IInfraredTorchControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cba2c83_6cb6_5a04_a6fc_3be7b33ff056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfraredTorchControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub CurrentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InfraredTorchMode) -> ::windows::core::HRESULT,
    pub SetCurrentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InfraredTorchMode) -> ::windows::core::HRESULT,
    pub MinPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PowerStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Power: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsoSpeedControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsoSpeedControl {
    type Vtable = IIsoSpeedControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsoSpeedControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27b6c322_25ad_4f1b_aaab_524ab376ca33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoSpeedControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedPresets: usize,
    #[cfg(feature = "deprecated")]
    pub Preset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsoSpeedPreset) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Preset: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: IsoSpeedPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPresetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsoSpeedControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IIsoSpeedControl2 {
    type Vtable = IIsoSpeedControl2_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsoSpeedControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f1578f2_6d77_4f8a_8c2f_6130b6395053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoSpeedControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isospeed: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeypadPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IKeypadPressedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3a43900_b4fa_49cd_9442_89af6568f601);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeypadPressedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TelephonyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TelephonyKey) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ILowLagPhotoControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5c4dd0_fadf_415d_aee6_3baa529300c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoSequenceControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ILowLagPhotoSequenceControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dcf909d_6d16_409c_bafe_b9a594c6fde6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxPastPhotos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub PastPhotoLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetPastPhotoLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaDeviceControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaDeviceControl {
    type Vtable = IMediaDeviceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaDeviceControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8dfa9_6f75_4863_ba0b_583f3036b4de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryGetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut bool, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrySetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaDeviceControlCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaDeviceControlCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23005816_eb85_43e2_b92b_8240d5ee70ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceControlCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AutoModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct IMediaDeviceController(::windows::core::IUnknown);
impl IMediaDeviceController {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAvailableMediaStreamProperties)(::windows::core::Vtable::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMediaStreamProperties)(::windows::core::Vtable::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<super::MediaProperties::IMediaEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<'a, P0, E0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::MediaProperties::IMediaEncodingProperties>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetMediaStreamPropertiesAsync)(::windows::core::Vtable::as_raw(this), mediastreamtype, mediaencodingproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::convert::From<IMediaDeviceController> for ::windows::core::IUnknown {
    fn from(value: IMediaDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMediaDeviceController> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMediaDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaDeviceController> for ::windows::core::IUnknown {
    fn from(value: &IMediaDeviceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMediaDeviceController> for ::windows::core::IInspectable {
    fn from(value: IMediaDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMediaDeviceController> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IMediaDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaDeviceController> for ::windows::core::IInspectable {
    fn from(value: &IMediaDeviceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMediaDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaDeviceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaDeviceController {}
impl ::core::fmt::Debug for IMediaDeviceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaDeviceController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaDeviceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f6f8f5ce-209a-48fb-86fc-d44578f317e6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMediaDeviceController {
    type Vtable = IMediaDeviceController_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaDeviceController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6f8f5ce_209a_48fb_86fc_d44578f317e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetAvailableMediaStreamProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetAvailableMediaStreamProperties: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetMediaStreamProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetMediaStreamProperties: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub SetMediaStreamPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    SetMediaStreamPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaDeviceStatics {
    type Vtable = IMediaDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa2d9a40_909f_4bba_bf8b_0c0d296f14f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetAudioCaptureSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAudioRenderSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetVideoCaptureSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDefaultAudioCaptureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDefaultAudioRenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioRenderDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioRenderDeviceChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IModuleCommandResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IModuleCommandResult {
    type Vtable = IModuleCommandResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IModuleCommandResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520d1eb4_1374_4c7d_b1e4_39dcdf3eae4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IModuleCommandResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SendCommandStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOpticalImageStabilizationControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IOpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IOpticalImageStabilizationControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfad9c1d_00bc_423b_8eb2_a0178ca94247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpticalImageStabilizationControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OpticalImageStabilizationMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: OpticalImageStabilizationMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPanelBasedOptimizationControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IPanelBasedOptimizationControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33323223_6247_5419_a5a4_3d808645d917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPanelBasedOptimizationControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Panel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Panel: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPanel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoConfirmationControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoConfirmationControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8f3f363_ff5e_4582_a9a8_0550f85a4a76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PixelFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPixelFormat: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRedialRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRedialRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eb55209_76ab_4c31_b40e_4b58379d580c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedialRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegionOfInterest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRegionOfInterest {
    type Vtable = IRegionOfInterest_Vtbl;
}
unsafe impl ::windows::core::Interface for IRegionOfInterest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5ecc834_ce66_4e05_a78f_cf391a5ec2d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionOfInterest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AutoFocusEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoFocusEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegionOfInterest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRegionOfInterest2 {
    type Vtable = IRegionOfInterest2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRegionOfInterest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19fe2a91_73aa_4d51_8a9d_56ccf7db7f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionOfInterest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RegionOfInterestType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RegionOfInterestType) -> ::windows::core::HRESULT,
    pub BoundsNormalized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBoundsNormalized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegionsOfInterestControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IRegionsOfInterestControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc323f527_ab0b_4558_8b5b_df5693db0378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionsOfInterestControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsWithLockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regions: *mut ::core::ffi::c_void, lockvalues: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsWithLockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearRegionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearRegionsAsync: usize,
    pub AutoFocusSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AutoWhiteBalanceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AutoExposureSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneModeControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneModeControl {
    type Vtable = ISceneModeControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneModeControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd48e5af7_8d59_4854_8c62_12c70ba89b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModeControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CaptureSceneMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenemode: CaptureSceneMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITorchControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITorchControl {
    type Vtable = ITorchControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ITorchControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6053665_8250_416c_919a_724296afa306);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITorchControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDeviceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVideoDeviceController {
    type Vtable = IVideoDeviceController_Vtbl;
}
unsafe impl ::windows::core::Interface for IVideoDeviceController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99555575_2e2e_40b8_b6c7_f82d10013210);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDeviceController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BacklightCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Zoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub TrySetPowerlineFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TrySetPowerlineFrequency: usize,
    #[cfg(feature = "Media_Capture")]
    pub TryGetPowerlineFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryGetPowerlineFrequency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDeviceControllerGetDevicePropertyResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IVideoDeviceControllerGetDevicePropertyResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5d88395_6ed5_4790_8b5d_0ef13935d0f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDeviceControllerGetDevicePropertyResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTemporalDenoisingControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IVideoTemporalDenoisingControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ab34735_3e2a_4a32_baff_4358c4fbdd57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTemporalDenoisingControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoTemporalDenoisingMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoTemporalDenoisingMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWhiteBalanceControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IWhiteBalanceControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781f047e_7162_49c8_a8f9_9481c565363e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWhiteBalanceControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Preset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ColorTemperaturePreset) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: ColorTemperaturePreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temperature: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IZoomControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IZoomControl {
    type Vtable = IZoomControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IZoomControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a1e0b12_32da_4c17_bfd7_8d0c73c8f5a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IZoomControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IZoomControl2 {
    type Vtable = IZoomControl2_Vtbl;
}
unsafe impl ::windows::core::Interface for IZoomControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69843db0_2e99_4641_8529_184f319d1671);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IZoomSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IZoomSettings {
    type Vtable = IZoomSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IZoomSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ad66b24_14b4_4bfd_b18f_88fe24463b52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ZoomTransitionMode) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoCaptureSettings(::windows::core::IUnknown);
impl AdvancedPhotoCaptureSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdvancedPhotoCaptureSettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AdvancedPhotoMode>(result__)
        }
    }
    pub fn SetMode(&self, value: AdvancedPhotoMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AdvancedPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvancedPhotoCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedPhotoCaptureSettings {}
impl ::core::fmt::Debug for AdvancedPhotoCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoCaptureSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AdvancedPhotoCaptureSettings;{08f3863a-0018-445b-93d2-646d1c5ed05c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for AdvancedPhotoCaptureSettings {
    const IID: ::windows::core::GUID = <IAdvancedPhotoCaptureSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdvancedPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoCaptureSettings";
}
impl ::core::convert::From<AdvancedPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: AdvancedPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AdvancedPhotoCaptureSettings> for &::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AdvancedPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: AdvancedPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AdvancedPhotoCaptureSettings> for &::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCaptureSettings {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoControl(::windows::core::IUnknown);
impl AdvancedPhotoControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AdvancedPhotoMode>(result__)
        }
    }
    pub fn Configure<'a, P0>(&self, settings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, AdvancedPhotoCaptureSettings>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Configure)(::windows::core::Vtable::as_raw(this), settings.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdvancedPhotoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvancedPhotoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedPhotoControl {}
impl ::core::fmt::Debug for AdvancedPhotoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AdvancedPhotoControl;{c5b15486-9001-4682-9309-68eae0080eec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for AdvancedPhotoControl {
    const IID: ::windows::core::GUID = <IAdvancedPhotoControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdvancedPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoControl";
}
impl ::core::convert::From<AdvancedPhotoControl> for ::windows::core::IUnknown {
    fn from(value: AdvancedPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoControl> for ::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AdvancedPhotoControl> for &::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AdvancedPhotoControl> for ::windows::core::IInspectable {
    fn from(value: AdvancedPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoControl> for ::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AdvancedPhotoControl> for &::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoControl {}
unsafe impl ::core::marker::Sync for AdvancedPhotoControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceController(::windows::core::IUnknown);
impl AudioDeviceController {
    pub fn SetMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMuted)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Muted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Muted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetVolumePercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVolumePercent)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn VolumePercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VolumePercent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAvailableMediaStreamProperties)(::windows::core::Vtable::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMediaStreamProperties)(::windows::core::Vtable::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<super::MediaProperties::IMediaEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<'a, P0, E0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::MediaProperties::IMediaEncodingProperties>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetMediaStreamPropertiesAsync)(::windows::core::Vtable::as_raw(this), mediastreamtype, mediaencodingproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceController {}
impl ::core::fmt::Debug for AudioDeviceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceController;{edd4a388-79c7-4f7c-90e8-ef934b21580a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AudioDeviceController {
    type Vtable = IAudioDeviceController_Vtbl;
}
unsafe impl ::windows::core::Interface for AudioDeviceController {
    const IID: ::windows::core::GUID = <IAudioDeviceController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceController";
}
impl ::core::convert::From<AudioDeviceController> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceController> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceController> for &::windows::core::IUnknown {
    fn from(value: &AudioDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AudioDeviceController> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceController> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceController> for &::windows::core::IInspectable {
    fn from(value: &AudioDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<AudioDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AudioDeviceController> for ::windows::core::InParam<'a, IMediaDeviceController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceModule(::windows::core::IUnknown);
impl AudioDeviceModule {
    pub fn ClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClassId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstanceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MajorVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MajorVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MinorVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinorVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendCommandAsync<'a, P0, E0>(&self, command: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendCommandAsync)(::windows::core::Vtable::as_raw(this), command.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioDeviceModule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceModule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceModule {}
impl ::core::fmt::Debug for AudioDeviceModule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceModule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceModule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModule;{86cfac36-47c1-4b33-9852-8773ec4be123})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AudioDeviceModule {
    type Vtable = IAudioDeviceModule_Vtbl;
}
unsafe impl ::windows::core::Interface for AudioDeviceModule {
    const IID: ::windows::core::GUID = <IAudioDeviceModule as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceModule {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModule";
}
impl ::core::convert::From<AudioDeviceModule> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceModule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceModule> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceModule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceModule> for &::windows::core::IUnknown {
    fn from(value: &AudioDeviceModule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AudioDeviceModule> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceModule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceModule> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceModule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceModule> for &::windows::core::IInspectable {
    fn from(value: &AudioDeviceModule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceModuleNotificationEventArgs(::windows::core::IUnknown);
impl AudioDeviceModuleNotificationEventArgs {
    pub fn Module(&self) -> ::windows::core::Result<AudioDeviceModule> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Module)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceModule>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn NotificationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioDeviceModuleNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceModuleNotificationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceModuleNotificationEventArgs {}
impl ::core::fmt::Debug for AudioDeviceModuleNotificationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceModuleNotificationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceModuleNotificationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs;{e3e3ccaf-224c-48be-956b-9a13134e96e8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AudioDeviceModuleNotificationEventArgs {
    const IID: ::windows::core::GUID = <IAudioDeviceModuleNotificationEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceModuleNotificationEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs";
}
impl ::core::convert::From<AudioDeviceModuleNotificationEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceModuleNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceModuleNotificationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceModuleNotificationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceModuleNotificationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &AudioDeviceModuleNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AudioDeviceModuleNotificationEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceModuleNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceModuleNotificationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceModuleNotificationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceModuleNotificationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &AudioDeviceModuleNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AudioDeviceModuleNotificationEventArgs {}
unsafe impl ::core::marker::Sync for AudioDeviceModuleNotificationEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceModulesManager(::windows::core::IUnknown);
impl AudioDeviceModulesManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ModuleNotificationReceived<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModuleNotificationReceived)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveModuleNotificationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveModuleNotificationReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllById(&self, moduleid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllById)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(moduleid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAll)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>(result__)
        }
    }
    pub fn Create(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioDeviceModulesManager> {
        Self::IAudioDeviceModulesManagerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi::<AudioDeviceModulesManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioDeviceModulesManagerFactory<R, F: FnOnce(&IAudioDeviceModulesManagerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AudioDeviceModulesManager, IAudioDeviceModulesManagerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AudioDeviceModulesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceModulesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceModulesManager {}
impl ::core::fmt::Debug for AudioDeviceModulesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceModulesManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceModulesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModulesManager;{6aa40c4d-960a-4d1c-b318-0022604547ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AudioDeviceModulesManager {
    const IID: ::windows::core::GUID = <IAudioDeviceModulesManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceModulesManager {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModulesManager";
}
impl ::core::convert::From<AudioDeviceModulesManager> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceModulesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceModulesManager> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceModulesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceModulesManager> for &::windows::core::IUnknown {
    fn from(value: &AudioDeviceModulesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AudioDeviceModulesManager> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceModulesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceModulesManager> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceModulesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AudioDeviceModulesManager> for &::windows::core::IInspectable {
    fn from(value: &AudioDeviceModulesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AudioDeviceModulesManager {}
unsafe impl ::core::marker::Sync for AudioDeviceModulesManager {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CallControl(::windows::core::IUnknown);
impl CallControl {
    pub fn IndicateNewIncomingCall(&self, enableringer: bool, callerid: &::windows::core::HSTRING) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndicateNewIncomingCall)(::windows::core::Vtable::as_raw(this), enableringer, ::core::mem::transmute_copy(callerid), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn IndicateNewOutgoingCall(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndicateNewOutgoingCall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn IndicateActiveCall(&self, calltoken: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).IndicateActiveCall)(::windows::core::Vtable::as_raw(this), calltoken).ok() }
    }
    pub fn EndCall(&self, calltoken: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).EndCall)(::windows::core::Vtable::as_raw(this), calltoken).ok() }
    }
    pub fn HasRinger(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasRinger)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnswerRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControlEventHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AnswerRequested)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnswerRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAnswerRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HangUpRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControlEventHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HangUpRequested)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHangUpRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveHangUpRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DialRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, DialRequestedEventHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DialRequested)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDialRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDialRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RedialRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, RedialRequestedEventHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RedialRequested)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRedialRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRedialRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeypadPressed<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, KeypadPressedEventHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeypadPressed)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeypadPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveKeypadPressed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioTransferRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControlEventHandler>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioTransferRequested)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioTransferRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAudioTransferRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CallControl>(result__)
        })
    }
    pub fn FromId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi::<CallControl>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICallControlStatics<R, F: FnOnce(&ICallControlStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CallControl, ICallControlStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CallControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CallControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallControl {}
impl ::core::fmt::Debug for CallControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CallControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CallControl;{a520d0d6-ae8d-45db-8011-ca49d3b3e578})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CallControl {
    type Vtable = ICallControl_Vtbl;
}
unsafe impl ::windows::core::Interface for CallControl {
    const IID: ::windows::core::GUID = <ICallControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CallControl {
    const NAME: &'static str = "Windows.Media.Devices.CallControl";
}
impl ::core::convert::From<CallControl> for ::windows::core::IUnknown {
    fn from(value: CallControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallControl> for ::windows::core::IUnknown {
    fn from(value: &CallControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CallControl> for &::windows::core::IUnknown {
    fn from(value: &CallControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CallControl> for ::windows::core::IInspectable {
    fn from(value: CallControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CallControl> for ::windows::core::IInspectable {
    fn from(value: &CallControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CallControl> for &::windows::core::IInspectable {
    fn from(value: &CallControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CallControl {}
unsafe impl ::core::marker::Sync for CallControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionInfo(::windows::core::IUnknown);
impl CameraOcclusionInfo {
    pub fn GetState(&self) -> ::windows::core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraOcclusionState>(result__)
        }
    }
    pub fn IsOcclusionKindSupported(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOcclusionKindSupported)(::windows::core::Vtable::as_raw(this), occlusionkind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StateChanged)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for CameraOcclusionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraOcclusionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraOcclusionInfo {}
impl ::core::fmt::Debug for CameraOcclusionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionInfo;{af6c4ad0-a84d-5db6-be58-a5da21cfe011})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for CameraOcclusionInfo {
    const IID: ::windows::core::GUID = <ICameraOcclusionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraOcclusionInfo {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionInfo";
}
impl ::core::convert::From<CameraOcclusionInfo> for ::windows::core::IUnknown {
    fn from(value: CameraOcclusionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraOcclusionInfo> for ::windows::core::IUnknown {
    fn from(value: &CameraOcclusionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraOcclusionInfo> for &::windows::core::IUnknown {
    fn from(value: &CameraOcclusionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CameraOcclusionInfo> for ::windows::core::IInspectable {
    fn from(value: CameraOcclusionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraOcclusionInfo> for ::windows::core::IInspectable {
    fn from(value: &CameraOcclusionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraOcclusionInfo> for &::windows::core::IInspectable {
    fn from(value: &CameraOcclusionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CameraOcclusionInfo {}
unsafe impl ::core::marker::Sync for CameraOcclusionInfo {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionState(::windows::core::IUnknown);
impl CameraOcclusionState {
    pub fn IsOccluded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOccluded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsOcclusionKind(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOcclusionKind)(::windows::core::Vtable::as_raw(this), occlusionkind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraOcclusionState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraOcclusionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraOcclusionState {}
impl ::core::fmt::Debug for CameraOcclusionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionState;{430adeb8-6842-5e55-9bde-04b4ef3a8a57})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CameraOcclusionState {
    type Vtable = ICameraOcclusionState_Vtbl;
}
unsafe impl ::windows::core::Interface for CameraOcclusionState {
    const IID: ::windows::core::GUID = <ICameraOcclusionState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraOcclusionState {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionState";
}
impl ::core::convert::From<CameraOcclusionState> for ::windows::core::IUnknown {
    fn from(value: CameraOcclusionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraOcclusionState> for ::windows::core::IUnknown {
    fn from(value: &CameraOcclusionState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraOcclusionState> for &::windows::core::IUnknown {
    fn from(value: &CameraOcclusionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CameraOcclusionState> for ::windows::core::IInspectable {
    fn from(value: CameraOcclusionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraOcclusionState> for ::windows::core::IInspectable {
    fn from(value: &CameraOcclusionState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraOcclusionState> for &::windows::core::IInspectable {
    fn from(value: &CameraOcclusionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CameraOcclusionState {}
unsafe impl ::core::marker::Sync for CameraOcclusionState {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionStateChangedEventArgs(::windows::core::IUnknown);
impl CameraOcclusionStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraOcclusionState>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraOcclusionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraOcclusionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraOcclusionStateChangedEventArgs {}
impl ::core::fmt::Debug for CameraOcclusionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionStateChangedEventArgs;{8512d848-c0de-57ca-a1ca-fb2c3d23df55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CameraOcclusionStateChangedEventArgs {
    const IID: ::windows::core::GUID = <ICameraOcclusionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraOcclusionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionStateChangedEventArgs";
}
impl ::core::convert::From<CameraOcclusionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CameraOcclusionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraOcclusionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CameraOcclusionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraOcclusionStateChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &CameraOcclusionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CameraOcclusionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CameraOcclusionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraOcclusionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CameraOcclusionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraOcclusionStateChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &CameraOcclusionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CameraOcclusionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for CameraOcclusionStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DefaultAudioCaptureDeviceChangedEventArgs(::windows::core::IUnknown);
impl DefaultAudioCaptureDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Role(&self) -> ::windows::core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Role)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceRole>(result__)
        }
    }
}
impl ::core::clone::Clone for DefaultAudioCaptureDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DefaultAudioCaptureDeviceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultAudioCaptureDeviceChangedEventArgs {}
impl ::core::fmt::Debug for DefaultAudioCaptureDeviceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultAudioCaptureDeviceChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DefaultAudioCaptureDeviceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs;{110f882f-1c05-4657-a18e-47c9b69f07ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DefaultAudioCaptureDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DefaultAudioCaptureDeviceChangedEventArgs {
    const IID: ::windows::core::GUID = <IDefaultAudioDeviceChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DefaultAudioCaptureDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs";
}
impl ::core::convert::From<DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<DefaultAudioCaptureDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DefaultAudioCaptureDeviceChangedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DefaultAudioCaptureDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::InParam<'a, IDefaultAudioDeviceChangedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DefaultAudioCaptureDeviceChangedEventArgs {}
unsafe impl ::core::marker::Sync for DefaultAudioCaptureDeviceChangedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DefaultAudioRenderDeviceChangedEventArgs(::windows::core::IUnknown);
impl DefaultAudioRenderDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Role(&self) -> ::windows::core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Role)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceRole>(result__)
        }
    }
}
impl ::core::clone::Clone for DefaultAudioRenderDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DefaultAudioRenderDeviceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultAudioRenderDeviceChangedEventArgs {}
impl ::core::fmt::Debug for DefaultAudioRenderDeviceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultAudioRenderDeviceChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DefaultAudioRenderDeviceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs;{110f882f-1c05-4657-a18e-47c9b69f07ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DefaultAudioRenderDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DefaultAudioRenderDeviceChangedEventArgs {
    const IID: ::windows::core::GUID = <IDefaultAudioDeviceChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DefaultAudioRenderDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs";
}
impl ::core::convert::From<DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<DefaultAudioRenderDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DefaultAudioRenderDeviceChangedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DefaultAudioRenderDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::InParam<'a, IDefaultAudioDeviceChangedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DefaultAudioRenderDeviceChangedEventArgs {}
unsafe impl ::core::marker::Sync for DefaultAudioRenderDeviceChangedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DialRequestedEventArgs(::windows::core::IUnknown);
impl DialRequestedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Contact(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contact)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for DialRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialRequestedEventArgs {}
impl ::core::fmt::Debug for DialRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DialRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DialRequestedEventArgs;{037b929e-953c-4286-8866-4f0f376c855a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DialRequestedEventArgs {
    const IID: ::windows::core::GUID = <IDialRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DialRequestedEventArgs";
}
impl ::core::convert::From<DialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DialRequestedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &DialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DialRequestedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &DialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DialRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DialRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowBounds(::windows::core::IUnknown);
impl DigitalWindowBounds {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DigitalWindowBounds, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NormalizedOriginTop(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalizedOriginTop)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetNormalizedOriginTop(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNormalizedOriginTop)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn NormalizedOriginLeft(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalizedOriginLeft)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetNormalizedOriginLeft(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNormalizedOriginLeft)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScale)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for DigitalWindowBounds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DigitalWindowBounds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DigitalWindowBounds {}
impl ::core::fmt::Debug for DigitalWindowBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowBounds").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowBounds;{dd4f21dd-d173-5c6b-8c25-bdd26d5122b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_Vtbl;
}
unsafe impl ::windows::core::Interface for DigitalWindowBounds {
    const IID: ::windows::core::GUID = <IDigitalWindowBounds as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DigitalWindowBounds {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowBounds";
}
impl ::core::convert::From<DigitalWindowBounds> for ::windows::core::IUnknown {
    fn from(value: DigitalWindowBounds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DigitalWindowBounds> for ::windows::core::IUnknown {
    fn from(value: &DigitalWindowBounds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DigitalWindowBounds> for &::windows::core::IUnknown {
    fn from(value: &DigitalWindowBounds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DigitalWindowBounds> for ::windows::core::IInspectable {
    fn from(value: DigitalWindowBounds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DigitalWindowBounds> for ::windows::core::IInspectable {
    fn from(value: &DigitalWindowBounds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DigitalWindowBounds> for &::windows::core::IInspectable {
    fn from(value: &DigitalWindowBounds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DigitalWindowBounds {}
unsafe impl ::core::marker::Sync for DigitalWindowBounds {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowCapability(::windows::core::IUnknown);
impl DigitalWindowCapability {
    pub fn Width(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MinScaleValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinScaleValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MaxScaleValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxScaleValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MinScaleValueWithoutUpsampling(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinScaleValueWithoutUpsampling)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NormalizedFieldOfViewLimit(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalizedFieldOfViewLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
}
impl ::core::clone::Clone for DigitalWindowCapability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DigitalWindowCapability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DigitalWindowCapability {}
impl ::core::fmt::Debug for DigitalWindowCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowCapability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowCapability;{d78bad2c-f721-5244-a196-b56ccbec606c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_Vtbl;
}
unsafe impl ::windows::core::Interface for DigitalWindowCapability {
    const IID: ::windows::core::GUID = <IDigitalWindowCapability as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DigitalWindowCapability {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowCapability";
}
impl ::core::convert::From<DigitalWindowCapability> for ::windows::core::IUnknown {
    fn from(value: DigitalWindowCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DigitalWindowCapability> for ::windows::core::IUnknown {
    fn from(value: &DigitalWindowCapability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DigitalWindowCapability> for &::windows::core::IUnknown {
    fn from(value: &DigitalWindowCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DigitalWindowCapability> for ::windows::core::IInspectable {
    fn from(value: DigitalWindowCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DigitalWindowCapability> for ::windows::core::IInspectable {
    fn from(value: &DigitalWindowCapability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DigitalWindowCapability> for &::windows::core::IInspectable {
    fn from(value: &DigitalWindowCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DigitalWindowCapability {}
unsafe impl ::core::marker::Sync for DigitalWindowCapability {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowControl(::windows::core::IUnknown);
impl DigitalWindowControl {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedModes(&self) -> ::windows::core::Result<::windows::core::Array<DigitalWindowMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<DigitalWindowMode>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn CurrentMode(&self) -> ::windows::core::Result<DigitalWindowMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<DigitalWindowMode>(result__)
        }
    }
    pub fn GetBounds(&self) -> ::windows::core::Result<DigitalWindowBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<DigitalWindowBounds>(result__)
        }
    }
    pub fn Configure(&self, digitalwindowmode: DigitalWindowMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Configure)(::windows::core::Vtable::as_raw(this), digitalwindowmode).ok() }
    }
    pub fn ConfigureWithBounds<'a, P0>(&self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, DigitalWindowBounds>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureWithBounds)(::windows::core::Vtable::as_raw(this), digitalwindowmode, digitalwindowbounds.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCapabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>>(result__)
        }
    }
    pub fn GetCapabilityForSize(&self, width: i32, height: i32) -> ::windows::core::Result<DigitalWindowCapability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapabilityForSize)(::windows::core::Vtable::as_raw(this), width, height, result__.as_mut_ptr()).from_abi::<DigitalWindowCapability>(result__)
        }
    }
}
impl ::core::clone::Clone for DigitalWindowControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DigitalWindowControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DigitalWindowControl {}
impl ::core::fmt::Debug for DigitalWindowControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowControl;{23b69eff-65d2-53ea-8780-de582b48b544})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DigitalWindowControl {
    type Vtable = IDigitalWindowControl_Vtbl;
}
unsafe impl ::windows::core::Interface for DigitalWindowControl {
    const IID: ::windows::core::GUID = <IDigitalWindowControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DigitalWindowControl {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowControl";
}
impl ::core::convert::From<DigitalWindowControl> for ::windows::core::IUnknown {
    fn from(value: DigitalWindowControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DigitalWindowControl> for ::windows::core::IUnknown {
    fn from(value: &DigitalWindowControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DigitalWindowControl> for &::windows::core::IUnknown {
    fn from(value: &DigitalWindowControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DigitalWindowControl> for ::windows::core::IInspectable {
    fn from(value: DigitalWindowControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DigitalWindowControl> for ::windows::core::IInspectable {
    fn from(value: &DigitalWindowControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DigitalWindowControl> for &::windows::core::IInspectable {
    fn from(value: &DigitalWindowControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DigitalWindowControl {}
unsafe impl ::core::marker::Sync for DigitalWindowControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ExposureCompensationControl(::windows::core::IUnknown);
impl ExposureCompensationControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, value: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ExposureCompensationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExposureCompensationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExposureCompensationControl {}
impl ::core::fmt::Debug for ExposureCompensationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExposureCompensationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExposureCompensationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposureCompensationControl;{81c8e834-dcec-4011-a610-1f3847e64aca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ExposureCompensationControl {
    type Vtable = IExposureCompensationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ExposureCompensationControl {
    const IID: ::windows::core::GUID = <IExposureCompensationControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureCompensationControl";
}
impl ::core::convert::From<ExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: ExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: &ExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ExposureCompensationControl> for &::windows::core::IUnknown {
    fn from(value: &ExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: ExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: &ExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ExposureCompensationControl> for &::windows::core::IInspectable {
    fn from(value: &ExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ExposureControl(::windows::core::IUnknown);
impl ExposureControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Auto)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAutoAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Min(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Max(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Step(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, shutterduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueAsync)(::windows::core::Vtable::as_raw(this), shutterduration, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ExposureControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExposureControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExposureControl {}
impl ::core::fmt::Debug for ExposureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExposureControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExposureControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposureControl;{09e8cbe2-ad96-4f28-a0e0-96ed7e1b5fd2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ExposureControl {
    type Vtable = IExposureControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ExposureControl {
    const IID: ::windows::core::GUID = <IExposureControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureControl";
}
impl ::core::convert::From<ExposureControl> for ::windows::core::IUnknown {
    fn from(value: ExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExposureControl> for ::windows::core::IUnknown {
    fn from(value: &ExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ExposureControl> for &::windows::core::IUnknown {
    fn from(value: &ExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ExposureControl> for ::windows::core::IInspectable {
    fn from(value: ExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExposureControl> for ::windows::core::IInspectable {
    fn from(value: &ExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ExposureControl> for &::windows::core::IInspectable {
    fn from(value: &ExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ExposurePriorityVideoControl(::windows::core::IUnknown);
impl ExposurePriorityVideoControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ExposurePriorityVideoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExposurePriorityVideoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExposurePriorityVideoControl {}
impl ::core::fmt::Debug for ExposurePriorityVideoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExposurePriorityVideoControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExposurePriorityVideoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposurePriorityVideoControl;{2cb240a3-5168-4271-9ea5-47621a98a352})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ExposurePriorityVideoControl {
    const IID: ::windows::core::GUID = <IExposurePriorityVideoControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExposurePriorityVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposurePriorityVideoControl";
}
impl ::core::convert::From<ExposurePriorityVideoControl> for ::windows::core::IUnknown {
    fn from(value: ExposurePriorityVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExposurePriorityVideoControl> for ::windows::core::IUnknown {
    fn from(value: &ExposurePriorityVideoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ExposurePriorityVideoControl> for &::windows::core::IUnknown {
    fn from(value: &ExposurePriorityVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ExposurePriorityVideoControl> for ::windows::core::IInspectable {
    fn from(value: ExposurePriorityVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExposurePriorityVideoControl> for ::windows::core::IInspectable {
    fn from(value: &ExposurePriorityVideoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ExposurePriorityVideoControl> for &::windows::core::IInspectable {
    fn from(value: &ExposurePriorityVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ExposurePriorityVideoControl {}
unsafe impl ::core::marker::Sync for ExposurePriorityVideoControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FlashControl(::windows::core::IUnknown);
impl FlashControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RedEyeReductionSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Auto)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAuto)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RedEyeReduction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRedEyeReduction)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerPercent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPowerPercent)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AssistantLightSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AssistantLightSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AssistantLightEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AssistantLightEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAssistantLightEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlashControl2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAssistantLightEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FlashControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FlashControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlashControl {}
impl ::core::fmt::Debug for FlashControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlashControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FlashControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FlashControl;{def41dbe-7d68-45e3-8c0f-be7bb32837d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FlashControl {
    type Vtable = IFlashControl_Vtbl;
}
unsafe impl ::windows::core::Interface for FlashControl {
    const IID: ::windows::core::GUID = <IFlashControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FlashControl {
    const NAME: &'static str = "Windows.Media.Devices.FlashControl";
}
impl ::core::convert::From<FlashControl> for ::windows::core::IUnknown {
    fn from(value: FlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlashControl> for ::windows::core::IUnknown {
    fn from(value: &FlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FlashControl> for &::windows::core::IUnknown {
    fn from(value: &FlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FlashControl> for ::windows::core::IInspectable {
    fn from(value: FlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlashControl> for ::windows::core::IInspectable {
    fn from(value: &FlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FlashControl> for &::windows::core::IInspectable {
    fn from(value: &FlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusControl(::windows::core::IUnknown);
impl FocusControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedPresets)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<FocusPreset>>(result__)
        }
    }
    pub fn Preset(&self) -> ::windows::core::Result<FocusPreset> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Preset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusPreset>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: FocusPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPresetAsync)(::windows::core::Vtable::as_raw(this), preset, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetWithCompletionOptionAsync(&self, preset: FocusPreset, completebeforefocus: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPresetWithCompletionOptionAsync)(::windows::core::Vtable::as_raw(this), preset, completebeforefocus, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, focus: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueAsync)(::windows::core::Vtable::as_raw(this), focus, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FocusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn FocusChangedSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusChangedSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn WaitForFocusSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WaitForFocusSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedFocusModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<FocusMode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusDistances(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedFocusDistances)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedFocusRanges)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<AutoFocusRange>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<FocusMode> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusMode>(result__)
        }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<MediaCaptureFocusState> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureFocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnlockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnlockAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LockAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Configure<'a, P0>(&self, settings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, FocusSettings>>,
    {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Configure)(::windows::core::Vtable::as_raw(this), settings.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for FocusControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusControl {}
impl ::core::fmt::Debug for FocusControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FocusControl;{c0d889f6-5228-4453-b153-85606592b238})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FocusControl {
    type Vtable = IFocusControl_Vtbl;
}
unsafe impl ::windows::core::Interface for FocusControl {
    const IID: ::windows::core::GUID = <IFocusControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FocusControl {
    const NAME: &'static str = "Windows.Media.Devices.FocusControl";
}
impl ::core::convert::From<FocusControl> for ::windows::core::IUnknown {
    fn from(value: FocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusControl> for ::windows::core::IUnknown {
    fn from(value: &FocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FocusControl> for &::windows::core::IUnknown {
    fn from(value: &FocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FocusControl> for ::windows::core::IInspectable {
    fn from(value: FocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusControl> for ::windows::core::IInspectable {
    fn from(value: &FocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FocusControl> for &::windows::core::IInspectable {
    fn from(value: &FocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusSettings(::windows::core::IUnknown);
impl FocusSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FocusSettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows::core::Result<FocusMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusMode>(result__)
        }
    }
    pub fn SetMode(&self, value: FocusMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AutoFocusRange(&self) -> ::windows::core::Result<AutoFocusRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoFocusRange)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AutoFocusRange>(result__)
        }
    }
    pub fn SetAutoFocusRange(&self, value: AutoFocusRange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoFocusRange)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Distance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<ManualFocusDistance>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Distance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<ManualFocusDistance>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDistance<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<ManualFocusDistance>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDistance)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn WaitForFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WaitForFocus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetWaitForFocus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWaitForFocus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DisableDriverFallback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisableDriverFallback)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDisableDriverFallback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisableDriverFallback)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FocusSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusSettings {}
impl ::core::fmt::Debug for FocusSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FocusSettings;{79958f6b-3263-4275-85d6-aeae891c96ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FocusSettings {
    type Vtable = IFocusSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for FocusSettings {
    const IID: ::windows::core::GUID = <IFocusSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FocusSettings {
    const NAME: &'static str = "Windows.Media.Devices.FocusSettings";
}
impl ::core::convert::From<FocusSettings> for ::windows::core::IUnknown {
    fn from(value: FocusSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusSettings> for ::windows::core::IUnknown {
    fn from(value: &FocusSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FocusSettings> for &::windows::core::IUnknown {
    fn from(value: &FocusSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FocusSettings> for ::windows::core::IInspectable {
    fn from(value: FocusSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusSettings> for ::windows::core::IInspectable {
    fn from(value: &FocusSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FocusSettings> for &::windows::core::IInspectable {
    fn from(value: &FocusSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for FocusSettings {}
unsafe impl ::core::marker::Sync for FocusSettings {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct HdrVideoControl(::windows::core::IUnknown);
impl HdrVideoControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HdrVideoMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<HdrVideoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HdrVideoMode>(result__)
        }
    }
    pub fn SetMode(&self, value: HdrVideoMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for HdrVideoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdrVideoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdrVideoControl {}
impl ::core::fmt::Debug for HdrVideoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdrVideoControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdrVideoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.HdrVideoControl;{55d8e2d0-30c0-43bf-9b9a-9799d70ced94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HdrVideoControl {
    type Vtable = IHdrVideoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for HdrVideoControl {
    const IID: ::windows::core::GUID = <IHdrVideoControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HdrVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.HdrVideoControl";
}
impl ::core::convert::From<HdrVideoControl> for ::windows::core::IUnknown {
    fn from(value: HdrVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdrVideoControl> for ::windows::core::IUnknown {
    fn from(value: &HdrVideoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HdrVideoControl> for &::windows::core::IUnknown {
    fn from(value: &HdrVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<HdrVideoControl> for ::windows::core::IInspectable {
    fn from(value: HdrVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdrVideoControl> for ::windows::core::IInspectable {
    fn from(value: &HdrVideoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HdrVideoControl> for &::windows::core::IInspectable {
    fn from(value: &HdrVideoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for HdrVideoControl {}
unsafe impl ::core::marker::Sync for HdrVideoControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct InfraredTorchControl(::windows::core::IUnknown);
impl InfraredTorchControl {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>>(result__)
        }
    }
    pub fn CurrentMode(&self) -> ::windows::core::Result<InfraredTorchMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InfraredTorchMode>(result__)
        }
    }
    pub fn SetCurrentMode(&self, value: InfraredTorchMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCurrentMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MinPower(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinPower)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxPower(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPower)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn PowerStep(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerStep)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Power(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Power)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetPower(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPower)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InfraredTorchControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InfraredTorchControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InfraredTorchControl {}
impl ::core::fmt::Debug for InfraredTorchControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InfraredTorchControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InfraredTorchControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.InfraredTorchControl;{1cba2c83-6cb6-5a04-a6fc-3be7b33ff056})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InfraredTorchControl {
    type Vtable = IInfraredTorchControl_Vtbl;
}
unsafe impl ::windows::core::Interface for InfraredTorchControl {
    const IID: ::windows::core::GUID = <IInfraredTorchControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InfraredTorchControl {
    const NAME: &'static str = "Windows.Media.Devices.InfraredTorchControl";
}
impl ::core::convert::From<InfraredTorchControl> for ::windows::core::IUnknown {
    fn from(value: InfraredTorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InfraredTorchControl> for ::windows::core::IUnknown {
    fn from(value: &InfraredTorchControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&InfraredTorchControl> for &::windows::core::IUnknown {
    fn from(value: &InfraredTorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<InfraredTorchControl> for ::windows::core::IInspectable {
    fn from(value: InfraredTorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InfraredTorchControl> for ::windows::core::IInspectable {
    fn from(value: &InfraredTorchControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&InfraredTorchControl> for &::windows::core::IInspectable {
    fn from(value: &InfraredTorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for InfraredTorchControl {}
unsafe impl ::core::marker::Sync for InfraredTorchControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct IsoSpeedControl(::windows::core::IUnknown);
impl IsoSpeedControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedPresets)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Preset(&self) -> ::windows::core::Result<IsoSpeedPreset> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Preset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IsoSpeedPreset>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetPresetAsync(&self, preset: IsoSpeedPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPresetAsync)(::windows::core::Vtable::as_raw(this), preset, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, isospeed: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueAsync)(::windows::core::Vtable::as_raw(this), isospeed, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Auto)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAutoAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for IsoSpeedControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsoSpeedControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsoSpeedControl {}
impl ::core::fmt::Debug for IsoSpeedControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsoSpeedControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IsoSpeedControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.IsoSpeedControl;{27b6c322-25ad-4f1b-aaab-524ab376ca33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IsoSpeedControl {
    type Vtable = IIsoSpeedControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IsoSpeedControl {
    const IID: ::windows::core::GUID = <IIsoSpeedControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.IsoSpeedControl";
}
impl ::core::convert::From<IsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: IsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: &IsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&IsoSpeedControl> for &::windows::core::IUnknown {
    fn from(value: &IsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<IsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: IsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: &IsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&IsoSpeedControl> for &::windows::core::IInspectable {
    fn from(value: &IsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct KeypadPressedEventArgs(::windows::core::IUnknown);
impl KeypadPressedEventArgs {
    pub fn TelephonyKey(&self) -> ::windows::core::Result<TelephonyKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TelephonyKey)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<TelephonyKey>(result__)
        }
    }
}
impl ::core::clone::Clone for KeypadPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeypadPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeypadPressedEventArgs {}
impl ::core::fmt::Debug for KeypadPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeypadPressedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeypadPressedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.KeypadPressedEventArgs;{d3a43900-b4fa-49cd-9442-89af6568f601})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for KeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for KeypadPressedEventArgs {
    const IID: ::windows::core::GUID = <IKeypadPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeypadPressedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.KeypadPressedEventArgs";
}
impl ::core::convert::From<KeypadPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeypadPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeypadPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeypadPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeypadPressedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &KeypadPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeypadPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeypadPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeypadPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeypadPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeypadPressedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &KeypadPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeypadPressedEventArgs {}
unsafe impl ::core::marker::Sync for KeypadPressedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoControl(::windows::core::IUnknown);
impl LowLagPhotoControl {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, P0, E0>(&self, captureproperties: P0) -> ::windows::core::Result<super::MediaProperties::MediaRatio>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::MediaProperties::IMediaEncodingProperties>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHighestConcurrentFrameRate)(::windows::core::Vtable::as_raw(this), captureproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentFrameRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ThumbnailEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnailEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ThumbnailFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaThumbnailFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnailFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredThumbnailSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredThumbnailSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardwareAcceleratedThumbnailSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLagPhotoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoControl {}
impl ::core::fmt::Debug for LowLagPhotoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagPhotoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.LowLagPhotoControl;{6d5c4dd0-fadf-415d-aee6-3baa529300c9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_Vtbl;
}
unsafe impl ::windows::core::Interface for LowLagPhotoControl {
    const IID: ::windows::core::GUID = <ILowLagPhotoControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLagPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoControl";
}
impl ::core::convert::From<LowLagPhotoControl> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoControl> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LowLagPhotoControl> for &::windows::core::IUnknown {
    fn from(value: &LowLagPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LowLagPhotoControl> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoControl> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LowLagPhotoControl> for &::windows::core::IInspectable {
    fn from(value: &LowLagPhotoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoSequenceControl(::windows::core::IUnknown);
impl LowLagPhotoSequenceControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPastPhotos(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPastPhotos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPhotosPerSecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn PastPhotoLimit(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PastPhotoLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetPastPhotoLimit(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPastPhotoLimit)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhotosPerSecondLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPhotosPerSecondLimit)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, P0, E0>(&self, captureproperties: P0) -> ::windows::core::Result<super::MediaProperties::MediaRatio>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::MediaProperties::IMediaEncodingProperties>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHighestConcurrentFrameRate)(::windows::core::Vtable::as_raw(this), captureproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentFrameRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ThumbnailEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnailEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ThumbnailFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaThumbnailFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnailFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredThumbnailSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredThumbnailSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardwareAcceleratedThumbnailSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLagPhotoSequenceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoSequenceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoSequenceControl {}
impl ::core::fmt::Debug for LowLagPhotoSequenceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoSequenceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagPhotoSequenceControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.LowLagPhotoSequenceControl;{3dcf909d-6d16-409c-bafe-b9a594c6fde6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for LowLagPhotoSequenceControl {
    const IID: ::windows::core::GUID = <ILowLagPhotoSequenceControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLagPhotoSequenceControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoSequenceControl";
}
impl ::core::convert::From<LowLagPhotoSequenceControl> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoSequenceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceControl> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoSequenceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceControl> for &::windows::core::IUnknown {
    fn from(value: &LowLagPhotoSequenceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LowLagPhotoSequenceControl> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoSequenceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceControl> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoSequenceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceControl> for &::windows::core::IInspectable {
    fn from(value: &LowLagPhotoSequenceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
pub struct MediaDevice;
impl MediaDevice {
    pub fn GetAudioCaptureSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioCaptureSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetAudioRenderSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioRenderSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetVideoCaptureSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVideoCaptureSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDefaultAudioCaptureId(role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAudioCaptureId)(::windows::core::Vtable::as_raw(this), role, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDefaultAudioRenderId(role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAudioRenderId)(::windows::core::Vtable::as_raw(this), role, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultAudioCaptureDeviceChanged<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>>,
    {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultAudioCaptureDeviceChanged)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultAudioCaptureDeviceChanged(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveDefaultAudioCaptureDeviceChanged)(::windows::core::Vtable::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultAudioRenderDeviceChanged<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>>,
    {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultAudioRenderDeviceChanged)(::windows::core::Vtable::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultAudioRenderDeviceChanged(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveDefaultAudioRenderDeviceChanged)(::windows::core::Vtable::as_raw(this), cookie).ok() })
    }
    #[doc(hidden)]
    pub fn IMediaDeviceStatics<R, F: FnOnce(&IMediaDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaDevice, IMediaDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MediaDevice {
    const NAME: &'static str = "Windows.Media.Devices.MediaDevice";
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaDeviceControl(::windows::core::IUnknown);
impl MediaDeviceControl {
    pub fn Capabilities(&self) -> ::windows::core::Result<MediaDeviceControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControlCapabilities>(result__)
        }
    }
    pub fn TryGetValue(&self, value: &mut f64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetValue)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetValue(&self, value: f64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetValue)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryGetAuto(&self, value: &mut bool) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAuto)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetAuto(&self, value: bool) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetAuto)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaDeviceControl {}
impl ::core::fmt::Debug for MediaDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaDeviceControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.MediaDeviceControl;{efa8dfa9-6f75-4863-ba0b-583f3036b4de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaDeviceControl {
    type Vtable = IMediaDeviceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaDeviceControl {
    const IID: ::windows::core::GUID = <IMediaDeviceControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaDeviceControl {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControl";
}
impl ::core::convert::From<MediaDeviceControl> for ::windows::core::IUnknown {
    fn from(value: MediaDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaDeviceControl> for ::windows::core::IUnknown {
    fn from(value: &MediaDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MediaDeviceControl> for &::windows::core::IUnknown {
    fn from(value: &MediaDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MediaDeviceControl> for ::windows::core::IInspectable {
    fn from(value: MediaDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaDeviceControl> for ::windows::core::IInspectable {
    fn from(value: &MediaDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MediaDeviceControl> for &::windows::core::IInspectable {
    fn from(value: &MediaDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaDeviceControlCapabilities(::windows::core::IUnknown);
impl MediaDeviceControlCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Default(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Default)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AutoModeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoModeSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaDeviceControlCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaDeviceControlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaDeviceControlCapabilities {}
impl ::core::fmt::Debug for MediaDeviceControlCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDeviceControlCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaDeviceControlCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.MediaDeviceControlCapabilities;{23005816-eb85-43e2-b92b-8240d5ee70ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaDeviceControlCapabilities {
    const IID: ::windows::core::GUID = <IMediaDeviceControlCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaDeviceControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControlCapabilities";
}
impl ::core::convert::From<MediaDeviceControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: MediaDeviceControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaDeviceControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: &MediaDeviceControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MediaDeviceControlCapabilities> for &::windows::core::IUnknown {
    fn from(value: &MediaDeviceControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MediaDeviceControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: MediaDeviceControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaDeviceControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: &MediaDeviceControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MediaDeviceControlCapabilities> for &::windows::core::IInspectable {
    fn from(value: &MediaDeviceControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ModuleCommandResult(::windows::core::IUnknown);
impl ModuleCommandResult {
    pub fn Status(&self) -> ::windows::core::Result<SendCommandStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<SendCommandStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Result)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for ModuleCommandResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ModuleCommandResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ModuleCommandResult {}
impl ::core::fmt::Debug for ModuleCommandResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ModuleCommandResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ModuleCommandResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ModuleCommandResult;{520d1eb4-1374-4c7d-b1e4-39dcdf3eae4e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ModuleCommandResult {
    type Vtable = IModuleCommandResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ModuleCommandResult {
    const IID: ::windows::core::GUID = <IModuleCommandResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ModuleCommandResult {
    const NAME: &'static str = "Windows.Media.Devices.ModuleCommandResult";
}
impl ::core::convert::From<ModuleCommandResult> for ::windows::core::IUnknown {
    fn from(value: ModuleCommandResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ModuleCommandResult> for ::windows::core::IUnknown {
    fn from(value: &ModuleCommandResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ModuleCommandResult> for &::windows::core::IUnknown {
    fn from(value: &ModuleCommandResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ModuleCommandResult> for ::windows::core::IInspectable {
    fn from(value: ModuleCommandResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ModuleCommandResult> for ::windows::core::IInspectable {
    fn from(value: &ModuleCommandResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ModuleCommandResult> for &::windows::core::IInspectable {
    fn from(value: &ModuleCommandResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct OpticalImageStabilizationControl(::windows::core::IUnknown);
impl OpticalImageStabilizationControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<OpticalImageStabilizationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<OpticalImageStabilizationMode>(result__)
        }
    }
    pub fn SetMode(&self, value: OpticalImageStabilizationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for OpticalImageStabilizationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OpticalImageStabilizationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OpticalImageStabilizationControl {}
impl ::core::fmt::Debug for OpticalImageStabilizationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpticalImageStabilizationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OpticalImageStabilizationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.OpticalImageStabilizationControl;{bfad9c1d-00bc-423b-8eb2-a0178ca94247})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for OpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for OpticalImageStabilizationControl {
    const IID: ::windows::core::GUID = <IOpticalImageStabilizationControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for OpticalImageStabilizationControl {
    const NAME: &'static str = "Windows.Media.Devices.OpticalImageStabilizationControl";
}
impl ::core::convert::From<OpticalImageStabilizationControl> for ::windows::core::IUnknown {
    fn from(value: OpticalImageStabilizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OpticalImageStabilizationControl> for ::windows::core::IUnknown {
    fn from(value: &OpticalImageStabilizationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&OpticalImageStabilizationControl> for &::windows::core::IUnknown {
    fn from(value: &OpticalImageStabilizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<OpticalImageStabilizationControl> for ::windows::core::IInspectable {
    fn from(value: OpticalImageStabilizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OpticalImageStabilizationControl> for ::windows::core::IInspectable {
    fn from(value: &OpticalImageStabilizationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&OpticalImageStabilizationControl> for &::windows::core::IInspectable {
    fn from(value: &OpticalImageStabilizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for OpticalImageStabilizationControl {}
unsafe impl ::core::marker::Sync for OpticalImageStabilizationControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct PanelBasedOptimizationControl(::windows::core::IUnknown);
impl PanelBasedOptimizationControl {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Panel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Panel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Enumeration::Panel>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPanel(&self, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPanel)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PanelBasedOptimizationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PanelBasedOptimizationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PanelBasedOptimizationControl {}
impl ::core::fmt::Debug for PanelBasedOptimizationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PanelBasedOptimizationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PanelBasedOptimizationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.PanelBasedOptimizationControl;{33323223-6247-5419-a5a4-3d808645d917})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for PanelBasedOptimizationControl {
    const IID: ::windows::core::GUID = <IPanelBasedOptimizationControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PanelBasedOptimizationControl {
    const NAME: &'static str = "Windows.Media.Devices.PanelBasedOptimizationControl";
}
impl ::core::convert::From<PanelBasedOptimizationControl> for ::windows::core::IUnknown {
    fn from(value: PanelBasedOptimizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PanelBasedOptimizationControl> for ::windows::core::IUnknown {
    fn from(value: &PanelBasedOptimizationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PanelBasedOptimizationControl> for &::windows::core::IUnknown {
    fn from(value: &PanelBasedOptimizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PanelBasedOptimizationControl> for ::windows::core::IInspectable {
    fn from(value: PanelBasedOptimizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PanelBasedOptimizationControl> for ::windows::core::IInspectable {
    fn from(value: &PanelBasedOptimizationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PanelBasedOptimizationControl> for &::windows::core::IInspectable {
    fn from(value: &PanelBasedOptimizationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PanelBasedOptimizationControl {}
unsafe impl ::core::marker::Sync for PanelBasedOptimizationControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct PhotoConfirmationControl(::windows::core::IUnknown);
impl PhotoConfirmationControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PixelFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPixelFormat(&self, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPixelFormat)(::windows::core::Vtable::as_raw(this), format).ok() }
    }
}
impl ::core::clone::Clone for PhotoConfirmationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoConfirmationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoConfirmationControl {}
impl ::core::fmt::Debug for PhotoConfirmationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoConfirmationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoConfirmationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.PhotoConfirmationControl;{c8f3f363-ff5e-4582-a9a8-0550f85a4a76})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_Vtbl;
}
unsafe impl ::windows::core::Interface for PhotoConfirmationControl {
    const IID: ::windows::core::GUID = <IPhotoConfirmationControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoConfirmationControl {
    const NAME: &'static str = "Windows.Media.Devices.PhotoConfirmationControl";
}
impl ::core::convert::From<PhotoConfirmationControl> for ::windows::core::IUnknown {
    fn from(value: PhotoConfirmationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoConfirmationControl> for ::windows::core::IUnknown {
    fn from(value: &PhotoConfirmationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoConfirmationControl> for &::windows::core::IUnknown {
    fn from(value: &PhotoConfirmationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoConfirmationControl> for ::windows::core::IInspectable {
    fn from(value: PhotoConfirmationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoConfirmationControl> for ::windows::core::IInspectable {
    fn from(value: &PhotoConfirmationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoConfirmationControl> for &::windows::core::IInspectable {
    fn from(value: &PhotoConfirmationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RedialRequestedEventArgs(::windows::core::IUnknown);
impl RedialRequestedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for RedialRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RedialRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RedialRequestedEventArgs {}
impl ::core::fmt::Debug for RedialRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RedialRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RedialRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RedialRequestedEventArgs;{7eb55209-76ab-4c31-b40e-4b58379d580c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RedialRequestedEventArgs {
    const IID: ::windows::core::GUID = <IRedialRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RedialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.RedialRequestedEventArgs";
}
impl ::core::convert::From<RedialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RedialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RedialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RedialRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RedialRequestedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &RedialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RedialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RedialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RedialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RedialRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RedialRequestedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &RedialRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for RedialRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RedialRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RegionOfInterest(::windows::core::IUnknown);
impl RegionOfInterest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RegionOfInterest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AutoFocusEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoFocusEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoFocusEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoFocusEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AutoWhiteBalanceEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoWhiteBalanceEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoWhiteBalanceEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoWhiteBalanceEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AutoExposureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoExposureEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoExposureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoExposureEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBounds(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBounds)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<RegionOfInterestType> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<RegionOfInterestType>(result__)
        }
    }
    pub fn SetType(&self, value: RegionOfInterestType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BoundsNormalized(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundsNormalized)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetBoundsNormalized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBoundsNormalized)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Weight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Weight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetWeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetWeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for RegionOfInterest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RegionOfInterest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegionOfInterest {}
impl ::core::fmt::Debug for RegionOfInterest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegionOfInterest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RegionOfInterest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RegionOfInterest;{e5ecc834-ce66-4e05-a78f-cf391a5ec2d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RegionOfInterest {
    type Vtable = IRegionOfInterest_Vtbl;
}
unsafe impl ::windows::core::Interface for RegionOfInterest {
    const IID: ::windows::core::GUID = <IRegionOfInterest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RegionOfInterest {
    const NAME: &'static str = "Windows.Media.Devices.RegionOfInterest";
}
impl ::core::convert::From<RegionOfInterest> for ::windows::core::IUnknown {
    fn from(value: RegionOfInterest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RegionOfInterest> for ::windows::core::IUnknown {
    fn from(value: &RegionOfInterest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RegionOfInterest> for &::windows::core::IUnknown {
    fn from(value: &RegionOfInterest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RegionOfInterest> for ::windows::core::IInspectable {
    fn from(value: RegionOfInterest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RegionOfInterest> for ::windows::core::IInspectable {
    fn from(value: &RegionOfInterest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RegionOfInterest> for &::windows::core::IInspectable {
    fn from(value: &RegionOfInterest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for RegionOfInterest {}
unsafe impl ::core::marker::Sync for RegionOfInterest {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RegionsOfInterestControl(::windows::core::IUnknown);
impl RegionsOfInterestControl {
    pub fn MaxRegions(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxRegions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRegionsAsync<'a, P0, E0>(&self, regions: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<RegionOfInterest>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetRegionsAsync)(::windows::core::Vtable::as_raw(this), regions.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRegionsWithLockAsync<'a, P0, E0>(&self, regions: P0, lockvalues: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<RegionOfInterest>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetRegionsWithLockAsync)(::windows::core::Vtable::as_raw(this), regions.try_into().map_err(|e| e.into())?.abi(), lockvalues, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearRegionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClearRegionsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn AutoFocusSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoFocusSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AutoWhiteBalanceSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoWhiteBalanceSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AutoExposureSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoExposureSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for RegionsOfInterestControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RegionsOfInterestControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegionsOfInterestControl {}
impl ::core::fmt::Debug for RegionsOfInterestControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegionsOfInterestControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RegionsOfInterestControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RegionsOfInterestControl;{c323f527-ab0b-4558-8b5b-df5693db0378})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_Vtbl;
}
unsafe impl ::windows::core::Interface for RegionsOfInterestControl {
    const IID: ::windows::core::GUID = <IRegionsOfInterestControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RegionsOfInterestControl {
    const NAME: &'static str = "Windows.Media.Devices.RegionsOfInterestControl";
}
impl ::core::convert::From<RegionsOfInterestControl> for ::windows::core::IUnknown {
    fn from(value: RegionsOfInterestControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RegionsOfInterestControl> for ::windows::core::IUnknown {
    fn from(value: &RegionsOfInterestControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RegionsOfInterestControl> for &::windows::core::IUnknown {
    fn from(value: &RegionsOfInterestControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RegionsOfInterestControl> for ::windows::core::IInspectable {
    fn from(value: RegionsOfInterestControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RegionsOfInterestControl> for ::windows::core::IInspectable {
    fn from(value: &RegionsOfInterestControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RegionsOfInterestControl> for &::windows::core::IInspectable {
    fn from(value: &RegionsOfInterestControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct SceneModeControl(::windows::core::IUnknown);
impl SceneModeControl {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<CaptureSceneMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CaptureSceneMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, scenemode: CaptureSceneMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueAsync)(::windows::core::Vtable::as_raw(this), scenemode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneModeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneModeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneModeControl {}
impl ::core::fmt::Debug for SceneModeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneModeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneModeControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.SceneModeControl;{d48e5af7-8d59-4854-8c62-12c70ba89b7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneModeControl {
    type Vtable = ISceneModeControl_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneModeControl {
    const IID: ::windows::core::GUID = <ISceneModeControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneModeControl {
    const NAME: &'static str = "Windows.Media.Devices.SceneModeControl";
}
impl ::core::convert::From<SceneModeControl> for ::windows::core::IUnknown {
    fn from(value: SceneModeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneModeControl> for ::windows::core::IUnknown {
    fn from(value: &SceneModeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SceneModeControl> for &::windows::core::IUnknown {
    fn from(value: &SceneModeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SceneModeControl> for ::windows::core::IInspectable {
    fn from(value: SceneModeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneModeControl> for ::windows::core::IInspectable {
    fn from(value: &SceneModeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SceneModeControl> for &::windows::core::IInspectable {
    fn from(value: &SceneModeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct TorchControl(::windows::core::IUnknown);
impl TorchControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerPercent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPowerPercent)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for TorchControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TorchControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TorchControl {}
impl ::core::fmt::Debug for TorchControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TorchControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TorchControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.TorchControl;{a6053665-8250-416c-919a-724296afa306})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TorchControl {
    type Vtable = ITorchControl_Vtbl;
}
unsafe impl ::windows::core::Interface for TorchControl {
    const IID: ::windows::core::GUID = <ITorchControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TorchControl {
    const NAME: &'static str = "Windows.Media.Devices.TorchControl";
}
impl ::core::convert::From<TorchControl> for ::windows::core::IUnknown {
    fn from(value: TorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TorchControl> for ::windows::core::IUnknown {
    fn from(value: &TorchControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&TorchControl> for &::windows::core::IUnknown {
    fn from(value: &TorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<TorchControl> for ::windows::core::IInspectable {
    fn from(value: TorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TorchControl> for ::windows::core::IInspectable {
    fn from(value: &TorchControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&TorchControl> for &::windows::core::IInspectable {
    fn from(value: &TorchControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceController(::windows::core::IUnknown);
impl VideoDeviceController {
    pub fn SetDeviceProperty<'a, P0>(&self, propertyid: &::windows::core::HSTRING, propertyvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDeviceProperty)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyid), propertyvalue.into().abi()).ok() }
    }
    pub fn GetDeviceProperty(&self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceProperty)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyid), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CameraOcclusionInfo(&self) -> ::windows::core::Result<CameraOcclusionInfo> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController10>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CameraOcclusionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraOcclusionInfo>(result__)
        }
    }
    pub fn LowLagPhotoSequence(&self) -> ::windows::core::Result<LowLagPhotoSequenceControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LowLagPhotoSequence)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<LowLagPhotoSequenceControl>(result__)
        }
    }
    pub fn LowLagPhoto(&self) -> ::windows::core::Result<LowLagPhotoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LowLagPhoto)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<LowLagPhotoControl>(result__)
        }
    }
    pub fn SceneModeControl(&self) -> ::windows::core::Result<SceneModeControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SceneModeControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneModeControl>(result__)
        }
    }
    pub fn TorchControl(&self) -> ::windows::core::Result<TorchControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TorchControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<TorchControl>(result__)
        }
    }
    pub fn FlashControl(&self) -> ::windows::core::Result<FlashControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlashControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FlashControl>(result__)
        }
    }
    pub fn WhiteBalanceControl(&self) -> ::windows::core::Result<WhiteBalanceControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WhiteBalanceControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<WhiteBalanceControl>(result__)
        }
    }
    pub fn ExposureControl(&self) -> ::windows::core::Result<ExposureControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExposureControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ExposureControl>(result__)
        }
    }
    pub fn FocusControl(&self) -> ::windows::core::Result<FocusControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusControl>(result__)
        }
    }
    pub fn ExposureCompensationControl(&self) -> ::windows::core::Result<ExposureCompensationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExposureCompensationControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ExposureCompensationControl>(result__)
        }
    }
    pub fn IsoSpeedControl(&self) -> ::windows::core::Result<IsoSpeedControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsoSpeedControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IsoSpeedControl>(result__)
        }
    }
    pub fn RegionsOfInterestControl(&self) -> ::windows::core::Result<RegionsOfInterestControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegionsOfInterestControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<RegionsOfInterestControl>(result__)
        }
    }
    pub fn PrimaryUse(&self) -> ::windows::core::Result<CaptureUse> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrimaryUse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CaptureUse>(result__)
        }
    }
    pub fn SetPrimaryUse(&self, value: CaptureUse) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrimaryUse)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Devices_Core\"`*"]
    #[cfg(feature = "Media_Devices_Core")]
    pub fn VariablePhotoSequenceController(&self) -> ::windows::core::Result<Core::VariablePhotoSequenceController> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VariablePhotoSequenceController)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<Core::VariablePhotoSequenceController>(result__)
        }
    }
    pub fn PhotoConfirmationControl(&self) -> ::windows::core::Result<PhotoConfirmationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhotoConfirmationControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoConfirmationControl>(result__)
        }
    }
    pub fn ZoomControl(&self) -> ::windows::core::Result<ZoomControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZoomControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ZoomControl>(result__)
        }
    }
    pub fn ExposurePriorityVideoControl(&self) -> ::windows::core::Result<ExposurePriorityVideoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExposurePriorityVideoControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ExposurePriorityVideoControl>(result__)
        }
    }
    pub fn DesiredOptimization(&self) -> ::windows::core::Result<MediaCaptureOptimization> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredOptimization)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureOptimization>(result__)
        }
    }
    pub fn SetDesiredOptimization(&self, value: MediaCaptureOptimization) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredOptimization)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn HdrVideoControl(&self) -> ::windows::core::Result<HdrVideoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HdrVideoControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<HdrVideoControl>(result__)
        }
    }
    pub fn OpticalImageStabilizationControl(&self) -> ::windows::core::Result<OpticalImageStabilizationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpticalImageStabilizationControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<OpticalImageStabilizationControl>(result__)
        }
    }
    pub fn AdvancedPhotoControl(&self) -> ::windows::core::Result<AdvancedPhotoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvancedPhotoControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AdvancedPhotoControl>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDevicePropertyById<'a, P0, E0>(&self, propertyid: &::windows::core::HSTRING, maxpropertyvaluesize: P0) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDevicePropertyById)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyid), maxpropertyvaluesize.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<VideoDeviceControllerGetDevicePropertyResult>(result__)
        }
    }
    pub fn SetDevicePropertyById<'a, P0>(&self, propertyid: &::windows::core::HSTRING, propertyvalue: P0) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDevicePropertyById)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyid), propertyvalue.into().abi(), result__.as_mut_ptr()).from_abi::<VideoDeviceControllerSetDevicePropertyStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDevicePropertyByExtendedId<'a, P0, E0>(&self, extendedpropertyid: &[u8], maxpropertyvaluesize: P0) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDevicePropertyByExtendedId)(::windows::core::Vtable::as_raw(this), extendedpropertyid.len() as u32, extendedpropertyid.as_ptr(), maxpropertyvaluesize.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<VideoDeviceControllerGetDevicePropertyResult>(result__)
        }
    }
    pub fn SetDevicePropertyByExtendedId(&self, extendedpropertyid: &[u8], propertyvalue: &[u8]) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDevicePropertyByExtendedId)(::windows::core::Vtable::as_raw(this), extendedpropertyid.len() as u32, extendedpropertyid.as_ptr(), propertyvalue.len() as u32, propertyvalue.as_ptr(), result__.as_mut_ptr()).from_abi::<VideoDeviceControllerSetDevicePropertyStatus>(result__)
        }
    }
    pub fn VideoTemporalDenoisingControl(&self) -> ::windows::core::Result<VideoTemporalDenoisingControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoTemporalDenoisingControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoTemporalDenoisingControl>(result__)
        }
    }
    pub fn InfraredTorchControl(&self) -> ::windows::core::Result<InfraredTorchControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InfraredTorchControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InfraredTorchControl>(result__)
        }
    }
    pub fn PanelBasedOptimizationControl(&self) -> ::windows::core::Result<PanelBasedOptimizationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PanelBasedOptimizationControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PanelBasedOptimizationControl>(result__)
        }
    }
    pub fn DigitalWindowControl(&self) -> ::windows::core::Result<DigitalWindowControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DigitalWindowControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<DigitalWindowControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAvailableMediaStreamProperties)(::windows::core::Vtable::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMediaStreamProperties)(::windows::core::Vtable::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<super::MediaProperties::IMediaEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<'a, P0, E0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::MediaProperties::IMediaEncodingProperties>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetMediaStreamPropertiesAsync)(::windows::core::Vtable::as_raw(this), mediastreamtype, mediaencodingproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Brightness)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contrast)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Hue(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn WhiteBalance(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WhiteBalance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn BacklightCompensation(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BacklightCompensation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Pan(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Tilt(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tilt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Zoom(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Zoom)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Roll(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Roll)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Exposure(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Exposure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Focus(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Focus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaDeviceControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn TrySetPowerlineFrequency(&self, value: super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetPowerlineFrequency)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn TryGetPowerlineFrequency(&self, value: &mut super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetPowerlineFrequency)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoDeviceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoDeviceController {}
impl ::core::fmt::Debug for VideoDeviceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoDeviceController;{99555575-2e2e-40b8-b6c7-f82d10013210})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VideoDeviceController {
    type Vtable = IVideoDeviceController_Vtbl;
}
unsafe impl ::windows::core::Interface for VideoDeviceController {
    const IID: ::windows::core::GUID = <IVideoDeviceController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceController";
}
impl ::core::convert::From<VideoDeviceController> for ::windows::core::IUnknown {
    fn from(value: VideoDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDeviceController> for ::windows::core::IUnknown {
    fn from(value: &VideoDeviceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VideoDeviceController> for &::windows::core::IUnknown {
    fn from(value: &VideoDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<VideoDeviceController> for ::windows::core::IInspectable {
    fn from(value: VideoDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDeviceController> for ::windows::core::IInspectable {
    fn from(value: &VideoDeviceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VideoDeviceController> for &::windows::core::IInspectable {
    fn from(value: &VideoDeviceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<VideoDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoDeviceController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoDeviceController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&VideoDeviceController> for ::windows::core::InParam<'a, IMediaDeviceController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoDeviceController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyResult(::windows::core::IUnknown);
impl VideoDeviceControllerGetDevicePropertyResult {
    pub fn Status(&self) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoDeviceControllerGetDevicePropertyStatus>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoDeviceControllerGetDevicePropertyResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoDeviceControllerGetDevicePropertyResult {}
impl ::core::fmt::Debug for VideoDeviceControllerGetDevicePropertyResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerGetDevicePropertyResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceControllerGetDevicePropertyResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult;{c5d88395-6ed5-4790-8b5d-0ef13935d0f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_Vtbl;
}
unsafe impl ::windows::core::Interface for VideoDeviceControllerGetDevicePropertyResult {
    const IID: ::windows::core::GUID = <IVideoDeviceControllerGetDevicePropertyResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoDeviceControllerGetDevicePropertyResult {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult";
}
impl ::core::convert::From<VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IUnknown {
    fn from(value: VideoDeviceControllerGetDevicePropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IUnknown {
    fn from(value: &VideoDeviceControllerGetDevicePropertyResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VideoDeviceControllerGetDevicePropertyResult> for &::windows::core::IUnknown {
    fn from(value: &VideoDeviceControllerGetDevicePropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IInspectable {
    fn from(value: VideoDeviceControllerGetDevicePropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IInspectable {
    fn from(value: &VideoDeviceControllerGetDevicePropertyResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VideoDeviceControllerGetDevicePropertyResult> for &::windows::core::IInspectable {
    fn from(value: &VideoDeviceControllerGetDevicePropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for VideoDeviceControllerGetDevicePropertyResult {}
unsafe impl ::core::marker::Sync for VideoDeviceControllerGetDevicePropertyResult {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoTemporalDenoisingControl(::windows::core::IUnknown);
impl VideoTemporalDenoisingControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<VideoTemporalDenoisingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoTemporalDenoisingMode>(result__)
        }
    }
    pub fn SetMode(&self, value: VideoTemporalDenoisingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for VideoTemporalDenoisingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTemporalDenoisingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTemporalDenoisingControl {}
impl ::core::fmt::Debug for VideoTemporalDenoisingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTemporalDenoisingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTemporalDenoisingControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoTemporalDenoisingControl;{7ab34735-3e2a-4a32-baff-4358c4fbdd57})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_Vtbl;
}
unsafe impl ::windows::core::Interface for VideoTemporalDenoisingControl {
    const IID: ::windows::core::GUID = <IVideoTemporalDenoisingControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoTemporalDenoisingControl {
    const NAME: &'static str = "Windows.Media.Devices.VideoTemporalDenoisingControl";
}
impl ::core::convert::From<VideoTemporalDenoisingControl> for ::windows::core::IUnknown {
    fn from(value: VideoTemporalDenoisingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTemporalDenoisingControl> for ::windows::core::IUnknown {
    fn from(value: &VideoTemporalDenoisingControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VideoTemporalDenoisingControl> for &::windows::core::IUnknown {
    fn from(value: &VideoTemporalDenoisingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<VideoTemporalDenoisingControl> for ::windows::core::IInspectable {
    fn from(value: VideoTemporalDenoisingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTemporalDenoisingControl> for ::windows::core::IInspectable {
    fn from(value: &VideoTemporalDenoisingControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VideoTemporalDenoisingControl> for &::windows::core::IInspectable {
    fn from(value: &VideoTemporalDenoisingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for VideoTemporalDenoisingControl {}
unsafe impl ::core::marker::Sync for VideoTemporalDenoisingControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct WhiteBalanceControl(::windows::core::IUnknown);
impl WhiteBalanceControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Preset(&self) -> ::windows::core::Result<ColorTemperaturePreset> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Preset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ColorTemperaturePreset>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: ColorTemperaturePreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPresetAsync)(::windows::core::Vtable::as_raw(this), preset, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, temperature: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueAsync)(::windows::core::Vtable::as_raw(this), temperature, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for WhiteBalanceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WhiteBalanceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WhiteBalanceControl {}
impl ::core::fmt::Debug for WhiteBalanceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WhiteBalanceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WhiteBalanceControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.WhiteBalanceControl;{781f047e-7162-49c8-a8f9-9481c565363e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_Vtbl;
}
unsafe impl ::windows::core::Interface for WhiteBalanceControl {
    const IID: ::windows::core::GUID = <IWhiteBalanceControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WhiteBalanceControl {
    const NAME: &'static str = "Windows.Media.Devices.WhiteBalanceControl";
}
impl ::core::convert::From<WhiteBalanceControl> for ::windows::core::IUnknown {
    fn from(value: WhiteBalanceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WhiteBalanceControl> for ::windows::core::IUnknown {
    fn from(value: &WhiteBalanceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WhiteBalanceControl> for &::windows::core::IUnknown {
    fn from(value: &WhiteBalanceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WhiteBalanceControl> for ::windows::core::IInspectable {
    fn from(value: WhiteBalanceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WhiteBalanceControl> for ::windows::core::IInspectable {
    fn from(value: &WhiteBalanceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WhiteBalanceControl> for &::windows::core::IInspectable {
    fn from(value: &WhiteBalanceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ZoomControl(::windows::core::IUnknown);
impl ZoomControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Step)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>> {
        let this = &::windows::core::Interface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode> {
        let this = &::windows::core::Interface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ZoomTransitionMode>(result__)
        }
    }
    pub fn Configure<'a, P0>(&self, settings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ZoomSettings>>,
    {
        let this = &::windows::core::Interface::cast::<IZoomControl2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Configure)(::windows::core::Vtable::as_raw(this), settings.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for ZoomControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ZoomControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ZoomControl {}
impl ::core::fmt::Debug for ZoomControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ZoomControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ZoomControl;{3a1e0b12-32da-4c17-bfd7-8d0c73c8f5a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ZoomControl {
    type Vtable = IZoomControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ZoomControl {
    const IID: ::windows::core::GUID = <IZoomControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ZoomControl {
    const NAME: &'static str = "Windows.Media.Devices.ZoomControl";
}
impl ::core::convert::From<ZoomControl> for ::windows::core::IUnknown {
    fn from(value: ZoomControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ZoomControl> for ::windows::core::IUnknown {
    fn from(value: &ZoomControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ZoomControl> for &::windows::core::IUnknown {
    fn from(value: &ZoomControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ZoomControl> for ::windows::core::IInspectable {
    fn from(value: ZoomControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ZoomControl> for ::windows::core::IInspectable {
    fn from(value: &ZoomControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ZoomControl> for &::windows::core::IInspectable {
    fn from(value: &ZoomControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ZoomSettings(::windows::core::IUnknown);
impl ZoomSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ZoomSettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ZoomTransitionMode>(result__)
        }
    }
    pub fn SetMode(&self, value: ZoomTransitionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ZoomSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ZoomSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ZoomSettings {}
impl ::core::fmt::Debug for ZoomSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ZoomSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ZoomSettings;{6ad66b24-14b4-4bfd-b18f-88fe24463b52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ZoomSettings {
    type Vtable = IZoomSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for ZoomSettings {
    const IID: ::windows::core::GUID = <IZoomSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ZoomSettings {
    const NAME: &'static str = "Windows.Media.Devices.ZoomSettings";
}
impl ::core::convert::From<ZoomSettings> for ::windows::core::IUnknown {
    fn from(value: ZoomSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ZoomSettings> for ::windows::core::IUnknown {
    fn from(value: &ZoomSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ZoomSettings> for &::windows::core::IUnknown {
    fn from(value: &ZoomSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ZoomSettings> for ::windows::core::IInspectable {
    fn from(value: ZoomSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ZoomSettings> for ::windows::core::IInspectable {
    fn from(value: &ZoomSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ZoomSettings> for &::windows::core::IInspectable {
    fn from(value: &ZoomSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ZoomSettings {}
unsafe impl ::core::marker::Sync for ZoomSettings {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Hdr: Self = Self(2i32);
    pub const LowLight: Self = Self(3i32);
}
impl ::core::marker::Copy for AdvancedPhotoMode {}
impl ::core::clone::Clone for AdvancedPhotoMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdvancedPhotoMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdvancedPhotoMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdvancedPhotoMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AdvancedPhotoMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioDeviceRole {}
impl ::core::clone::Clone for AudioDeviceRole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDeviceRole {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioDeviceRole {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioDeviceRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceRole").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AudioDeviceRole;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: Self = Self(0i32);
    pub const Macro: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoFocusRange {}
impl ::core::clone::Clone for AutoFocusRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoFocusRange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoFocusRange {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoFocusRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoFocusRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutoFocusRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AutoFocusRange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: Self = Self(0i32);
    pub const CameraHardware: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraOcclusionKind {}
impl ::core::clone::Clone for CameraOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraOcclusionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraOcclusionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraOcclusionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraOcclusionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: Self = Self(0i32);
    pub const Streaming: Self = Self(1i32);
    pub const BlockedForPrivacy: Self = Self(2i32);
    pub const Shutdown: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraStreamState {}
impl ::core::clone::Clone for CameraStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraStreamState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CameraStreamState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraStreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraStreamState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraStreamState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraStreamState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Macro: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const Sport: Self = Self(4i32);
    pub const Snow: Self = Self(5i32);
    pub const Night: Self = Self(6i32);
    pub const Beach: Self = Self(7i32);
    pub const Sunset: Self = Self(8i32);
    pub const Candlelight: Self = Self(9i32);
    pub const Landscape: Self = Self(10i32);
    pub const NightPortrait: Self = Self(11i32);
    pub const Backlit: Self = Self(12i32);
}
impl ::core::marker::Copy for CaptureSceneMode {}
impl ::core::clone::Clone for CaptureSceneMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CaptureSceneMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CaptureSceneMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CaptureSceneMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaptureSceneMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CaptureSceneMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureSceneMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CaptureUse {}
impl ::core::clone::Clone for CaptureUse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CaptureUse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CaptureUse {
    type Abi = Self;
}
impl ::core::fmt::Debug for CaptureUse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaptureUse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CaptureUse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureUse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Cloudy: Self = Self(2i32);
    pub const Daylight: Self = Self(3i32);
    pub const Flash: Self = Self(4i32);
    pub const Fluorescent: Self = Self(5i32);
    pub const Tungsten: Self = Self(6i32);
    pub const Candlelight: Self = Self(7i32);
}
impl ::core::marker::Copy for ColorTemperaturePreset {}
impl ::core::clone::Clone for ColorTemperaturePreset {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ColorTemperaturePreset {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ColorTemperaturePreset {
    type Abi = Self;
}
impl ::core::fmt::Debug for ColorTemperaturePreset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorTemperaturePreset").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorTemperaturePreset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ColorTemperaturePreset;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for DigitalWindowMode {}
impl ::core::clone::Clone for DigitalWindowMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DigitalWindowMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DigitalWindowMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for DigitalWindowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.DigitalWindowMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Continuous: Self = Self(2i32);
    pub const Manual: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusMode {}
impl ::core::clone::Clone for FocusMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FocusMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FocusMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const AutoMacro: Self = Self(2i32);
    pub const AutoNormal: Self = Self(3i32);
    pub const AutoInfinity: Self = Self(4i32);
    pub const AutoHyperfocal: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusPreset {}
impl ::core::clone::Clone for FocusPreset {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusPreset {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FocusPreset {
    type Abi = Self;
}
impl ::core::fmt::Debug for FocusPreset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusPreset").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusPreset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusPreset;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for HdrVideoMode {}
impl ::core::clone::Clone for HdrVideoMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdrVideoMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdrVideoMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdrVideoMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdrVideoMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdrVideoMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.HdrVideoMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const AlternatingFrameIllumination: Self = Self(2i32);
}
impl ::core::marker::Copy for InfraredTorchMode {}
impl ::core::clone::Clone for InfraredTorchMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InfraredTorchMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InfraredTorchMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InfraredTorchMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InfraredTorchMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InfraredTorchMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.InfraredTorchMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsoSpeedPreset(pub i32);
#[cfg(feature = "deprecated")]
impl IsoSpeedPreset {
    pub const Auto: Self = Self(0i32);
    pub const Iso50: Self = Self(1i32);
    pub const Iso80: Self = Self(2i32);
    pub const Iso100: Self = Self(3i32);
    pub const Iso200: Self = Self(4i32);
    pub const Iso400: Self = Self(5i32);
    pub const Iso800: Self = Self(6i32);
    pub const Iso1600: Self = Self(7i32);
    pub const Iso3200: Self = Self(8i32);
    pub const Iso6400: Self = Self(9i32);
    pub const Iso12800: Self = Self(10i32);
    pub const Iso25600: Self = Self(11i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for IsoSpeedPreset {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IsoSpeedPreset {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for IsoSpeedPreset {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for IsoSpeedPreset {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IsoSpeedPreset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsoSpeedPreset").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for IsoSpeedPreset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.IsoSpeedPreset;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: Self = Self(0i32);
    pub const Hyperfocal: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for ManualFocusDistance {}
impl ::core::clone::Clone for ManualFocusDistance {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManualFocusDistance {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ManualFocusDistance {
    type Abi = Self;
}
impl ::core::fmt::Debug for ManualFocusDistance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManualFocusDistance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManualFocusDistance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ManualFocusDistance;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: Self = Self(0i32);
    pub const Lost: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Focused: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaCaptureFocusState {}
impl ::core::clone::Clone for MediaCaptureFocusState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureFocusState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureFocusState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureFocusState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFocusState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureFocusState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureFocusState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: Self = Self(0i32);
    pub const Quality: Self = Self(1i32);
    pub const Latency: Self = Self(2i32);
    pub const Power: Self = Self(3i32);
    pub const LatencyThenQuality: Self = Self(4i32);
    pub const LatencyThenPower: Self = Self(5i32);
    pub const PowerAndQuality: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaCaptureOptimization {}
impl ::core::clone::Clone for MediaCaptureOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureOptimization {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureOptimization {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureOptimization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureOptimization").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureOptimization {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureOptimization;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: Self = Self(0i32);
    pub const ReleaseHardwareResources: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCapturePauseBehavior {}
impl ::core::clone::Clone for MediaCapturePauseBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCapturePauseBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaCapturePauseBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCapturePauseBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCapturePauseBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCapturePauseBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCapturePauseBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for OpticalImageStabilizationMode {}
impl ::core::clone::Clone for OpticalImageStabilizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OpticalImageStabilizationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OpticalImageStabilizationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for OpticalImageStabilizationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpticalImageStabilizationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OpticalImageStabilizationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.OpticalImageStabilizationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: Self = Self(0i32);
    pub const Face: Self = Self(1i32);
}
impl ::core::marker::Copy for RegionOfInterestType {}
impl ::core::clone::Clone for RegionOfInterestType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RegionOfInterestType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RegionOfInterestType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RegionOfInterestType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegionOfInterestType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RegionOfInterestType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.RegionOfInterestType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for SendCommandStatus {}
impl ::core::clone::Clone for SendCommandStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SendCommandStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SendCommandStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SendCommandStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SendCommandStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SendCommandStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.SendCommandStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
    pub const A: Self = Self(12i32);
    pub const B: Self = Self(13i32);
    pub const C: Self = Self(14i32);
    pub const D: Self = Self(15i32);
}
impl ::core::marker::Copy for TelephonyKey {}
impl ::core::clone::Clone for TelephonyKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TelephonyKey {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TelephonyKey {
    type Abi = Self;
}
impl ::core::fmt::Debug for TelephonyKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TelephonyKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TelephonyKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.TelephonyKey;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoDeviceControllerGetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerGetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const BufferTooSmall: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(5i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for VideoDeviceControllerGetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoDeviceControllerGetDevicePropertyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoDeviceControllerGetDevicePropertyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoDeviceControllerGetDevicePropertyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerGetDevicePropertyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceControllerGetDevicePropertyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for VideoDeviceControllerSetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerSetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoDeviceControllerSetDevicePropertyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoDeviceControllerSetDevicePropertyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoDeviceControllerSetDevicePropertyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerSetDevicePropertyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceControllerSetDevicePropertyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerSetDevicePropertyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoTemporalDenoisingMode {}
impl ::core::clone::Clone for VideoTemporalDenoisingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoTemporalDenoisingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoTemporalDenoisingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoTemporalDenoisingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTemporalDenoisingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTemporalDenoisingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoTemporalDenoisingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: Self = Self(0i32);
    pub const Direct: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl ::core::marker::Copy for ZoomTransitionMode {}
impl ::core::clone::Clone for ZoomTransitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ZoomTransitionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ZoomTransitionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ZoomTransitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomTransitionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ZoomTransitionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ZoomTransitionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CallControlEventHandler(pub ::windows::core::IUnknown);
impl CallControlEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = CallControlEventHandlerBox::<F> { vtable: &CallControlEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, sender: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControl>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), sender.into().abi()).ok() }
    }
}
#[repr(C)]
struct CallControlEventHandlerBox<F: FnMut(&::core::option::Option<CallControl>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const CallControlEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> CallControlEventHandlerBox<F> {
    const VTABLE: CallControlEventHandler_Vtbl = CallControlEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<CallControlEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for CallControlEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CallControlEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallControlEventHandler {}
impl ::core::fmt::Debug for CallControlEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallControlEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for CallControlEventHandler {
    type Vtable = CallControlEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for CallControlEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x596f759f_50df_4454_bc63_4d3d01b61958);
}
unsafe impl ::windows::core::RuntimeType for CallControlEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{596f759f-50df-4454-bc63-4d3d01b61958}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct CallControlEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DialRequestedEventHandler(pub ::windows::core::IUnknown);
impl DialRequestedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<DialRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DialRequestedEventHandlerBox::<F> { vtable: &DialRequestedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, P1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControl>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, DialRequestedEventArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), sender.into().abi(), e.into().abi()).ok() }
    }
}
#[repr(C)]
struct DialRequestedEventHandlerBox<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<DialRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DialRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<DialRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DialRequestedEventHandlerBox<F> {
    const VTABLE: DialRequestedEventHandler_Vtbl = DialRequestedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DialRequestedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DialRequestedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialRequestedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialRequestedEventHandler {}
impl ::core::fmt::Debug for DialRequestedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialRequestedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for DialRequestedEventHandler {
    type Vtable = DialRequestedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for DialRequestedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5abbffdb_c21f_4bc4_891b_257e28c1b1a4);
}
unsafe impl ::windows::core::RuntimeType for DialRequestedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5abbffdb-c21f-4bc4-891b-257e28c1b1a4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DialRequestedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct KeypadPressedEventHandler(pub ::windows::core::IUnknown);
impl KeypadPressedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<KeypadPressedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = KeypadPressedEventHandlerBox::<F> { vtable: &KeypadPressedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, P1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControl>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, KeypadPressedEventArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), sender.into().abi(), e.into().abi()).ok() }
    }
}
#[repr(C)]
struct KeypadPressedEventHandlerBox<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<KeypadPressedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const KeypadPressedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<KeypadPressedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> KeypadPressedEventHandlerBox<F> {
    const VTABLE: KeypadPressedEventHandler_Vtbl = KeypadPressedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<KeypadPressedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for KeypadPressedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeypadPressedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeypadPressedEventHandler {}
impl ::core::fmt::Debug for KeypadPressedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeypadPressedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for KeypadPressedEventHandler {
    type Vtable = KeypadPressedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for KeypadPressedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe637a454_c527_422c_8926_c9af83b559a0);
}
unsafe impl ::windows::core::RuntimeType for KeypadPressedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e637a454-c527-422c-8926-c9af83b559a0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct KeypadPressedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RedialRequestedEventHandler(pub ::windows::core::IUnknown);
impl RedialRequestedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<RedialRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RedialRequestedEventHandlerBox::<F> { vtable: &RedialRequestedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, P1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CallControl>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, RedialRequestedEventArgs>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), sender.into().abi(), e.into().abi()).ok() }
    }
}
#[repr(C)]
struct RedialRequestedEventHandlerBox<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<RedialRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RedialRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<RedialRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> RedialRequestedEventHandlerBox<F> {
    const VTABLE: RedialRequestedEventHandler_Vtbl = RedialRequestedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<RedialRequestedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for RedialRequestedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RedialRequestedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RedialRequestedEventHandler {}
impl ::core::fmt::Debug for RedialRequestedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RedialRequestedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for RedialRequestedEventHandler {
    type Vtable = RedialRequestedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for RedialRequestedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbaf257d1_4ebd_4b84_9f47_6ec43d75d8b1);
}
unsafe impl ::windows::core::RuntimeType for RedialRequestedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{baf257d1-4ebd-4b84-9f47-6ec43d75d8b1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RedialRequestedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
